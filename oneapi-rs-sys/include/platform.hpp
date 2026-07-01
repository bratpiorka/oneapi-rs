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
struct PlatformPtr;

class Platform {
public:
  Platform(sycl::platform p) : inner(p) {}

  static std::unique_ptr<std::vector<PlatformPtr>> get_platforms();
  rust::String get_version() const;
  rust::String get_name() const;
  rust::String get_vendor() const;

private:
  sycl::platform inner;
};
} // namespace sycl_shims
