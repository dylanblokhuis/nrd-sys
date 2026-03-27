use std::{
    ffi::{c_void, CStr},
    fmt,
    ops::Deref,
    ptr,
};

use crate::api::{
    result_from_ffi, Denoiser, DescriptorType, Error, Identifier, ResourceType, Sampler,
};
use crate::ffi;

/// One entry in [`Instance::try_new`]'s denoiser list.
#[derive(Debug, Clone, Copy)]
pub struct DenoiserSlot {
    pub identifier: Identifier,
    pub denoiser: Denoiser,
}

/// Immutable view of [`ffi::nrd_InstanceDesc`]; pointers are valid for the lifetime of the [`Instance`].
#[derive(Debug, Clone, Copy)]
pub struct InstanceDescription<'a> {
    raw: &'a ffi::nrd_InstanceDesc,
}

impl<'a> InstanceDescription<'a> {
    pub fn raw(&self) -> &'a ffi::nrd_InstanceDesc {
        self.raw
    }

    pub fn constant_buffer_and_samplers_space_index(&self) -> u32 {
        self.raw.constantBufferAndSamplersSpaceIndex
    }

    pub fn resources_space_index(&self) -> u32 {
        self.raw.resourcesSpaceIndex
    }

    pub fn constant_buffer_register_index(&self) -> u32 {
        self.raw.constantBufferRegisterIndex
    }

    pub fn samplers_base_register_index(&self) -> u32 {
        self.raw.samplersBaseRegisterIndex
    }

    pub fn resources_base_register_index(&self) -> u32 {
        self.raw.resourcesBaseRegisterIndex
    }

    pub fn constant_buffer_max_data_size(&self) -> u32 {
        self.raw.constantBufferMaxDataSize
    }

    pub fn samplers(&self) -> &'a [Sampler] {
        if self.raw.samplers.is_null() || self.raw.samplersNum == 0 {
            return &[];
        }
        unsafe {
            std::slice::from_raw_parts(self.raw.samplers as *const Sampler, self.raw.samplersNum as usize)
        }
    }

    pub fn shader_entry_point(&self) -> Option<&'a CStr> {
        if self.raw.shaderEntryPoint.is_null() {
            return None;
        }
        Some(unsafe { CStr::from_ptr(self.raw.shaderEntryPoint) })
    }

    pub fn pipelines(&self) -> &'a [ffi::nrd_PipelineDesc] {
        if self.raw.pipelines.is_null() || self.raw.pipelinesNum == 0 {
            return &[];
        }
        unsafe {
            std::slice::from_raw_parts(self.raw.pipelines, self.raw.pipelinesNum as usize)
        }
    }

    pub fn permanent_pool(&self) -> &'a [ffi::nrd_TextureDesc] {
        if self.raw.permanentPool.is_null() || self.raw.permanentPoolSize == 0 {
            return &[];
        }
        unsafe {
            std::slice::from_raw_parts(self.raw.permanentPool, self.raw.permanentPoolSize as usize)
        }
    }

    pub fn transient_pool(&self) -> &'a [ffi::nrd_TextureDesc] {
        if self.raw.transientPool.is_null() || self.raw.transientPoolSize == 0 {
            return &[];
        }
        unsafe {
            std::slice::from_raw_parts(self.raw.transientPool, self.raw.transientPoolSize as usize)
        }
    }

    pub fn descriptor_pool_desc(&self) -> &ffi::nrd_DescriptorPoolDesc {
        &self.raw.descriptorPoolDesc
    }
}

/// Opaque NRD instance. Destroyed on drop.
pub struct Instance(*mut ffi::nrd_Instance);

impl Instance {
    /// Creates an instance from a populated [`ffi::nrd_InstanceCreationDesc`].
    pub fn try_new(desc: &ffi::nrd_InstanceCreationDesc) -> Result<Self, Error> {
        let mut instance = ptr::null_mut();
        let code = unsafe { ffi::nrd_CreateInstance(desc, &mut instance) };
        result_from_ffi(code)?;
        if instance.is_null() {
            return Err(Error::Failure);
        }
        Ok(Self(instance))
    }

    /// Convenience: build [`ffi::nrd_InstanceCreationDesc`] from slots and default (null) allocators.
    pub fn try_new_denoisers(denoisers: &[DenoiserSlot]) -> Result<Self, Error> {
        let alloc = allocation_callbacks_none();
        let ffi_denoisers: Vec<ffi::nrd_DenoiserDesc> = denoisers
            .iter()
            .map(|s| ffi::nrd_DenoiserDesc {
                identifier: s.identifier.0,
                denoiser: s.denoiser as u32,
            })
            .collect();
        let desc = ffi::nrd_InstanceCreationDesc {
            allocationCallbacks: alloc,
            denoisers: ffi_denoisers.as_ptr(),
            denoisersNum: ffi_denoisers.len() as u32,
        };
        Self::try_new(&desc)
    }

