// Copyright 2017 Dasein Phaos aka. Luxko
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Types to describe a heap

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

impl HeapDesc{
    /// construction
    #[inline]
    pub fn new(size: u64, properties: HeapProperties, flags: HeapFlags) -> HeapDesc {
        HeapDesc{
            size, properties, flags, alignment: Default::default(),
        }
    }
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

impl Default for HeapProperties {
    #[inline]
    fn default() -> Self {
        HeapProperties::new(Default::default())
    }
}

bitflags!{
    /// [heap type](https://msdn.microsoft.com/library/windows/desktop/dn770374(v=vs.85).aspx).
    #[repr(C)]
    pub struct HeapType: u32 {
        /// GPU RW, no CPU access. This is the default heap type.
        const HEAP_TYPE_DEFAULT   = 1;
        /// Optimal for CPU write.
        /// Best for CPU write-once, GPU read-once data.
        /// Resources in this heap must be created with `GENERATE_READ` state, and
        /// cannot be changed away.
        const HEAP_TYPE_UPLOAD    = 2;
        /// Optimal for CPU write.
        /// Best for GPU write-once, CPU readable data.
        /// Resources in this heap must be created with `COPY_DEST` state, and
        /// cannot be changed away from this.
        const HEAP_TYPE_READBACK  = 3;
        /// Custom heap for advanced usage.
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
        /// The default cpu page property.
        const PAGE_PROPERTY_UNKNOWN        = 0;
        /// The CPU cannot access the heap, thus no property available.
        const PAGE_PROPERTY_NOT_AVAILABLE  = 1;
        const PAGE_PROPERTY_WRITE_COMBINE  = 2;
        const PAGE_PROPERTY_WRITE_BACK     = 3;
    }
}

impl Default for PageProperty {
    #[inline]
    fn default() -> PageProperty {
        PAGE_PROPERTY_UNKNOWN
    }
}

bitflags!{
    /// memory pool preference. [more info](https://msdn.microsoft.com/library/windows/desktop/dn770381(v=vs.85).aspx)
    #[repr(C)]
    pub struct MemoryPoolPreference: u32 {
        /// The default pool preference.
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
    #[repr(C)]
    pub struct HeapAlignment: u64 {
        /// alias for 64kb, the default.
        const HEAP_ALIGNMENT_DEFAULT = 0;
        /// 64kb aligned.
        const HEAP_ALIGNMENT_DEFAULT_RESOURCE_PLACEMENT = ::winapi::D3D12_DEFAULT_RESOURCE_PLACEMENT_ALIGNMENT as u64;
        /// 4mb aligned. MSAA resource heap must use this alignment.
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
    /// misc heap options. [more info](https://msdn.microsoft.com/library/windows/desktop/dn986730(v=vs.85).aspx)
    #[repr(C)]
    pub struct HeapFlags: u32 {
        /// The default, no options specified.
        const HEAP_FLAG_NONE                            = 0;
        /// a [shared heap](https://msdn.microsoft.com/library/windows/desktop/mt186623(v=vs.85).aspx)
        const HEAP_FLAG_SHARED                          = 0x1;
        /// the heap isn't allowed to contain buffers
        const HEAP_FLAG_DENY_BUFFERS                    = 0x4;
        /// the heap can contain swapchain surfaces
        const HEAP_FLAG_ALLOW_DISPLAY                   = 0x8;
        /// the heap can be shored across adapters
        const HEAP_FLAG_SHARED_CROSS_ADAPTER            = 0x20;
        /// the heap can't store render target or depth stencil textures
        const HEAP_FLAG_DENY_RT_DS_TEXTURES             = 0x40;
        /// the heap can't contain textures without `ALLOW_RENDER_TARGET` or `ALLOW_DEPTH_STENCIL` flags
        const HEAP_FLAG_DENY_NON_RT_DS_TEXTURES         = 0x80;
        /// unsupported
        const HEAP_FLAG_HARDWARE_PROTECTED              = 0x100;
        /// allow tools to support `MEM_WRITE_WATCH`
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