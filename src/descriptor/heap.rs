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
use super::desc::{SrvDesc, CbvDesc, RtvDesc, DsvDesc, SamplerDesc};


/// descriptor heap builder struct
#[derive(Copy, Clone, Debug)]
pub struct DescriptorHeapBuilder {
    pub num_descriptors: u32,
    pub node_mask: u32,
}

impl DescriptorHeapBuilder {
    #[inline]
    pub fn new(num_descriptors: u32) -> Self {
        DescriptorHeapBuilder{
            num_descriptors,
            node_mask: 0,
        }
    }

    pub fn build_csu_heap_shader_visible(&self, device: &mut Device) -> Result<CsuHeapSv, WinError> {
        let desc = ::winapi::D3D12_DESCRIPTOR_HEAP_DESC{
            Type: ::winapi::D3D12_DESCRIPTOR_HEAP_TYPE_CBV_SRV_UAV,
            NumDescriptors: self.num_descriptors,
            Flags: ::winapi::D3D12_DESCRIPTOR_HEAP_FLAG_SHADER_VISIBLE,
            NodeMask: self.node_mask,
        };
        unsafe {
            let mut ret = ::std::mem::uninitialized();
            let hr = device.ptr.CreateDescriptorHeap(
                &desc, & ::dxguid::IID_ID3D12DescriptorHeap,
                &mut ret as *mut *mut _ as *mut *mut _
            );
            WinError::from_hresult_or_ok(hr, || CsuHeapSv{
                ptr: ComPtr::new(ret),
                num_descriptors: self.num_descriptors,
                handle_increment_size: device.ptr.GetDescriptorHandleIncrementSize(::winapi::D3D12_DESCRIPTOR_HEAP_TYPE_CBV_SRV_UAV)
            })
        }
    }

