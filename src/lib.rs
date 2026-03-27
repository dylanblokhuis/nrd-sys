//! Rust bindings for [NVIDIA NRD](https://github.com/NVIDIAGameWorks/RayTracingDenoiser).
//!
//! - [`ffi`] — raw `bindgen` output (`nrd_*` types and functions).
//! - Everything else at the crate root is the safe-ish API in [`api`] (re-exported for convenience).

pub mod ffi;

pub mod api;

pub use api::*;

pub use ffi::{
    NRD_VERSION_BUILD, NRD_VERSION_DATE, NRD_VERSION_MAJOR, NRD_VERSION_MINOR,
};
