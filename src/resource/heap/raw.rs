// Copyright 2017 Dasein Phaos aka. Luxko
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.\

//! Defines a raw heap

use winapi::ID3D12Heap;
use comptr::ComPtr;
use super::{HeapDesc, HeapAlignment};

/// a continous memory region
#[derive(Clone, Debug)]
pub struct RawHeap {
    pub(crate) ptr: ComPtr<ID3D12Heap>,
    size: u64,
    alignment: HeapAlignment,
}

impl RawHeap {
    /// get a heap from a ComPtr
    #[inline]
    pub fn from_comptr(ptr: ComPtr<ID3D12Heap>) -> RawHeap {
        let mut ret = RawHeap{ptr, size: 0, alignment: Default::default()};
        let desc = ret.get_desc();
        ret.size = desc.size;
        ret.alignment = desc.alignment;
        ret
    }

    /// get heap descriptions
    #[inline]
    pub fn get_desc(&mut self) -> HeapDesc {
        unsafe {
            let mut ret = ::std::mem::uninitialized();
            self.ptr.GetDesc(&mut ret);
            ::std::mem::transmute(ret)
        }
    }

    /// get heap size
    #[inline]
    pub fn size(&self) -> u64 {
        self.size
    }

    /// get heap alignment
    #[inline]
    pub fn alignment(&self) -> HeapAlignment {
        self.alignment
    }
}
