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
    // ./Include/NRD.h

    bindgen::Builder::default()
        .header("Include/NRD.h")
        .clang_arg("-IInclude")
        .clang_arg("-xc++")
        .clang_arg("-std=c++14")
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file("src/ffi.rs")
        .expect("Couldn't write bindings!");

    println!("cargo:rustc-link-lib=NRD");
    // println!(
    //     "cargo:rustc-link-search={}",
    //     std::env::var("OUT_DIR").unwrap()
    // );
}
