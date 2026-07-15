//
// Copyright (C) 2026 Intel Corporation
//
// Under the MIT License or the Apache License v2.0.
// See LICENSE-MIT and LICENSE-APACHE for license information.
// SPDX-License-Identifier: MIT OR Apache-2.0
//

#include "oneapi-rs-sys/include/event.hpp"
#include "oneapi-rs-sys/src/event-sys.rs.h"

namespace sycl_shims::event {
void wait(std::unique_ptr<Event> & event) {
  event->wait();
}
} // namespace sycl_shims::event
