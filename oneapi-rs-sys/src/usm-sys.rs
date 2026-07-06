//
// Copyright (C) 2026 Intel Corporation
//
// Under the MIT License or the Apache License v2.0.
// See LICENSE-MIT and LICENSE-APACHE for license information.
// SPDX-License-Identifier: MIT OR Apache-2.0
//

#[cxx::bridge(namespace = "sycl_shims::usm")]
pub mod ffi {
    unsafe extern "C++" {
        #[namespace = "sycl_shims"]
        type Queue = crate::types::ffi::Queue;
    }

    extern "C++" {
        include!("oneapi-rs-sys/include/usm.hpp");
        unsafe fn malloc_device(size: usize, queue: &Queue) -> Result<*mut u8>;
    }
}
