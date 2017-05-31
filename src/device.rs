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
use command::{CommandQueue, CommandQueueDesc, CommandAllocator, CommandListType};
use resource::{Heap, HeapDesc};

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

    /// attempts to create a command queue with given description
    pub fn create_command_queue(
        &mut self, desc: &CommandQueueDesc
    ) -> Result<CommandQueue, WinError> {
        unsafe {
            let mut ret = ::std::mem::uninitialized();
            let hr = self.ptr.CreateCommandQueue(
                desc as *const _ as *const ::winapi::D3D12_COMMAND_QUEUE_DESC,
                & ::dxguid::IID_ID3D12CommandQueue,
                &mut ret as *mut *mut _ as *mut *mut c_void
            );

            WinError::from_hresult_or_ok(hr, || CommandQueue{
                ptr: ComPtr::new(ret)
            })
        }
    }

    /// attempts to create a command allocator
    pub fn create_command_allocator(
        &mut self, list_type: CommandListType
    ) -> Result<CommandAllocator, WinError> {
        unsafe {
            let mut ret = ::std::mem::uninitialized();
            let hr = self.ptr.CreateCommandAllocator(
                ::std::mem::transmute(list_type),
                & ::dxguid::IID_ID3D12CommandQueue,
                &mut ret as *mut *mut _ as *mut *mut c_void
            );

            WinError::from_hresult_or_ok(hr, || CommandAllocator{
                ptr: ComPtr::new(ret)
            })
        }
    }

    /// attempts to create a heap
    pub fn create_heap(&mut self, desc: &HeapDesc) -> Result<Heap, WinError> {
        unsafe {
            let mut ret = ::std::mem::uninitialized();
            let hr = self.ptr.CreateHeap(
                desc as *const _ as *const ::winapi::D3D12_HEAP_DESC,
                & ::dxguid::IID_ID3D12Heap,
                &mut ret as *mut *mut _ as *mut *mut c_void
            );

            WinError::from_hresult_or_ok(hr, || Heap{
                ptr: ComPtr::new(ret)
            })
        }
    }

    // TODO: attempts to create a command list. blocker: PSO
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

/// a COM object created by some `Device`
pub trait DeviceChild {
    /// get the parent device of `self`
    fn get_device(&mut self) -> Result<Device, WinError>;
}

// utility macro for `impl DeviceChild for Child{ptr:ComPtr<T>}`
macro_rules! impl_device_child {
    ($Child: ty, $ptr: ident) => {
        impl DeviceChild for $Child {
            fn get_device(&mut self) -> Result<Device, WinError> { unsafe {
                let mut ptr: *mut ::winapi::ID3D12Device = ::std::mem::uninitialized();
                let hr = self.$ptr.GetDevice(
                    & ::dxguid::IID_ID3D12Device,
                    &mut ptr as *mut *mut _ as *mut *mut ::std::os::raw::c_void
                );
                ::error::WinError::from_hresult_or_ok(hr, || {
                    ::device::Device{ptr: ::comptr::ComPtr::new(ptr)}
                })
            }}
        }
    }
}

impl_device_child!(CommandQueue, ptr);
impl_device_child!(CommandAllocator, ptr);
