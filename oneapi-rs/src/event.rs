//
// Copyright (C) 2026 Intel Corporation
//
// Under the MIT License or the Apache License v2.0.
// See LICENSE-MIT and LICENSE-APACHE for license information.
// SPDX-License-Identifier: MIT OR Apache-2.0
//

use oneapi_rs_sys::event::ffi;

pub struct Event(pub(crate) cxx::UniquePtr<ffi::Event>);

impl Event {
    pub fn wait(&mut self) {
        ffi::wait(&mut self.0);
    }
}

impl From<cxx::UniquePtr<ffi::Event>> for Event {
    fn from(value: cxx::UniquePtr<ffi::Event>) -> Self {
        Self(value)
    }
}
