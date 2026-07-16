//
// Copyright (C) 2026 Intel Corporation
//
// Under the MIT License or the Apache License v2.0.
// See LICENSE-MIT and LICENSE-APACHE for license information.
// SPDX-License-Identifier: MIT OR Apache-2.0
//

#include "oneapi-rs-sys/include/event.hpp"
#include "oneapi-rs-sys/src/event-sys.rs.h"

using sycl::info::event_command_status;

namespace sycl_shims::event {
void wait(std::unique_ptr<Event> & event) {
  event->wait();
}
std::unique_ptr<Event> clone(Event const & event) {
  return std::make_unique<Event>(sycl::event(event));
}
EventCommandStatus get_command_execution_status(Event const & event) {
  auto status = event.get_info<sycl::info::event::command_execution_status>();
  switch (status) {
    case event_command_status::submitted:
      return EventCommandStatus::Submitted;
    case event_command_status::running:
      return EventCommandStatus::Running;
    case event_command_status::complete:
      return EventCommandStatus::Complete;
    default:
      return EventCommandStatus::Unknown;
  }
}
} // namespace sycl_shims::event
