//! Writes each pipeline's SPIR-V from libNRD into `embed/spirv/` for the `embed-msl` build feature.
//!
//! Run from the crate root after updating NRD:
//! `cargo run --example export_spirv_for_embed`
//!
//! Filenames are `000.spv`, `001.spv`, … in pipeline order. Also writes `entry_point.txt` (shader
//! entry name for spirv-cross). Keep this in sync with the denoisers you enable in `build.rs` /
//! `embed-msl` (default: single `ReblurDiffuse` slot).

use std::path::PathBuf;

fn main() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let out_dir = root.join("embed/spirv");
    std::fs::create_dir_all(&out_dir).expect("create embed/spirv");

    let _lib = nrd_sys::LibraryInfo::query().expect("LibraryInfo::query");

    let instance = nrd_sys::Instance::try_new_denoisers(&[nrd_sys::DenoiserSlot {
        identifier: nrd_sys::Identifier(0),
        denoiser: nrd_sys::Denoiser::ReblurDiffuse,
    }])
    .expect("Create NRD instance");

    let inst_desc = instance.description().expect("instance description");
    let entry = inst_desc
        .shader_entry_point()
        .and_then(|cs| cs.to_str().ok())
        .unwrap_or("main");

    std::fs::write(out_dir.join("entry_point.txt"), entry).expect("write entry_point.txt");

    for (i, pipeline) in inst_desc.pipelines().iter().enumerate() {
        let spirv = unsafe {
            std::slice::from_raw_parts(
                pipeline.computeShaderSPIRV.bytecode as *const u8,
                pipeline.computeShaderSPIRV.size as usize,
            )
        };
        if spirv.is_empty() {
            eprintln!("pipeline {i}: empty SPIR-V, skipping");
            continue;
        }
        let path = out_dir.join(format!("{i:03}.spv"));
        std::fs::write(&path, spirv).expect("write .spv");
        println!("{}", path.display());
    }

    println!(
        "Exported {} pipelines under {} (entry point: {entry})",
        inst_desc.pipelines().len(),
        out_dir.display()
    );
}
