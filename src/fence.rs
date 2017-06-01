// Copyright 2017 Dasein Phaos aka. Luxko
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! a fence is used for synchronization of CPUs and GPUs

use winapi::ID3D12Fence;
use comptr::ComPtr;
use error::WinError;

/// a fence
#[derive(Clone, Debug)]
pub struct Fence {
    pub ptr: ComPtr<ID3D12Fence>,
}

impl Fence {
    /// get the current value of the fence
    #[inline]
    pub fn get_completed_value(&mut self) -> u64 {
        unsafe {self.ptr.GetCompletedValue() }
    }

    // TODO: add events?

    /// set the fence to the specified value from CPU side
    #[inline]
    pub fn signal(&mut self, value: u64) -> Result<(), WinError> {
        unsafe {WinError::from_hresult(
            self.ptr.Signal(value)
        )}
    }
}

bitflags!{
    /// misc fence options
    #[repr(C)]
    pub struct FenceFlags: u32 {
        const FENCE_FLAG_NONE = 0;
        const FENCE_FLAG_SHARED = 0x1;
        const FENCE_FLAG_SHARED_CROSS_ADAPTER = 0x2;
    }
}

impl Default for FenceFlags {
    #[inline]
    fn default() -> FenceFlags {
        FENCE_FLAG_NONE
    }
}
