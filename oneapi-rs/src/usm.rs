use std::{alloc::Layout, ptr::NonNull};

use oneapi_rs_sys::usm::ffi;

use crate::{allocator::{AllocError, Allocator}, queue::Queue};

pub struct DeviceAllocator<'a> {
    queue: &'a Queue
}

impl<'a> From<&'a Queue> for DeviceAllocator<'a> {
    fn from(queue: &'a Queue) -> Self {
        Self { queue }
    }
}

unsafe impl<'a> Allocator for DeviceAllocator<'a> {
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        let ptr = unsafe { ffi::malloc_device(layout.size(), &self.queue.0) }
            .map_err(|e| AllocError::Other(e.to_string()))?;

        let ptr = NonNull::new(ptr).ok_or(AllocError::NoMemory)?;
        let slice = NonNull::slice_from_raw_parts(ptr, layout.size());

        Ok(slice)
    }

    unsafe fn deallocate(&self, ptr: NonNull<u8>, _layout: Layout) {
        unsafe { ffi::free(ptr.as_ptr(), &self.queue.0); }
    }
}
