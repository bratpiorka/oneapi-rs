//
// Copyright (C) 2026 Intel Corporation
//
// Under the MIT License or the Apache License v2.0.
// See LICENSE-MIT and LICENSE-APACHE for license information.
// SPDX-License-Identifier: MIT OR Apache-2.0
//

#[cxx::bridge(namespace = "sycl_shims::queue")]
pub mod ffi {
    unsafe extern "C++" {
        include!("oneapi-rs-sys/include/queue.hpp");

        #[namespace = "sycl_shims"]
        type Queue = crate::types::ffi::Queue;
        #[namespace = "sycl_shims"]
        type Device = crate::types::ffi::Device;

        fn new_queue() -> UniquePtr<Queue>;
        fn new_queue_from_device(device: &Device) -> UniquePtr<Queue>;
    }
}
