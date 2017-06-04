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
use command::{CommandQueue, CommandQueueDesc, CommandAllocator, CommandListType, GraphicsCommandList};
use resource::*;
use pipeline::rootsig::{RootSig, RootSigDescBlob};
use pipeline::{PipelineState, GraphicsPipelineStateBuilder};
use fence::{Fence, FenceFlags};
use descriptor::{CbvSrvUavHeap, RtvHeap, DsvHeap, SamplerHeap, DescriptorHeapBuilder};

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

    /// attempts to create a graphics pipeline state object
    #[inline]
    pub fn create_graphics_pso<'a>(
        &mut self, pso: &mut GraphicsPipelineStateBuilder<'a>
    ) -> Result<PipelineState, WinError> {
        pso.build(self)
    }

    /// attempts to create a root signature from a description blob
    #[inline]
    pub fn create_root_sig(
        &mut self, node_mask: u32, desc_blob: &RootSigDescBlob
    ) -> Result<RootSig, WinError> {
        unsafe {
            let pblob = desc_blob.ptr.as_mut_ptr();
            let length = (*pblob).GetBufferSize();
            let pblob = (*pblob).GetBufferPointer();

            let mut ret = ::std::mem::uninitialized();
            let hr = self.ptr.CreateRootSignature(
                node_mask, pblob, length,
                & ::dxguid::IID_ID3D12RootSignature,
                &mut ret as *mut *mut _ as *mut *mut _
            );
            
            WinError::from_hresult_or_ok(hr, || RootSig{
                ptr: ComPtr::new(ret)
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

            WinError::from_hresult_or_ok(hr, || Heap::from_comptr(
                ComPtr::new(ret)
            ))
        }
    }

    /// attempts to create a committed resource
    pub fn create_committed_resource(
        &mut self, heap_properties: &HeapProperties, 
        heap_flags: HeapFlags, desc: &ResourceDesc
    ) -> Result<CommittedResource, WinError> {
        let initial_state = match heap_properties.heap_type {
            HEAP_TYPE_UPLOAD => ::winapi::D3D12_RESOURCE_STATE_GENERIC_READ,
            HEAP_TYPE_READBACK => ::winapi::D3D12_RESOURCE_STATE_COPY_DEST,
            _ => ::winapi::D3D12_RESOURCE_STATE_COMMON,
        };
        unsafe {
            let mut ptr = ::std::mem::uninitialized();
            let hr = self.ptr.CreateCommittedResource(
                heap_properties as *const _ as *const _,
                ::std::mem::transmute(heap_flags),
                desc as *const _ as *const _,
                initial_state,
                ::std::ptr::null(),
                & ::dxguid::IID_ID3D12Resource,
                &mut ptr as *mut _ as *mut _
            );

            WinError::from_hresult_or_ok(hr, || CommittedResource::from_raw(
                RawResource{ptr: ComPtr::new(ptr)}
            ))
        }
    }

    /// attempts to create a fence
    pub fn create_fence(&mut self, initial_value: u64, flags: FenceFlags) -> Result<Fence, WinError> {
        unsafe {
            let mut ret = ::std::mem::uninitialized();
            let hr = self.ptr.CreateFence(
                initial_value,
                ::std::mem::transmute(flags),
                & ::dxguid::IID_ID3D12Fence,
                &mut ret as *mut *mut _ as *mut *mut _
            );

            WinError::from_hresult_or_ok(hr, || Fence{
                ptr: ComPtr::new(ret)
            })
        }
    }

    /// get resource allocation info from the resource description
    pub fn get_resource_alloc_info(&mut self, desc: &ResourceDesc, visible_mask: u32) -> ResourceAllocInfo {
        unsafe {
            let mut ret = ::std::mem::uninitialized();
            self.ptr.GetResourceAllocationInfo(
                visible_mask, 1, desc as *const _ as *const _, &mut ret
            );
            ::std::mem::transmute(ret)
        }
    }

    /// attempts to create a placed resource
    pub fn create_placed_resource(
        &mut self, heap: &mut Heap, heap_offset: u64, desc: &ResourceDesc
    ) -> Result<PlacedResource, WinError> {
        let heap_properties = heap.get_desc().properties;
        let initial_state = match heap_properties.heap_type {
            HEAP_TYPE_UPLOAD => ::winapi::D3D12_RESOURCE_STATE_GENERIC_READ,
            HEAP_TYPE_READBACK => ::winapi::D3D12_RESOURCE_STATE_COPY_DEST,
            _ => ::winapi::D3D12_RESOURCE_STATE_COMMON,
        };
        let alloc_info = self.get_resource_alloc_info(
            desc, heap_properties.visible_node_mask
        );
        unsafe {
            let mut ptr = ::std::mem::uninitialized();
            let hr = self.ptr.CreatePlacedResource(
                heap.ptr.as_mut_ptr(),
                heap_offset,
                desc as *const _ as *const _,
                initial_state,
                ::std::ptr::null(),
                & ::dxguid::IID_ID3D12Resource,
                &mut ptr as *mut _ as *mut _
            );

            WinError::from_hresult_or_ok(hr, || PlacedResource::from_raw(
                RawResource{ptr: ComPtr::new(ptr)}, heap.clone(), heap_offset, alloc_info
            ))
        }
    }

    #[inline]
    pub fn create_cbv_srv_uav_heap(&mut self, builder: &DescriptorHeapBuilder) -> Result<CbvSrvUavHeap, WinError> {
        builder.build_cbv_srv_uav_heap(self)
    }

    #[inline]
    pub fn create_rtv_heap(&mut self, builder: &DescriptorHeapBuilder) -> Result<RtvHeap, WinError> {
        builder.build_rtv_heap(self)
    }

    #[inline]
    pub fn create_dsv_heap(&mut self, builder: &DescriptorHeapBuilder) -> Result<DsvHeap, WinError> {
        builder.build_dsv_heap(self)
    }

    #[inline]
    pub fn create_sampler_heap(&mut self, builder: &DescriptorHeapBuilder) -> Result<SamplerHeap, WinError> {
        builder.build_sampler_heap(self)
    }

    // TODO: typed command lists? relation ship with command allocators?
    #[inline]
    pub fn create_command_list(
        &mut self, node_mask: u32, list_type: CommandListType,
        alloc: &mut CommandAllocator, initial_state: Option<&PipelineState>
    ) -> Result<GraphicsCommandList, WinError> {
        let initial_state = if let Some(state) = initial_state {
            state.ptr.as_mut_ptr()
        } else {
            ::std::ptr::null_mut()
        };
        unsafe {
            let mut ret = ::std::mem::uninitialized();
            let hr = self.ptr.CreateCommandList(
                node_mask, ::std::mem::transmute(list_type), alloc.ptr.as_mut_ptr(),
                initial_state, & ::dxguid::IID_ID3D12GraphicsCommandList,
                &mut ret as *mut *mut _ as *mut *mut _
            );

            WinError::from_hresult_or_ok(hr, || GraphicsCommandList{
                ptr: ComPtr::new(ret)
            })
        }
    }

    // TODO: attempts to create a pipeline state. blocker: PSO desc

    // TODO: add method for ReservedResouce creation. blocker: ReservedResource
    // TODO: add methods for resource tiling
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
impl_device_child!(Heap, ptr);
impl_device_child!(RawResource, ptr);
impl_device_child!(Fence, ptr);
impl_device_child!(CbvSrvUavHeap, ptr);
impl_device_child!(DsvHeap, ptr);
impl_device_child!(RtvHeap, ptr);
impl_device_child!(SamplerHeap, ptr);
impl_device_child!(GraphicsCommandList, ptr);
impl_device_child!(PipelineState, ptr);

impl DeviceChild for CommittedResource {
    #[inline]
    fn get_device(&mut self) -> Result<Device, WinError> {
        self.as_raw().get_device()
    }
}

impl DeviceChild for PlacedResource {
    #[inline]
    fn get_device(&mut self) -> Result<Device, WinError> {
        self.as_raw().get_device()
    }
}
