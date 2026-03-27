//! Safe, idiomatic wrappers over the raw [`crate::ffi`] bindings.

mod enums;
mod error;
mod instance;
mod library;

pub use enums::*;
pub use error::Error;
pub(crate) use error::result_from_ffi;

/// Fallible NRD operations use this [`Result`](std::result::Result) type.
pub type NrdResult<T> = std::result::Result<T, Error>;
pub use instance::{
    allocation_callbacks_none, DenoiserSlot, DispatchDesc, Instance, InstanceDescription,
    ResourceBinding,
};
pub use library::LibraryInfo;

use std::ffi::CStr;

/// Per-denoiser id assigned by the application; must be unique within an instance.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Identifier(pub u32);

// ——— Settings (layout-identical to NRD C structs) ———

pub type CommonSettings = crate::ffi::nrd_CommonSettings;
pub type ReblurHitDistanceParameters = crate::ffi::nrd_ReblurHitDistanceParameters;
pub type ReblurAntilagSettings = crate::ffi::nrd_ReblurAntilagSettings;
pub type ReblurResponsiveAccumulationSettings = crate::ffi::nrd_ReblurResponsiveAccumulationSettings;
pub type ReblurConvergenceSettings = crate::ffi::nrd_ReblurConvergenceSettings;
pub type ReblurSettings = crate::ffi::nrd_ReblurSettings;
pub type RelaxAntilagSettings = crate::ffi::nrd_RelaxAntilagSettings;
pub type RelaxSettings = crate::ffi::nrd_RelaxSettings;
pub type SigmaSettings = crate::ffi::nrd_SigmaSettings;
pub type ReferenceSettings = crate::ffi::nrd_ReferenceSettings;

pub type TextureDesc = crate::ffi::nrd_TextureDesc;
pub type PipelineDesc = crate::ffi::nrd_PipelineDesc;
pub type ComputeShaderDesc = crate::ffi::nrd_ComputeShaderDesc;
pub type ResourceRangeDesc = crate::ffi::nrd_ResourceRangeDesc;
pub type DescriptorPoolDesc = crate::ffi::nrd_DescriptorPoolDesc;
pub type InstanceCreationDesc = crate::ffi::nrd_InstanceCreationDesc;
pub type AllocationCallbacks = crate::ffi::nrd_AllocationCallbacks;
pub type SpirvBindingOffsets = crate::ffi::nrd_SPIRVBindingOffsets;

pub const REBLUR_MAX_HISTORY_FRAME_NUM: u32 = crate::ffi::nrd_REBLUR_MAX_HISTORY_FRAME_NUM;
pub const REBLUR_DEFAULT_ACCUMULATION_TIME: f32 = crate::ffi::nrd_REBLUR_DEFAULT_ACCUMULATION_TIME;
pub const RELAX_MAX_HISTORY_FRAME_NUM: u32 = crate::ffi::nrd_RELAX_MAX_HISTORY_FRAME_NUM;
pub const RELAX_DEFAULT_ACCUMULATION_TIME: f32 = crate::ffi::nrd_RELAX_DEFAULT_ACCUMULATION_TIME;
pub const SIGMA_MAX_HISTORY_FRAME_NUM: u32 = crate::ffi::nrd_SIGMA_MAX_HISTORY_FRAME_NUM;
pub const SIGMA_DEFAULT_ACCUMULATION_TIME: f32 = crate::ffi::nrd_SIGMA_DEFAULT_ACCUMULATION_TIME;
pub const REFERENCE_MAX_HISTORY_FRAME_NUM: u32 = crate::ffi::nrd_REFERENCE_MAX_HISTORY_FRAME_NUM;
pub const REFERENCE_DEFAULT_ACCUMULATION_TIME: f32 =
    crate::ffi::nrd_REFERENCE_DEFAULT_ACCUMULATION_TIME;

/// Matches `nrd::GetMaxAccumulatedFrameNum` in NRDSettings.h.
#[inline]
pub fn max_accumulated_frame_num(accumulation_time_seconds: f32, fps: f32) -> u32 {
    (accumulation_time_seconds * fps + 0.5) as u32
}

/// English name for a resource type (NRD helper).
pub fn resource_type_name(ty: ResourceType) -> Option<&'static CStr> {
    let p = unsafe { crate::ffi::nrd_GetResourceTypeString(ty as u32) };
    if p.is_null() {
        return None;
    }
    Some(unsafe { CStr::from_ptr(p) })
}

/// English name for a denoiser kind (NRD helper).
pub fn denoiser_name(d: Denoiser) -> Option<&'static CStr> {
    let p = unsafe { crate::ffi::nrd_GetDenoiserString(d as u32) };
    if p.is_null() {
        return None;
    }
    Some(unsafe { CStr::from_ptr(p) })
}

/// Default [`CommonSettings`] matching NRD C++ struct defaults.
pub fn default_common_settings() -> CommonSettings {
    CommonSettings {
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
        denoisingRange: 500_000.0,
        disocclusionThreshold: 0.01,
        disocclusionThresholdAlternate: 0.05,
        cameraAttachedReflectionMaterialID: 999.0,
        strandMaterialID: 999.0,
        historyFixAlternatePixelStrideMaterialID: 999.0,
        strandThickness: 80e-6,
        splitScreen: 0.0,
        printfAt: [9999, 9999],
        debug: 0.0,
        rectOrigin: [0; 2],
        frameIndex: 0,
        accumulationMode: AccumulationMode::Continue as u8,
        isMotionVectorInWorldSpace: false,
        isHistoryConfidenceAvailable: false,
        isDisocclusionThresholdMixAvailable: false,
        enableValidation: false,
    }
}

