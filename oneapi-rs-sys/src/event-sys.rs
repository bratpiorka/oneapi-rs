//
// Copyright (C) 2026 Intel Corporation
//
// Under the MIT License or the Apache License v2.0.
// See LICENSE-MIT and LICENSE-APACHE for license information.
// SPDX-License-Identifier: MIT OR Apache-2.0
//

#[cxx::bridge(namespace = "sycl_shims::event")]
pub mod ffi {
    unsafe extern "C++" {
        include!("oneapi-rs-sys/include/event.hpp");

        #[namespace = "sycl_shims"]
        type Event = crate::types::ffi::Event;

        fn wait(event: &mut UniquePtr<Event>);
    }
}