    pub fn description(&self) -> Result<InstanceDescription<'_>, Error> {
        let p = unsafe { ffi::nrd_GetInstanceDesc(self.0) };
        if p.is_null() {
            return Err(Error::Failure);
        }
        Ok(InstanceDescription {
            raw: unsafe { &*p },
        })
    }

    pub fn set_common_settings(&mut self, settings: &ffi::nrd_CommonSettings) -> Result<(), Error> {
        let code = unsafe { ffi::nrd_SetCommonSettings(self.0, settings) };
        result_from_ffi(code)
    }

    pub fn set_reblur_settings(
        &mut self,
        id: Identifier,
        settings: &ffi::nrd_ReblurSettings,
    ) -> Result<(), Error> {
        let code = unsafe {
            ffi::nrd_SetDenoiserSettings(self.0, id.0, settings as *const _ as *const c_void)
        };
        result_from_ffi(code)
    }

    pub fn set_relax_settings(
        &mut self,
        id: Identifier,
        settings: &ffi::nrd_RelaxSettings,
    ) -> Result<(), Error> {
        let code = unsafe {
            ffi::nrd_SetDenoiserSettings(self.0, id.0, settings as *const _ as *const c_void)
        };
        result_from_ffi(code)
    }

    pub fn set_sigma_settings(
        &mut self,
        id: Identifier,
        settings: &ffi::nrd_SigmaSettings,
    ) -> Result<(), Error> {
        let code = unsafe {
            ffi::nrd_SetDenoiserSettings(self.0, id.0, settings as *const _ as *const c_void)
        };
        result_from_ffi(code)
    }

    pub fn set_reference_settings(
        &mut self,
        id: Identifier,
        settings: &ffi::nrd_ReferenceSettings,
    ) -> Result<(), Error> {
        let code = unsafe {
            ffi::nrd_SetDenoiserSettings(self.0, id.0, settings as *const _ as *const c_void)
        };
        result_from_ffi(code)
    }

    /// Returned [`DispatchDesc`] values point at NRD-owned memory; the slice is invalidated by the next call on this instance.
    pub fn compute_dispatches(&mut self, identifiers: &[Identifier]) -> Result<Vec<DispatchDesc<'_>>, Error> {
        let ids: Vec<u32> = identifiers.iter().map(|i| i.0).collect();
        let mut len: u32 = 0;
        let mut dispatches: *const ffi::nrd_DispatchDesc = ptr::null();
        let code = unsafe {
            ffi::nrd_GetComputeDispatches(
                self.0,
                ids.as_ptr(),
                ids.len() as u32,
                &mut dispatches as *mut _,
                &mut len as *mut _,
            )
        };
        result_from_ffi(code)?;
        if len == 0 || dispatches.is_null() {
            return Ok(Vec::new());
        }
        let mut out = Vec::with_capacity(len as usize);
        for i in 0..len as usize {
            out.push(DispatchDesc(
                unsafe { dispatches.add(i) },
                std::marker::PhantomData,
            ));
        }
        Ok(out)
    }

    pub fn as_raw_mut(&mut self) -> *mut ffi::nrd_Instance {
        self.0
    }

    pub fn as_raw(&self) -> *const ffi::nrd_Instance {
        self.0
    }
}

impl Drop for Instance {
    fn drop(&mut self) {
        if !self.0.is_null() {
            unsafe {
                ffi::nrd_DestroyInstance(self.0);
            }
            self.0 = ptr::null_mut();
        }
    }
}

/// NRD-owned dispatch descriptor; do not use after the next [`Instance::compute_dispatches`] on the same instance.
#[derive(Clone, Copy)]
pub struct DispatchDesc<'a>(
    *const ffi::nrd_DispatchDesc,
    std::marker::PhantomData<&'a ()>,
);

impl<'a> Deref for DispatchDesc<'a> {
    type Target = ffi::nrd_DispatchDesc;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}

impl<'a> fmt::Debug for DispatchDesc<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DispatchDesc")
            .field("name", &self.name())
            .field("identifier", &self.identifier)
            .field("resources_num", &self.resourcesNum)
            .field("constant_buffer_data_size", &self.constantBufferDataSize)
            .field(
                "constant_buffer_data_matches_previous_dispatch",
                &self.constantBufferDataMatchesPreviousDispatch,
            )
            .field("pipeline_index", &self.pipelineIndex)
            .field("grid_width", &self.gridWidth)
            .field("grid_height", &self.gridHeight)
            .finish()
    }
}

impl<'a> DispatchDesc<'a> {
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

    pub fn resources(&self) -> &[ResourceBinding] {
        if self.resources.is_null() || self.resourcesNum == 0 {
            return &[];
        }
        unsafe {
            std::slice::from_raw_parts(self.resources as *const ResourceBinding, self.resourcesNum as usize)
        }
    }

    pub fn name(&self) -> Option<&CStr> {
        if self.name.is_null() {
            return None;
        }
        Some(unsafe { CStr::from_ptr(self.name) })
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ResourceBinding {
    pub descriptor_type: DescriptorType,
    pub resource_type: ResourceType,
    pub index_in_pool: u16,
}

pub fn allocation_callbacks_none() -> ffi::nrd_AllocationCallbacks {
    ffi::nrd_AllocationCallbacks {
        Allocate: None,
        Reallocate: None,
        Free: None,
        userArg: ptr::null_mut(),
    }
}
