// Copyright 2017 Dasein Phaos aka. Luxko
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! resource

mod usage;
pub use self::usage::*;

mod heap;
pub use self::heap::*;

mod raw;
pub use self::raw::*;

mod barrier;
pub use self::barrier::*;

use format::*;
use swapchain::SampleDesc;

/// resource description
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ResourceDesc {
    pub dimension: ResourceDimension,
    pub alignment: u64,
    pub width: u64,
    pub height: u32,
    pub depth: u16,
    pub mip_levels: u16,
    pub format: DxgiFormat,
    pub sample_desc: SampleDesc,
    pub layout: TextureLayout,
    pub flags: ResourceFlags,
}

impl ResourceDesc{
    /// a buffer description
    #[inline]
    pub fn buffer(alignment: u64, size: u64, flags: ResourceFlags) -> ResourceDesc{
        ResourceDesc{
            dimension: RESOURCE_DIMENSION_BUFFER,
            alignment,
            width: size,
            height: 1,
            depth: 1,
            mip_levels: 1,
            format: DXGI_FORMAT_UNKNOWN,
            sample_desc: Default::default(),
            layout: TEXTURE_LAYOUT_ROW_MAJOR, // TODO: double check unknown
            flags: flags,
        }
    }

    /// a tex1d description
    #[inline]
    pub fn tex1d(
        alignment: u64, length: u64, array_size: u16,
        mip_levels: u16, format: DxgiFormat, flags: ResourceFlags
    ) -> ResourceDesc{
        ResourceDesc{
            dimension: RESOURCE_DIMENSION_TEXTURE1D,
            alignment,
            width: length,
            height: 1,
            depth: array_size,
            mip_levels: mip_levels,
            format: format,
            sample_desc: Default::default(),
            layout: TEXTURE_LAYOUT_UNKNOWN,
            flags: flags,
        }
    }

    /// a tex2d description
    #[inline]
    pub fn tex2d(
        alignment: u64, width: u64, height: u32,
        array_size: u16, mip_levels: u16,
        format: DxgiFormat, flags: ResourceFlags
    ) -> ResourceDesc{
        ResourceDesc{
            dimension: RESOURCE_DIMENSION_TEXTURE2D,
            alignment,
            width: width,
            height: height,
            depth: array_size,
            mip_levels: mip_levels,
            format: format,
            sample_desc: Default::default(),
            layout: TEXTURE_LAYOUT_UNKNOWN,
            flags: flags,
        }
    }

    /// a tex3d description
    #[inline]
    pub fn tex3d(
        alignment: u64, width: u64, height: u32,
        depth: u16, mip_levels: u16,
        format: DxgiFormat, flags: ResourceFlags
    ) -> ResourceDesc{
        ResourceDesc{
            dimension: RESOURCE_DIMENSION_TEXTURE3D,
            alignment, width, height, depth,
            mip_levels: mip_levels,
            format: format,
            sample_desc: Default::default(),
            layout: TEXTURE_LAYOUT_UNKNOWN,
            flags: flags,
        }
    }
}

impl From<ResourceDesc> for ::winapi::D3D12_RESOURCE_DESC {
    #[inline]
    fn from(desc: ResourceDesc) -> Self {
        unsafe{ ::std::mem::transmute(desc)}
    }
}

bitflags!{
    /// dimension i.e. type of the resource
    #[repr(C)]
    pub struct ResourceDimension: u32 {
        const RESOURCE_DIMENSION_UNKNOWN    = 0;
        const RESOURCE_DIMENSION_BUFFER     = 1;
        const RESOURCE_DIMENSION_TEXTURE1D  = 2;
        const RESOURCE_DIMENSION_TEXTURE2D  = 3;
        const RESOURCE_DIMENSION_TEXTURE3D  = 4;
    }
}

