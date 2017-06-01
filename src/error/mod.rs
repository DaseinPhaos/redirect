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
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct WinError {
    pub hr: HRESULT,
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
