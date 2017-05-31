// Copyright 2017 Dasein Phaos aka. Luxko
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! API entry point.

use comptr::ComPtr;
use winapi::{IDXGIFactory4, IDXGIAdapter1, IDXGISwapChain3};
use error::WinError;
use std::os::raw::c_void;
use swapchain::{SwapChain, SwapChainDesc};
use device::Device;

/// dxgi API entry point
#[derive(Debug, Clone)]
pub struct Factory {
    pub ptr: ComPtr<IDXGIFactory4>,
}

impl Factory {
    /// try to create a new DXGI factory
    pub fn new() -> Result<Factory, WinError> {
        unsafe {
            let mut ptr: *mut IDXGIFactory4 = ::std::mem::uninitialized();
            let hr = ::dxgi::CreateDXGIFactory1(
                & ::dxguid::IID_IDXGIFactory4,
                &mut ptr as *mut *mut _ as *mut *mut c_void
            );
            WinError::from_hresult_or_ok(hr, || Factory{
                ptr: ComPtr::new(ptr)
            })
        }
    }

    /// enumerate available adapters
    #[inline]
    pub fn enumerate_adapters(&mut self) -> AdapterIter {
        AdapterIter{
            idx: 0, factory: self
        }
    }

    /// create a swap chain
    #[inline]
    pub fn create_swapchain(&mut self, device: &Device) -> Result<SwapChain, WinError> {
        unsafe {
            let mut ptr: *mut IDXGISwapChain3 = ::std::mem::uninitialized();
            let hr = self.ptr.CreateSwapChain(
                device.ptr.as_mut_ptr(),

            );
            WinError
        }
    }
}

/// iterator returned by a factory to retrieve available adapters
pub struct AdapterIter<'a> {
    idx: u32,
    factory: &'a mut Factory,
}

impl<'a> Iterator for AdapterIter<'a> {
    type Item = Adapter;
    
    #[inline]
    fn next(&mut self) -> Option<Adapter> {
        let old_idx = self.idx;
        unsafe {
            let mut padapter: *mut IDXGIAdapter1 = ::std::mem::uninitialized();
            let hr = self.factory.ptr.EnumAdapters1(
                old_idx,
                &mut padapter as *mut *mut _ as *mut *mut IDXGIAdapter1
            );
            WinError::from_hresult(hr).ok().map(|()| {
                self.idx += 1;
                Adapter{
                    ptr: ComPtr::new(padapter)
                }
            })
        }
    }
}

/// a display subsystem
#[derive(Debug, Clone)]
pub struct Adapter {
    pub ptr: ComPtr<IDXGIAdapter1>,
}

impl Adapter {

    pub fn get_desc
}
