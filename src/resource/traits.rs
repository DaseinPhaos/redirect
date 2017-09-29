// Copyright 2017 Dasein Phaos aka. Luxko
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Resource Traits

use super::raw::RawResource;
use error::WinError;
use descriptor::{CsuHeap, CbvDesc, SrvBufferDesc, SrvDesc, SrvDimension, UavDimension, UavBufferDesc, UavDesc};
use device::Device;
use super::buffer::BufferSlice;
use pipeline::ia::{IndexBufferView, VertexBufferView};

pub trait Resource {
    /// returns the raw resource reference
    fn as_raw(&self) -> &RawResource;
    /// returns the mutable raw resource reference
    fn as_raw_mut(&mut self) ->&mut RawResource;
}

/// a resource that allows rtv
pub unsafe trait AllowRenderTarget: Resource { }

/// a resource that allows dsv
pub unsafe trait AllowDepthStencil: Resource { }

/// a resource that allows uav
pub unsafe trait AllowUnorderedAccess: Resource { }

/// a resource that allows srv
pub unsafe trait AllowSharedResource: Resource { }

/// a resource that is placed on a heap
pub unsafe trait Placed: Resource {
    type Heap: super::heap::traits::Heap;

    fn get_placed_heap(&mut self) -> &mut Self::Heap;

    fn get_heap_offset(&self) -> u64;
}

// TODO: is this valid?
// pub unsafe trait AllowDisplay: Resource { }

// unsafe impl<T, H> AllowDisplay for T 
// where T: Placed<Heap=H>,
//       H: super::heap::traits::AllowDisplay { }

/// a resource that is only accessible through GPU
pub unsafe trait GpuOnly: Resource { }

unsafe impl<T, H> GpuOnly for T 
where T: Placed<Heap=H>, H: super::heap::traits::GpuOnly { }

/// a resouce backed up by an upload heap
pub unsafe trait Upload: Resource { }

unsafe impl<T, H> Upload for T 
where T: Placed<Heap=H>, H: super::heap::traits::Upload { }

/// a resource backed up by an readback heap
pub unsafe trait Readback: Resource { }

unsafe impl<T, H> Readback for T 
where T: Placed<Heap=H>, H: super::heap::traits::Readback { }

/// a buffer
pub unsafe trait Buffer: Resource {
    /// get the size of the buffer
    fn get_size(&self) -> u64;

    /// Check if this buffer can hold the given `buffer_slice`
    #[inline]
    fn is_compatible_with(&self, buffer_slice: BufferSlice) -> bool {
        (buffer_slice.offset + buffer_slice.length as u64) * buffer_slice.byte_stride as u64 <= self.get_size()
    }

    /// create a cbv for this buffer on `csu_heap` at `index`
    #[inline]
    fn create_cbv<DH: CsuHeap>(
        &mut self, device: &mut Device, csu_heap: &mut DH, index: u32
    ) {
        csu_heap.create_cbv(device, &CbvDesc{
            buffer_location: self.as_raw_mut().get_gpu_vaddress(),
            size: self.get_size() as u32, // TODO: is this safe?
        }, index);
    }

    // TODO: investigate rtv on buffer, https://msdn.microsoft.com/en-us/library/windows/desktop/dn770342(v=vs.85).aspx

    /// Create a srv for this buffer on `csu_heap` at `index` with the given `slice`
    /// TODO: verify, raw buffer support
    #[inline]
    fn create_srv<DH: CsuHeap>(
        &mut self, device: &mut Device, csu_heap: &mut DH,
        index: u32, slice: BufferSlice
    ) {
        debug_assert!(self.is_compatible_with(slice));
        csu_heap.create_srv(device, Some(self.as_raw()), Some(&SrvDesc{
            format: ::format::DXGI_FORMAT_UNKNOWN, // TODO: double check format for buffer
            dimension: SrvDimension::Buffer(SrvBufferDesc{
                offset: slice.offset, num_elements: slice.length,
                byte_stride: slice.byte_stride, raw: 0 // TODO: support raw buffers?
            }),
            component_mapping: Default::default(), // TODO: would different component mapping make sense here?
        }), index);
    }

    /// Create a uav for this buffer on `csu_heap` at `index` with the given `slice`
    /// TODO: verify
    #[inline]
    fn create_uav<DH: CsuHeap, B: AllowUnorderedAccess + Buffer>(
        buf: &mut B, device: &mut Device, csu_heap: &mut DH,
        index: u32, slice: BufferSlice
    ) {
        debug_assert!(buf.is_compatible_with(slice));
        csu_heap.create_uav(device, Some(buf.as_raw()), None, Some(&UavDesc{
            format: ::format::DXGI_FORMAT_UNKNOWN, // TODO: double check format for buffer
            dimension: UavDimension::Buffer(UavBufferDesc{
                offset: slice.offset, num_elements: slice.length,
                byte_stride: slice.byte_stride, 
                counter_offset: 0,
                raw: 0 // TODO: support raw buffers?
            })
        }), index);
    }

