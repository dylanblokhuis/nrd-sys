use std::path::PathBuf;
use std::process::Command;

fn main() {
    // copy NRD.dylib to OUT_DIR
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let nrd_lib_path = std::path::Path::new(&out_dir).join("libNRD.dylib");
    if !nrd_lib_path.exists() {
        #[cfg(target_os = "macos")]
        {
            std::fs::copy("Include/libNRD.dylib", &nrd_lib_path)
                .expect("Failed to copy libNRD.dylib to OUT_DIR");
        }
        #[cfg(target_os = "linux")]
        {
            let so_path = std::path::Path::new(&out_dir).join("libNRD.so");
            std::fs::copy("Include/libNRD.so", &so_path)
                .expect("Failed to copy libNRD.so to OUT_DIR");
        }
        #[cfg(target_os = "windows")]
        {
            let dll_path = std::path::Path::new(&out_dir).join("NRD.dll");
            std::fs::copy("Include/NRD.dll", &dll_path).expect("Failed to copy NRD.dll to OUT_DIR");
        }
    }

    // bindgen::Builder::default()
    //     .header("Include/NRD.h")
    //     .clang_arg("-IInclude")
    //     .clang_arg("-xc++")
    //     .clang_arg("-std=c++14")
    //     .clang_arg("-stdlib=libc++")
    //     .generate()
    //     .expect("Unable to generate bindings")
    //     .write_to_file("src/ffi.rs")
    //     .expect("Couldn't write bindings!");

    println!("cargo:rustc-link-lib=NRD");
    println!(
        "cargo:rustc-link-search={}",
        std::env::var("OUT_DIR").unwrap()
    );
    println!("cargo:rerun-if-changed=Include/NRD.h");
    println!("cargo:rerun-if-changed=Include/libNRD.dylib");
    println!("cargo:rerun-if-changed=Include/libNRD.so");
    println!("cargo:rerun-if-changed=embed/spirv");
    // println!("cargo:rerun-if-changed=Include/NRD.dll");

    write_embedded_msl_rust();
}

fn write_embedded_msl_rust() {
    let out_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let gen_path = out_dir.join("embedded_msl.rs");
    let msl_dir = out_dir.join("msl");
    let _ = std::fs::remove_dir_all(&msl_dir);

    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    let embed_msl = std::env::var("CARGO_FEATURE_EMBED_MSL").is_ok();

    if !embed_msl || target_os != "macos" {
        std::fs::write(
            &gen_path,
            r#"pub fn msl_shader_for_pipeline(_index: usize) -> Option<super::MslShaderDesc> {
    None
}
pub const EMBEDDED_PIPELINE_COUNT: usize = 0;
"#,
        )
        .expect("write stub embedded_msl.rs");
        return;
    }

    let manifest_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let spirv_dir = manifest_dir.join("embed/spirv");
    let mut spv_files: Vec<PathBuf> = std::fs::read_dir(&spirv_dir)
        .unwrap_or_else(|e| {
            panic!(
                "embed-msl: read {}: {e}. Run `cargo run --example export_spirv_for_embed` first.",
                spirv_dir.display()
            )
        })
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|x| x == "spv"))
        .collect();
    spv_files.sort();
    if spv_files.is_empty() {
        panic!(
            "embed-msl: no .spv files in {}. Run `cargo run --example export_spirv_for_embed`.",
            spirv_dir.display()
        );
    }

    std::fs::create_dir_all(&msl_dir).expect("create OUT_DIR/msl");

    let spirv_cross = std::env::var_os("SPIRV_CROSS")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("spirv-cross"));
    let msl_version = std::env::var("NRD_MSL_VERSION").unwrap_or_else(|_| "20300".into());
    let entry = std::fs::read_to_string(spirv_dir.join("entry_point.txt"))
        .unwrap_or_else(|_| "NRD_CS_MAIN".to_string());
    let entry = entry
        .lines()
        .next()
        .unwrap_or("NRD_CS_MAIN")
        .trim()
        .to_string();
    if entry.is_empty() {
        panic!("embed/spirv/entry_point.txt is empty");
    }

    for (i, spv) in spv_files.iter().enumerate() {
        let metal_path = msl_dir.join(format!("{i:03}.metal"));
        let status = Command::new(&spirv_cross)
            .arg(spv)
            .arg("--msl")
            .arg("--msl-version")
            .arg(&msl_version)
            .arg("--stage")
            .arg("comp")
            .arg("--entry")
            .arg(&entry)
            .arg("--output")
            .arg(&metal_path)
            .status()
            .unwrap_or_else(|e| {
                panic!(
                    "embed-msl: failed to run spirv-cross ({spirv_cross:?}): {e}. Install SPIRV-Cross or set SPIRV_CROSS."
                )
            });
        if !status.success() {
            panic!(
                "embed-msl: spirv-cross failed for {} (status {status})",
                spv.display()
            );
        }
    }

    let n = spv_files.len();
    let mut lines = String::new();
    for i in 0..n {
        lines.push_str(&format!(
            "static MSL_{i}: &str = include_str!(concat!(env!(\"OUT_DIR\"), \"/msl/{i:03}.metal\"));\n"
        ));
    }
    lines.push_str(
        "pub fn msl_shader_for_pipeline(index: usize) -> Option<super::MslShaderDesc> {\n",
    );
    lines.push_str("    let source = match index {\n");
    for i in 0..n {
        lines.push_str(&format!("        {i} => MSL_{i},\n"));
    }
    lines.push_str("        _ => return None,\n");
    lines.push_str("    };\n");
    lines.push_str("    Some(super::MslShaderDesc { source })\n");
    lines.push_str("}\n");
    lines.push_str(&format!(
        "pub const EMBEDDED_PIPELINE_COUNT: usize = {n};\n"
    ));

    std::fs::write(&gen_path, lines).expect("write embedded_msl.rs");
}
