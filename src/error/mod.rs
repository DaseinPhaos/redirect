// Copyright 2017 Dasein Phaos aka. Luxko
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! error types

use ::winapi::{HRESULT, SUCCEEDED};

/// an winerror
// TODO: add useful error messages
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct WinError {
    pub hr: HRESULT,
}

impl WinError {
    #[inline]
    pub fn description(&self) -> &'static str {
        match self.hr {
            ::winapi::E_OUTOFMEMORY => "E_OUYOFMEMORY",
            ::winapi::DXGI_ERROR_INVALID_CALL => "DXGI_ERROR_INVALID_CALL",
            ::winapi::DXGI_ERROR_DEVICE_HUNG => "DXGI_ERROR_DEVICE_HUNG",
            ::winapi::DXGI_ERROR_DEVICE_REMOVED => "DXGI_ERROR_DEVICE_REMOVED",
            ::winapi::DXGI_ERROR_DEVICE_RESET => "DXGI_ERROR_DEVICE_RESET",
            ::winapi::DXGI_ERROR_DRIVER_INTERNAL_ERROR => "DXGI_ERROR_DRIVER_INTERNAL_ERROR",
            _ => "Other unknown errors",
        }
    }
}

impl ::std::fmt::Debug for WinError {
    #[inline]
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        write!(f, "WinError {:X} {{ {} }}", self.hr, self.description())
    }
}

impl WinError {
    /// construct an error from an `HRESULT`
    #[inline]
    pub fn from_hresult(hr: HRESULT) -> Result<(), WinError> {
        if SUCCEEDED(hr) { Ok(()) }
        else { Err(WinError{hr})}
    }

    /// construct an `Result` from an `HRESULT` and a closure
    #[inline]
    pub fn from_hresult_or_ok<F, T>(hr: HRESULT, f: F) -> Result<T, WinError>
        where F: FnOnce() -> T
    {
        if SUCCEEDED(hr) { Ok(f()) }
        else { Err(WinError{hr}) }
    }
}