bitflags! {
    /// texture layout
    #[repr(C)]
    pub struct TextureLayout: u32 {
        /// adapter-dependent layout. driver choose optimal layout
        /// during resource creation
        const TEXTURE_LAYOUT_UNKNOWN                 = 0;
        /// data for the texture is stored in row-major order.
        /// only the following texture properties are supported:
        ///
        /// - `RESOURCE_DIMENSION_TEXTURE2D`
        /// - single mip level
        /// - single array slice
        /// 64kb alignment
        /// non-MSAA
        /// no `RESOURCE_FLAG_ALLOW_DEPTH_STENCIL`
        /// cannot be a YUV format
        ///
        /// Note that buffers are row major
        const TEXTURE_LAYOUT_ROW_MAJOR               = 1;
        const TEXTURE_LAYOUT_64KB_UNDEFINED_SWIZZLE  = 2;
        const TEXTURE_LAYOUT_64KB_STANDARD_SWIZZLE   = 3;
    }
}

impl Default for TextureLayout {
    #[inline]
    fn default() -> Self {
        TEXTURE_LAYOUT_UNKNOWN
    }
}

bitflags!{
    /// misc flags for resources. [more info](https://msdn.microsoft.com/zh-cn/library/windows/desktop/dn986742(v=vs.85).aspx)
    #[repr(C)]
    pub struct ResourceFlags: u32 {
        const RESOURCE_FLAG_NONE                       = 0;
        const RESOURCE_FLAG_ALLOW_RENDER_TARGET        = 0x1;
        const RESOURCE_FLAG_ALLOW_DEPTH_STENCIL        = 0x2;
        const RESOURCE_FLAG_ALLOW_UNORDERED_ACCESS     = 0x4;
        const RESOURCE_FLAG_DENY_SHADER_RESOURCE       = 0x8;
        const RESOURCE_FLAG_ALLOW_CROSS_ADAPTER        = 0x10;
        const RESOURCE_FLAG_ALLOW_SIMULTANEOUS_ACCESS  = 0x20;
    }
}

impl Default for ResourceFlags {
    #[inline]
    fn default() -> Self {
        RESOURCE_FLAG_NONE
    }
}

bitflags!{
    /// the state of a resource regarding how it is being used. [more](https://msdn.microsoft.com/zh-cn/library/windows/desktop/dn986744%28v=vs.85%29.aspx?f=255&MSPPError=-2147217396)
    #[repr(C)]
    pub struct ResourceStates: u32 {
        /// resource should be in this state when
        ///
        /// - being translated across `COPY` queue to/from `DIRECT/COMPUTE` queues
        /// - for CPU accessing
        const RESOURCE_STATE_COMMON                      = 0;
        /// a subresource should be in this state when accessed as a vertex buffer or constant buffer
        const RESOURCE_STATE_VERTEX_AND_CONSTANT_BUFFER  = 0x1;
        /// a subresource should be in this state when accessed as a index buffer
        const RESOURCE_STATE_INDEX_BUFFER                = 0x2;
        /// a subresource should be in this state when used as a render target
        const RESOURCE_STATE_RENDER_TARGET               = 0x4;
        /// a subresource should be in this state when accessed via an UAV.
        /// when in this state, a resource can be accessed for RW from multiple
        /// command queues simultaneously.
        const RESOURCE_STATE_UNORDERED_ACCESS            = 0x8;
        /// a subresource should be in this state when used for depth write. mutual exclusive
        const RESOURCE_STATE_DEPTH_WRITE                 = 0x10;
        /// a subresource should be in this state when used for depth read.
        const RESOURCE_STATE_DEPTH_READ                  = 0x20;
        /// a subresource should be in this state when accessed as a SRV from any stage other than PS
        const RESOURCE_STATE_NON_PIXEL_SHADER_RESOURCE   = 0x40;
        /// a subresource should be in this state when accessed as a SRV from PS
        const RESOURCE_STATE_PIXEL_SHADER_RESOURCE       = 0x80;
        /// a subresource is used with stream output
        const RESOURCE_STATE_STREAM_OUT                  = 0x100;
        /// the resource is used as indirect argument
        const RESOURCE_STATE_INDIRECT_ARGUMENT           = 0x200;
        /// used as the destination in a copy operation
        const RESOURCE_STATE_COPY_DEST                   = 0x400;
        /// used as the src in a copy operation
        const RESOURCE_STATE_COPY_SOURCE                 = 0x800;
        /// used as the destination in a resolve operation
        const RESOURCE_STATE_RESOLVE_DEST                = 0x1000;
        /// used as the src in a resolve operation
        const RESOURCE_STATE_RESOLVE_SOURCE              = 0x2000;
        /// required starting state for upload heaps.
        /// when in this state, a resource can be accessed for reading from
        /// multiple command queues simultaneously.
        const RESOURCE_STATE_GENERIC_READ = ((((0x1|0x2)|0x40)|0x80)|0x200)|0x800;
        /// alias for `COMMON`
        const RESOURCE_STATE_PRESENT                     = 0;
        /// used for [predication](https://msdn.microsoft.com/zh-cn/library/windows/desktop/dn903927(v=vs.85).aspx)
        const RESOURCE_STATE_PREDICATION                 = 0x200;
    }
}

