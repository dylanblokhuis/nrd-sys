use crate::ffi;

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
pub enum ResultCode {
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

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CheckerboardMode {
    Off = ffi::nrd_CheckerboardMode_OFF,
    Black = ffi::nrd_CheckerboardMode_BLACK,
    White = ffi::nrd_CheckerboardMode_WHITE,
    MaxNum = ffi::nrd_CheckerboardMode_MAX_NUM,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccumulationMode {
    Continue = ffi::nrd_AccumulationMode_CONTINUE,
    Restart = ffi::nrd_AccumulationMode_RESTART,
    ClearAndRestart = ffi::nrd_AccumulationMode_CLEAR_AND_RESTART,
    MaxNum = ffi::nrd_AccumulationMode_MAX_NUM,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HitDistanceReconstructionMode {
    Off = ffi::nrd_HitDistanceReconstructionMode_OFF,
    Area3x3 = ffi::nrd_HitDistanceReconstructionMode_AREA_3X3,
    Area5x5 = ffi::nrd_HitDistanceReconstructionMode_AREA_5X5,
    MaxNum = ffi::nrd_HitDistanceReconstructionMode_MAX_NUM,
}
