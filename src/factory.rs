// Copyright 2017 Dasein Phaos aka. Luxko
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! this module contains `Factory`, the API entry point,
//! along with several other fundamental structures such as
//! `Adapter` and `Output`.

use comptr::ComPtr;
use winapi::{IDXGIFactory4, IDXGIAdapter3, IDXGIAdapter1, IDXGISwapChain3, IDXGISwapChain1, IDXGIOutput};
use error::WinError;
use std::os::raw::c_void;
use swapchain::{SwapChain, SwapChainDesc, FullScreenDesc};
use command::CommandQueue;

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
    pub fn create_swapchain_for_hwnd(
        &mut self, queue: &CommandQueue, // FIXME: this should be a command queue
        hwnd: ::winapi::HWND, // TODO: change?
        desc: &SwapChainDesc,
        fullscreen_desc: Option<&FullScreenDesc>,
        restrict_output: Option<&Output>
    ) -> Result<SwapChain, WinError> {
        let fullscreen_desc = if let Some(desc) = fullscreen_desc {
            desc as *const _ as *const ::winapi::DXGI_SWAP_CHAIN_FULLSCREEN_DESC
        } else {
            ::std::ptr::null()
        };
        let restrict_output = if let Some(output) = restrict_output {
            output.ptr.as_mut_ptr()
        } else {
            ::std::ptr::null_mut()
        };
        unsafe {
            let mut ptr: *mut IDXGISwapChain3 = ::std::mem::uninitialized();
            let hr = self.ptr.CreateSwapChainForHwnd(
                queue.ptr.as_mut_ptr() as *mut _,
                hwnd,
                desc as *const _ as *const ::winapi::DXGI_SWAP_CHAIN_DESC1,
                fullscreen_desc,
                restrict_output,
                &mut ptr as *mut *mut _ as *mut *mut IDXGISwapChain1 // TODO: double check
            );
            WinError::from_hresult_or_ok(hr, || SwapChain{
                ptr: ComPtr::new(ptr)
            })
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
            if let Err(_) = WinError::from_hresult(hr) {
                return None;
            }
            let mut adapter1 = ComPtr::new(padapter);

            let mut padapter: *mut IDXGIAdapter3 = ::std::mem::uninitialized();
            let hr = adapter1.QueryInterface(
                & ::dxguid::IID_IDXGIAdapter3,
                &mut padapter as *mut *mut _ as *mut *mut _
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
    pub ptr: ComPtr<IDXGIAdapter3>,
}

impl Adapter {
    /// get basic descriptions about the adapter
    pub fn get_desc(&mut self) -> Result<AdapterDesc, WinError> {
        unsafe {
            let mut ret = ::std::mem::uninitialized();
            let hr = self.ptr.GetDesc1(&mut ret);
            WinError::from_hresult_or_ok(hr, || ::std::mem::transmute(ret))
        }
    }

    /// enumerate outputs of this adapter
    #[inline]
    pub fn enumerate_outputs(&mut self) -> OutputIter {
        OutputIter{
            idx: 0,
            adapter: self
        }
    }

    /// query adapter memory infos
    #[inline]
    pub fn query_mem_info(&mut self, node_idx: u32, local: bool) -> Result<VideoMemInfo, WinError> {
        let mem_seg_group = if local {
            ::winapi::DXGI_MEMORY_SEGMENT_GROUP_LOCAL
        } else {
            ::winapi::DXGI_MEMORY_SEGMENT_GROUP_NON_LOCAL
        };
        unsafe {
            let mut ret: VideoMemInfo = ::std::mem::uninitialized();
            let hr = self.ptr.QueryVideoMemoryInfo(
                node_idx, mem_seg_group, 
                &mut ret as *mut _ as *mut _
            );
            WinError::from_hresult_or_ok(hr, || {
                ret
            })
        }
    }
}

/// video memory informaiton
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct VideoMemInfo {
    pub budget: u64,
    pub current_usage: u64,
    pub available_for_reservation: u64,
    pub current_reservation: u64,
}

/// adapter description
#[repr(C)]
pub struct AdapterDesc {
    /// a string description of the adapter
    pub description: [::winapi::WCHAR; 128],
    /// PCI ID of the hardware vendor
    pub vendor_id: u32,
    /// PCI ID of the hardware device
    pub device_id: u32,
    /// PCI ID of the revision number of the adapter
    pub revision: u32,
    /// dedicated video memory not shared with CPU
    pub dedicated_vmem: usize,
    /// dedicated system memory not shared with CPU
    pub dedicated_smem: usize,
    /// shared system memory
    pub shared_smem: usize,
    /// locally unique id for the adapter
    pub luid: ::winapi::LUID,
    /// misc flags
    pub flags: AdapterFlags,
}

impl ::std::fmt::Debug for AdapterDesc {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        write!(f, "AdapterDesc {{ description: {:?}, vendor_id: {:?}, device_id: {:?}, revision: {:?}, dedicated_vmem: {:?}, dedicated_smem: {:?}, shared_smem: {:?}, luid: {:?}, flags: {:?} }}", ::format::from_wchar_slice(&self.description), self.vendor_id, self.device_id, self.revision, self.dedicated_vmem, self.dedicated_smem, self.shared_smem, self.luid, self.flags)
    }
}

impl ::std::fmt::Display for AdapterDesc {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        write!(f, "{:?}", ::format::from_wchar_slice(&self.description))
    }
}

