//
// Copyright (C) 2026 Intel Corporation
//
// Under the MIT License or the Apache License v2.0.
// See LICENSE-MIT and LICENSE-APACHE for license information.
// SPDX-License-Identifier: MIT OR Apache-2.0
//

#pragma once

#include "rust/cxx.h"

#include <sycl/sycl.hpp>

#include <memory>
#include <vector>

namespace sycl_shims {
struct DevicePtr;

class Device {
public:
  Device(sycl::device p) : inner(p) {}

  static rust::Vec<DevicePtr> get_devices();

private:
  sycl::device inner;
};
} // namespace sycl_shims
