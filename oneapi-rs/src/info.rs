//
// Copyright (C) 2026 Intel Corporation
//
// Under the MIT License or the Apache License v2.0.
// See LICENSE-MIT and LICENSE-APACHE for license information.
// SPDX-License-Identifier: MIT OR Apache-2.0
//

#[path = "./info/platform-info.rs"]
pub mod platform;

#[path = "./info/device-info.rs"]
pub mod device;

/// The type of the SYCL device.
pub use oneapi_rs_sys::device::ffi::DeviceType;
