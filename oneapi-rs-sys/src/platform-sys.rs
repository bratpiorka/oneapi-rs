//
// Copyright (C) 2026 Intel Corporation
//
// Under the MIT License or the Apache License v2.0.
// See LICENSE-MIT and LICENSE-APACHE for license information.
// SPDX-License-Identifier: MIT OR Apache-2.0
//

#[cxx::bridge(namespace = "sycl_shims")]
pub mod ffi {
    // This is a workaround - cxx currently doesn't support passing
    // around C++ vectors of pointers directly
    // https://github.com/dtolnay/cxx/issues/774#issuecomment-808674945
    struct PlatformPtr {
        // This has to be a SharedPtr because a shared struct needs to be
        // copy-constructible
        ptr: SharedPtr<Platform>
    }

    unsafe extern "C++" {
        include!("oneapi-rs-sys/include/platform.hpp");
 
        type Platform;

        #[Self = "Platform"]
        fn get_platforms() -> UniquePtr<CxxVector<PlatformPtr>>;

        fn get_version(&self) -> String;
        fn get_name(&self) -> String;
        fn get_vendor(&self) -> String;
    }
}
