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

    println!("cargo:rustc-link-lib=NRD");
    println!(
        "cargo:rustc-link-search={}",
        std::env::var("OUT_DIR").unwrap()
    );
    println!("cargo:rerun-if-changed=Include/NRD.h");
    println!("cargo:rerun-if-changed=Include/libNRD.dylib");
    println!("cargo:rerun-if-changed=Include/libNRD.so");
    // println!("cargo:rerun-if-changed=Include/NRD.dll");
}
