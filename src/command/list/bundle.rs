// Copyright 2017 Dasein Phaos aka. Luxko
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! command list bundles

use super::*;

/// An allocator for GPU commands
#[derive(Debug)]
pub struct BundleCommandAllocator {
    pub ptr: ComPtr<ID3D12CommandAllocator>,
}

impl BundleCommandAllocator {
    /// indicates that the associated memory would be recycled by the allocator.
    #[inline]
    pub fn reset(&mut self) -> Result<(), WinError> {
        let hr = unsafe {self.ptr.Reset()};
        WinError::from_hresult(hr)
    }
}

/// a command list bundle
#[derive(Clone, Debug)]
pub struct Bundle {
    pub ptr: ComPtr<ID3D12GraphicsCommandList>,
}

impl Bundle {
    /// start command recording. [more](https://msdn.microsoft.com/zh-cn/library/windows/desktop/dn903895(v=vs.85).aspx)
    pub fn start<'b>(
        mut self, alloc: &'b mut BundleCommandAllocator, 
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
    pub alloc: &'a mut BundleCommandAllocator,
}

impl<'a> BundleRecording<'a> {
    // TODO: double check descriptor heap settings
    #[inline]
    pub fn set_descriptor_heaps(
        mut self, cbv_srv_uav_heap: Option<&CbvSrvUavHeap>,
        rtv_heap: Option<&RtvHeap>, dsv_heap: Option<&DsvHeap>,
        sampler_heap: Option<&SamplerHeap>
    ) -> BundleRecordingWithHeap<'a> {
        let mut heaps = [
            ::std::ptr::null_mut(), ::std::ptr::null_mut(),
            ::std::ptr::null_mut(), ::std::ptr::null_mut(),
        ];
        if let Some(heap) = cbv_srv_uav_heap {
            heaps[0] = heap.ptr.as_mut_ptr();
        }
        if let Some(heap) = rtv_heap {
            heaps[1] = heap.ptr.as_mut_ptr();
        }
        if let Some(heap) = dsv_heap {
            heaps[2] = heap.ptr.as_mut_ptr();
        }
        if let Some(heap) = sampler_heap {
            heaps[3] = heap.ptr.as_mut_ptr();
        }
        unsafe {
            self.ptr.SetDescriptorHeaps(4, heaps.as_mut_ptr());
        }
        
        BundleRecordingWithHeap{
            ptr: self.ptr, alloc: self.alloc
        }
    }

    /// reset a bundle back to the initial state. [more](https://msdn.microsoft.com/zh-cn/library/windows/desktop/dn903895(v=vs.85).aspx)
    pub fn reset<'b>(
        mut self, alloc: &'b mut BundleCommandAllocator, 
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

/// a command list bundle during recording state with descriptor heap already set
#[derive(Debug)]
pub struct BundleRecordingWithHeap<'a> {
    pub ptr: ComPtr<ID3D12GraphicsCommandList>,
    /// command allocator used to back up command recording
    pub alloc: &'a mut BundleCommandAllocator,
}

impl<'a> BundleRecordingWithHeap<'a> {
    /// reset a bundle back to the initial state. [more](https://msdn.microsoft.com/zh-cn/library/windows/desktop/dn903895(v=vs.85).aspx)
    pub fn reset<'b>(
        mut self, alloc: &'b mut BundleCommandAllocator, 
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