    pub fn build_csu_heap(&self, device: &mut Device) -> Result<CsuHeapNsv, WinError> {
        let desc = ::winapi::D3D12_DESCRIPTOR_HEAP_DESC{
            Type: ::winapi::D3D12_DESCRIPTOR_HEAP_TYPE_CBV_SRV_UAV,
            NumDescriptors: self.num_descriptors,
            Flags: ::winapi::D3D12_DESCRIPTOR_HEAP_FLAG_NONE,
            NodeMask: self.node_mask,
        };
        unsafe {
            let mut ret = ::std::mem::uninitialized();
            let hr = device.ptr.CreateDescriptorHeap(
                &desc, & ::dxguid::IID_ID3D12DescriptorHeap,
                &mut ret as *mut *mut _ as *mut *mut _
            );
            WinError::from_hresult_or_ok(hr, || CsuHeapNsv{
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
            Flags: ::winapi::D3D12_DESCRIPTOR_HEAP_FLAG_NONE, // can't be shader visible
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
            Flags: ::winapi::D3D12_DESCRIPTOR_HEAP_FLAG_NONE, // can't be shader visible
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

    pub fn build_sampler_heap_shader_visible(&self, device: &mut Device) -> Result<SamplerHeapSv, WinError> {
        let desc = ::winapi::D3D12_DESCRIPTOR_HEAP_DESC{
            Type: ::winapi::D3D12_DESCRIPTOR_HEAP_TYPE_SAMPLER,
            NumDescriptors: self.num_descriptors,
            Flags: ::winapi::D3D12_DESCRIPTOR_HEAP_FLAG_SHADER_VISIBLE,
            NodeMask: self.node_mask
        };
        unsafe {
            let mut ret = ::std::mem::uninitialized();
            let hr = device.ptr.CreateDescriptorHeap(
                &desc, & ::dxguid::IID_ID3D12DescriptorHeap,
                &mut ret as *mut *mut _ as *mut *mut _
            );
            WinError::from_hresult_or_ok(hr, || SamplerHeapSv{
                ptr: ComPtr::new(ret),
                num_descriptors: self.num_descriptors,
                handle_increment_size: device.ptr.GetDescriptorHandleIncrementSize(::winapi::D3D12_DESCRIPTOR_HEAP_TYPE_SAMPLER)
            })
        }
    }

    pub fn build_sampler_heap(&self, device: &mut Device) -> Result<SamplerHeapNsv, WinError> {
        let desc = ::winapi::D3D12_DESCRIPTOR_HEAP_DESC{
            Type: ::winapi::D3D12_DESCRIPTOR_HEAP_TYPE_SAMPLER,
            NumDescriptors: self.num_descriptors,
            Flags: ::winapi::D3D12_DESCRIPTOR_HEAP_FLAG_NONE,
            NodeMask: self.node_mask
        };
        unsafe {
            let mut ret = ::std::mem::uninitialized();
            let hr = device.ptr.CreateDescriptorHeap(
                &desc, & ::dxguid::IID_ID3D12DescriptorHeap,
                &mut ret as *mut *mut _ as *mut *mut _
            );
            WinError::from_hresult_or_ok(hr, || SamplerHeapNsv{
                ptr: ComPtr::new(ret),
                num_descriptors: self.num_descriptors,
                handle_increment_size: device.ptr.GetDescriptorHandleIncrementSize(::winapi::D3D12_DESCRIPTOR_HEAP_TYPE_SAMPLER)
            })
        }
    }
}

pub trait CsuHeap: DescriptorHeap {
    /// create a srv on this heap at `index`.
    /// `None` resource creates a null-binding providing 0 reads and discared writes.
    /// `None` desc creates a default view if possible, inheriting resource format and descriptions
    fn create_srv(
        &mut self, device: &mut Device,
        resource: Option<&RawResource>,  // TODO: typed resources?
        desc: Option<&SrvDesc>,
        index: u32
    );

    /// create a uav on this heap at `index`.
    /// `None` resource creates a null-binding providing 0 reads and discared writes.
    /// `None` desc creates a default view if possible, inheriting resource format and descriptions. [more info](https://msdn.microsoft.com/zh-cn/library/windows/desktop/dn788674(v=vs.85).aspx)
    fn create_uav(
        &mut self, device: &mut Device,
        resource: Option<&RawResource>,  // TODO: typed resources?
        counter: Option<&RawResource>,
        desc: Option<&SrvDesc>,
        index: u32
    );

    /// create a cbv on this heap at `index`.
    /// TODO: double check optional desc
    fn create_cbv(
        &mut self, device: &mut Device, desc: &CbvDesc, index: u32
    );
}

macro_rules! impl_csu_heap {
    ($Heap: ty) => {
impl CsuHeap for $Heap{
    fn create_srv(
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
                self.get_cpu_handle(index).into()
            )
        }
    }
    
    fn create_uav(
        &mut self, device: &mut Device,
        resource: Option<&RawResource>,  // TODO: typed resources?
        counter: Option<&RawResource>,
        desc: Option<&SrvDesc>,
        index: u32
    ) {
        let presource = if let Some(resource) = resource {
            resource.ptr.as_mut_ptr()
        } else {
            ::std::ptr::null_mut()
        };
        let pcounter = if let Some(counter) = counter {
            counter.ptr.as_mut_ptr()
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
            device.ptr.CreateUnorderedAccessView(
                presource, pcounter, pdesc as *const _, 
                self.get_cpu_handle(index).into()
            )
        }
    }

    fn create_cbv(
        &mut self, device: &mut Device, desc: &CbvDesc, index: u32
    ) {
        unsafe {
            device.ptr.CreateConstantBufferView(
                desc as *const _ as *const _,
                self.get_cpu_handle(index).into()                
            )
        }
    }
}
    }
}

/// a heap that can hold cbv, srv and uavs, that is not shader visible.
#[derive(Clone, Debug)]
pub struct CsuHeapNsv {
    pub ptr: ComPtr<ID3D12DescriptorHeap>,
    num_descriptors: u32,
    handle_increment_size: u32,
}

/// a heap that can hold cbv, srv and uavs, that is shader visible.
#[derive(Clone, Debug)]
pub struct CsuHeapSv {
    pub ptr: ComPtr<ID3D12DescriptorHeap>,
    num_descriptors: u32,
    handle_increment_size: u32,
}

impl_csu_heap!(CsuHeapNsv);
impl_csu_heap!(CsuHeapSv);

/// a heap that can hold render target views
#[derive(Clone, Debug)]
pub struct RtvHeap {
    pub ptr: ComPtr<ID3D12DescriptorHeap>,
    num_descriptors: u32,
    handle_increment_size: u32,
}

impl RtvHeap {
    /// create a render target view on the given resource.
    /// A `None` desc means to create a default view if possible
    pub fn create_rtv(
        &mut self, device: &mut Device, resource: Option<&mut RawResource>,
        desc: Option<&RtvDesc>, index: u32
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
            device.ptr.CreateRenderTargetView(
                presource, pdesc as *const _, 
                self.get_cpu_handle(index).into()
            )
        }
    }
}

/// a heap that can hold depth stencil views
#[derive(Clone, Debug)]
pub struct DsvHeap {
    pub ptr: ComPtr<ID3D12DescriptorHeap>,
    num_descriptors: u32,
    handle_increment_size: u32,
}

impl DsvHeap{
    /// create a depth stencil view on the given resource.
    /// A `None` desc means to create a default view if possible
    pub fn create_dsv(
        &mut self, device: &mut Device, resource: Option<&mut RawResource>,
        desc: Option<&DsvDesc>, index: u32
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
            device.ptr.CreateDepthStencilView(
                presource, pdesc as *const _, 
                self.get_cpu_handle(index).into()
            )
        }
    }
}

