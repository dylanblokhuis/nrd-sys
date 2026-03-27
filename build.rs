//! Builds [NVIDIA Ray Tracing Denoiser](https://github.com/dylanblokhuis/NVIDIA_RayTracingDenoiser) via CMake.
//!
//! Source tree: git submodule at `third_party/NVIDIA_RayTracingDenoiser`, or set `NRD_SYS_NRD_SOURCE`.
//!
//! Link mode: enable Cargo feature `static` for a static archive; default is shared (`.dylib` / `.so` / `.dll`).

use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").expect("OUT_DIR"));
    let manifest_dir =
        PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR"));

    let nrd_source = nrd_source_dir(&manifest_dir);
    let cmake_lists = nrd_source.join("CMakeLists.txt");
    if !cmake_lists.is_file() {
        panic!(
            "NRD source not found at {} (missing CMakeLists.txt).\n\
             Initialize the submodule:\n\
               git submodule update --init --recursive third_party/NVIDIA_RayTracingDenoiser\n\
             Or set NRD_SYS_NRD_SOURCE to the repository root.",
            nrd_source.display()
        );
    }

    let build_static = env::var_os("CARGO_FEATURE_STATIC").is_some();
    let out_display = out_dir.to_str().expect("OUT_DIR utf-8");
    // Keep generated shaders under OUT_DIR so incremental CMake runs do not clash with stale
    // files in the source tree `third_party/.../_Shaders`.
    let shaders_out = out_dir.join("nrd_shaders");
    fs::create_dir_all(&shaders_out).expect("create nrd_shaders");
    let shaders_display = shaders_out.to_str().expect("nrd_shaders path utf-8");

    println!("cargo:rerun-if-env-changed=NRD_SYS_NRD_SOURCE");
    println!("cargo:rerun-if-changed={}", cmake_lists.display());

    let target = env::var("TARGET").unwrap_or_default();
    let target_is_apple = target.contains("apple");

    let mut config = cmake::Config::new(&nrd_source);
    config
        .out_dir(&out_dir)
        .profile("Release")
        .define("NRD_SHADERS_PATH", shaders_display)
        .define(
            "NRD_STATIC_LIBRARY",
            if build_static { "ON" } else { "OFF" },
        )
        .define("CMAKE_RUNTIME_OUTPUT_DIRECTORY", out_display)
        .define("CMAKE_LIBRARY_OUTPUT_DIRECTORY", out_display)
        .define("CMAKE_ARCHIVE_OUTPUT_DIRECTORY", out_display)
        // NRD does not define an `install` CMake target; build the library target explicitly.
        .build_target("NRD");

    if target_is_apple {
        config.define("NRD_EMBEDS_METAL_SHADERS", "ON");
    }

    let _dst = config.build();

    emit_link_lines(&out_dir, build_static, &target);
}

fn nrd_source_dir(manifest_dir: &Path) -> PathBuf {
    env::var_os("NRD_SYS_NRD_SOURCE")
        .map(PathBuf::from)
        .unwrap_or_else(|| manifest_dir.join("third_party/NVIDIA_RayTracingDenoiser"))
}

fn emit_link_lines(out_dir: &Path, build_static: bool, target: &str) {
    if build_static {
        println!("cargo:rustc-link-lib=static=NRD");
        println!("cargo:rustc-link-search=native={}", out_dir.display());
        if target.contains("apple") {
            println!("cargo:rustc-link-lib=c++");
        } else if target.contains("linux") || target.contains("android") {
            println!("cargo:rustc-link-lib=stdc++");
        }
    } else if target.contains("windows") {
        println!("cargo:rustc-link-lib=NRD");
        println!("cargo:rustc-link-search=native={}", out_dir.display());
    } else {
        println!("cargo:rustc-link-lib=dylib=NRD");
        println!("cargo:rustc-link-search=native={}", out_dir.display());

        if target.contains("apple") {
            println!("cargo:rustc-link-arg=-Wl,-rpath,{}", out_dir.display());
        } else if target.contains("linux") {
            println!("cargo:rustc-link-arg=-Wl,-rpath,{}", out_dir.display());
        }
    }
}
