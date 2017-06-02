// Copyright 2017 Dasein Phaos aka. Luxko
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! a descriptor heap is where the descriptors resides on

use comptr::ComPtr;
use winapi::{ID3D12DescriptorHeap, D3D12_DESCRIPTOR_HEAP_TYPE_CBV_SRV_UAV, D3D12_DESCRIPTOR_HEAP_TYPE_DSV, D3D12_DESCRIPTOR_HEAP_TYPE_RTV, D3D12_DESCRIPTOR_HEAP_TYPE_SAMPLER};
use resource::RawResource;
use error::WinError;
use device::Device;
use super::desc::SrvDesc;


/// descriptor heap builder struct
#[derive(Copy, Clone, Debug)]
pub struct DescriptorHeapBuilder {
    pub shader_visible: bool,
    pub num_descriptors: u32,
    pub node_mask: u32,
}

impl DescriptorHeapBuilder {
    #[inline]
    pub fn new(num_descriptors: u32) -> Self {
        DescriptorHeapBuilder{
            shader_visible: true,
            num_descriptors,
            node_mask: 0,
        }
    }

    pub fn build_cbv_srv_uav_heap(&self, device: &mut Device) -> Result<CbvSrvUavHeap, WinError> {
        let desc = ::winapi::D3D12_DESCRIPTOR_HEAP_DESC{
            Type: ::winapi::D3D12_DESCRIPTOR_HEAP_TYPE_CBV_SRV_UAV,
            NumDescriptors: self.num_descriptors,
            Flags: if self.shader_visible {
                ::winapi::D3D12_DESCRIPTOR_HEAP_FLAG_SHADER_VISIBLE
            } else {
                ::winapi::D3D12_DESCRIPTOR_HEAP_FLAG_NONE
            },
            NodeMask: self.node_mask
        };
        unsafe {
            let mut ret = ::std::mem::uninitialized();
            let hr = device.ptr.CreateDescriptorHeap(
                &desc, & ::dxguid::IID_ID3D12DescriptorHeap,
                &mut ret as *mut *mut _ as *mut *mut _
            );
            WinError::from_hresult_or_ok(hr, || CbvSrvUavHeap{
                ptr: ComPtr::new(ret),
                num_descriptors: self.num_descriptors,
                handle_increment_size: device.ptr.GetDescriptorHandleIncrementSize(::winapi::D3D12_DESCRIPTOR_HEAP_TYPE_CBV_SRV_UAV)
            })
        }
    }

    pub fn build_rtv_heap(&self, device: &mut Device) -> Result<RtvHeap, WinError> {
        let desc = ::winapi::D3D12_DESCRIPTOR_HEAP_DESC{
            Type: ::winapi::D3D12_DESCRIPTOR_HEAP_TYPE_RTV,
            NumDescriptors: self.num_descriptors,
            Flags: if self.shader_visible {
                ::winapi::D3D12_DESCRIPTOR_HEAP_FLAG_SHADER_VISIBLE
            } else {
                ::winapi::D3D12_DESCRIPTOR_HEAP_FLAG_NONE
            },
            NodeMask: self.node_mask
        };
        unsafe {
            let mut ret = ::std::mem::uninitialized();
            let hr = device.ptr.CreateDescriptorHeap(
                &desc, & ::dxguid::IID_ID3D12DescriptorHeap,
                &mut ret as *mut *mut _ as *mut *mut _
            );
            WinError::from_hresult_or_ok(hr, || RtvHeap{
                ptr: ComPtr::new(ret),
                num_descriptors: self.num_descriptors,
                handle_increment_size: device.ptr.GetDescriptorHandleIncrementSize(::winapi::D3D12_DESCRIPTOR_HEAP_TYPE_RTV)
            })
        }
    }

