use std::ffi::c_void;

pub mod ffi;

pub use ffi::{
    AccumulationMode, CheckerboardMode, CommonSettings, Denoiser, DenoiserDesc, DescriptorType,
    DispatchDesc, Format, HitDistanceParameters, HitDistanceReconstructionMode, Identifier,
    ReblurAntilagSettings, ReblurSettings, ReferenceSettings, RelaxAntilagSettings, ResourceDesc,
    ResourceType, SPIRVBindingOffsets, Sampler, SigmaSettings, TextureDesc,
};
mod allocator {
    use std::alloc::{alloc, dealloc, Layout};
    use std::ffi::c_void;
    use std::ptr;

    /// A plain malloc-based allocate callback
    #[no_mangle]
    pub extern "C" fn allocate(
        _user_arg: *const c_void,
        size: usize,
        alignment: usize,
    ) -> *mut c_void {
        // SAFETY: `from_size_align_unchecked` is fine because the C API promises
        // to only ever ask for sane size+alignment pairs.
        let layout = unsafe { Layout::from_size_align_unchecked(size, alignment) };
        let ptr = unsafe { alloc(layout) };
        if ptr.is_null() {
            ptr::null_mut()
        } else {
            ptr as *mut c_void
        }
    }

    /// A realloc callback built on std::alloc::realloc
    #[no_mangle]
    pub extern "C" fn reallocate(
        _user_arg: *const c_void,
        memory: *mut c_void,
        old_size: usize,
        old_alignment: usize,
        new_size: usize,
        new_alignment: usize,
    ) -> *mut c_void {
        // first deallocate the old block, then alloc a new one:
        // (you could also use `std::alloc::realloc` directly, see below)
        free(_user_arg, memory, old_size, old_alignment);
        allocate(_user_arg, new_size, new_alignment)

        // OR — if you trust the allocator to do in-place grow/shrink:
        /*
        let old_layout = unsafe { Layout::from_size_align_unchecked(old_size, old_alignment) };
        let new_ptr = unsafe { realloc(memory as *mut u8, old_layout, new_size) };
        new_ptr as *mut c_void
        */
    }

    /// A deallocate callback
    #[no_mangle]
    pub extern "C" fn free(
        _user_arg: *const c_void,
        memory: *mut c_void,
        size: usize,
        alignment: usize,
    ) {
        if memory.is_null() {
            return;
        }
        let layout = unsafe { Layout::from_size_align_unchecked(size, alignment) };
        unsafe {
            dealloc(memory as *mut u8, layout);
        }
    }
}

pub trait DenoiserSettings {}
impl DenoiserSettings for ffi::ReblurSettings {}
impl DenoiserSettings for ffi::RelaxSettings {}
impl DenoiserSettings for ffi::ReferenceSettings {}
impl DenoiserSettings for ffi::SigmaSettings {}

pub struct Instance(*mut c_void);
unsafe impl Send for Instance {}
unsafe impl Sync for Instance {}

impl Instance {
    pub fn library_desc() -> &'static ffi::LibraryDesc {
        unsafe { ffi::GetLibraryDesc() }
    }
    pub fn new(denoisers: &[ffi::DenoiserDesc]) -> Result<Self, ffi::Result> {
        let desc = ffi::InstanceCreationDesc {
            allocation_allbacks: ffi::AllocationCallbacks {
                allocate: allocator::allocate,
                reallocate: allocator::reallocate,
                free: allocator::free,
                user_arg: std::ptr::null(),
            },
            denoisers: denoisers.as_ptr(),
            denoisers_num: denoisers.len() as u32,
        };
        let mut ptr: *mut c_void = std::ptr::null_mut();
        let result = unsafe { ffi::CreateInstance(&desc, &mut ptr) };
        match result {
            ffi::Result::Success => Ok(Self(ptr)),
            _ => Err(result),
        }
    }

    pub fn desc(&self) -> &ffi::InstanceDesc {
        unsafe {
            let ptr = ffi::GetInstanceDesc(self.0);
            &*ptr
        }
    }

    pub fn set_common_settings(
        &mut self,
        settings: &ffi::CommonSettings,
    ) -> Result<(), ffi::Result> {
        unsafe {
            let result = ffi::SetCommonSettings(self.0, settings);
            match result {
                ffi::Result::Success => Ok(()),
                _ => Err(result),
            }
        }
    }
    pub fn set_denoiser_settings<T>(
        &mut self,
        identifier: ffi::Identifier,
        reblur_settings: &T,
    ) -> Result<(), ffi::Result> {
        unsafe {
            ffi::SetDenoiserSettings(
                self.0,
                identifier,
                reblur_settings as *const _ as *const c_void,
            )
            .ok(())
        }
    }

    pub fn get_compute_dispatches(
        &mut self,
        identifiers: &[ffi::Identifier],
    ) -> Result<&[ffi::DispatchDesc], ffi::Result> {
        unsafe {
            let mut dispatches: *const ffi::DispatchDesc = std::ptr::null();
            let mut dispatches_count: u32 = 0;
            let result = ffi::GetComputeDispatches(
                self.0,
                identifiers.as_ptr(),
                identifiers.len() as u32,
                &mut dispatches,
                &mut dispatches_count,
            );
            match result {
                ffi::Result::Success => Ok(std::slice::from_raw_parts(
                    dispatches,
                    dispatches_count as usize,
                )),
                _ => Err(result),
            }
        }
    }
}

impl Drop for Instance {
    fn drop(&mut self) {
        unsafe {
            ffi::DestroyInstance(self.0);
        }
    }
}
