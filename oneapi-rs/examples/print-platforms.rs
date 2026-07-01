//
// Copyright (C) 2026 Intel Corporation
//
// Under the MIT License or the Apache License v2.0.
// See LICENSE-MIT and LICENSE-APACHE for license information.
// SPDX-License-Identifier: MIT OR Apache-2.0
//

use oneapi_rs::platform::Platform;
use oneapi_rs::info::platform::*;

fn main() {
    for platform in Platform::get_platforms() {
        println!("Platform:");
        println!("- Name: {}", platform.get_info::<Name>());
        println!("- Vendor: {}", platform.get_info::<Vendor>());
        println!("- Vendor: {}", platform.get_info::<Vendor>());
    }
}
