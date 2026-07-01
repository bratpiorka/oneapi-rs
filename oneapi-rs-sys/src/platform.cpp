//
// Copyright (C) 2026 Intel Corporation
//
// Under the MIT License or the Apache License v2.0.
// See LICENSE-MIT and LICENSE-APACHE for license information.
// SPDX-License-Identifier: MIT OR Apache-2.0
//

#include "oneapi-rs-sys/include/platform.hpp"
#include "oneapi-rs-sys/src/platform-sys.rs.h"

namespace sycl_shims {
rust::Vec<PlatformPtr> Platform::get_platforms() {
  rust::Vec<PlatformPtr> platforms;

  for (auto &&platform : sycl::platform::get_platforms())
    platforms.push_back(PlatformPtr { std::make_unique<Platform>(platform) });

  return platforms;
}

rust::String Platform::get_version() const {
  return this->inner.get_info<sycl::info::platform::version>();
}

rust::String Platform::get_name() const {
  return this->inner.get_info<sycl::info::platform::name>();
}

rust::String Platform::get_vendor() const {
  return this->inner.get_info<sycl::info::platform::vendor>();
}
} // namespace sycl_shims
