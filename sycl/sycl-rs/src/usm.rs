//
// Copyright (C) 2026 Intel Corporation
//
// Under the MIT License or the Apache License v2.0.
// See LICENSE-MIT and LICENSE-APACHE for license information.
// SPDX-License-Identifier: MIT OR Apache-2.0
//

use crate::{SyclError, queue::Queue};

use allocator_api2::alloc::{AllocError, Allocator};
use sycl_rs_sys::usm::ffi;

use std::{alloc::Layout, io::Write, marker::PhantomData, ptr::NonNull};

type CxxResult<T> = cxx::core::result::Result<T, cxx::Exception>;

/// An instance of a USM allocator.
pub struct UsmAllocator<T: UsmAllocatorKind> {
    queue: Queue,
    _kind: PhantomData<T>,
}

/// A marker trait for USM allocators.
///
/// # Safety
///
/// A type implementing this trait must be a valid USM allocator managed by a SYCL runtime.
pub unsafe trait UsmAlloc: Allocator {}

unsafe impl<T: UsmAllocatorKind> UsmAlloc for UsmAllocator<T> {}

pub trait UsmAllocatorKind {
    /// Allocates uninitialized memory.
    ///
    /// # Safety
    ///
    /// The caller must not read uninitialized memory. The caller must also free this
    /// memory manually.
    unsafe fn alloc(alignment: usize, num_bytes: usize, queue: &Queue) -> CxxResult<*mut u8>;
}

/// A marker trait for host-accessible USM allocators.
///
/// # Safety
///
/// A type implementing this trait must be a valid USM allocator managed by a SYCL runtime,
/// that allocates memory accessible from the host.
pub unsafe trait HostAccessible {}

impl<T: UsmAllocatorKind> TryFrom<&Queue> for UsmAllocator<T> {
    type Error = SyclError;

    fn try_from(queue: &Queue) -> crate::Result<Self> {
        Ok(Self {
            queue: sycl_rs_sys::queue::ffi::clone(&queue.0)?.into(),
            _kind: PhantomData,
        })
    }
}

unsafe impl<T: UsmAllocatorKind> Allocator for UsmAllocator<T> {
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        let ptr = unsafe { T::alloc(layout.align(), layout.size(), &self.queue) }
            .map_err(|_| AllocError)?;
        let ptr = NonNull::new(ptr).ok_or(AllocError)?;
        Ok(NonNull::slice_from_raw_parts(ptr, layout.size()))
    }

    unsafe fn deallocate(&self, ptr: NonNull<u8>, _layout: Layout) {
        if let Err(error) = unsafe { ffi::free(ptr.as_ptr(), &self.queue.0) } {
            // A failed stderr write must not cause a second panic during unwinding.
            let _ = writeln!(
                std::io::stderr().lock(),
                "sycl-rs: USM deallocation failed: {error}"
            );
        }
    }
}

/// An allocator for Device-side arrays
///
/// Safety: memory allocated by this allocator cannot be accessed on the host side
#[allow(dead_code)]
pub struct DeviceAllocator;

impl UsmAllocatorKind for DeviceAllocator {
    unsafe fn alloc(alignment: usize, num_bytes: usize, queue: &Queue) -> CxxResult<*mut u8> {
        unsafe { ffi::aligned_alloc_device(alignment, num_bytes, &queue.0) }
    }
}

/// An allocator for Host-side arrays
pub struct HostAllocator;

impl UsmAllocatorKind for HostAllocator {
    unsafe fn alloc(alignment: usize, num_bytes: usize, queue: &Queue) -> CxxResult<*mut u8> {
        unsafe { ffi::aligned_alloc_host(alignment, num_bytes, &queue.0) }
    }
}

unsafe impl HostAccessible for UsmAllocator<HostAllocator> {}

/// An allocator for shared memory arrays
pub struct SharedAllocator;

impl UsmAllocatorKind for SharedAllocator {
    unsafe fn alloc(alignment: usize, num_bytes: usize, queue: &Queue) -> CxxResult<*mut u8> {
        unsafe { ffi::aligned_alloc_shared(alignment, num_bytes, &queue.0) }
    }
}

unsafe impl HostAccessible for UsmAllocator<SharedAllocator> {}
