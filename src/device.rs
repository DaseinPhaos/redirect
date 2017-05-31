// Copyright 2017 Dasein Phaos aka. Luxko
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! defines `Device`, interface for a 3D display adapter

use comptr::ComPtr;
use winapi::ID3D12Device;
use error::WinError;
use std::os::raw::c_void;
use factory::Adapter;

/// a 3D display adapter
#[derive(Debug, Clone)]
pub struct Device {
    pub ptr: ComPtr<ID3D12Device>,
}

impl Device {
    /// attempt to create a device from the given adapter and feature level.
    /// `None` means the default adapter would be used.
    pub fn new(
        adapter: Option<&Adapter>, level: FeatureLevel
    ) -> Result<Device, WinError> {
        let padapter = if let Some(adapter) = adapter {
            adapter.ptr.as_mut_ptr() as *mut ::winapi::IUnknown
        } else {
            ::std::ptr::null_mut()
        };
        unsafe {
            let mut ptr: *mut ID3D12Device = ::std::mem::uninitialized();
            let hr = ::d3d12::D3D12CreateDevice(
                padapter,
                level.into(),
                &::dxguid::IID_ID3D12Device,
                &mut ptr as *mut *mut _ as *mut *mut c_void
            );
            WinError::from_hresult_or_ok(hr, || Device{
                ptr: ComPtr::new(ptr)
            })
        }
    }


}

bitflags! {
    /// set of features targeted by a device
    #[repr(C)]
    pub struct FeatureLevel: u32 {
        const FEATURE_LEVEL_11_0 = 0xb000;
        const FEATURE_LEVEL_11_1 = 0xb100;
        const FEATURE_LEVEL_12_0 = 0xc000;
        const FEATURE_LEVEL_12_1 = 0xc100;
    }
}

impl From<FeatureLevel> for ::winapi::D3D_FEATURE_LEVEL {
    #[inline]
    fn from(level: FeatureLevel) -> Self {
        ::winapi::D3D_FEATURE_LEVEL(level.bits())
    }
}
