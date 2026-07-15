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

namespace sycl_shims::event {
void wait(std::unique_ptr<Event> &);
std::unique_ptr<Event> clone(Event const &);
} // namespace sycl_shims::event