/// Default [`ReblurSettings`] matching NRD C++ struct defaults.
pub fn default_reblur_settings() -> ReblurSettings {
    use crate::ffi;
    ReblurSettings {
        hitDistanceParameters: ReblurHitDistanceParameters {
            A: 3.0,
            B: 0.1,
            C: 20.0,
        },
        antilagSettings: ReblurAntilagSettings {
            luminanceSigmaScale: 2.0,
            luminanceSensitivity: 3.0,
        },
        responsiveAccumulationSettings: ReblurResponsiveAccumulationSettings {
            roughnessThreshold: 0.0,
            minAccumulatedFrameNum: 3,
        },
        convergenceSettings: ReblurConvergenceSettings {
            s: 1.0,
            b: 0.2,
            p: 0.8,
        },
        maxAccumulatedFrameNum: 30,
        maxFastAccumulatedFrameNum: 6,
        maxStabilizedFrameNum: ffi::nrd_REBLUR_MAX_HISTORY_FRAME_NUM,
        historyFixFrameNum: 3,
        historyFixBasePixelStride: 14,
        historyFixAlternatePixelStride: 14,
        fastHistoryClampingSigmaScale: 2.0,
        diffusePrepassBlurRadius: 30.0,
        specularPrepassBlurRadius: 50.0,
        minHitDistanceWeight: 0.1,
        minBlurRadius: 1.0,
        maxBlurRadius: 30.0,
        lobeAngleFraction: 0.15,
        roughnessFraction: 0.15,
        planeDistanceSensitivity: 0.02,
        specularProbabilityThresholdsForMvModification: [0.5, 0.9],
        fireflySuppressorMinRelativeScale: 2.0,
        minMaterialForDiffuse: 4.0,
        minMaterialForSpecular: 4.0,
        checkerboardMode: CheckerboardMode::Off as u8,
        hitDistanceReconstructionMode: HitDistanceReconstructionMode::Off as u8,
        enableAntiFirefly: true,
        usePrepassOnlyForSpecularMotionEstimation: false,
        returnHistoryLengthInsteadOfOcclusion: false,
    }
}

/// Default [`RelaxSettings`] matching NRD C++ struct defaults.
pub fn default_relax_settings() -> RelaxSettings {
    RelaxSettings {
        antilagSettings: RelaxAntilagSettings {
            accelerationAmount: 0.3,
            spatialSigmaScale: 4.5,
            temporalSigmaScale: 0.5,
            resetAmount: 0.5,
        },
        diffuseMaxAccumulatedFrameNum: 30,
        specularMaxAccumulatedFrameNum: 30,
        diffuseMaxFastAccumulatedFrameNum: 6,
        specularMaxFastAccumulatedFrameNum: 6,
        historyFixFrameNum: 3,
        historyFixBasePixelStride: 14,
        historyFixAlternatePixelStride: 14,
        historyFixEdgeStoppingNormalPower: 8.0,
        fastHistoryClampingSigmaScale: 2.0,
        diffusePrepassBlurRadius: 30.0,
        specularPrepassBlurRadius: 50.0,
        minHitDistanceWeight: 0.1,
        spatialVarianceEstimationHistoryThreshold: 3,
        diffusePhiLuminance: 2.0,
        specularPhiLuminance: 1.0,
        lobeAngleFraction: 0.5,
        roughnessFraction: 0.15,
        specularVarianceBoost: 0.0,
        specularLobeAngleSlack: 0.15,
        atrousIterationNum: 5,
        diffuseMinLuminanceWeight: 0.0,
        specularMinLuminanceWeight: 0.0,
        depthThreshold: 0.003,
        confidenceDrivenRelaxationMultiplier: 0.0,
        confidenceDrivenLuminanceEdgeStoppingRelaxation: 0.0,
        confidenceDrivenNormalEdgeStoppingRelaxation: 0.0,
        luminanceEdgeStoppingRelaxation: 0.5,
        normalEdgeStoppingRelaxation: 0.3,
        roughnessEdgeStoppingRelaxation: 1.0,
        checkerboardMode: CheckerboardMode::Off as u8,
        hitDistanceReconstructionMode: HitDistanceReconstructionMode::Off as u8,
        minMaterialForDiffuse: 4.0,
        minMaterialForSpecular: 4.0,
        enableAntiFirefly: false,
        enableRoughnessEdgeStopping: true,
    }
}

/// Default [`SigmaSettings`] matching NRD C++ struct defaults.
pub fn default_sigma_settings() -> SigmaSettings {
    SigmaSettings {
        lightDirection: [0.0, 0.0, 0.0],
        planeDistanceSensitivity: 0.02,
        maxStabilizedFrameNum: 5,
    }
}

/// Default [`ReferenceSettings`] matching NRD C++ struct defaults.
pub fn default_reference_settings() -> ReferenceSettings {
    ReferenceSettings {
        maxAccumulatedFrameNum: 120,
    }
}