    pub fn build_dsv_heap(&self, device: &mut Device) -> Result<DsvHeap, WinError> {
        let desc = ::winapi::D3D12_DESCRIPTOR_HEAP_DESC{
            Type: ::winapi::D3D12_DESCRIPTOR_HEAP_TYPE_DSV,
            NumDescriptors: self.num_descriptors,
            Flags: if self.shader_visible {
                ::winapi::D3D12_DESCRIPTOR_HEAP_FLAG_SHADER_VISIBLE
            } else {
                ::winapi::D3D12_DESCRIPTOR_HEAP_FLAG_NONE
            },
            NodeMask: self.node_mask
        };
        unsafe {
            let mut ret = ::std::mem::uninitialized();
            let hr = device.ptr.CreateDescriptorHeap(
                &desc, & ::dxguid::IID_ID3D12DescriptorHeap,
                &mut ret as *mut *mut _ as *mut *mut _
            );
            WinError::from_hresult_or_ok(hr, || DsvHeap{
                ptr: ComPtr::new(ret),
                num_descriptors: self.num_descriptors,
                handle_increment_size: device.ptr.GetDescriptorHandleIncrementSize(::winapi::D3D12_DESCRIPTOR_HEAP_TYPE_DSV)
            })
        }
    }

    pub fn build_sampler_heap(&self, device: &mut Device) -> Result<SamplerHeap, WinError> {
        let desc = ::winapi::D3D12_DESCRIPTOR_HEAP_DESC{
            Type: ::winapi::D3D12_DESCRIPTOR_HEAP_TYPE_SAMPLER,
            NumDescriptors: self.num_descriptors,
            Flags: if self.shader_visible {
                ::winapi::D3D12_DESCRIPTOR_HEAP_FLAG_SHADER_VISIBLE
            } else {
                ::winapi::D3D12_DESCRIPTOR_HEAP_FLAG_NONE
            },
            NodeMask: self.node_mask
        };
        unsafe {
            let mut ret = ::std::mem::uninitialized();
            let hr = device.ptr.CreateDescriptorHeap(
                &desc, & ::dxguid::IID_ID3D12DescriptorHeap,
                &mut ret as *mut *mut _ as *mut *mut _
            );
            WinError::from_hresult_or_ok(hr, || SamplerHeap{
                ptr: ComPtr::new(ret),
                num_descriptors: self.num_descriptors,
                handle_increment_size: device.ptr.GetDescriptorHandleIncrementSize(::winapi::D3D12_DESCRIPTOR_HEAP_TYPE_SAMPLER)
            })
        }
    }
}


#[derive(Clone, Debug)]
pub struct CbvSrvUavHeap {
    pub ptr: ComPtr<ID3D12DescriptorHeap>,
    pub num_descriptors: u32,
    pub handle_increment_size: u32,
}

impl CbvSrvUavHeap{
    /// create a srv on this heap at `index`.
    /// `None` resource creates a null-binding providing 0 reads and discared writes.
    /// `None` desc creates a default view if possible, inheriting resource format and descriptions
    pub fn create_srv(
        &mut self, device: &mut Device,
        resource: Option<&RawResource>,  // TODO: typed resources?
        desc: Option<&SrvDesc>,
        index: u32
    ) {
        let presource = if let Some(resource) = resource {
            resource.ptr.as_mut_ptr()
        } else {
            ::std::ptr::null_mut()
        };
        let cdesc = if let Some(desc) = desc {
            desc.into_cstruct()
        } else {
            unsafe {::std::mem::uninitialized()}
        };
        let pdesc = if desc.is_some() { &cdesc as *const _ } else {::std::ptr::null()};
        unsafe {
            device.ptr.CreateShaderResourceView(
                presource, pdesc as *const _, 
                ::std::mem::transmute(self.get_cpu_handle(index))
            )
        }
    }
}

#[derive(Clone, Debug)]
pub struct RtvHeap {
    pub ptr: ComPtr<ID3D12DescriptorHeap>,
    pub num_descriptors: u32,
    pub handle_increment_size: u32,
}

