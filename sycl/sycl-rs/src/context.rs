//
// Copyright (C) 2026 Intel Corporation
//
// Under the MIT License or the Apache License v2.0.
// See LICENSE-MIT and LICENSE-APACHE for license information.
// SPDX-License-Identifier: MIT OR Apache-2.0
//

use sycl_rs_sys::{context::ffi, kernel_bundle, types::ffi::DevicePtr};

use crate::{Result, device::Device, kernel::SourceKernelBundle};

/// A context represents the runtime data structures and state required by a SYCL backend API
/// to interact with a group of devices associated with a platform.
pub struct Context(pub(crate) cxx::UniquePtr<ffi::Context>);

impl From<cxx::UniquePtr<ffi::Context>> for Context {
    fn from(value: cxx::UniquePtr<ffi::Context>) -> Self {
        Self(value)
    }
}

impl Context {
    /// Constructs a context for devices from the same platform.
    ///
    /// Returns an error if the device list is empty or context construction fails.
    pub fn new(devices: &[&Device]) -> Result<Self> {
        let devices = devices
            .iter()
            .map(|d| DevicePtr {
                ptr: (*d).clone().0,
            })
            .collect::<Vec<_>>();

        ffi::new_context(devices).map(Into::into)
    }

    pub fn create_kernel_bundle_from_source(&self, source: &str) -> Result<SourceKernelBundle> {
        kernel_bundle::ffi::create_kernel_bundle_from_source(&self.0, source).map(Into::into)
    }
}
