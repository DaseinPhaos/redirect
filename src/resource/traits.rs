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
