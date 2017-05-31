// Copyright 2017 Dasein Phaos aka. Luxko
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! a continous GPU memory region.

use winapi::ID3D12Heap;
use comptr::ComPtr;
use error::WinError;

/// a continous memory region
#[derive(Clone, Debug)]
pub struct Heap {
    pub ptr: ComPtr<ID3D12Heap>,
}

impl Heap {
    /// get heap descriptions
    pub fn get_desc(&mut self) -> HeapDesc {
        unsafe {
            let mut ret = ::std::mem::uninitialized();
            self.ptr.GetDesc(&mut ret);
            ::std::mem::transmute(ret)
        }
    }
}

/// description of a heap
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct HeapDesc {
    /// heap size in bytes
    pub size: u64,
    /// heap properties
    pub properties: HeapProperties,
    /// alignment
    pub alignment: HeapAlignment,
    /// misc flags
    pub flags: HeapFlags,
}

/// describes heap properties
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct HeapProperties {
    /// heap type
    pub heap_type: HeapType,
    /// cpu page property
    pub page: PageProperty,
    /// memory pool preference
    pub pool_preference: MemoryPoolPreference,
    pub creation_node_mask: u32,
    pub visible_node_mask: u32,
}

impl HeapProperties {
    #[inline]
    pub fn new(heap_type: HeapType) -> HeapProperties {
        HeapProperties{
            heap_type,
            page: Default::default(),
            pool_preference: Default::default(),
            creation_node_mask: 0,
            visible_node_mask: 0,
        }
    }
}

bitflags!{
    /// [heap type](https://msdn.microsoft.com/zh-cn/library/windows/desktop/dn770374(v=vs.85).aspx).
    #[repr(C)]
    pub struct HeapType: u32 {
        const HEAP_TYPE_DEFAULT   = 1;
        const HEAP_TYPE_UPLOAD    = 2;
        const HEAP_TYPE_READBACK  = 3;
        const HEAP_TYPE_CUSTOM    = 4;
    }
}

impl Default for HeapType {
    #[inline]
    fn default() -> HeapType {
        HEAP_TYPE_DEFAULT
    }
}

bitflags!{
    /// cpu page properties.
    #[repr(C)]
    pub struct PageProperty: u32 {
        const CPU_PAGE_PROPERTY_UNKNOWN        = 0;
        const CPU_PAGE_PROPERTY_NOT_AVAILABLE  = 1;
        const CPU_PAGE_PROPERTY_WRITE_COMBINE  = 2;
        const CPU_PAGE_PROPERTY_WRITE_BACK     = 3;
    }
}

impl Default for PageProperty {
    #[inline]
    fn default() -> PageProperty {
        CPU_PAGE_PROPERTY_UNKNOWN
    }
}

bitflags!{
    /// memory pool preference. [more info](https://msdn.microsoft.com/zh-cn/library/windows/desktop/dn770381(v=vs.85).aspx)
    #[repr(C)]
    pub struct MemoryPoolPreference: u32 {
        const MEMORY_POOL_UNKNOWN  = 0;
        const MEMORY_POOL_L0       = 1;
        const MEMORY_POOL_L1       = 2;
    }
}

impl Default for MemoryPoolPreference {
    #[inline]
    fn default() -> MemoryPoolPreference {
        MEMORY_POOL_UNKNOWN
    }
}

bitflags!{
    /// heap alignment
    pub struct HeapAlignment: u64 {
        /// alias for 64kb
        const HEAP_ALIGNMENT_DEFAULT = 0;
        const HEAP_ALIGNMENT_DEFAULT_RESOURCE_PLACEMENT = ::winapi::D3D12_DEFAULT_RESOURCE_PLACEMENT_ALIGNMENT as u64;
        const HEAP_ALIGNMENT_DEFAULT_MSAA_RESOURCE_PLACEMENT = ::winapi::D3D12_DEFAULT_MSAA_RESOURCE_PLACEMENT_ALIGNMENT as u64;
    }
}

impl Default for HeapAlignment {
    #[inline]
    fn default() -> Self {
        HEAP_ALIGNMENT_DEFAULT
    }
}

bitflags!{
    /// misc heap options. [more info](https://msdn.microsoft.com/zh-cn/library/windows/desktop/dn986730(v=vs.85).aspx)
    pub struct HeapFlags: u32 {
        const HEAP_FLAG_NONE                            = 0;
        const HEAP_FLAG_SHARED                          = 0x1;
        const HEAP_FLAG_DENY_BUFFERS                    = 0x4;
        const HEAP_FLAG_ALLOW_DISPLAY                   = 0x8;
        const HEAP_FLAG_SHARED_CROSS_ADAPTER            = 0x20;
        const HEAP_FLAG_DENY_RT_DS_TEXTURES             = 0x40;
        const HEAP_FLAG_DENY_NON_RT_DS_TEXTURES         = 0x80;
        const HEAP_FLAG_HARDWARE_PROTECTED              = 0x100;
        const HEAP_FLAG_ALLOW_WRITE_WATCH               = 0x200;
        const HEAP_FLAG_ALLOW_ALL_BUFFERS_AND_TEXTURES  = 0;
        const HEAP_FLAG_ALLOW_ONLY_BUFFERS              = 0xc0;
        const HEAP_FLAG_ALLOW_ONLY_NON_RT_DS_TEXTURES   = 0x44;
        const HEAP_FLAG_ALLOW_ONLY_RT_DS_TEXTURES       = 0x84;
    }
}

impl Default for HeapFlags {
    #[inline]
    fn default() -> Self {
        HEAP_FLAG_NONE
    }
}
