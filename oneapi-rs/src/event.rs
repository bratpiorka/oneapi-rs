//
// Copyright (C) 2026 Intel Corporation
//
// Under the MIT License or the Apache License v2.0.
// See LICENSE-MIT and LICENSE-APACHE for license information.
// SPDX-License-Identifier: MIT OR Apache-2.0
//

use std::{pin::Pin, task::{Context, Poll}};

use oneapi_rs_sys::event::ffi;

use pin_project::pin_project;

use crate::info::{EventCommandStatus, event::{CommandExecutionStatus, EventInfo}};

pub struct Event(pub(crate) cxx::UniquePtr<ffi::Event>);

impl Event {
    pub fn wait(&mut self) {
        ffi::wait(&mut self.0);
    }

    pub fn get_info<T: EventInfo>(&self) -> T::Item {
        T::get_item(self)
    }
}

impl From<cxx::UniquePtr<ffi::Event>> for Event {
    fn from(value: cxx::UniquePtr<ffi::Event>) -> Self {
        Self(value)
    }
}

impl Clone for Event {
    fn clone(&self) -> Self {
        ffi::clone(&self.0).into()
    }
}

#[pin_project]
pub struct EventFuture {
    event: Event,
    set_callback: bool
}

impl Future for EventFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.event.get_info::<CommandExecutionStatus>() == EventCommandStatus::Complete {
            Poll::Ready(())
        }
        else {
            if self.set_callback == false {
                let this = self.project();
                *this.set_callback = true;
                let waker = Box::new(cx.waker().clone().into());
                ffi::register_callback(&this.event.0, waker);
            }
            Poll::Pending
        }
    }
}

impl IntoFuture for Event {
    type Output = ();
    type IntoFuture = EventFuture;

    fn into_future(self) -> Self::IntoFuture {
        EventFuture {
            event: self,
            set_callback: false
        }
    }
}
