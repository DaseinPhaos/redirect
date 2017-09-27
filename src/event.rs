// Copyright 2017 Dasein Phaos aka. Luxko
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! RAII Win32 event wrapper

use winapi::HANDLE;
use kernel32::{CreateEventExW, ResetEvent, SetEvent, CloseHandle, GetLastError, WaitForSingleObject};

/// RAII Win32 event
pub struct Event {
    handle: HANDLE,
    
}

impl Event {
    /// Obtain the `handle` and gain exclusive ownership to it.
    ///
    /// # Safety
    ///
    /// Caller must ensure that `handle` is a valid event,
    /// and that it would not be used elsewhere afterwards.
    #[inline]
    pub unsafe fn from_handle(handle: HANDLE) -> Event {
        debug_assert!(!handle.is_null());
        Event{handle}
    }

    #[inline]
    pub fn new<'a>(
        flags: Flag, access: AccessRight // TODO: names?
    ) -> Result<Event, u32> {
        let handle = unsafe {CreateEventExW(
            ::std::ptr::null_mut(),
            ::std::ptr::null_mut(),
            flags.bits, access.bits
        )};
        if handle.is_null() {unsafe {Err(GetLastError())}}
        else { Ok(Event{handle}) }
    }

    /// Reset a manually reset event
    #[inline]
    pub fn reset(&self) -> Result<(), u32> {unsafe {
        match ResetEvent(self.handle) {
            0 => Err(GetLastError()),
            _ => Ok(())
        }
    }}

    /// set the event
    #[inline]
    pub fn set(&self) -> Result<(), u32> {unsafe {
        match SetEvent(self.handle) {
            0 => Err(GetLastError()),
            _ => Ok(())
        }
    }}

    /// wait for this event to be set
    #[inline]
    pub fn wait(&self) -> Result<(), u32> {
        self.wait_for(::winapi::INFINITE)
    }

    /// wait for `milli_sec` ms
    #[inline]
    pub fn wait_for(&self, milli_sec: u32) -> Result<(), u32> {unsafe {
        match WaitForSingleObject(self.handle, milli_sec) {
            0 => Ok(()),
            err => Err(err)
        }
    }}

    /// get the event raw handle
    #[inline]
    pub fn get(&self) -> HANDLE {
        self.handle
    }
}

impl Default for Event {
    fn default() -> Event {
        match Event::new(Default::default(), Default::default()) {
            Ok(event) => event,
            Err(_) => panic!("Failed to create event!"),
        }
    }
}

impl Drop for Event {
    fn drop(&mut self) {unsafe {
        CloseHandle(self.handle);
    }}
}

unsafe impl Send for Event {}
unsafe impl Sync for Event {}

/// Event access rights
bitflags!{
    /// see https://msdn.microsoft.com/en-us/library/windows/desktop/ms686670(v=vs.85).aspx
    #[repr(C)]
    pub struct AccessRight: u32 {
        /// Check this flag if you want to wait for the event.
        const SYNCHRONIZE = 0x00100000;
        /// Check this flag if you want everything.
        /// This is the default flag.
        const ALL_ACCESS = 0x1F0003;
        /// Check this flag if you want to set/reset the event.
        const MODIFY_STATE = 0x0002;
    }
}

impl Default for AccessRight {
    fn default() -> AccessRight {
        ALL_ACCESS
    }
}

bitflags!{
    /// Event flags
    #[repr(C)]
    pub struct Flag: u32 {
        /// The event would be set right after creation.
        const INITIAL_SET = 0x00000002;
        /// The event need manually resetting after being set.
        const MANUAL_RESET = 0x00000001;
        /// The event would be inititially unset, and would automatically reset.
        const NONE = 0x0;
    }
}

impl Default for Flag {
    fn default() -> Flag {
        NONE
    }
}
