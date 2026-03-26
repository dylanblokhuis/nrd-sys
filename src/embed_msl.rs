//! Build-time embedded Metal Shading Language for NRD pipelines.
//!
//! Enable the `embed-msl` feature and build for `target_os = "macos"`. The build script runs
//! `spirv-cross` on SPIR-V files under `embed/spirv/` (see [`export_spirv_for_embed`] example).
//!
//! At runtime, use [`InstanceDescription::pipelines_with_msl`](crate::InstanceDescription::pipelines_with_msl)
//! to pair each [`ffi::nrd_PipelineDesc`] with optional [`MslShaderDesc`].

use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct MslShaderDesc {
    pub source: &'static str,
}

impl MslShaderDesc {
    #[inline]
    pub fn size(self) -> usize {
        self.source.len()
    }
}

impl fmt::Debug for MslShaderDesc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MslShaderDesc")
            .field("size", &self.source.len())
            .finish_non_exhaustive()
    }
}

mod _generated {
    include!(concat!(env!("OUT_DIR"), "/embedded_msl.rs"));
}

#[inline]
pub fn msl_shader_for_pipeline(index: usize) -> Option<MslShaderDesc> {
    _generated::msl_shader_for_pipeline(index)
}

/// Number of pipelines with embedded MSL (matches `embed/spirv/*.spv` count when `embed-msl` is active).
pub const EMBEDDED_PIPELINE_COUNT: usize = _generated::EMBEDDED_PIPELINE_COUNT;
