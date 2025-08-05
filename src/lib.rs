pub mod ffi;

use std::{
    ffi::{c_void, CStr},
    fmt::Debug,
    ops::Deref,
    ptr,
};

#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum Sampler {
    NearestClamp = ffi::nrd_Sampler_NEAREST_CLAMP,
    LinearClamp = ffi::nrd_Sampler_LINEAR_CLAMP,
    MaxNum = ffi::nrd_Sampler_MAX_NUM,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum NormalEncoding {
    Rgba8Unorm = ffi::nrd_NormalEncoding_RGBA8_UNORM,
    Rgba8Snorm = ffi::nrd_NormalEncoding_RGBA8_SNORM,
    R10G10B10A2Unorm = ffi::nrd_NormalEncoding_R10_G10_B10_A2_UNORM,
    Rgba16Unorm = ffi::nrd_NormalEncoding_RGBA16_UNORM,
    Rgba16Snorm = ffi::nrd_NormalEncoding_RGBA16_SNORM,
    MaxNum = ffi::nrd_NormalEncoding_MAX_NUM,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Result {
    Success = ffi::nrd_Result_SUCCESS,
    Failure = ffi::nrd_Result_FAILURE,
    InvalidArgument = ffi::nrd_Result_INVALID_ARGUMENT,
    Unsupported = ffi::nrd_Result_UNSUPPORTED,
    NonUniqueIdentifier = ffi::nrd_Result_NON_UNIQUE_IDENTIFIER,
    MaxNum = ffi::nrd_Result_MAX_NUM,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResourceType {
    InMv = ffi::nrd_ResourceType_IN_MV,
    InNormalRoughness = ffi::nrd_ResourceType_IN_NORMAL_ROUGHNESS,
    InViewz = ffi::nrd_ResourceType_IN_VIEWZ,
    InDiffConfidence = ffi::nrd_ResourceType_IN_DIFF_CONFIDENCE,
    InSpecConfidence = ffi::nrd_ResourceType_IN_SPEC_CONFIDENCE,
    InDisocclusionThresholdMix = ffi::nrd_ResourceType_IN_DISOCCLUSION_THRESHOLD_MIX,
    InBasecolorMetalness = ffi::nrd_ResourceType_IN_BASECOLOR_METALNESS,
    InDiffRadianceHitdist = ffi::nrd_ResourceType_IN_DIFF_RADIANCE_HITDIST,
    InSpecRadianceHitdist = ffi::nrd_ResourceType_IN_SPEC_RADIANCE_HITDIST,
    InDiffHitdist = ffi::nrd_ResourceType_IN_DIFF_HITDIST,
    InSpecHitdist = ffi::nrd_ResourceType_IN_SPEC_HITDIST,
    InDiffDirectionHitdist = ffi::nrd_ResourceType_IN_DIFF_DIRECTION_HITDIST,
    InDiffSh0 = ffi::nrd_ResourceType_IN_DIFF_SH0,
    InDiffSh1 = ffi::nrd_ResourceType_IN_DIFF_SH1,
    InSpecSh0 = ffi::nrd_ResourceType_IN_SPEC_SH0,
    InSpecSh1 = ffi::nrd_ResourceType_IN_SPEC_SH1,
    InPenumbra = ffi::nrd_ResourceType_IN_PENUMBRA,
    InTranslucency = ffi::nrd_ResourceType_IN_TRANSLUCENCY,
    InSignal = ffi::nrd_ResourceType_IN_SIGNAL,
    OutDiffRadianceHitdist = ffi::nrd_ResourceType_OUT_DIFF_RADIANCE_HITDIST,
    OutSpecRadianceHitdist = ffi::nrd_ResourceType_OUT_SPEC_RADIANCE_HITDIST,
    OutDiffSh0 = ffi::nrd_ResourceType_OUT_DIFF_SH0,
    OutDiffSh1 = ffi::nrd_ResourceType_OUT_DIFF_SH1,
    OutSpecSh0 = ffi::nrd_ResourceType_OUT_SPEC_SH0,
    OutSpecSh1 = ffi::nrd_ResourceType_OUT_SPEC_SH1,
    OutDiffHitdist = ffi::nrd_ResourceType_OUT_DIFF_HITDIST,
    OutSpecHitdist = ffi::nrd_ResourceType_OUT_SPEC_HITDIST,
    OutDiffDirectionHitdist = ffi::nrd_ResourceType_OUT_DIFF_DIRECTION_HITDIST,
    OutShadowTranslucency = ffi::nrd_ResourceType_OUT_SHADOW_TRANSLUCENCY,
    OutSignal = ffi::nrd_ResourceType_OUT_SIGNAL,
    OutValidation = ffi::nrd_ResourceType_OUT_VALIDATION,
    TransientPool = ffi::nrd_ResourceType_TRANSIENT_POOL,
    PermanentPool = ffi::nrd_ResourceType_PERMANENT_POOL,
    MaxNum = ffi::nrd_ResourceType_MAX_NUM,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Denoiser {
    ReblurDiffuse = ffi::nrd_Denoiser_REBLUR_DIFFUSE,
    ReblurDiffuseOcclusion = ffi::nrd_Denoiser_REBLUR_DIFFUSE_OCCLUSION,
    ReblurDiffuseSh = ffi::nrd_Denoiser_REBLUR_DIFFUSE_SH,
    ReblurSpecular = ffi::nrd_Denoiser_REBLUR_SPECULAR,
    ReblurSpecularOcclusion = ffi::nrd_Denoiser_REBLUR_SPECULAR_OCCLUSION,
    ReblurSpecularSh = ffi::nrd_Denoiser_REBLUR_SPECULAR_SH,
    ReblurDiffuseSpecular = ffi::nrd_Denoiser_REBLUR_DIFFUSE_SPECULAR,
    ReblurDiffuseSpecularOcclusion = ffi::nrd_Denoiser_REBLUR_DIFFUSE_SPECULAR_OCCLUSION,
    ReblurDiffuseSpecularSh = ffi::nrd_Denoiser_REBLUR_DIFFUSE_SPECULAR_SH,
    ReblurDiffuseDirectionalOcclusion = ffi::nrd_Denoiser_REBLUR_DIFFUSE_DIRECTIONAL_OCCLUSION,
    RelaxDiffuse = ffi::nrd_Denoiser_RELAX_DIFFUSE,
    RelaxDiffuseSh = ffi::nrd_Denoiser_RELAX_DIFFUSE_SH,
    RelaxSpecular = ffi::nrd_Denoiser_RELAX_SPECULAR,
    RelaxSpecularSh = ffi::nrd_Denoiser_RELAX_SPECULAR_SH,
    RelaxDiffuseSpecular = ffi::nrd_Denoiser_RELAX_DIFFUSE_SPECULAR,
    RelaxDiffuseSpecularSh = ffi::nrd_Denoiser_RELAX_DIFFUSE_SPECULAR_SH,
    SigmaShadow = ffi::nrd_Denoiser_SIGMA_SHADOW,
    SigmaShadowTranslucency = ffi::nrd_Denoiser_SIGMA_SHADOW_TRANSLUCENCY,
    Reference = ffi::nrd_Denoiser_REFERENCE,
    MaxNum = ffi::nrd_Denoiser_MAX_NUM,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Format {
    R8Unorm = ffi::nrd_Format_R8_UNORM,
    R8Snorm = ffi::nrd_Format_R8_SNORM,
    R8Uint = ffi::nrd_Format_R8_UINT,
    R8Sint = ffi::nrd_Format_R8_SINT,
    Rg8Unorm = ffi::nrd_Format_RG8_UNORM,
    Rg8Snorm = ffi::nrd_Format_RG8_SNORM,
    Rg8Uint = ffi::nrd_Format_RG8_UINT,
    Rg8Sint = ffi::nrd_Format_RG8_SINT,
    Rgba8Unorm = ffi::nrd_Format_RGBA8_UNORM,
    Rgba8Snorm = ffi::nrd_Format_RGBA8_SNORM,
    Rgba8Uint = ffi::nrd_Format_RGBA8_UINT,
    Rgba8Sint = ffi::nrd_Format_RGBA8_SINT,
    Rgba8Srgb = ffi::nrd_Format_RGBA8_SRGB,
    R16Unorm = ffi::nrd_Format_R16_UNORM,
    R16Snorm = ffi::nrd_Format_R16_SNORM,
    R16Uint = ffi::nrd_Format_R16_UINT,
    R16Sint = ffi::nrd_Format_R16_SINT,
    R16Sfloat = ffi::nrd_Format_R16_SFLOAT,
    Rg16Unorm = ffi::nrd_Format_RG16_UNORM,
    Rg16Snorm = ffi::nrd_Format_RG16_SNORM,
    Rg16Uint = ffi::nrd_Format_RG16_UINT,
    Rg16Sint = ffi::nrd_Format_RG16_SINT,
    Rg16Sfloat = ffi::nrd_Format_RG16_SFLOAT,
    Rgba16Unorm = ffi::nrd_Format_RGBA16_UNORM,
    Rgba16Snorm = ffi::nrd_Format_RGBA16_SNORM,
    Rgba16Uint = ffi::nrd_Format_RGBA16_UINT,
    Rgba16Sint = ffi::nrd_Format_RGBA16_SINT,
    Rgba16Sfloat = ffi::nrd_Format_RGBA16_SFLOAT,
    R32Uint = ffi::nrd_Format_R32_UINT,
    R32Sint = ffi::nrd_Format_R32_SINT,
    R32Sfloat = ffi::nrd_Format_R32_SFLOAT,
    Rg32Uint = ffi::nrd_Format_RG32_UINT,
    Rg32Sint = ffi::nrd_Format_RG32_SINT,
    Rg32Sfloat = ffi::nrd_Format_RG32_SFLOAT,
    Rgb32Uint = ffi::nrd_Format_RGB32_UINT,
    Rgb32Sint = ffi::nrd_Format_RGB32_SINT,
    Rgb32Sfloat = ffi::nrd_Format_RGB32_SFLOAT,
    Rgba32Uint = ffi::nrd_Format_RGBA32_UINT,
    Rgba32Sint = ffi::nrd_Format_RGBA32_SINT,
    Rgba32Sfloat = ffi::nrd_Format_RGBA32_SFLOAT,
    R10G10B10A2Unorm = ffi::nrd_Format_R10_G10_B10_A2_UNORM,
    R10G10B10A2Uint = ffi::nrd_Format_R10_G10_B10_A2_UINT,
    R11G11B10Ufloat = ffi::nrd_Format_R11_G11_B10_UFLOAT,
    R9G9B9E5Ufloat = ffi::nrd_Format_R9_G9_B9_E5_UFLOAT,
    MaxNum = ffi::nrd_Format_MAX_NUM,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DescriptorType {
    Texture = ffi::nrd_DescriptorType_TEXTURE,
    StorageTexture = ffi::nrd_DescriptorType_STORAGE_TEXTURE,
    MaxNum = ffi::nrd_DescriptorType_MAX_NUM,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RoughnessEncoding {
    SqLinear = ffi::nrd_RoughnessEncoding_SQ_LINEAR,
    Linear = ffi::nrd_RoughnessEncoding_LINEAR,
    SqrtLinear = ffi::nrd_RoughnessEncoding_SQRT_LINEAR,
    MaxNum = ffi::nrd_RoughnessEncoding_MAX_NUM,
}

#[derive(Debug, Clone, Copy)]
pub struct LibraryDesc(*const ffi::nrd_LibraryDesc);

impl LibraryDesc {
    pub fn new() -> Self {
        let desc = unsafe { ffi::nrd_GetLibraryDesc() };
        Self(desc)
    }

    pub fn spirv_binding_offsets(&self) -> &ffi::nrd_SPIRVBindingOffsets {
        unsafe { &(*self.0).spirvBindingOffsets }
    }

    pub fn supported_denoisers(&self) -> &[Denoiser] {
        unsafe {
            std::slice::from_raw_parts(
                (*self.0).supportedDenoisers as *const Denoiser,
                (*self.0).supportedDenoisersNum as usize,
            )
        }
    }

    pub fn version_major(&self) -> u8 {
        unsafe { (*self.0).versionMajor }
    }

    pub fn version_minor(&self) -> u8 {
        unsafe { (*self.0).versionMinor }
    }

    pub fn version_build(&self) -> u8 {
        unsafe { (*self.0).versionBuild }
    }

    pub fn normal_encoding(&self) -> NormalEncoding {
        unsafe { std::mem::transmute((*self.0).normalEncoding) }
    }

    pub fn roughness_encoding(&self) -> RoughnessEncoding {
        unsafe { std::mem::transmute((*self.0).roughnessEncoding) }
    }

    pub fn create_instance(&self, desc: &ffi::nrd_InstanceCreationDesc) -> Instance {
        let mut instance = ptr::null_mut();
        check_result(unsafe { ffi::nrd_CreateInstance(desc, &mut instance) });
        Instance(instance)
    }
}

fn check_result(result: u32) {
    let result = unsafe { std::mem::transmute::<u32, Result>(result) };
    if result != Result::Success {
        panic!("NRD operation failed with result: {:?}", result);
    }
}

pub struct Instance(*mut ffi::nrd_Instance);

impl Instance {
    pub fn get_desc(&self) -> ffi::nrd_InstanceDesc {
        let desc = unsafe { ffi::nrd_GetInstanceDesc(self.0) };
        if desc.is_null() {
            panic!("Failed to get instance description");
        }
        unsafe { *desc }
    }

    pub fn get_default_common_settings(&self) -> ffi::nrd_CommonSettings {
        ffi::nrd_CommonSettings {
            viewToClipMatrix: [0.0; 16],
            viewToClipMatrixPrev: [0.0; 16],
            worldToViewMatrix: [0.0; 16],
            worldToViewMatrixPrev: [0.0; 16],
            worldPrevToWorldMatrix: [
                1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
            ],
            motionVectorScale: [1.0, 1.0, 0.0],
            cameraJitter: [0.0; 2],
            cameraJitterPrev: [0.0; 2],
            resourceSize: [0; 2],
            resourceSizePrev: [0; 2],
            rectSize: [0; 2],
            rectSizePrev: [0; 2],
            viewZScale: 1.0,
            timeDeltaBetweenFrames: 0.0,
            denoisingRange: 500000.0,
            disocclusionThreshold: 0.01,
            disocclusionThresholdAlternate: 0.05,
            cameraAttachedReflectionMaterialID: 999.0,
            strandMaterialID: 999.0,
            strandThickness: 80e-6,
            splitScreen: 0.0,
            printfAt: [9999, 9999],
            debug: 0.0,
            rectOrigin: [0; 2],
            frameIndex: 0,
            accumulationMode: ffi::nrd_AccumulationMode_CONTINUE,
            isMotionVectorInWorldSpace: false,
            isHistoryConfidenceAvailable: false,
            isDisocclusionThresholdMixAvailable: false,
            isBaseColorMetalnessAvailable: false,
            enableValidation: false,
        }
    }

    pub fn set_common_settings(&self, settings: &ffi::nrd_CommonSettings) {
        check_result(unsafe { ffi::nrd_SetCommonSettings(self.0, &*settings) });
    }

    pub fn get_default_reblur_settings(&self) -> ffi::nrd_ReblurSettings {
        ffi::nrd_ReblurSettings {
            hitDistanceParameters: ffi::nrd_HitDistanceParameters {
                A: 3.0,
                B: 0.1,
                C: 20.0,
                D: -25.0,
            },
            antilagSettings: ffi::nrd_ReblurAntilagSettings {
                luminanceSensitivity: 4.0,
                luminanceSigmaScale: 3.0,
            },
            maxAccumulatedFrameNum: 30,
            maxFastAccumulatedFrameNum: 6,
            maxStabilizedFrameNum: ffi::nrd_REBLUR_MAX_HISTORY_FRAME_NUM,
            historyFixFrameNum: 3,
            historyFixBasePixelStride: 14,
            diffusePrepassBlurRadius: 30.0,
            specularPrepassBlurRadius: 50.0,
            minHitDistanceWeight: 0.1,
            minBlurRadius: 1.0,
            maxBlurRadius: 30.0,
            lobeAngleFraction: 0.15,
            roughnessFraction: 0.15,
            responsiveAccumulationRoughnessThreshold: 0.0,
            planeDistanceSensitivity: 0.02,
            specularProbabilityThresholdsForMvModification: [0.5, 0.9],
            fireflySuppressorMinRelativeScale: 2.0,
            minMaterialForDiffuse: 4.0,
            minMaterialForSpecular: 4.0,
            checkerboardMode: ffi::nrd_CheckerboardMode_OFF,
            hitDistanceReconstructionMode: ffi::nrd_HitDistanceReconstructionMode_OFF,
            enableAntiFirefly: false,
            usePrepassOnlyForSpecularMotionEstimation: false,
        }
    }

    pub fn set_reblur_settings(
        &self,
        identifier: ffi::nrd_Identifier,
        settings: &ffi::nrd_ReblurSettings,
    ) {
        check_result(unsafe {
            ffi::nrd_SetDenoiserSettings(self.0, identifier, settings as *const _ as *const c_void)
        });
    }

    pub fn get_dispatches(&self, identifiers: &[ffi::nrd_Identifier]) -> Vec<DispatchDesc> {
        let mut len: u32 = 0;
        let mut dispatches: *const ffi::nrd_DispatchDesc = ptr::null();
        unsafe {
            ffi::nrd_GetComputeDispatches(
                self.0,
                identifiers.as_ptr(),
                identifiers.len() as u32,
                &mut dispatches as *mut _,
                &mut len as *mut _,
            )
        };

        let mut dispatches_v = Vec::with_capacity(len as usize);
        if len == 0 || dispatches.is_null() {
            return dispatches_v;
        }
        for i in 0..len as usize {
            let desc = unsafe { DispatchDesc(dispatches.add(i)) };
            dispatches_v.push(desc);
        }
        dispatches_v
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ResourceDesc {
    pub descriptor_type: DescriptorType,
    pub ty: ResourceType,
    pub index_in_pool: u16,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct DispatchDesc(*const ffi::nrd_DispatchDesc);
impl Deref for DispatchDesc {
    type Target = ffi::nrd_DispatchDesc;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}

unsafe impl Send for DispatchDesc {}
unsafe impl Sync for DispatchDesc {}

impl DispatchDesc {
    pub fn constant_buffer(&self) -> &[u8] {
        if self.constantBufferDataSize == 0 {
            return &[];
        }
        unsafe {
            std::slice::from_raw_parts(
                self.constantBufferData,
                self.constantBufferDataSize as usize,
            )
        }
    }
    pub fn resources(&self) -> &[ResourceDesc] {
        unsafe {
            std::slice::from_raw_parts(
                self.resources as *const ResourceDesc,
                self.resourcesNum as usize,
            )
        }
    }
    pub fn name(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.name) }
    }
}

// struct InstanceDesc(*const ffi::nrd_InstanceDesc);
// impl InstanceDesc {
//     pub fn new() -> Self {
//         let desc = unsafe { ffi::nrd_CreateInstance() };
//         Self(desc)
//     }

//     pub fn samplers(&self) -> &[Sampler] {
//         unsafe {
//             std::slice::from_raw_parts(
//                 (*self.0).samplers as *const Sampler,
//                 (*self.0).samplersNum as usize,
//             )
//         }
//     }

//     pub fn pipelines(&self) -> &[ffi::nrd_PipelineDesc] {
//         unsafe { std::slice::from_raw_parts((*self.0).pipelines, (*self.0).pipelinesNum as usize) }
//     }

//     pub fn permanent_pool(&self) -> &[ffi::nrd_TextureDesc] {
//         unsafe {
//             std::slice::from_raw_parts((*self.0).permanentPool, (*self.0).permanentPoolSize as usize)
//         }
//     }

//     pub fn transient_pool(&self) -> &[ffi::nrd_TextureDesc] {
//         unsafe {
//             std::slice::from_raw_parts((*self.0).transient

// assert_eq!(ffi::nrd_NormalEncoding_MAX_NUM, 5);

// #[repr(C)]
// #[derive(Debug, Clone)]
// pub struct SPIRVBindingOffsets {
//     pub sampler_offset: u32,
//     pub texture_offset: u32,
//     pub constant_buffer_offset: u32,
//     pub storage_texture_and_buffer_offset: u32,
// }

// #[repr(C)]
// pub struct LibraryDesc {
//     pub spirv_binding_offsets: SPIRVBindingOffsets,
//     supported_denoisers: *const Denoiser,
//     supported_denoisers_num: u32,
//     pub version_major: u8,
//     pub version_minor: u8,
//     pub version_build: u8,
//     pub normal_encoding: NormalEncoding,
//     pub roughness_encoding: RoughnessEncoding,
// }

// impl LibraryDesc {
//     pub fn supported_denoisers(&self) -> &[Denoiser] {
//         unsafe {
//             std::slice::from_raw_parts(
//                 self.supported_denoisers,
//                 self.supported_denoisers_num as usize,
//             )
//         }
//     }
// }

// impl Debug for LibraryDesc {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.debug_struct("LibraryDesc")
//             .field("spirv_binding_offsets", &self.spirv_binding_offsets)
//             .field("supported_denoisers", &self.supported_denoisers())
//             .field("version_major", &self.version_major)
//             .field("version_minor", &self.version_minor)
//             .field("version_build", &self.version_build)
//             .field("normal_encoding", &self.normal_encoding)
//             .field("roughness_encoding", &self.roughness_encoding)
//             .finish()
//     }
// }

// #[repr(transparent)]
// #[derive(Clone, Copy, Debug)]
// pub struct Identifier(pub u32);

// #[repr(C)]
// pub struct DenoiserDesc {
//     pub identifier: Identifier,
//     pub denoiser: Denoiser,
// }

// #[repr(C)]
// pub(crate) struct AllocationCallbacks {
//     pub(crate) allocate:
//         extern "C" fn(user_arg: *const c_void, size: usize, alignment: usize) -> *mut c_void,
//     pub(crate) reallocate: extern "C" fn(
//         user_arg: *const c_void,
//         memory: *mut c_void,
//         old_size: usize,
//         old_alignment: usize,
//         new_size: usize,
//         new_alignment: usize,
//     ) -> *mut c_void,
//     pub(crate) free:
//         extern "C" fn(user_arg: *const c_void, memory: *mut c_void, size: usize, alignment: usize),
//     pub(crate) user_arg: *const c_void,
// }

// #[repr(C)]
// pub(crate) struct InstanceCreationDesc {
//     pub allocation_allbacks: AllocationCallbacks,
//     pub denoisers: *const DenoiserDesc,
//     pub denoisers_num: u32,
// }

// #[repr(C)]
// pub struct ComputeShaderDesc {
//     bytecode: *const c_void,
//     size: u64,
// }
// impl Debug for ComputeShaderDesc {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.write_fmt(format_args!("ComputeShaderDesc({} bytes)", self.size))
//     }
// }
// impl std::ops::Deref for ComputeShaderDesc {
//     type Target = [u8];
//     fn deref(&self) -> &Self::Target {
//         unsafe { std::slice::from_raw_parts(self.bytecode as *const u8, self.size as usize) }
//     }
// }

// #[repr(u32)]
// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// pub enum DescriptorType {
//     // read-only, SRV
//     Texture,

//     // read-write, UAV
//     StorageTexture,
// }

// #[repr(C)]
// #[derive(Debug)]
// pub struct ResourceRangeDesc {
//     pub descriptor_type: DescriptorType,
//     pub base_register_index: u32,
//     pub descriptors_num: u32,
// }

// #[repr(u32)]
// #[derive(Debug, Clone, Copy)]
// #[allow(non_camel_case_types)]
// pub enum Format {
//     R8_UNORM,
//     R8_SNORM,
//     R8_UINT,
//     R8_SINT,

//     RG8_UNORM,
//     RG8_SNORM,
//     RG8_UINT,
//     RG8_SINT,

//     RGBA8_UNORM,
//     RGBA8_SNORM,
//     RGBA8_UINT,
//     RGBA8_SINT,
//     RGBA8_SRGB,

//     R16_UNORM,
//     R16_SNORM,
//     R16_UINT,
//     R16_SINT,
//     R16_SFLOAT,

//     RG16_UNORM,
//     RG16_SNORM,
//     RG16_UINT,
//     RG16_SINT,
//     RG16_SFLOAT,

//     RGBA16_UNORM,
//     RGBA16_SNORM,
//     RGBA16_UINT,
//     RGBA16_SINT,
//     RGBA16_SFLOAT,

//     R32_UINT,
//     R32_SINT,
//     R32_SFLOAT,

//     RG32_UINT,
//     RG32_SINT,
//     RG32_SFLOAT,

//     RGB32_UINT,
//     RGB32_SINT,
//     RGB32_SFLOAT,

//     RGBA32_UINT,
//     RGBA32_SINT,
//     RGBA32_SFLOAT,

//     R10_G10_B10_A2_UNORM,
//     R10_G10_B10_A2_UINT,
//     R11_G11_B10_UFLOAT,
//     R9_G9_B9_E5_UFLOAT,
// }

// #[repr(C)]
// pub struct PipelineDesc {
//     pub compute_shader_dxbc: ComputeShaderDesc,
//     pub compute_shader_dxil: ComputeShaderDesc,
//     pub compute_shader_spirv: ComputeShaderDesc,
//     shader_file_name: *const c_char,
//     shader_entry_point_name: *const c_char,
//     resource_ranges: *const ResourceRangeDesc,
//     resource_ranges_num: u32,
//     pub has_constant_data: bool,
// }
// impl PipelineDesc {
//     pub fn shader_file_name(&self) -> &CStr {
//         unsafe { CStr::from_ptr(self.shader_file_name) }
//     }
//     pub fn shader_entry_point_name(&self) -> &CStr {
//         unsafe { CStr::from_ptr(self.shader_entry_point_name) }
//     }
//     pub fn resource_ranges(&self) -> &[ResourceRangeDesc] {
//         unsafe {
//             std::slice::from_raw_parts(self.resource_ranges, self.resource_ranges_num as usize)
//         }
//     }
// }
// impl Debug for PipelineDesc {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.debug_struct("PipelineDesc")
//             .field("compute_shader_dxbc", &self.compute_shader_dxbc)
//             .field("compute_shader_dxil", &self.compute_shader_dxil)
//             .field("compute_shader_spirv", &self.compute_shader_spirv)
//             .field("shader_file_name", &self.shader_file_name())
//             .field("shader_entry_point_name", &self.shader_entry_point_name())
//             .field("resource_ranges", &self.resource_ranges())
//             .field("has_constant_data", &self.has_constant_data)
//             .finish()
//     }
// }

// #[repr(C)]
// #[derive(Debug, Clone)]
// pub struct TextureDesc {
//     pub format: Format,
//     pub downsample_factor: u16,
// }

// #[repr(C)]
// #[derive(Debug)]
// pub struct DescriptorPoolDesc {
//     pub sets_max_num: u32,
//     pub constant_buffers_max_num: u32,
//     pub samplers_max_num: u32,
//     pub textures_max_num: u32,
//     pub storage_textures_max_num: u32,
// }

// #[repr(C)]
// pub struct InstanceDesc(ffi::nrd_InstanceDesc);
// impl InstanceDesc {
//     pub fn samplers(&self) -> &[Sampler] {
//         unsafe {
//             std::slice::from_raw_parts(
//                 self.0.samplers as *const Sampler,
//                 self.0.samplersNum as usize,
//             )
//         }
//     }
//     pub fn pipelines(&self) -> &[ffi::nrd_PipelineDesc] {
//         unsafe { std::slice::from_raw_parts(self.0.pipelines, self.0.pipelinesNum as usize) }
//     }
//     pub fn permanent_pool(&self) -> &[ffi::nrd_TextureDesc] {
//         unsafe {
//             std::slice::from_raw_parts(self.0.permanentPool, self.0.permanentPoolSize as usize)
//         }
//     }
//     pub fn transient_pool(&self) -> &[ffi::nrd_TextureDesc] {
//         unsafe {
//             std::slice::from_raw_parts(self.0.transientPool, self.0.transientPoolSize as usize)
//         }
//     }
// }

// #[repr(u8)]
// #[derive(Debug, Clone, Copy)]
// pub enum AccumulationMode {
//     // Common mode (accumulation continues normally)
//     Continue,

//     // Discards history and resets accumulation
//     Restart,

//     // Like RESTART, but additionally clears resources from potential garbage
//     ClearAndRestart,
// }

// #[repr(C)]
// #[derive(Debug)]
// pub struct CommonSettings {
//     ///
//     ///  Matrix requirements:
//     ///
//     ///      - usage - vector is a column
//     ///      - layout - column-major
//     ///      - non jittered!
//     ///
//     ///  LH / RH projection matrix (INF far plane is supported) with non-swizzled rows, i.e. clip-space depth = z / w
//     pub view_to_clip_matrix: [f32; 16],

//     ///
//     ///  Previous projection matrix
//     pub view_to_clip_matrix_prev: [f32; 16],

//     ///
//     ///  World-space to camera-space matrix
//     pub world_to_view_matrix: [f32; 16],

//     ///
//     ///  If coordinate system moves with the camera, camera delta must be included to reflect camera motion
//     pub world_to_view_matrix_prev: [f32; 16],

//     ///
//     ///  (Optional) Previous world-space to current world-space matrix. It is for virtual normals, where a coordinate
//     ///
//     ///  system of the virtual space changes frame to frame, such as in a case of animated intermediary reflecting
//     ///
//     ///  surfaces when primary surface replacement is used for them.
//     pub world_prev_to_world_matrix: [f32; 16],

//     ///
//     ///  used as "IN_MV * motionVectorScale" (use .z = 0 for 2D screen-space motion)
//     pub motion_vector_scale: [f32; 3],

//     ///
//     ///  [-0.5; 0.5] - sampleUv = pixelUv + cameraJitter
//     pub camera_jitter: [f32; 2],
//     pub camera_jitter_prev: [f32; 2],

//     ///
//     ///  Flexible dynamic resolution scaling support
//     pub resource_size: [u16; 2],
//     pub resource_size_prev: [u16; 2],
//     pub rect_size: [u16; 2],
//     pub rect_size_prev: [u16; 2],

//     ///
//     ///  (>0) - viewZ = IN_VIEWZ * viewZScale (mostly for FP16 viewZ)
//     pub view_z_scale: f32,

//     ///
//     ///  (Optional) (ms) - user provided if > 0, otherwise - tracked internally
//     pub time_delta_between_frames: f32,

//     ///
//     ///  (units > 0) - use TLAS or tracing range
//     ///
//     ///  It's highly recommended to use "viewZ > denoisingRange" for INF (sky) pixels
//     pub denoising_range: f32,

//     ///
//     ///  [0.01; 0.02] - two samples considered occluded if relative distance difference is greater than this slope-scaled threshold
//     pub disocclusion_threshold: f32,

//     ///
//     ///  (Optional) [0.02; 0.2] - an alternative disocclusion threshold, which is mixed to based on:
//     ///
//     ///  - "strandThickness", if there is "strandMaterialID" match
//     ///
//     ///  - IN_DISOCCLUSION_THRESHOLD_MIX texture, if "isDisocclusionThresholdMixAvailable = true" (has higher priority and ignores "strandMaterialID")    pub disocclusion_threshold_alternate: f32,
//     pub disocclusion_threshold_alternate: f32,

//     ///
//     ///  (Optional) (>=0) - marks reflections of camera attached objects (requires "NormalEncoding::R10_G10_B10_A2_UNORM")
//     ///
//     ///  This material ID marks reflections of objects attached to the camera, not objects themselves. Unfortunately, this is only an improvement
//     ///
//     ///  for critical cases, but not a generic solution. A generic solution requires reflection MVs, which NRD currently doesn't ask for
//     pub camera_attached_reflection_material_id: f32,

//     ///
//     ///  (Optional) (>=0) - marks hair (grass) geometry to enable "under-the-hood" tweaks (requires "NormalEncoding::R10_G10_B10_A2_UNORM")
//     pub strand_material_id: f32,

//     ///
//     ///  (units > 0) - defines how "disocclusionThreshold" blends into "disocclusionThresholdAlternate" = pixelSize / (pixelSize + strandThickness)
//     pub strand_thickness: f32,

//     ///
//     ///  [0; 1] - enables "noisy input / denoised output" comparison
//     pub split_screen: f32,

//     ///
//     ///  For internal needs
//     pub printf_at: [u16; 2],
//     ///
//     ///  For internal needs
//     pub debug: f32,

//     ///
//     ///  (Optional) (pixels) - viewport origin
//     ///
//     ///  IMPORTANT: gets applied only to non-noisy guides (aka g-buffer), including IN_DIFF_CONFIDENCE, IN_SPEC_CONFIDENCE,
//     ///
//     ///  IN_DISOCCLUSION_THRESHOLD_MIX and IN_BASECOLOR_METALNESS. Must be manually enabled via NRD_USE_VIEWPORT_OFFSET macro switch
//     pub rect_origin: [u32; 2],

//     ///
//     ///  A consecutive number
//     pub frame_index: u32,

//     ///
//     ///  To reset history set to RESTART / CLEAR_AND_RESTART for one frame
//     pub accumulation_mode: AccumulationMode,

//     ///
//     ///  If "true" IN_MV is 3D motion in world-space (0 should be everywhere if the scene is static),
//     ///
//     ///  otherwise it's 2D (+ optional Z delta) screen-space motion (0 should be everywhere if the camera doesn't move) (recommended value = true)
//     pub is_motion_vector_in_world_space: bool,

//     ///
//     ///  If "true" IN_DIFF_CONFIDENCE and IN_SPEC_CONFIDENCE are available
//     pub is_history_confidence_available: bool,

//     ///
//     ///  If "true" IN_DISOCCLUSION_THRESHOLD_MIX is available
//     pub is_disocclusion_threshold_mix_available: bool,

//     ///
//     ///  If "true" IN_BASECOLOR_METALNESS is available
//     pub is_base_color_metalness_available: bool,

//     ///
//     ///  Enables debug overlay in OUT_VALIDATION, requires "InstanceCreationDesc::allowValidation = true"
//     pub enable_validation: bool,
// }
// impl Default for CommonSettings {
//     fn default() -> Self {
//         const IDENTITY: [f32; 16] = [
//             1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
//         ];
//         Self {
//             view_to_clip_matrix: [0.0; 16],
//             view_to_clip_matrix_prev: [0.0; 16],
//             world_to_view_matrix: [0.0; 16],
//             world_to_view_matrix_prev: [0.0; 16],
//             world_prev_to_world_matrix: IDENTITY,
//             motion_vector_scale: [1.0, 1.0, 0.0],
//             camera_jitter: [0.0; 2],
//             camera_jitter_prev: [0.0; 2],
//             resource_size: [0, 0],
//             resource_size_prev: [0, 0],
//             rect_size: [0, 0],
//             rect_size_prev: [0, 0],
//             view_z_scale: 1.0,
//             time_delta_between_frames: 0.0,
//             denoising_range: 500000.0,
//             disocclusion_threshold: 0.01,
//             disocclusion_threshold_alternate: 0.05,
//             camera_attached_reflection_material_id: 999.0,
//             strand_material_id: 999.0,
//             strand_thickness: 80e-6,
//             split_screen: 0.0,
//             printf_at: [9999, 9999],
//             debug: 0.0,
//             rect_origin: [0, 0],
//             frame_index: 0,
//             accumulation_mode: AccumulationMode::Continue,
//             is_motion_vector_in_world_space: false,
//             is_history_confidence_available: false,
//             is_disocclusion_threshold_mix_available: false,
//             is_base_color_metalness_available: false,
//             enable_validation: false,
//         }
//     }
// }

// #[repr(u32)]
// #[allow(non_camel_case_types)]
// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// pub enum ResourceType {
//     //=============================================================================================================================
//     // NON-NOISY INPUTS
//     //=============================================================================================================================

//     // 3D world-space motion (RGBA16f+) or 2D screen-space motion (RG16f+), MVs
//     // must be non-jittered, MV = previous - current
//     IN_MV,

//     // Data must match encoding in "NRD_FrontEnd_PackNormalAndRoughness" and
//     // "NRD_FrontEnd_UnpackNormalAndRoughness" (RGBA8+)
//     IN_NORMAL_ROUGHNESS,

//     // Linear view depth for primary rays (R16f+)
//     IN_VIEWZ,

//     // (Optional) User-provided history confidence in range 0-1, i.e. antilag
//     // (R8+) Used only if "CommonSettings::isHistoryConfidenceAvailable = true"
//     // and "NRD_USE_HISTORY_CONFIDENCE = 1"
//     IN_DIFF_CONFIDENCE,
//     IN_SPEC_CONFIDENCE,

//     // (Optional) User-provided disocclusion threshold selector in range 0-1 (R8+)
//     // Disocclusion threshold is mixed between "disocclusionThreshold" and
//     // "disocclusionThresholdAlternate" Used only if
//     // "CommonSettings::isDisocclusionThresholdMixAvailable = true" and
//     // "NRD_USE_DISOCCLUSION_THRESHOLD_MIX = 1"
//     IN_DISOCCLUSION_THRESHOLD_MIX,

//     // (Optional) Base color (can be decoupled to diffuse and specular albedo
//     // based on metalness) and metalness (RGBA8+) Used only if
//     // "CommonSettings::isBaseColorMetalnessAvailable = true" and
//     // "NRD_USE_BASECOLOR_METALNESS = 1". Currently used only by REBLUR (if
//     // Temporal Stabilization pass is available and "stabilizationStrength != 0")
//     // to patch MV if specular (virtual) motion prevails on diffuse (surface)
//     // motion
//     IN_BASECOLOR_METALNESS,

//     //=============================================================================================================================
//     // NOISY INPUTS
//     //=============================================================================================================================

//     // Radiance and hit distance (RGBA16f+)
//     //      REBLUR: use "REBLUR_FrontEnd_PackRadianceAndNormHitDist" for encoding
//     //      RELAX: use "RELAX_FrontEnd_PackRadianceAndHitDist" for encoding
//     IN_DIFF_RADIANCE_HITDIST,
//     IN_SPEC_RADIANCE_HITDIST,

//     // Hit distance (R8+)
//     //      REBLUR: use "REBLUR_FrontEnd_GetNormHitDist" for encoding
//     IN_DIFF_HITDIST,
//     IN_SPEC_HITDIST,

//     // Sampling direction and normalized hit distance (RGBA8+)
//     //      REBLUR: use "REBLUR_FrontEnd_PackDirectionalOcclusion" for encoding
//     IN_DIFF_DIRECTION_HITDIST,

//     // SH data (2x RGBA16f+)
//     //      REBLUR: use "REBLUR_FrontEnd_PackSh" for encoding
//     //      RELAX: use "RELAX_FrontEnd_PackSh" for encoding
//     IN_DIFF_SH0,
//     IN_DIFF_SH1,
//     IN_SPEC_SH0,
//     IN_SPEC_SH1,

//     // Penumbra and optional translucency (R16f+ and RGBA8+ for translucency)
//     //      SIGMA: use "SIGMA_FrontEnd_PackPenumbra" for penumbra properties
//     //      encoding SIGMA: use "SIGMA_FrontEnd_PackTranslucency" for translucency
//     //      encoding
//     IN_PENUMBRA,
//     IN_TRANSLUCENCY,

//     // Some signal (R8+)
//     IN_SIGNAL,

//     // Primary and secondary world-space positions (RGBA16f+)
//     IN_DELTA_PRIMARY_POS,
//     IN_DELTA_SECONDARY_POS,

//     //=============================================================================================================================
//     // OUTPUTS
//     //=============================================================================================================================

//     // IMPORTANT: Most of denoisers do not write into output pixels outside of
//     // "CommonSettings::denoisingRange"!

//     // Radiance and hit distance
//     //      REBLUR: use "REBLUR_BackEnd_UnpackRadianceAndNormHitDist" for decoding
//     //      (RGBA16f+) RELAX: use "RELAX_BackEnd_UnpackRadiance" for decoding
//     //      (R11G11B10f+)
//     OUT_DIFF_RADIANCE_HITDIST, // IMPORTANT: used as history if
//     // "stabilizationStrength != 0"
//     OUT_SPEC_RADIANCE_HITDIST, // IMPORTANT: used as history if
//     // "stabilizationStrength != 0"

//     // SH data
//     //      REBLUR: use "REBLUR_BackEnd_UnpackSh" for decoding (2x RGBA16f+)
//     //      RELAX: use "RELAX_BackEnd_UnpackSh" for decoding (2x RGBA16f+)
//     OUT_DIFF_SH0, // IMPORTANT: used as history if "stabilizationStrength != 0"
//     OUT_DIFF_SH1, // IMPORTANT: used as history if "stabilizationStrength != 0"
//     OUT_SPEC_SH0, // IMPORTANT: used as history if "stabilizationStrength != 0"
//     OUT_SPEC_SH1, // IMPORTANT: used as history if "stabilizationStrength != 0"

//     // Normalized hit distance (R8+)
//     OUT_DIFF_HITDIST,
//     OUT_SPEC_HITDIST,

//     // Bent normal and normalized hit distance (RGBA8+)
//     //      REBLUR: use "REBLUR_BackEnd_UnpackDirectionalOcclusion" for decoding
//     OUT_DIFF_DIRECTION_HITDIST, // IMPORTANT: used as history if
//     // "stabilizationStrength != 0"

//     // Shadow and optional transcluceny (R8+ or RGBA8+)
//     //      SIGMA: use "SIGMA_BackEnd_UnpackShadow" for decoding
//     OUT_SHADOW_TRANSLUCENCY, // IMPORTANT: used as history if
//     // "stabilizationStrength != 0"

//     // Denoised signal (R8+)
//     OUT_SIGNAL,

//     // (Optional) Debug output (RGBA8+), .w = transparency
//     // Used if "CommonSettings::enableValidation = true"
//     OUT_VALIDATION,

//     //=============================================================================================================================
//     // POOLS
//     //=============================================================================================================================

//     // Can be reused after denoising
//     TRANSIENT_POOL,

//     // Dedicated to NRD, can't be reused
//     PERMANENT_POOL,
// }

// #[repr(C)]
// #[derive(Debug, Clone, Copy)]
// pub struct ResourceDesc {
//     pub descriptor_type: DescriptorType,
//     pub ty: ResourceType,
//     pub index_in_pool: u16,
// }

// #[repr(C)]
// #[derive(Clone, Copy)]
// pub struct DispatchDesc {
//     name: *const c_char,
//     identifier: Identifier,
//     resources: *const ResourceDesc,
//     resources_num: u32,
//     constant_buffer_data: *const u8,
//     constant_buffer_data_size: u32,
//     constant_buffer_data_matches_previous_dispatch: bool,
//     pub pipeline_index: u16,
//     pub grid_width: u16,
//     pub grid_height: u16,
// }
// unsafe impl Send for DispatchDesc {}
// unsafe impl Sync for DispatchDesc {}

// impl DispatchDesc {
//     pub fn constant_buffer(&self) -> &[u8] {
//         if self.constant_buffer_data.is_null() {
//             return &[];
//         }
//         unsafe {
//             std::slice::from_raw_parts(
//                 self.constant_buffer_data,
//                 self.constant_buffer_data_size as usize,
//             )
//         }
//     }
//     pub fn resources(&self) -> &[ResourceDesc] {
//         unsafe { std::slice::from_raw_parts(self.resources, self.resources_num as usize) }
//     }
//     pub fn name(&self) -> &CStr {
//         unsafe { CStr::from_ptr(self.name) }
//     }
// }

// impl Debug for DispatchDesc {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.debug_struct("DispatchDesc")
//             .field("name", &self.name())
//             .field("resources", &self.resources())
//             .field(
//                 "constant_buffer",
//                 &format!("{} bytes", self.constant_buffer().len()),
//             )
//             .field("pipeline_index", &self.pipeline_index)
//             .field("grid_width", &self.grid_width)
//             .field("grid_height", &self.grid_height)
//             .finish()
//     }
// }

// #[repr(C)]
// #[derive(Clone)]
// pub struct HitDistanceParameters {
//     // (units) - constant value
//     // IMPORTANT: if your unit is not "meter", you must convert it from "meters" to "units" manually!
//     pub a: f32,

//     // (> 0) - viewZ based linear scale (1 m - 10 cm, 10 m - 1 m, 100 m - 10 m)
//     pub b: f32,

//     // (>= 1) - roughness based scale, use values > 1 to get bigger hit distance for low roughness
//     pub c: f32,

//     // (<= 0) - absolute value should be big enough to collapse "exp2( D * roughness ^ 2 )" to "~0" for roughness = 1
//     pub d: f32,
// }

// impl Default for HitDistanceParameters {
//     fn default() -> Self {
//         Self {
//             a: 3.0,
//             b: 0.1,
//             c: 20.0,
//             d: -25.0,
//         }
//     }
// }

// #[repr(C)]
// #[derive(Clone)]
// pub struct ReblurAntilagSettings {
//     /// [1; 5] - delta is reduced by local variance multiplied by this value
//     ///
//     /// can be 3.0 or even less if signal is good
//     pub luminance_sigma_scale: f32,
//     pub hit_distance_sigma_scale: f32,

//     /// [1; 5] - antilag sensitivity (smaller values increase sensitivity)
//     ///
//     /// can be 2.0 or even less if signal is good
//     pub luminance_sensitivity: f32,
//     pub hit_distance_sensitivity: f32,
// }

// impl Default for ReblurAntilagSettings {
//     fn default() -> Self {
//         Self {
//             luminance_sigma_scale: 4.0,
//             hit_distance_sigma_scale: 3.0,
//             luminance_sensitivity: 3.0,
//             hit_distance_sensitivity: 2.0,
//         }
//     }
// }

// #[repr(u8)]
// #[derive(Clone, Copy, Debug)]
// pub enum CheckerboardMode {
//     Off,
//     Black,
//     White,
// }

// #[repr(u8)]
// #[derive(Clone, Copy, Debug)]
// pub enum HitDistanceReconstructionMode {
//     // Probabilistic split at primary hit is not used, hence hit distance is always valid (reconstruction is not needed)
//     Off,

//     // If hit distance is invalid due to probabilistic sampling, reconstruct using 3x3 neighbors.
//     // Probability at primary hit must be clamped to [1/4; 3/4] range to guarantee a sample in this area
//     Area3x3,

//     // If hit distance is invalid due to probabilistic sampling, reconstruct using 5x5 neighbors.
//     // Probability at primary hit must be clamped to [1/16; 15/16] range to guarantee a sample in this area
//     Area5x5,
// }
// #[repr(C)]
// #[derive(Clone)]
// pub struct ReblurSettings {
//     pub hit_distance_parameters: HitDistanceParameters,
//     pub antilag_settings: ReblurAntilagSettings,

//     // [0; REBLUR_MAX_HISTORY_FRAME_NUM] - maximum number of linearly accumulated frames (= FPS * "time of accumulation")
//     pub max_accumulated_frame_num: u32,

//     // [0; REBLUR_MAX_HISTORY_FRAME_NUM] - maximum number of linearly accumulated frames in fast history (less than "maxAccumulatedFrameNum")
//     pub max_fast_accumulated_frame_num: u32,

//     // [0; maxAccumulatedFrameNum] - maximum number of linearly accumulated frames for stabilized radiance
//     // "0" disables the stabilization pass
//     // Values ">= maxAccumulatedFrameNum"  get clamped to "maxAccumulatedFrameNum"
//     pub max_stabilized_frame_num: u32,

//     // [0; maxAccumulatedFrameNum] - maximum number of linearly accumulated frames for stabilized hit distance
//     // 0 - allows to reach parity with "REBLUR_OCCLUSION"
//     pub max_stabilized_frame_num_for_hit_distance: u32,

//     // [0; REBLUR_MAX_HISTORY_FRAME_NUM] - number of reconstructed frames after history reset (less than "maxFastAccumulatedFrameNum")
//     pub history_fix_frame_num: u32,

//     /// (pixels) - pre-accumulation spatial reuse pass blur radius (0 = disabled, recommended in case of probabilistic sampling)
//     pub diffuse_prepass_blur_radius: f32,
//     pub specular_prepass_blur_radius: f32,

//     // (pixels) - min denoising radius (for converged state)
//     pub min_blur_radius: f32,

//     // (pixels) - base (max) denoising radius (gets reduced over time)
//     pub max_blur_radius: f32,

//     // (normalized %) - base fraction of diffuse or specular lobe angle used to drive normal based rejection
//     pub lobe_angle_fraction: f32,

//     // (normalized %) - base fraction of center roughness used to drive roughness based rejection
//     pub roughness_fraction: f32,

//     // [0; 1] - if roughness < this, temporal accumulation becomes responsive and driven by roughness (useful for animated water)
//     pub responsive_accumulation_roughness_threshold: f32,

//     // (normalized %) - represents maximum allowed deviation from the local tangent plane
//     pub plane_distance_sensitivity: f32,
//     // IN_MV = lerp(IN_MV, specularMotion, smoothstep(this[0], this[1], specularProbability))
//     pub specular_probability_thresholds_for_mv_modification: [f32; 2],

//     // [1; 3] - undesired sporadic outliers suppression to keep output stable (smaller values maximize suppression in exchange of bias)
//     pub firefly_suppressor_min_relative_scale: f32,

//     // If not OFF and used for DIFFUSE_SPECULAR, defines diffuse orientation, specular orientation is the opposite
//     pub checkerboard_mode: CheckerboardMode,

//     pub hit_distance_reconstruction_mode: HitDistanceReconstructionMode,

//     // Adds bias in case of badly defined signals, but tries to fight with fireflies
//     pub enable_anti_firefly: bool,

//     // Boosts performance by sacrificing IQ
//     pub enable_performance_mode: bool,

//     // (Optional) material ID comparison: enableMaterialTest ? materialID[x] == materialID[y] : 1 (requires "NormalEncoding::R10_G10_B10_A2_UNORM")
//     pub enable_material_test_for_diffuse: bool,
//     pub enable_material_test_for_specular: bool,

//     // In rare cases, when bright samples are so sparse that any other bright neighbor can't
//     // be reached, pre-pass transforms a standalone bright pixel into a standalone bright blob,
//     // worsening the situation. Despite that it's a problem of sampling, the denoiser needs to
//     // handle it somehow on its side too. Diffuse pre-pass can be just disabled, but for specular
//     // it's still needed to find optimal hit distance for tracking. This boolean allow to use
//     // specular pre-pass for tracking purposes only (use with care)
//     pub use_prepass_only_for_specular_motion_estimation: bool,
// }

// impl Default for ReblurSettings {
//     fn default() -> Self {
//         Self {
//             hit_distance_parameters: Default::default(),
//             antilag_settings: Default::default(),
//             max_accumulated_frame_num: 30,
//             max_fast_accumulated_frame_num: 6,
//             max_stabilized_frame_num: 63,
//             max_stabilized_frame_num_for_hit_distance: 63,
//             history_fix_frame_num: 3,
//             diffuse_prepass_blur_radius: 30.0,
//             specular_prepass_blur_radius: 50.0,
//             min_blur_radius: 1.0,
//             max_blur_radius: 30.0,
//             lobe_angle_fraction: 0.15,
//             roughness_fraction: 0.15,
//             responsive_accumulation_roughness_threshold: 0.0,
//             plane_distance_sensitivity: 0.02,
//             specular_probability_thresholds_for_mv_modification: [0.5, 0.9],
//             firefly_suppressor_min_relative_scale: 2.0,
//             checkerboard_mode: CheckerboardMode::Off,
//             hit_distance_reconstruction_mode: HitDistanceReconstructionMode::Off,
//             enable_anti_firefly: false,
//             enable_performance_mode: false,
//             enable_material_test_for_diffuse: false,
//             enable_material_test_for_specular: false,
//             use_prepass_only_for_specular_motion_estimation: false,
//         }
//     }
// }

// #[repr(C)]
// pub struct SigmaSettings {
//     // Direction to the light source
//     // IMPORTANT: it is needed only for directional light sources (sun)
//     pub light_direction: [f32; 3],

//     // (normalized %) - represents maximum allowed deviation from the local tangent plane
//     pub plane_distance_sensitivity: f32,

//     // [0; SIGMA_MAX_HISTORY_FRAME_NUM] - maximum number of linearly accumulated frames
//     // 0 - disables the stabilization pass
//     pub max_stabilized_frame_num: u32,
// }
// impl Default for SigmaSettings {
//     fn default() -> Self {
//         Self {
//             light_direction: [0.0, 0.0, 0.0],
//             plane_distance_sensitivity: 0.02,
//             max_stabilized_frame_num: 5,
//         }
//     }
// }

// #[repr(C)]
// pub struct RelaxAntilagSettings {
//     // IMPORTANT: History acceleration and reset amounts for specular are made 2x-3x weaker than values for diffuse below
//     // due to specific specular logic that does additional history acceleration and reset

//     // [0; 1] - amount of history acceleration if history clamping happened in pixel
//     acceleration_amount: f32,

//     // (> 0) - history is being reset if delta between history and raw input is larger than spatial sigma + temporal sigma
//     spatial_sigma_scale: f32,

//     // (> 0) - history is being reset if delta between history and raw input is larger than spatial sigma + temporal sigma
//     temporal_sigma_scale: f32,

//     // [0; 1] - amount of history reset, 0.0 - no reset, 1.0 - full reset
//     reset_amount: f32,
// }

// impl Default for RelaxAntilagSettings {
//     fn default() -> Self {
//         Self {
//             acceleration_amount: 0.3,
//             spatial_sigma_scale: 4.5,
//             temporal_sigma_scale: 0.5,
//             reset_amount: 0.5,
//         }
//     }
// }

// // RELAX_DIFFUSE_SPECULAR
// #[repr(C)]

// pub struct RelaxSettings {
//     pub antilag_settings: RelaxAntilagSettings,
//     // [0; RELAX_MAX_HISTORY_FRAME_NUM] - maximum number of linearly accumulated frames ( = FPS * "time of accumulation")
//     pub diffuse_max_accumulated_frame_num: u32,
//     pub specular_max_accumulated_frame_num: u32,

//     // [0; diffuseMaxAccumulatedFrameNum) - maximum number of linearly accumulated frames for diffuse fast history
//     // Values ">= diffuseMaxAccumulatedFrameNum" disable diffuse fast history
//     // Usually 5x times shorter than the main history
//     pub diffuse_max_fast_accumulated_frame_num: u32,
//     // [0; specularMaxAccumulatedFrameNum) - maximum number of linearly accumulated frames for specular fast history
//     // Values ">= specularMaxAccumulatedFrameNum" disable specular fast history
//     // Usually 5x times shorter than the main history
//     pub specular_max_fast_accumulated_frame_num: u32,

//     // [0; 3] - number of reconstructed frames after history reset (less than "maxFastAccumulatedFrameNum")
//     pub history_fix_frame_num: u32,

//     // (>= 0) - history length threshold below which spatial variance estimation will be executed
//     pub spatial_variance_estimation_history_threshold: u32,

//     // (pixels) - pre-accumulation spatial reuse pass blur radius (0 = disabled, must be used in case of probabilistic sampling)
//     pub diffuse_prepass_blur_radius: f32,
//     pub specular_prepass_blur_radius: f32,

//     // A-trous edge stopping Luminance sensitivity
//     pub diffuse_phi_luminance: f32,
//     pub specular_phi_luminance: f32,

//     // (normalized %) - base fraction of diffuse or specular lobe angle used to drive normal based rejection
//     pub lobe_angle_fraction: f32,

//     // (normalized %) - base fraction of center roughness used to drive roughness based rejection
//     pub roughness_fraction: f32,

//     // (>= 0) - how much variance we inject to specular if reprojection confidence is low
//     pub specular_variance_boost: f32,

//     // (degrees) - slack for the specular lobe angle used in normal based rejection of specular during A-Trous passes
//     pub specular_lobe_angle_slack: f32,

//     // (> 0) - normal edge stopper for history reconstruction pass
//     pub history_fix_edge_stopping_normal_power: f32,

//     // [1; 3] - standard deviation scale of color box for clamping main "slow" history to responsive "fast" history
//     pub history_clamping_color_box_sigma_scale: f32,

//     // [2; 8] - number of iterations for A-Trous wavelet transform
//     pub atrous_iteration_num: u32,

//     // [0; 1] - A-trous edge stopping Luminance weight minimum
//     pub diffuse_min_luminance_weight: f32,
//     pub specular_min_luminance_weight: f32,

//     // (normalized %) - Depth threshold for spatial passes
//     pub depth_threshold: f32,

//     // Confidence inputs can affect spatial blurs, relaxing some weights in areas with low confidence
//     pub confidence_driven_relaxation_multiplier: f32,
//     pub confidence_driven_luminance_edge_stopping_relaxation: f32,
//     pub confidence_driven_normal_edge_stopping_relaxation: f32,

//     // How much we relax roughness based rejection for spatial filter in areas where specular reprojection is low
//     pub luminance_edge_stopping_relaxation: f32,
//     pub normal_edge_stopping_relaxation: f32,

//     // How much we relax rejection for spatial filter based on roughness and view vector
//     pub roughness_edge_stopping_relaxation: f32,

//     // If not OFF and used for DIFFUSE_SPECULAR, defines diffuse orientation, specular orientation is the opposite
//     pub checkerboard_mode: CheckerboardMode,

//     // Must be used only in case of probabilistic sampling (not checkerboarding), when a pixel can be skipped and have "0" (invalid) hit distance
//     pub hit_distance_reconstruction_mode: HitDistanceReconstructionMode,

//     // Firefly suppression
//     pub enable_anti_firefly: bool,

//     // Roughness based rejection
//     pub enable_roughness_edge_stopping: bool,

//     // Spatial passes do optional material index comparison as: ( materialEnabled ? material[ center ] == material[ sample ] : 1 )
//     pub enable_material_test_for_diffuse: bool,
//     pub enable_material_test_for_specular: bool,
// }

// impl Default for RelaxSettings {
//     fn default() -> Self {
//         Self {
//             antilag_settings: Default::default(),
//             diffuse_max_accumulated_frame_num: 30,
//             specular_max_accumulated_frame_num: 30,

//             diffuse_max_fast_accumulated_frame_num: 6,
//             specular_max_fast_accumulated_frame_num: 6,

//             history_fix_frame_num: 3,
//             spatial_variance_estimation_history_threshold: 3,

//             diffuse_prepass_blur_radius: 30.0,
//             specular_prepass_blur_radius: 50.0,

//             diffuse_phi_luminance: 2.0,
//             specular_phi_luminance: 1.0,

//             lobe_angle_fraction: 0.5,
//             roughness_fraction: 0.15,

//             specular_variance_boost: 0.0,
//             specular_lobe_angle_slack: 0.15,
//             history_fix_edge_stopping_normal_power: 8.0,
//             history_clamping_color_box_sigma_scale: 2.0,
//             atrous_iteration_num: 5,
//             diffuse_min_luminance_weight: 0.0,
//             specular_min_luminance_weight: 0.0,
//             depth_threshold: 0.003,
//             confidence_driven_relaxation_multiplier: 0.0,
//             confidence_driven_luminance_edge_stopping_relaxation: 0.0,
//             confidence_driven_normal_edge_stopping_relaxation: 0.0,
//             luminance_edge_stopping_relaxation: 0.5,
//             normal_edge_stopping_relaxation: 0.3,
//             roughness_edge_stopping_relaxation: 1.0,
//             checkerboard_mode: CheckerboardMode::Off,
//             hit_distance_reconstruction_mode: HitDistanceReconstructionMode::Off,
//             enable_anti_firefly: false,
//             enable_roughness_edge_stopping: false,
//             enable_material_test_for_diffuse: false,
//             enable_material_test_for_specular: false,
//         }
//     }
// }

// pub struct ReferenceSettings {
//     // (>= 0) - maximum number of linearly accumulated frames ( = FPS * "time of accumulation")
//     pub max_accumulated_frame_num: u32,
// }
// impl Default for ReferenceSettings {
//     fn default() -> Self {
//         Self {
//             max_accumulated_frame_num: 1024,
//         }
//     }
// }

// #[cfg(any(
//     target_env = "msvc",
//     all(not(target_arch = "aarch64"), not(target_arch = "x86_64"),)
// ))]
// macro_rules! nrd_abi {
//     ($($toks: tt)+) => {
//         extern "fastcall" {$($toks)+}
//     };
// }

// #[cfg(not(any(
//     target_env = "msvc",
//     all(not(target_arch = "aarch64"), not(target_arch = "x86_64"),)
// )))]
// macro_rules! nrd_abi {
//     ($($toks: tt)+) => {
//         extern "C" {$($toks)+}
//     };
// }

// nrd_abi! {
//     pub(crate) fn GetLibraryDesc() -> &'static LibraryDesc;
//     pub(crate) fn CreateInstance(desc: &InstanceCreationDesc, instance: &mut *mut c_void)
//         -> Result;
//     pub(crate) fn DestroyInstance(instance: *mut c_void);
//     pub(crate) fn GetInstanceDesc(instance: *mut c_void) -> *const InstanceDesc;
//     pub(crate) fn SetCommonSettings(instance: *mut c_void, settings: &CommonSettings) -> Result;
//     pub(crate) fn SetDenoiserSettings(
//         instance: *mut c_void,
//         identifier: Identifier,
//         denoiserSettings: *const c_void,
//     ) -> Result;
//     pub(crate) fn GetComputeDispatches(
//         instance: *mut c_void,
//         identifiers: *const Identifier,
//         identifiers_num: u32,
//         descs: &mut *const DispatchDesc,
//         descs_num: &mut u32,
//     ) -> Result;
// }

// pub trait DenoiserSettings {}
// impl DenoiserSettings for ffi::ReblurSettings {}
// impl DenoiserSettings for ffi::RelaxSettings {}
// impl DenoiserSettings for ffi::ReferenceSettings {}
// impl DenoiserSettings for ffi::SigmaSettings {}

// pub struct Instance(*mut c_void);
// unsafe impl Send for Instance {}
// unsafe impl Sync for Instance {}

// impl Instance {
//     pub fn library_desc() -> &'static ffi::LibraryDesc {
//         unsafe { ffi::GetLibraryDesc() }
//     }
//     pub fn new(denoisers: &[ffi::DenoiserDesc]) -> Result<Self, ffi::Result> {
//         let desc = ffi::InstanceCreationDesc {
//             allocation_allbacks: ffi::AllocationCallbacks {
//                 allocate: allocator::allocate,
//                 reallocate: allocator::reallocate,
//                 free: allocator::free,
//                 user_arg: std::ptr::null(),
//             },
//             denoisers: denoisers.as_ptr(),
//             denoisers_num: denoisers.len() as u32,
//         };
//         let mut ptr: *mut c_void = std::ptr::null_mut();
//         let result = unsafe { ffi::CreateInstance(&desc, &mut ptr) };
//         match result {
//             ffi::Result::Success => Ok(Self(ptr)),
//             _ => Err(result),
//         }
//     }

//     pub fn desc(&self) -> &ffi::InstanceDesc {
//         unsafe {
//             let ptr = ffi::GetInstanceDesc(self.0);
//             &*ptr
//         }
//     }

//     pub fn set_common_settings(
//         &mut self,
//         settings: &ffi::CommonSettings,
//     ) -> Result<(), ffi::Result> {
//         unsafe {
//             let result = ffi::SetCommonSettings(self.0, settings);
//             match result {
//                 ffi::Result::Success => Ok(()),
//                 _ => Err(result),
//             }
//         }
//     }
//     pub fn set_denoiser_settings<T>(
//         &mut self,
//         identifier: ffi::Identifier,
//         reblur_settings: &T,
//     ) -> Result<(), ffi::Result> {
//         unsafe {
//             ffi::SetDenoiserSettings(
//                 self.0,
//                 identifier,
//                 reblur_settings as *const _ as *const c_void,
//             )
//             .ok(())
//         }
//     }

//     pub fn get_compute_dispatches(
//         &mut self,
//         identifiers: &[ffi::Identifier],
//     ) -> Result<&[ffi::DispatchDesc], ffi::Result> {
//         unsafe {
//             let mut dispatches: *const ffi::DispatchDesc = std::ptr::null();
//             let mut dispatches_count: u32 = 0;
//             let result = ffi::GetComputeDispatches(
//                 self.0,
//                 identifiers.as_ptr(),
//                 identifiers.len() as u32,
//                 &mut dispatches,
//                 &mut dispatches_count,
//             );
//             match result {
//                 ffi::Result::Success => Ok(std::slice::from_raw_parts(
//                     dispatches,
//                     dispatches_count as usize,
//                 )),
//                 _ => Err(result),
//             }
//         }
//     }
// }

// impl Drop for Instance {
//     fn drop(&mut self) {
//         unsafe {
//             ffi::DestroyInstance(self.0);
//         }
//     }
// }