/// a heap that can hold samplers, that is not shader visible
#[derive(Clone, Debug)]
pub struct SamplerHeapNsv {
    pub ptr: ComPtr<ID3D12DescriptorHeap>,
    num_descriptors: u32,
    handle_increment_size: u32,
}

/// a heap that can hold samplers, that is shader visible
#[derive(Clone, Debug)]
pub struct SamplerHeapSv {
    pub ptr: ComPtr<ID3D12DescriptorHeap>,
    num_descriptors: u32,
    handle_increment_size: u32,
}

pub trait SamplerHeap: DescriptorHeap {
    /// create a sampler object on the heap
    fn create_sampler(
        &mut self, device: &mut Device, desc: &SamplerDesc, index: u32
    );
}

macro_rules! impl_sampler_heap {
    ($Heap: ty) => {
impl SamplerHeap for $Heap {
    fn create_sampler(
        &mut self, device: &mut Device, desc: &SamplerDesc, index: u32
    ) {
        unsafe {
            device.ptr.CreateSampler(
                desc as *const _ as *const _,
                self.get_cpu_handle(index).into()
            )
        }
    }
}
    }
}

impl_sampler_heap!(SamplerHeapNsv);
impl_sampler_heap!(SamplerHeapSv);

/// represents a descriptor heap
pub trait DescriptorHeap {
    type CpuHandle: Into<::winapi::D3D12_CPU_DESCRIPTOR_HANDLE>;
    type GpuHandle: Into<::winapi::D3D12_GPU_DESCRIPTOR_HANDLE>;
    /// get the raw pointer
    fn as_raw_ptr(&mut self) -> &mut ComPtr<ID3D12DescriptorHeap>;

    /// get type of the heap
    fn get_type(&self) -> ::winapi::D3D12_DESCRIPTOR_HEAP_TYPE;

    /// get Cpu handle of a descriptor at `offset` on the heap
    fn get_cpu_handle(&mut self, offset: u32) -> Self::CpuHandle;

    /// get Gpu handle of a descriptor at `offset` on the heap
    fn get_gpu_handle(&mut self, offset: u32) -> Self::GpuHandle;

    /// get the number of descriptors this heap can hold
    fn len(&self) -> u32;

    /// get the handle incremental size of this heap
    fn get_handle_increment_size(&self) -> u32;
}

macro_rules! impl_handles {
    ($CpuHandle: ident, $GpuHandle: ident) => {
        #[derive(Copy, Clone, Debug)]
        #[repr(C)]
        pub struct $CpuHandle {
            pub ptr: usize,
        }

        impl From<$CpuHandle> for ::winapi::D3D12_CPU_DESCRIPTOR_HANDLE {
            #[inline]
            fn from(h: $CpuHandle) -> Self {
                unsafe { ::std::mem::transmute(h) }
            }
        }

        #[derive(Copy, Clone, Debug)]
        #[repr(C)]
        pub struct $GpuHandle {
            pub ptr: u64,
        }

        impl From<$GpuHandle> for ::winapi::D3D12_GPU_DESCRIPTOR_HANDLE {
            #[inline]
            fn from(h: $GpuHandle) -> Self {
                unsafe { ::std::mem::transmute(h) }
            }
        }
    }
}

impl_handles!(CpuCsuHandle, GpuCsuHandle);
impl_handles!(CpuRtvHandle, GpuRtvHandle);
impl_handles!(CpuDsvHandle, GpuDsvHandle);
impl_handles!(CpuSamplerHandle, GpuSamplerHandle);

