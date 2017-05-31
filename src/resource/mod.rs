// Copyright 2017 Dasein Phaos aka. Luxko
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! resource

pub mod usage;
pub use self::usage::*;

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
            dimension: RESOURCE_DIMENSION_TEXTURE1D,
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
            dimension: RESOURCE_DIMENSION_TEXTURE1D,
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

// TODO: find out a sound way to work with different types of resources