#[derive(Clone, Debug)]
pub struct DsvHeap {
    pub ptr: ComPtr<ID3D12DescriptorHeap>,
    pub num_descriptors: u32,
    pub handle_increment_size: u32,
}

#[derive(Clone, Debug)]
pub struct SamplerHeap {
    pub ptr: ComPtr<ID3D12DescriptorHeap>,
    pub num_descriptors: u32,
    pub handle_increment_size: u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct CpuDHHandle {
    pub ptr: usize,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct GpuDHHandle {
    pub ptr: u64,
}

pub trait DescriptorHeap {
    /// get type of the heap
    fn get_type(&self) -> ::winapi::D3D12_DESCRIPTOR_HEAP_TYPE;

    /// get Cpu handle of a descriptor at `offset` on the heap
    fn get_cpu_handle(&mut self, offset: u32) -> CpuDHHandle;

    /// get Gpu handle of a descriptor at `offset` on the heap
    fn get_gpu_handle(&mut self, offset: u32) -> GpuDHHandle;

    /// perform immediate copy of a slice of descriptors on CPU side through the given device
    fn copy_descriptors_to(
        &mut self, dst: &mut Self, device: &mut Device,
        src_offset: u32, dst_offset: u32, num_descriptors: u32
    );
}

macro_rules! impl_dh {
    ($Heap: ty, $ptr: ident, $msize: ident, $item_size: ident, $Type: ident) => {
        impl DescriptorHeap for $Heap {
            #[inline]
            fn get_type(&self) -> ::winapi::D3D12_DESCRIPTOR_HEAP_TYPE {
                $Type
            }

            fn get_cpu_handle(&mut self, offset: u32) -> CpuDHHandle {
                assert!(offset<self.$msize);
                let mut ret = CpuDHHandle{ptr: 0};
                unsafe {
                    self.$ptr.GetCPUDescriptorHandleForHeapStart(
                        &mut ret as *mut _ as *mut _
                    );
                }
                ret.ptr += offset as usize *self.$item_size as usize;
                ret
            }

            fn get_gpu_handle(&mut self, offset: u32) -> GpuDHHandle {
                assert!(offset<self.$msize);
                let mut ret = GpuDHHandle{ptr: 0};
                unsafe {
                    self.$ptr.GetGPUDescriptorHandleForHeapStart(
                        &mut ret as *mut _ as *mut _
                    );
                }
                ret.ptr += offset as u64 *self.$item_size as u64;
                ret
            }

            fn copy_descriptors_to(
                &mut self, dst: &mut Self, device: &mut Device,
                src_offset: u32, dst_offset: u32, num_descriptors: u32
            ) {
                assert!(src_offset+num_descriptors<=self.$msize);
                assert!(dst_offset+num_descriptors<=dst.$msize);
                let heap_type = self.get_type();
                unsafe {
                    device.ptr.CopyDescriptorsSimple(
                        num_descriptors,
                        ::std::mem::transmute(dst.get_cpu_handle(dst_offset)),
                        ::std::mem::transmute(self.get_cpu_handle(src_offset)),
                        heap_type
                    )
                }
            }
        }
    }
}

impl_dh!(CbvSrvUavHeap, ptr, num_descriptors, handle_increment_size, D3D12_DESCRIPTOR_HEAP_TYPE_CBV_SRV_UAV);
impl_dh!(DsvHeap, ptr, num_descriptors, handle_increment_size,  D3D12_DESCRIPTOR_HEAP_TYPE_DSV);
impl_dh!(RtvHeap, ptr, num_descriptors, handle_increment_size, D3D12_DESCRIPTOR_HEAP_TYPE_RTV);
impl_dh!(SamplerHeap, ptr, num_descriptors, handle_increment_size, D3D12_DESCRIPTOR_HEAP_TYPE_SAMPLER);
