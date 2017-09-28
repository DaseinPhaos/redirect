// Copyright 2017 Dasein Phaos aka. Luxko
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! attributes used to describe a resource

use format::*;
use swapchain::SampleDesc;

/// resource description
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ResourceDesc {
    pub dimension: ResourceDimension,
    pub alignment: ResourceAlignment,
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
    pub fn buffer(size: u64, flags: ResourceFlags) -> ResourceDesc{
        ResourceDesc{
            dimension: RESOURCE_DIMENSION_BUFFER,
            alignment: Default::default(),
            width: size,
            height: 1,
            depth: 1,
            mip_levels: 1,
            format: DXGI_FORMAT_UNKNOWN,
            sample_desc: Default::default(),
            layout: TEXTURE_LAYOUT_ROW_MAJOR,
            flags: flags,
        }
    }

    /// a tex1d description
    #[inline]
    pub fn tex1d(
        length: u64, array_size: u16, mip_levels: u16, format: DxgiFormat,
        flags: ResourceFlags, alignment: ResourceAlignment
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
        width: u64, height: u32, array_size: u16, mip_levels: u16,
        format: DxgiFormat, flags: ResourceFlags, alignment: ResourceAlignment
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
        width: u64, height: u32, depth: u16, mip_levels: u16,
        format: DxgiFormat, flags: ResourceFlags, alignment: ResourceAlignment
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
    /// alignment of the resource
    #[repr(C)]
    pub struct ResourceAlignment: u64 {
        /// 4mb for msaa textures, 64kb for everything else. This is the deefault.
        const RESOURCE_DEFAULT_ALIGNED = 0;
        /// 4kb aligned
        const RESOURCE_4KB_ALIGNED = 0x1_000;
        /// 64kb aligned
        const RESOURCE_64KB_ALIGNED = 0x10_000;
        /// 4mb aligned
        const RESOURCE_4MB_ALIGNED = 0x1_000_000;
    }
}

impl Default for ResourceAlignment {
    #[inline]
    fn default() -> ResourceAlignment {
        RESOURCE_DEFAULT_ALIGNED
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
        /// Note that buffers should be row major
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
    /// misc flags for resources. [more info](https://msdn.microsoft.com/library/windows/desktop/dn986742(v=vs.85).aspx)
    #[repr(C)]
    pub struct ResourceFlags: u32 {
        /// Default flag
        const RESOURCE_FLAG_NONE                       = 0;
        /// Allow the resource to be used as render target.
        /// - Should be used with format supporting render target capabilities
        ///   At current feature level.
        /// - Can't be used in conjunction with RowMajorLayout when ... (see doc for more)
        /// - Can't be useed with 4kb resource alignment, or AllowDepthStencil, pr DenyRtDsTextures
        const RESOURCE_FLAG_ALLOW_RENDER_TARGET        = 0x1;
        /// Allow dsv on the resource, allow resource state transition to DepthWrite/DepthRead.
        /// - Texture format must support depth stencil capability at the current feature level.
        /// - Cannot be used with:
        ///   - Buffer,
        ///   - 4kb resource alignment,
        ///   - AllowRenderTarget,
        ///   - AllowUnorderedAccess,
        ///   - AllowSimultaneousAccess,
        ///   - 64KbStandardSwizzle,
        ///   - RowMajor,
        ///   - DenyRtDsTextures,
        ///   - AllowDisplay
        const RESOURCE_FLAG_ALLOW_DEPTH_STENCIL        = 0x2;
        /// Allow uav on the resource, allow resource state transition to `UnorderedAccess`.
        /// - Texture format must support unordered access capability at the current feature level.
        /// - Cannot be used with MSAA and ... (see doc for more)
        const RESOURCE_FLAG_ALLOW_UNORDERED_ACCESS     = 0x4;
        /// Disallows a srv to be created for the resource,
        /// as well as disables the resource to transition into the state of SharedResources.
        /// Must be used with AllowDepthStencil
        const RESOURCE_FLAG_DENY_SHADER_RESOURCE       = 0x8;
        /// Allows the resource to be used for cross-adapter data, as well as the same features enabled by AllowSimultaneousAccess.
        /// Must be used with heaps with `SharedCrossAdapter` and not `AllowDisplay`.
        const RESOURCE_FLAG_ALLOW_CROSS_ADAPTER        = 0x10;
        /// Allows a resource to be simultaneously accessed by multiple different queues, devices or processes
        /// - Cannot be used with Buffer, coz bbuffer always have this property..>.<
        /// - Cannot be used with MSAA
        const RESOURCE_FLAG_ALLOW_SIMULTANEOUS_ACCESS  = 0x20;
    }
}

impl Default for ResourceFlags {
    #[inline]
    fn default() -> Self {
        RESOURCE_FLAG_NONE
    }
}
