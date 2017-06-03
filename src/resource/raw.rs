// Copyright 2017 Dasein Phaos aka. Luxko
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Raw resource

use winapi::ID3D12Resource;
use comptr::ComPtr;
use error::WinError;
use super::*;
use format::Box3u;

/// a raw resource
#[derive(Clone, Debug)]
pub struct RawResource {
    pub ptr: ComPtr<ID3D12Resource>,
}

impl RawResource {
    /// get resource description
    #[inline]
    pub fn get_desc(&mut self) -> ResourceDesc {
        unsafe {
            let mut ret = ::std::mem::uninitialized();
            self.ptr.GetDesc(&mut ret);
            ::std::mem::transmute(ret)
        }
    }

    /// get the GPU virtual address for a buffer resource, `0` for texture resources
    #[inline]
    pub fn get_gpu_vaddress(&mut self) -> GpuVAddress {
        unsafe { GpuVAddress{ptr: self.ptr.GetGPUVirtualAddress()}}
    }

    /// attempt to get the attached heap's info. This method would only work
    /// on committed or placed resources, not on reserved ones.
    #[inline]
    pub fn get_heap_info(&mut self) -> Result<(HeapProperties, HeapFlags), WinError> {
        unsafe {
            let mut hp = ::std::mem::uninitialized();
            let mut hf = ::std::mem::uninitialized();
            let hr = self.ptr.GetHeapProperties(&mut hp, &mut hf);
            WinError::from_hresult_or_ok(hr, || (
                ::std::mem::transmute(hp), ::std::mem::transmute(hf)
            ))
        }
    }

    /// get a CPU pointer to the specified subresource. [more info](https://msdn.microsoft.com/zh-cn/library/windows/desktop/dn788712(v=vs.85).aspx)
    pub unsafe fn map(
        &mut self, subresource: u32, range: Option<(usize, usize)>
    ) -> Result<*mut u8, WinError> {
        let mut d3drange: ::winapi::D3D12_RANGE = ::std::mem::uninitialized();
        let prange = if let Some(range) = range {
            // assert!(range.0<=range.1);
            d3drange.Begin = range.0 as _;
            d3drange.End = range.1 as _;
            &d3drange as *const _
        } else {
            ::std::ptr::null()
        };
        let mut ret = ::std::mem::uninitialized();
        let hr = self.ptr.Map(subresource, prange, &mut ret);
        WinError::from_hresult_or_ok(hr, || ret as *mut u8)
    }

    /// invalidates the CPU pointer to the specified subresource
    pub unsafe fn unmap(
        &mut self, subresource: u32, range: Option<(usize, usize)>
    ) {
        let mut d3drange: ::winapi::D3D12_RANGE = ::std::mem::uninitialized();
        let prange = if let Some(range) = range {
            // assert!(range.0<=range.1);
            d3drange.Begin = range.0 as _;
            d3drange.End = range.1 as _;
            &d3drange as *const _
        } else {
            ::std::ptr::null()
        };
        self.ptr.Unmap(subresource, prange);
    }

    /// use CPU to copy data from a subresource
    pub unsafe fn read_from_subresource(
        &mut self, dst_desc: ResourceChunkDesc,
        src_subresource: u32, src_box: Option<&Box3u>
    ) -> Result<(), WinError> {
        let pbox = if let Some(src_box) = src_box {
            src_box as *const _ as *const ::winapi::D3D12_BOX
        } else {
            ::std::ptr::null()
        };
        WinError::from_hresult(
            self.ptr.ReadFromSubresource(
                dst_desc.data as *mut u8 as *mut _,
                dst_desc.row_pitch, dst_desc.depth_pitch,
                src_subresource, pbox
            )
        )
    }

    /// use CPU to copy data into a subresource. [more info](https://msdn.microsoft.com/zh-cn/library/windows/desktop/dn914416(v=vs.85).aspx)
    pub unsafe fn write_to_subresource(
        &mut self, dst_subresource: u32, dst_box: Option<&Box3u>, 
        src_desc: ResourceChunkDesc
    ) -> Result<(), WinError> {
        let pbox = if let Some(dst_box) = dst_box {
            dst_box as *const _ as *const ::winapi::D3D12_BOX
        } else {
            ::std::ptr::null()
        };
        WinError::from_hresult(
            self.ptr.WriteToSubresource(
                dst_subresource, pbox,
                src_desc.data as *const u8 as *const _,
                src_desc.row_pitch, src_desc.depth_pitch
            )
        )
    }
}

/// describes a chunk of resource
#[derive(Copy, Clone, Debug)]
pub struct ResourceChunkDesc {
    /// pointer to the head of data
    pub data: *mut u8,
    /// distance from one row of data to the next
    pub row_pitch: u32,
    /// distance from one depth slice of data to the next
    pub depth_pitch: u32,
}

/// GPU virtual device
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct GpuVAddress {
    pub ptr: u64,
}

impl From<GpuVAddress> for ::winapi::D3D12_GPU_VIRTUAL_ADDRESS {
    #[inline]
    fn from(addr: GpuVAddress) -> Self {
        unsafe {::std::mem::transmute(addr)}
    }
}
