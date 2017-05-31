// Copyright 2017 Dasein Phaos aka. Luxko
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! formats used by direct3d

pub use ::winapi::dxgiformat::*;
pub type DxgiFormat = DXGI_FORMAT;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Bool{inner: ::winapi::BOOL}

impl Bool {
    #[inline]
    pub fn from_win_bool(b: ::winapi::BOOL) -> Self{
        debug_assert!(b== ::winapi::TRUE || b== ::winapi::FALSE);
        Bool{inner:b}
    }

    #[inline]
    pub fn to_win_bool(self) -> ::winapi::BOOL {
        self.inner
    }

    #[inline]
    pub fn is_true(self) -> bool {
        self.inner == ::winapi::TRUE
    }
}

impl From<bool> for Bool {
    #[inline]
    fn from(v: bool) -> Bool {
        if v {
            Bool{inner: ::winapi::TRUE}
        } else {
            Bool{inner: ::winapi::FALSE}
        }
    }
}

impl From<Bool> for ::winapi::BOOL {
    #[inline]
    fn from(v: Bool) -> Self {
        v.inner
    }
}
