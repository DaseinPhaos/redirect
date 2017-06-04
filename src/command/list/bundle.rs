// Copyright 2017 Dasein Phaos aka. Luxko
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! command list bundles

use super::*;

/// a command list bundle
#[derive(Clone, Debug)]
pub struct Bundle {
    pub ptr: ComPtr<ID3D12GraphicsCommandList>,
}

impl Bundle {
    /// start command recording. [more](https://msdn.microsoft.com/zh-cn/library/windows/desktop/dn903895(v=vs.85).aspx)
    pub fn start<'b>(
        mut self, alloc: &'b mut CommandAllocator, 
        initial_state: Option<&PipelineState>
    ) -> Result<BundleRecording<'b>, (WinError, Self)> {
        let p_initial_state = if let Some(initial_state) = initial_state {
            initial_state.ptr.as_mut_ptr()
        } else {
            ::std::ptr::null_mut()
        };
        unsafe {
            let result = WinError::from_hresult(self.ptr.Reset(alloc.ptr.as_mut_ptr(), p_initial_state));
            if result.is_ok() {
                Ok(BundleRecording{ ptr: self.ptr, alloc})
            } else {
                Err((result.unwrap_err(), self))
            }
        }
    }
}

/// a command list bundle during recording state
#[derive(Debug)]
pub struct BundleRecording<'a> {
    pub ptr: ComPtr<ID3D12GraphicsCommandList>,
    /// command allocator used to back up command recording
    pub alloc: &'a mut CommandAllocator,
}

impl<'a> BundleRecording<'a> {
    /// reset a bundle back to the initial state. [more](https://msdn.microsoft.com/zh-cn/library/windows/desktop/dn903895(v=vs.85).aspx)
    pub fn reset<'b>(
        mut self, alloc: &'b mut CommandAllocator, 
        initial_state: Option<&PipelineState>
    ) -> Result<BundleRecording<'b>, (WinError, Self)> {
        let p_initial_state = if let Some(initial_state) = initial_state {
            initial_state.ptr.as_mut_ptr()
        } else {
            ::std::ptr::null_mut()
        };
        unsafe {
            let result = WinError::from_hresult(self.ptr.Reset(alloc.ptr.as_mut_ptr(), p_initial_state));
            if result.is_ok() {
                Ok(BundleRecording{ ptr: self.ptr, alloc})
            } else {
                Err((result.unwrap_err(), self))
            }
        }
    }

    /// close the current recording
    #[inline]
    pub fn close(mut self) -> Result<Bundle, WinError> {
        unsafe{
            WinError::from_hresult_or_ok(self.ptr.Close(), move || Bundle{
                ptr: self.ptr
            })
        }
    }
}
