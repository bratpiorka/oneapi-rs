//
// Copyright (C) 2026 Intel Corporation
//
// Under the MIT License or the Apache License v2.0.
// See LICENSE-MIT and LICENSE-APACHE for license information.
// SPDX-License-Identifier: MIT OR Apache-2.0
//

// WA for Clippy issue https://github.com/rust-lang/rust-clippy/issues/16317
#[allow(clippy::missing_safety_doc)]
#[cxx::bridge(namespace = "sycl_shims::usm")]
pub mod ffi {
    unsafe extern "C++" {
        #[namespace = "sycl_shims"]
        type Queue = crate::types::ffi::Queue;
    }

    extern "C++" {
        include!("sycl-rs-sys/include/usm.hpp");
        /// # Safety
        ///
        /// `queue` must refer to a valid SYCL queue and `alignment` must be a supported power of
        /// two. The returned allocation must be released with `free` using a compatible queue.
        unsafe fn aligned_alloc_device(
            alignment: usize,
            num_bytes: usize,
            queue: &Queue,
        ) -> Result<*mut u8>;
        /// # Safety
        ///
        /// `queue` must refer to a valid SYCL queue and `alignment` must be a supported power of
        /// two. The returned allocation must be released with `free` using a compatible queue.
        unsafe fn aligned_alloc_host(
            alignment: usize,
            num_bytes: usize,
            queue: &Queue,
        ) -> Result<*mut u8>;
        /// # Safety
        ///
        /// `queue` must refer to a valid SYCL queue and `alignment` must be a supported power of
        /// two. The returned allocation must be released with `free` using a compatible queue.
        unsafe fn aligned_alloc_shared(
            alignment: usize,
            num_bytes: usize,
            queue: &Queue,
        ) -> Result<*mut u8>;
        /// # Safety
        ///
        /// `ptr` must be a live USM allocation associated with `queue` and must not be used after
        /// this call.
        unsafe fn free(ptr: *mut u8, queue: &Queue);
    }
}
