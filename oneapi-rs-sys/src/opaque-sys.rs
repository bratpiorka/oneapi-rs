//
// Copyright (C) 2026 Intel Corporation
//
// Under the MIT License or the Apache License v2.0.
// See LICENSE-MIT and LICENSE-APACHE for license information.
// SPDX-License-Identifier: MIT OR Apache-2.0
//

#[cxx::bridge(namespace = "sycl_shims")]
pub mod ffi {
    unsafe extern "C++" {
        include!("oneapi-rs-sys/include/opaque.hpp");
        type Device;
        type Platform;
    }

    impl UniquePtr<Device> {}
    impl UniquePtr<Platform> {}
}