impl Default for ResourceStates {
    #[inline]
    fn default() -> Self {
        RESOURCE_STATE_GENERIC_READ
    }
}

// TODO: find out a sound way to work with different types of resources

/// a committed resource, backed up by an implicit heap
#[derive(Clone, Debug)]
pub struct CommittedResource{
    raw: RawResource
}

impl CommittedResource {
    #[inline]
    pub unsafe fn from_raw(raw: RawResource) -> CommittedResource {
        CommittedResource{raw}
    }

    #[inline]
    pub fn as_raw(&mut self) -> &mut RawResource {
        &mut self.raw
    }
}

/// a placed resource, backed up by an explicit heap
#[derive(Clone, Debug)]
pub struct PlacedResource{
    raw: RawResource,
    heap: Heap,
    heap_offset: u64,
    alloc_info: ResourceAllocInfo,
}

impl PlacedResource {
    #[inline]
    pub unsafe fn from_raw(
        raw: RawResource, heap: Heap, 
        heap_offset: u64, alloc_info: ResourceAllocInfo) -> Self {
        PlacedResource{
            raw, heap, heap_offset, alloc_info
        }
    }

    #[inline]
    pub fn as_raw(&mut self) -> &mut RawResource {
        &mut self.raw
    }

    #[inline]
    pub fn get_placed_heap(&self) -> &Heap {
        &self.heap
    }

    #[inline]
    pub fn get_heap_offset(&self) -> u64 {
        self.heap_offset
    }

    #[inline]
    pub fn get_alloc_info(&self) -> ResourceAllocInfo {
        self.alloc_info
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct ResourceAllocInfo {
    /// consumed size of the resource on paged heap
    pub size: u64,
    pub alignment: u64,
}

/// describes a resource used for GPU texture copying
#[derive(Copy, Clone, Debug)]
pub struct TextureCopyLocation {
    ptr: *mut ::winapi::ID3D12Resource,
    pub copy_type: TextureCopyType,
}

impl From<TextureCopyLocation> for ::winapi::D3D12_TEXTURE_COPY_LOCATION {
    #[inline]
    fn from(loc: TextureCopyLocation) -> Self {
        unsafe {
            let mut ret: Self = ::std::mem::uninitialized();
            ret.pResource = loc.ptr;
            match loc.copy_type {
                TextureCopyType::SubresourceIndex(idx) => {
                    ret.Type = ::winapi::D3D12_TEXTURE_COPY_TYPE_SUBRESOURCE_INDEX;
                    ret.u = ::std::mem::transmute_copy(&idx);
                },
                TextureCopyType::PlacedFootprint(footprint) => {
                    ret.Type = ::winapi::D3D12_TEXTURE_COPY_TYPE_PLACED_FOOTPRINT;
                    ret.u = ::std::mem::transmute(footprint);
                },
            }

            ret
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum TextureCopyType {
    SubresourceIndex(u32),
    PlacedFootprint(PlacedSubresourceFootprint),
}

/// [more info](https://msdn.microsoft.com/zh-cn/library/windows/desktop/dn986749(v=vs.85).aspx)
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct PlacedSubresourceFootprint {
    /// offset within the parent resource
    pub offset: u64,
    pub format: DxgiFormat,
    pub width: u32,
    pub height: u32,
    pub depth: u32,
    pub row_pitch: u32,
}

// TODO: reserved resource?
