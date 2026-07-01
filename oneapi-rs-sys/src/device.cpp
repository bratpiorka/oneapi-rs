//
// Copyright (C) 2026 Intel Corporation
//
// Under the MIT License or the Apache License v2.0.
// See LICENSE-MIT and LICENSE-APACHE for license information.
// SPDX-License-Identifier: MIT OR Apache-2.0
//

#include "oneapi-rs-sys/include/device.hpp"
#include "oneapi-rs-sys/src/device-sys.rs.h"

namespace sycl_shims {
rust::Vec<DevicePtr> Device::get_devices() {
  rust::Vec<DevicePtr> devices;

  for (auto &&device : sycl::device::get_devices())
    devices.push_back(DevicePtr { std::make_unique<Device>(device) });

  return devices;
}
} // namespace sycl_shims
