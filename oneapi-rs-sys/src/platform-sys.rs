//
// Copyright (C) 2026 Intel Corporation
//
// Under the MIT License or the Apache License v2.0.
// See LICENSE-MIT and LICENSE-APACHE for license information.
// SPDX-License-Identifier: MIT OR Apache-2.0
//

#[cxx::bridge(namespace = "sycl_shims::platform")]
pub mod ffi {
    #[namespace = "sycl_shims"]
    extern "C++" {
        include!("oneapi-rs-sys/src/opaque-sys.rs.h");
        type DevicePtr = crate::opaque::ffi::DevicePtr;
        type PlatformPtr = crate::opaque::ffi::PlatformPtr;
    }

    unsafe extern "C++" {
        include!("oneapi-rs-sys/include/platform.hpp");

        #[namespace = "sycl_shims"]
        type Device = crate::opaque::ffi::Device;
        #[namespace = "sycl_shims"]
        type Platform = crate::opaque::ffi::Platform;

        fn get_platforms() -> Vec<PlatformPtr>;

        fn get_devices(platform: &Platform) -> Vec<DevicePtr>;

        fn get_version(platform: &Platform) -> String;
        fn get_name(platform: &Platform) -> String;
        fn get_vendor(platform: &Platform) -> String;
    }
}
