//
// Copyright (C) 2026 Intel Corporation
//
// Under the MIT License or the Apache License v2.0.
// See LICENSE-MIT and LICENSE-APACHE for license information.
// SPDX-License-Identifier: MIT OR Apache-2.0
//

use crate::event::Event;
use oneapi_rs_sys::event::ffi;

pub trait EventInfo {
    type Item;
    fn get_item(event: &Event) -> Self::Item;
}

/// Returns the event status of the action associated with this event.
pub struct CommandExecutionStatus;
impl EventInfo for CommandExecutionStatus {
    type Item = crate::info::EventCommandStatus;
    fn get_item(event: &Event) -> Self::Item {
        ffi::get_command_execution_status(&event.0)
    }
}