    /// Create a uav for this buffer on `csu_heap` at `index` with the given `slice` and `counter`
    /// TODO: verify
    #[inline]
    fn create_uav_with_counter<DH: CsuHeap, CounterBuf: Buffer, B: AllowUnorderedAccess + Buffer>(
        buf: &mut B, device: &mut Device, csu_heap: &mut DH,
        index: u32, slice: BufferSlice, counter: &mut CounterBuf,
        counter_offset: u64
    ) {
        debug_assert!(buf.is_compatible_with(slice));
        debug_assert!(counter_offset%4 == 0);
        debug_assert!(counter_offset <= counter.get_size());
        csu_heap.create_uav(device, Some(buf.as_raw()), Some(counter.as_raw()), Some(&UavDesc{
            format: ::format::DXGI_FORMAT_UNKNOWN, // TODO: double check format for buffer
            dimension: UavDimension::Buffer(UavBufferDesc{
                offset: slice.offset, num_elements: slice.length,
                byte_stride: slice.byte_stride, 
                counter_offset, raw: 0 // TODO: support raw buffers?
            })
        }), index);
    }

    /// Create a vbv for this buffer
    #[inline]
    fn create_vbv(&mut self, size: u32, stride: u32) -> VertexBufferView {
        debug_assert!(size as u64 <= self.get_size());
        VertexBufferView{
            location: self.as_raw_mut().get_gpu_vaddress(), size, stride
        }
    }

    /// Create an ibv for this buffer
    #[inline]
    fn create_ibv_u32(&mut self, size: u32) -> IndexBufferView {
        debug_assert!(size as u64 <= self.get_size());
        IndexBufferView{
            location: self.as_raw_mut().get_gpu_vaddress(), size,
            format: ::format::DXGI_FORMAT_R32_UINT,
        }
    }

    /// Create an ibv for this buffer
    #[inline]
    fn create_ibv_u16(&mut self, size: u32) -> IndexBufferView {
        debug_assert!(size as u64 <= self.get_size());
        IndexBufferView{
            location: self.as_raw_mut().get_gpu_vaddress(), size,
            format: ::format::DXGI_FORMAT_R16_UINT,
        }
    }
}

/// a cpu-writable buffer
// FIXME: data type safety?
pub unsafe trait CpuWriteBuffer: Buffer {
    fn write<T>(&mut self, data: T, range: Option<(usize, usize)>) -> Result<(), WinError>;

    fn write_slice<T>(&mut self, data: &[T], range: Option<(usize, usize)>) -> Result<(), WinError>;
}

unsafe impl<B: Buffer + Upload> CpuWriteBuffer for B {
    #[inline]
    fn write<T>(&mut self, data: T, range: Option<(usize, usize)>) -> Result<(), WinError> {
        if let Some(range) = range {
            debug_assert!(range.0 <= range.1);
            debug_assert!(range.1 <= self.get_size() as usize);
            debug_assert!(::std::mem::size_of::<T>() <= range.1 - range.0);
        }
        else {
            debug_assert!(::std::mem::size_of::<T>() <= self.get_size() as usize);
        }

        let raw = self.as_raw_mut();
        unsafe {
            let dst = raw.map(0, range)? as *mut T;
            ::std::ptr::copy_nonoverlapping(&data, dst, 1);
            raw.unmap(0, range);
        }
        Ok(())
    }

    fn write_slice<T>(&mut self, data: &[T], range: Option<(usize, usize)>) -> Result<(), WinError> {
        if let Some(range) = range {
            debug_assert!(range.0 <= range.1);
            debug_assert!(range.1 <= self.get_size() as usize);
            debug_assert!(::std::mem::size_of::<T>() * data.len() <= range.1 - range.0);
        }
        else {
            debug_assert!(::std::mem::size_of::<T>() * data.len() <= self.get_size() as usize);
        }

        let raw = self.as_raw_mut();
        unsafe {
            let dst = raw.map(0, range)? as *mut T;
            ::std::ptr::copy_nonoverlapping(data.as_ptr(), dst, data.len());
            raw.unmap(0, range);
        }
        Ok(())
    }
}

/// a cpu-readable buffer
// FIXME: data type safety?
pub unsafe trait CpuReadBuffer: Buffer {
    /// return the content of the buffer in `range`
    fn read(&mut self, range: Option<(usize, usize)>) -> Result<Vec<u8>, WinError>;
}

unsafe impl<B: Buffer + Readback> CpuReadBuffer for B {
    fn read(&mut self, range: Option<(usize, usize)>) -> Result<Vec<u8>, WinError> {
        let length = if let Some(range) = range {
            debug_assert!(range.0 <= range.1);
            debug_assert!(range.1 <= self.get_size() as usize);
            range.1 - range.0
        }
        else {
            self.get_size() as usize
        };

        let mut ret = Vec::with_capacity(length);
        let raw = self.as_raw_mut();
        unsafe {
            ret.set_len(length);
            let src = raw.map(0, range)?;
            ::std::ptr::copy_nonoverlapping(src, ret.as_mut_slice().as_mut_ptr(), length);
            raw.unmap(0, range);
        }
        Ok(ret)
    }
}