bitflags!{
    /// adapter flags
    #[repr(C)]
    pub struct AdapterFlags: u32 {
        const ADAPTER_FLAG_NONE = 0;
        /// reserved flag
        const ADAPTER_FLAG_REMOTE = 1;
        /// software adapter
        const ADAPTER_FLAG_SOFTWARE = 2;
    }
}


/// iterator returned by an adapter to retrieve available outputs
pub struct OutputIter<'a> {
    idx: u32,
    adapter: &'a mut Adapter,
}

impl<'a> Iterator for OutputIter<'a> {
    type Item = Output;

    fn next(&mut self) -> Option<Output> {
        let oldidx = self.idx;
        unsafe {
            let mut ptr: *mut IDXGIOutput = ::std::mem::uninitialized();
            let hr = self.adapter.ptr.EnumOutputs(
                oldidx, &mut ptr
            );
            WinError::from_hresult(hr).ok().map(|()| {
                self.idx += 1;
                Output{ptr: ComPtr::new(ptr)}
            })
        }
    }
}

/// a display output, such as a monitor
#[derive(Debug, Clone)]
pub struct Output {
    pub ptr: ComPtr<IDXGIOutput>,
}

impl Output {
    // TODO: add more methods?

    /// get basic description for the output
    #[inline]
    pub fn get_desc(&mut self) -> Result<OutputDesc, WinError> {
        unsafe {
            let mut ret = ::std::mem::uninitialized();
            let hr = self.ptr.GetDesc(&mut ret as *mut _ as *mut _);
            WinError::from_hresult_or_ok(hr, || ret)
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct OutputDesc {
    pub name: [::winapi::WCHAR; 32],
    pub descktop_coordinates: ::format::Rect,
    pub attached_to_desktop: ::format::Bool,
    pub rotation: RotationMode,
    pub hmonitor: ::winapi::HMONITOR,
}

impl ::std::fmt::Display for OutputDesc {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        write!(f, "{:?}", ::format::from_wchar_slice(&self.name))
    }
}

bitflags!{
    /// rotation mode for the monitor
    #[repr(C)]
    pub struct RotationMode: u32 {
        const ROTATION_MODE_UNSPECIFIED = 0;
        const ROTATION_MODE_IDENTITY = 1;
        const ROTATION_MODE_ROTATE90 = 2;
        const ROTATION_MODE_ROTATE180 = 3;
        const ROTATION_MODE_ROTATE270 = 4;
    }
}
