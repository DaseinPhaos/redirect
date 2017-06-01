// Copyright 2017 Dasein Phaos aka. Luxko
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! texture sampling schemes

/// describes a static sampler
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct StaticSamplerDesc {
    /// filtering method
    pub filter: Filter,
    /// address mode for `u` outside [0, 1]
    pub address_u: TextureAddressMode,
    /// address mode for `v` outside [0, 1]
    pub address_v: TextureAddressMode,
    /// address mode for `w` outside [0, 1]
    pub address_w: TextureAddressMode,
    /// mipmap level bias
    pub mip_bias: f32,
    /// claming value for anisotropic filters, valid between [1..16]
    pub max_anisotropy: u32,
    /// function used to compare sampled data against existing sampled data
    pub comparison_func: ComparisonFunc,
    /// border color to use if address mod is `BORDER`
    pub border_color: BorderColor,
    /// lower end of the mipmap level to clamp access to
    pub min_lod: f32,
    /// higher end of the mipmap level to clamp access to
    pub max_lod: f32,
    /// shader register
    pub shader_register: u32,
    /// register space
    pub register_space: u32,
}

impl StaticSamplerDesc {
    /// construct a new description with the given filter and default options
    #[inline]
    pub fn new(filter: Filter, shader_register: u32, register_space: u32) -> StaticSamplerDesc {
        StaticSamplerDesc{
            filter, address_u: Default::default(),
            address_v: Default::default(),
            address_w: Default::default(),
            mip_bias: 0.0f32,
            max_anisotropy: 1,
            comparison_func: COMPARISON_FUNC_ALWAYS,
            border_color: Default::default(),
            min_lod: 0.0f32,
            max_lod: 1.0f32,
            shader_register,
            register_space,
        }
    }
}

bitflags!{
    /// filtering options for texture sampling. [more info](https://msdn.microsoft.com/zh-cn/library/windows/desktop/dn770367(v=vs.85).aspx)
    #[repr(C)]
    pub struct Filter: u32 {
        const FILTER_MIN_MAG_MIP_POINT                           = 0;
        const FILTER_MIN_MAG_POINT_MIP_LINEAR                    = 0x1;
        const FILTER_MIN_POINT_MAG_LINEAR_MIP_POINT              = 0x4;
        const FILTER_MIN_POINT_MAG_MIP_LINEAR                    = 0x5;
        const FILTER_MIN_LINEAR_MAG_MIP_POINT                    = 0x10;
        const FILTER_MIN_LINEAR_MAG_POINT_MIP_LINEAR             = 0x11;
        const FILTER_MIN_MAG_LINEAR_MIP_POINT                    = 0x14;
        const FILTER_MIN_MAG_MIP_LINEAR                          = 0x15;
        const FILTER_ANISOTROPIC                                 = 0x55;
        const FILTER_COMPARISON_MIN_MAG_MIP_POINT                = 0x80;
        const FILTER_COMPARISON_MIN_MAG_POINT_MIP_LINEAR         = 0x81;
        const FILTER_COMPARISON_MIN_POINT_MAG_LINEAR_MIP_POINT   = 0x84;
        const FILTER_COMPARISON_MIN_POINT_MAG_MIP_LINEAR         = 0x85;
        const FILTER_COMPARISON_MIN_LINEAR_MAG_MIP_POINT         = 0x90;
        const FILTER_COMPARISON_MIN_LINEAR_MAG_POINT_MIP_LINEAR  = 0x91;
        const FILTER_COMPARISON_MIN_MAG_LINEAR_MIP_POINT         = 0x94;
        const FILTER_COMPARISON_MIN_MAG_MIP_LINEAR               = 0x95;
        const FILTER_COMPARISON_ANISOTROPIC                      = 0xd5;
        const FILTER_MINIMUM_MIN_MAG_MIP_POINT                   = 0x100;
        const FILTER_MINIMUM_MIN_MAG_POINT_MIP_LINEAR            = 0x101;
        const FILTER_MINIMUM_MIN_POINT_MAG_LINEAR_MIP_POINT      = 0x104;
        const FILTER_MINIMUM_MIN_POINT_MAG_MIP_LINEAR            = 0x105;
        const FILTER_MINIMUM_MIN_LINEAR_MAG_MIP_POINT            = 0x110;
        const FILTER_MINIMUM_MIN_LINEAR_MAG_POINT_MIP_LINEAR     = 0x111;
        const FILTER_MINIMUM_MIN_MAG_LINEAR_MIP_POINT            = 0x114;
        const FILTER_MINIMUM_MIN_MAG_MIP_LINEAR                  = 0x115;
        const FILTER_MINIMUM_ANISOTROPIC                         = 0x155;
        const FILTER_MAXIMUM_MIN_MAG_MIP_POINT                   = 0x180;
        const FILTER_MAXIMUM_MIN_MAG_POINT_MIP_LINEAR            = 0x181;
        const FILTER_MAXIMUM_MIN_POINT_MAG_LINEAR_MIP_POINT      = 0x184;
        const FILTER_MAXIMUM_MIN_POINT_MAG_MIP_LINEAR            = 0x185;
        const FILTER_MAXIMUM_MIN_LINEAR_MAG_MIP_POINT            = 0x190;
        const FILTER_MAXIMUM_MIN_LINEAR_MAG_POINT_MIP_LINEAR     = 0x191;
        const FILTER_MAXIMUM_MIN_MAG_LINEAR_MIP_POINT            = 0x194;
        const FILTER_MAXIMUM_MIN_MAG_MIP_LINEAR                  = 0x195;
        const FILTER_MAXIMUM_ANISOTROPIC                         = 0x1d5;
    }
}

