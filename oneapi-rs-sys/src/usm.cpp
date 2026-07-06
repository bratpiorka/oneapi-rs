//
// Copyright (C) 2026 Intel Corporation
//
// Under the MIT License or the Apache License v2.0.
// See LICENSE-MIT and LICENSE-APACHE for license information.
// SPDX-License-Identifier: MIT OR Apache-2.0
//

#include "oneapi-rs-sys/include/usm.hpp"
#include "oneapi-rs-sys/src/usm-sys.rs.h"

namespace sycl_shims::usm {
std::uint8_t* malloc_device(std::size_t size, Queue const & queue) {
  return static_cast<std::uint8_t*>(sycl::malloc_device(size, queue));
}
} // namespace sycl_shims::usm
