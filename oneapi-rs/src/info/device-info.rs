//
// Copyright (C) 2026 Intel Corporation
//
// Under the MIT License or the Apache License v2.0.
// See LICENSE-MIT and LICENSE-APACHE for license information.
// SPDX-License-Identifier: MIT OR Apache-2.0
//

use crate::device::Device;
use oneapi_rs_sys::device::ffi;

pub trait DeviceInfo {
    type Item;
    fn get_item(device: &Device) -> Self::Item;
}

pub struct DeviceType;
impl DeviceInfo for DeviceType {
    type Item = crate::info::DeviceType;
    fn get_item(device: &Device) -> Self::Item {
        ffi::get_device_type(&device.0)
    }
}

pub struct Version;
impl DeviceInfo for Version {
    type Item = String;
    fn get_item(device: &Device) -> Self::Item {
        ffi::get_version(&device.0)
    }
}

pub struct Name;
impl DeviceInfo for Name {
    type Item = String;
    fn get_item(device: &Device) -> Self::Item {
        ffi::get_name(&device.0)
    }
}
