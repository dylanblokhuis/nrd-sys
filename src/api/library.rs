use crate::api::{Denoiser, Error, NormalEncoding, RoughnessEncoding};
use crate::ffi;

/// Snapshot of [`ffi::nrd_LibraryDesc`] with owned `supported_denoisers` list.
#[derive(Debug, Clone)]
pub struct LibraryInfo {
    pub spirv_binding_offsets: ffi::nrd_SPIRVBindingOffsets,
    pub supported_denoisers: Vec<Denoiser>,
    pub version_major: u8,
    pub version_minor: u8,
    pub version_build: u8,
    pub normal_encoding: NormalEncoding,
    pub roughness_encoding: RoughnessEncoding,
}

impl LibraryInfo {
    /// Loads library metadata and ensures linked `(major, minor)` matches this crate’s `NRD_VERSION_*` (build may differ).
    pub fn query() -> Result<Self, Error> {
        let desc = unsafe { ffi::nrd_GetLibraryDesc() };
        if desc.is_null() {
            return Err(Error::Failure);
        }
        unsafe {
            if (*desc).versionMajor as u32 != ffi::NRD_VERSION_MAJOR
                || (*desc).versionMinor as u32 != ffi::NRD_VERSION_MINOR
            {
                return Err(Error::VersionMismatch {
                    expected_major: ffi::NRD_VERSION_MAJOR,
                    expected_minor: ffi::NRD_VERSION_MINOR,
                    linked_major: (*desc).versionMajor,
                    linked_minor: (*desc).versionMinor,
                    linked_build: (*desc).versionBuild,
                });
            }

            Ok(Self::from_ptr(desc))
        }
    }

    /// Reads metadata from the loaded NRD library without comparing versions.
    ///
    /// Prefer [`LibraryInfo::query`]. Use this only when you intentionally mix header and binary versions
    /// and accept ABI risk.
    pub fn linked_unchecked() -> Result<Self, Error> {
        let desc = unsafe { ffi::nrd_GetLibraryDesc() };
        if desc.is_null() {
            return Err(Error::Failure);
        }
        unsafe { Ok(Self::from_ptr(desc)) }
    }

    unsafe fn from_ptr(desc: *const ffi::nrd_LibraryDesc) -> Self {
        let supported = std::slice::from_raw_parts(
            (*desc).supportedDenoisers as *const Denoiser,
            (*desc).supportedDenoisersNum as usize,
        )
        .to_vec();

        Self {
            spirv_binding_offsets: (*desc).spirvBindingOffsets,
            supported_denoisers: supported,
            version_major: (*desc).versionMajor,
            version_minor: (*desc).versionMinor,
            version_build: (*desc).versionBuild,
            normal_encoding: std::mem::transmute((*desc).normalEncoding),
            roughness_encoding: std::mem::transmute((*desc).roughnessEncoding),
        }
    }
}
