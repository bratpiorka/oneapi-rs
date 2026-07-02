//
// Copyright (C) 2026 Intel Corporation
//
// Under the MIT License or the Apache License v2.0.
// See LICENSE-MIT and LICENSE-APACHE for license information.
// SPDX-License-Identifier: MIT OR Apache-2.0
//

#pragma once

#include "rust/cxx.h"
#include "oneapi-rs-sys/include/opaque.hpp"

#include <memory>
#include <vector>

namespace sycl_shims::device {
struct DevicePtr;
enum class DeviceType: std::uint8_t;

rust::Vec<DevicePtr> get_devices();
DeviceType get_device_type(Device const& device);
rust::String get_version(Device const& device);
rust::String get_name(Device const& device);
std::unique_ptr<Platform> get_platform(Device const& device);
} // namespace sycl_shims::device
