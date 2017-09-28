// Copyright 2017 Dasein Phaos aka. Luxko
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Heap traits

use super::raw::*;
use super::description::*;
use super::{DefaultHeap, UploadHeap, ReadbackHeap};

/// a resource heap
pub trait Heap {
    /// returns the raw heap reference
    fn as_raw(&self) -> &RawHeap;
    /// returns the mutable raw heap reference
    fn as_raw_mut(&mut self) ->&mut RawHeap;

    /// get heap size
    #[inline]
    fn size(&self) -> u64 {
        self.as_raw().size()
    }

    /// get heap alignment
    #[inline]
    fn alignment(&self) -> HeapAlignment {
        self.as_raw().alignment()
    }
}

/// a resource heap that accept buffers
pub unsafe trait AcceptBuffer: Heap { }

/// a resource heap that allows display
pub unsafe trait AllowDisplay: Heap { }

/// a resource heap that accepts MS textures
pub unsafe trait AcceptMsTexture: Heap { }

/// a resource heap with type Upload
pub unsafe trait Upload: Heap { }

/// a resource heap with type Readback
pub unsafe trait Readback: Heap { }

/// a resource heap with type Default
pub unsafe trait GpuOnly: Heap { }

pub unsafe trait AllowRtDs: Heap { }

pub unsafe trait AllowNonRtDs: Heap { }

impl_as_raw!(Heap, DefaultHeap, RawHeap);
unsafe impl AcceptBuffer for DefaultHeap {}
unsafe impl GpuOnly for DefaultHeap {}
unsafe impl AllowRtDs for DefaultHeap {}
unsafe impl AllowNonRtDs for DefaultHeap {}

impl_as_raw!(Heap, UploadHeap, RawHeap);
unsafe impl AcceptBuffer for UploadHeap {}
unsafe impl Upload for UploadHeap {}
unsafe impl AllowRtDs for UploadHeap {}
unsafe impl AllowNonRtDs for UploadHeap {}

impl_as_raw!(Heap, ReadbackHeap, RawHeap);
unsafe impl AcceptBuffer for ReadbackHeap {}
unsafe impl Readback for ReadbackHeap {}
unsafe impl AllowRtDs for ReadbackHeap {}
unsafe impl AllowNonRtDs for ReadbackHeap {}