bitflags!{
    /// texture addressing modes when coordinates outside normalized boundary
    #[repr(C)]
    pub struct TextureAddressMode: u32 {
        /// tile at every integer junction, essentially repeating the textures
        const TEXTURE_ADDRESS_MODE_WRAP = 1;
        /// filp at every integer junction
        const TEXTURE_ADDRESS_MODE_MIRROR = 2;
        /// clamp to values at normalized boundary
        const TEXTURE_ADDRESS_MODE_CLAMP = 3;
        /// set to a "border color"
        const TEXTURE_ADDRESS_MODE_BORDER = 4;
        /// take the absolution value of texture coordinates, then clamp to the boundary
        const TEXTURE_ADDRESS_MODE_MIRROR_ONCE = 5;
    }
}

impl Default for TextureAddressMode {
    #[inline]
    fn default() -> Self {
        TEXTURE_ADDRESS_MODE_WRAP
    }
}

bitflags!{
    /// comparison options, specifying under which circumstance the comparison passes
    #[repr(C)]
    pub struct ComparisonFunc: u32 {
        const COMPARISON_FUNC_NEVER          = 1;
        const COMPARISON_FUNC_LESS           = 2;
        const COMPARISON_FUNC_EQUAL          = 3;
        const COMPARISON_FUNC_LESS_EQUAL     = 4;
        const COMPARISON_FUNC_GREATER        = 5;
        const COMPARISON_FUNC_NOT_EQUAL      = 6;
        const COMPARISON_FUNC_GREATER_EQUAL  = 7;
        const COMPARISON_FUNC_ALWAYS         = 8;
    }
}

bitflags!{
    /// border colors
    pub struct BorderColor: u32 {
        const BORDER_COLOR_TRANSPARENT_BLACK = 0;
        const BORDER_COLOR_OPAQUE_BLACK = BORDER_COLOR_TRANSPARENT_BLACK.bits + 1;
        const BORDER_COLOR_OPAQUE_WHITE = BORDER_COLOR_OPAQUE_BLACK.bits + 1;
    }
}

impl Default for BorderColor {
    #[inline]
    fn default() -> BorderColor {
        BORDER_COLOR_TRANSPARENT_BLACK
    }
}
