use std::{alloc::Layout, ptr::NonNull};

use oneapi_rs_sys::usm::ffi;

use crate::{allocator::{AllocError, Allocator}, queue::Queue};
type CxxResult<T> = cxx::core::result::Result<T, cxx::Exception>;

trait UsmAllocator {
    unsafe fn alloc(&self, alignment: usize, bytes: usize) -> CxxResult<*mut u8>;
    fn get_queue(&self) -> &Queue;
}

pub struct DeviceAllocator<'a> {
    queue: &'a Queue
}

impl<'a> From<&'a Queue> for DeviceAllocator<'a> {
    fn from(queue: &'a Queue) -> Self {
        Self { queue }
    }
}

impl<'a> UsmAllocator for DeviceAllocator<'a> {
    unsafe fn alloc(&self, alignment: usize, bytes: usize) -> CxxResult<*mut u8> {
        unsafe { ffi::aligned_alloc_device(alignment, bytes, &self.queue.0) }
    }

    fn get_queue(&self) -> &Queue {
        &self.queue
    }
}

unsafe impl<T> Allocator for T
where T: UsmAllocator {
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        let ptr = unsafe { self.alloc(layout.align(), layout.size()) }
            .map_err(|e| AllocError::Other(e.to_string()))?;

        let ptr = NonNull::new(ptr).ok_or(AllocError::NoMemory)?;
        let slice = NonNull::slice_from_raw_parts(ptr, layout.size());

        Ok(slice)
    }

    unsafe fn deallocate(&self, ptr: NonNull<u8>, _layout: Layout) {
        unsafe { ffi::free(ptr.as_ptr(), &self.get_queue().0); }
    }
}