macro_rules! impl_dh {
    ($Heap: ty, $ptr: ident, $msize: ident, $item_size: ident, $Type: ident, $CpuHandle: ident, $GpuHandle: ident) => {
        impl DescriptorHeap for $Heap {
            type GpuHandle = $GpuHandle;
            type CpuHandle = $CpuHandle;
            #[inline]
            fn get_type(&self) -> ::winapi::D3D12_DESCRIPTOR_HEAP_TYPE {
                $Type
            }

            #[inline]
            fn as_raw_ptr(&mut self) -> &mut ComPtr<ID3D12DescriptorHeap> {
                &mut self.$ptr
            }

            fn get_cpu_handle(&mut self, offset: u32) -> $CpuHandle {
                assert!(offset<self.$msize);
                let mut ret = $CpuHandle{ptr: 0};
                unsafe {
                    self.$ptr.GetCPUDescriptorHandleForHeapStart(
                        &mut ret as *mut _ as *mut _
                    );
                }
                ret.ptr += offset as usize *self.$item_size as usize;
                ret
            }

            fn get_gpu_handle(&mut self, offset: u32) -> $GpuHandle {
                assert!(offset<self.$msize);
                let mut ret = $GpuHandle{ptr: 0};
                unsafe {
                    self.$ptr.GetGPUDescriptorHandleForHeapStart(
                        &mut ret as *mut _ as *mut _
                    );
                }
                ret.ptr += offset as u64 *self.$item_size as u64;
                ret
            }

            #[inline]
            fn len(&self) -> u32 { self.$msize }

            #[inline]
            fn get_handle_increment_size(&self) -> u32 { self.$item_size }
        }
    }
}

impl_dh!(CsuHeapSv, ptr, num_descriptors, handle_increment_size, D3D12_DESCRIPTOR_HEAP_TYPE_CBV_SRV_UAV, CpuCsuHandle, GpuCsuHandle);
impl_dh!(CsuHeapNsv, ptr, num_descriptors, handle_increment_size, D3D12_DESCRIPTOR_HEAP_TYPE_CBV_SRV_UAV, CpuCsuHandle, GpuCsuHandle);
impl_dh!(DsvHeap, ptr, num_descriptors, handle_increment_size,  D3D12_DESCRIPTOR_HEAP_TYPE_DSV, CpuDsvHandle, GpuDsvHandle);
impl_dh!(RtvHeap, ptr, num_descriptors, handle_increment_size, D3D12_DESCRIPTOR_HEAP_TYPE_RTV, CpuRtvHandle, GpuRtvHandle);
impl_dh!(SamplerHeapSv, ptr, num_descriptors, handle_increment_size, D3D12_DESCRIPTOR_HEAP_TYPE_SAMPLER, CpuSamplerHandle, GpuSamplerHandle);
impl_dh!(SamplerHeapNsv, ptr, num_descriptors, handle_increment_size, D3D12_DESCRIPTOR_HEAP_TYPE_SAMPLER, CpuSamplerHandle, GpuSamplerHandle);

/// represents a descriptor heap that can be used as a copy source
pub trait DescriptorHeapCopy<T: DescriptorHeap>: DescriptorHeap {
    /// perform immediate copy of a slice of descriptors on CPU timeslice
    /// through the given device. `self` must not be shader visible.
    // TODO: type check for shader visibility?
    fn copy_descriptors_to(
        &mut self, dst: &mut T, device: &mut Device,
        src_offset: u32, dst_offset: u32, num_descriptors: u32
    ) {
        assert!(src_offset+num_descriptors<=self.len());
        assert!(dst_offset+num_descriptors<=dst.len());
        let heap_type = self.get_type();
        unsafe {
            device.ptr.CopyDescriptorsSimple(
                num_descriptors,
                dst.get_cpu_handle(dst_offset).into(),
                self.get_cpu_handle(src_offset).into(),
                heap_type
            )
        }
    }
}

impl DescriptorHeapCopy<CsuHeapSv> for CsuHeapNsv {}
impl DescriptorHeapCopy<CsuHeapNsv> for CsuHeapNsv {}
impl DescriptorHeapCopy<SamplerHeapSv> for SamplerHeapNsv {}
impl DescriptorHeapCopy<SamplerHeapNsv> for SamplerHeapNsv {}
impl DescriptorHeapCopy<RtvHeap> for RtvHeap {}
impl DescriptorHeapCopy<DsvHeap> for DsvHeap {}
