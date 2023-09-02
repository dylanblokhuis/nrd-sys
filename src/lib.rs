#![feature(allocator_api)]

use std::{
    alloc::{AllocError, Allocator, Layout},
    ffi::c_void,
    ptr::NonNull,
};

pub mod ffi;

mod allocator {
    use std::alloc::{Allocator, Layout};
    use std::ffi::c_void;
    use std::ptr::NonNull;
    pub extern "C" fn allocate(
        _user_arg: *const c_void,
        size: usize,
        alignment: usize,
    ) -> *mut c_void {
        println!("Allocated {} {}", size, alignment);
        unsafe {
            match std::alloc::Global.allocate(Layout::from_size_align_unchecked(size, alignment)) {
                Ok(ptr) => ptr.as_ptr() as *mut c_void,
                Err(_) => std::ptr::null_mut(),
            }
        }
    }
    pub extern "C" fn reallocate(
        user_arg: *const c_void,
        memory: *mut c_void,
        old_size: usize,
        old_alignment: usize,
        new_size: usize,
        new_alignment: usize,
    ) -> *mut c_void {
        println!("Reallocated");
        free(user_arg, memory, old_size, old_alignment);
        allocate(user_arg, new_size, new_alignment)
    }
    pub extern "C" fn free(_user_arg: *const c_void, memory: *mut c_void, size: usize, alignment: usize) {
        println!("Freed {} {}", size, alignment);
        let memory = NonNull::new(memory).unwrap();
        unsafe {
            std::alloc::Global.deallocate(memory.cast(), Layout::from_size_align_unchecked(size, alignment))
        }
    }
}

pub struct Instance(*mut c_void);
impl Instance {
    pub fn new(
        denoisers: &[ffi::DenoiserDesc],
    ) -> Result<Self, ffi::Result> {
        let desc = ffi::InstanceCreationDesc {
            memory_allocator_interface: ffi::MemoryAllocatorInterface {
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
}

impl Drop for Instance {
    fn drop(&mut self) {
        unsafe {
            ffi::DestroyInstance(self.0);
        }
    }
}
