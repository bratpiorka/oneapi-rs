//
// Copyright (C) 2026 Intel Corporation
//
// Under the MIT License or the Apache License v2.0.
// See LICENSE-MIT and LICENSE-APACHE for license information.
// SPDX-License-Identifier: MIT OR Apache-2.0
//

use oneapi_rs::device::Device;
use oneapi_rs::info::device::*;

fn main() {
    for device in Device::get_devices() {
        println!("Device:");
        println!("- Type: {:?}", device.get_info::<DeviceType>());
    }
}
