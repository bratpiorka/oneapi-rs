//
// Copyright (C) 2026 Intel Corporation
//
// Under the MIT License or the Apache License v2.0.
// See LICENSE-MIT and LICENSE-APACHE for license information.
// SPDX-License-Identifier: MIT OR Apache-2.0
//

use crate::platform::Platform;
use oneapi_rs_sys::platform::ffi;

pub trait PlatformInfo {
    type Item;
    fn get_item(platform: &Platform) -> Self::Item;
}

pub struct Version;
impl PlatformInfo for Version {
    type Item = String;
    fn get_item(platform: &Platform) -> Self::Item {
        ffi::get_version(&platform.0)
    }
}

pub struct Name;
impl PlatformInfo for Name {
    type Item = String;
    fn get_item(platform: &Platform) -> Self::Item {
        ffi::get_name(&platform.0)
    }
}

pub struct Vendor;
impl PlatformInfo for Vendor {
    type Item = String;
    fn get_item(platform: &Platform) -> Self::Item {
        ffi::get_vendor(&platform.0)
    }
}
