//
// Copyright (C) 2026 Intel Corporation
//
// Under the MIT License or the Apache License v2.0.
// See LICENSE-MIT and LICENSE-APACHE for license information.
// SPDX-License-Identifier: MIT OR Apache-2.0
//

use oneapi_rs_sys::platform::ffi;

use crate::{device::Device, info::platform::PlatformInfo};

/// Abstraction for SYCL platform.
///
/// The `Platform` struct encapsulates a single SYCL platform on which SYCL kernel functions may be executed.
/// A SYCL platform must be associated with a single SYCL backend.
///
/// A `Platform` is also associated with one or more SYCL devices associated with the same SYCL backend.
pub struct Platform(pub(crate) cxx::UniquePtr<ffi::Platform>);

impl Platform {
    /// Returns a [`Vec`] containing all SYCL platforms from all SYCL backends available in the system.
    pub fn get_platforms() -> Vec<Self> {
        ffi::get_platforms()
            .into_iter()
            .map(|platform| Self(platform.ptr))
            .collect()
    }

    /// Returns a [`Vec`] containing all the root devices associated with this `Platform`.
    pub fn get_devices(&self) -> Vec<Device> {
        ffi::get_devices(&self.0)
            .into_iter()
            .map(|device| Device(device.ptr))
            .collect()
    }

    /// Queries this `Platform` for information requested by the generic parameter `Param`.
    /// The associated type `Param::Item` must be defined in accordance with the info parameters
    /// in [`oneapi_rs::info::platform`](`crate::info::platform`) to facilitate returning the type associated with the `Param` parameter.
    pub fn get_info<T: PlatformInfo>(&self) -> T::Item {
        T::get_item(self)
    }
}
