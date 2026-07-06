//
// Copyright (C) 2026 Intel Corporation
//
// Under the MIT License or the Apache License v2.0.
// See LICENSE-MIT and LICENSE-APACHE for license information.
// SPDX-License-Identifier: MIT OR Apache-2.0
//

#pragma once

#include "oneapi-rs-sys/include/types.hpp"
#include "rust/cxx.h"

#include <memory>

namespace sycl_shims::queue {
std::unique_ptr<Queue> new_queue();
std::unique_ptr<Queue> new_queue_from_device(Device const &);
} // namespace sycl_shims::queue
