//
// Copyright (C) 2026 Intel Corporation
//
// Under the MIT License or the Apache License v2.0.
// See LICENSE-MIT and LICENSE-APACHE for license information.
// SPDX-License-Identifier: MIT OR Apache-2.0
//

use crate::device::Device;

pub trait DeviceInfo {
    type Item;
    fn get_item(device: &Device) -> Self::Item;
}

pub struct DeviceType;
impl DeviceInfo for DeviceType {
    type Item = crate::info::DeviceType;
    fn get_item(device: &Device) -> Self::Item {
        device.0.get_device_type()
    }
}
