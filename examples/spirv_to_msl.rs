//! Dumps NRD embedded SPIR-V to temporary `.spv` files and runs the `spirv-cross` CLI to emit MSL.
//!
//! Requirements: `spirv-cross` on `PATH`, or set `SPIRV_CROSS` to the full binary path.
//!
//! Environment:
//! - `SPIRV_CROSS` — path to the spirv-cross executable (default: `spirv-cross`)
//! - `NRD_MSL_OUT` — output directory for `.metal` files (default: `target/nrd_msl`)
//! - `NRD_MSL_VERSION` — spirv-cross `--msl-version` value, e.g. `20300` for MSL 2.3 (default: `20300`)

use std::ffi::c_char;
use std::path::{Path, PathBuf};
use std::process::Command;

fn spirv_cross_executable() -> PathBuf {
    std::env::var_os("SPIRV_CROSS")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("spirv-cross"))
}

fn msl_output_dir() -> PathBuf {
    std::env::var_os("NRD_MSL_OUT")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("target/nrd_msl"))
}

/// spirv-cross `--msl-version` (MMmmpp, e.g. 20300 → MSL 2.3). Subgroup ops need ≥ 2.0.
fn msl_version_flag() -> String {
    std::env::var("NRD_MSL_VERSION").unwrap_or_else(|_| "20300".to_string())
}

fn shader_identifier_string(id: &[c_char; 256]) -> String {
    let bytes =
        unsafe { std::slice::from_raw_parts(id.as_ptr() as *const u8, id.len()) };
    let end = bytes.iter().position(|&b| b == 0).unwrap_or(bytes.len());
    let s = String::from_utf8_lossy(&bytes[..end]);
    sanitize_filename(&s)
}

fn sanitize_filename(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            '/' | '\\' | ':' | '|' | '<' | '>' | '"' | '?' | '*' | '\0' => '_',
            c if c.is_control() => '_',
            c => c,
        })
        .collect()
}

fn run_spirv_cross(
    exe: &Path,
    spv_path: &Path,
    out_metal: &Path,
    entry: Option<&str>,
    msl_version: &str,
) -> Result<(), String> {
    let mut cmd = Command::new(exe);
    cmd.arg(spv_path)
        .arg("--msl")
        .arg("--msl-version")
        .arg(msl_version)
        .arg("--stage")
        .arg("comp");
    if let Some(e) = entry {
        cmd.arg("--entry").arg(e);
    }
    cmd.arg("--output").arg(out_metal);
    let status = cmd.status().map_err(|e| {
        format!(
            "failed to spawn spirv-cross ({exe:?}): {e}; install SPIRV-Cross or set SPIRV_CROSS"
        )
    })?;
    if !status.success() {
        return Err(format!(
            "spirv-cross exited with {status} for {}",
            spv_path.display()
        ));
    }
    Ok(())
}

fn main() {
    let exe = spirv_cross_executable();
    let msl_version = msl_version_flag();
    let out_dir = msl_output_dir();
    std::fs::create_dir_all(&out_dir).unwrap_or_else(|e| {
        panic!("create output dir {}: {e}", out_dir.display());
    });

    let _lib = nrd_sys::LibraryInfo::query().expect(
        "linked libNRD major.minor must match this crate's headers; update Include/libNRD or regenerate ffi",
    );

    let instance = nrd_sys::Instance::try_new_denoisers(&[nrd_sys::DenoiserSlot {
        identifier: nrd_sys::Identifier(0),
        denoiser: nrd_sys::Denoiser::ReblurDiffuse,
    }])
    .expect("Create NRD instance");

    let inst_desc = instance.description().expect("instance description");
    let entry = inst_desc
        .shader_entry_point()
        .and_then(|cs| cs.to_str().ok());

    let tmp = std::env::temp_dir().join(format!(
        "nrd-sys-spirv-cross-{}",
        std::process::id()
    ));
    std::fs::create_dir_all(&tmp).expect("temp dir for .spv");

    for (i, pipeline) in inst_desc.pipelines().iter().enumerate() {
        let spirv = unsafe {
            std::slice::from_raw_parts(
                pipeline.computeShaderSPIRV.bytecode as *const u8,
                pipeline.computeShaderSPIRV.size as usize,
            )
        };
        if spirv.is_empty() {
            eprintln!("pipeline {i}: empty SPIR-V, skip");
            continue;
        }

        let base = shader_identifier_string(&pipeline.shaderIdentifier);
        let name = if base.is_empty() {
            format!("pipeline_{i}")
        } else {
            format!("{i:03}_{base}")
        };

        let spv_path = tmp.join(format!("{name}.spv"));
        std::fs::write(&spv_path, spirv).expect("write temp spv");

        let metal_path = out_dir.join(format!("{name}.metal"));
        println!("{} -> {}", spv_path.display(), metal_path.display());
        if let Err(e) = run_spirv_cross(&exe, &spv_path, &metal_path, entry, &msl_version) {
            eprintln!("error: {e}");
            std::process::exit(1);
        }
    }

    let _ = std::fs::remove_dir_all(&tmp);
    println!("MSL written under {}", out_dir.display());
}
