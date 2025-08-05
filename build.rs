// use std::{ffi::OsString, io::Write};

// fn get_install_path() -> impl Iterator<Item = (&'static str, OsString)> {
//     get_download_name().iter().map(|download_name| {
//         let mut path: std::path::PathBuf = std::env::var("OUT_DIR").unwrap().into();
//         path.push(download_name);
//         (*download_name, path.into_os_string())
//     })
// }

// fn get_download_name() -> &'static [&'static str] {
//     #[cfg(target_family = "unix")]
//     {
//         return &["libNRD.so"];
//     }
//     #[cfg(target_family = "windows")]
//     {
//         return &["NRD.lib", "NRD.dll"];
//     }
//     #[allow(unreachable_code)]
//     {
//         panic!()
//     }
// }

fn main() {
    // for (download_name, install_path) in get_install_path() {
    //     if !std::fs::exists(&install_path).expect("Unable to check library file location") {
    //         let data = sysreq::get(format!(
    //             "https://github.com/dust-engine/nrd-sys/releases/download/v0.2/{}",
    //             download_name
    //         ))
    //         .expect("Download file error");
    //         let mut file =
    //             std::fs::File::create(install_path).expect("Unable to create library file");
    //         file.write_all(&data).expect("Unable to write library file");
    //     }
    // }

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

    bindgen::Builder::default()
        .header("Include/NRD.h")
        .clang_arg("-IInclude")
        .clang_arg("-xc++")
        .clang_arg("-std=c++14")
        .clang_arg("-stdlib=libc++")
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file("src/ffi.rs")
        .expect("Couldn't write bindings!");

    println!("cargo:rustc-link-lib=NRD");
    println!(
        "cargo:rustc-link-search={}",
        std::env::var("OUT_DIR").unwrap()
    );
}
