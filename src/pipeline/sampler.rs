// Copyright 2017 Dasein Phaos aka. Luxko
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! texture sampling schemes

use super::*;

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
            comparison_func: ComparisonFunc::ALWAYS,
            border_color: Default::default(),
            min_lod: 0.0f32,
            max_lod: 1.0f32,
            shader_register,
            register_space,
        }
    }
}

bitflags!{
    /// filtering options for texture sampling. [more info](https://msdn.microsoft.com/library/windows/desktop/dn770367(v=vs.85).aspx)
    #[repr(C)]
    pub struct Filter: u32 {
        const MIN_MAG_MIP_POINT                           = 0;
        const MIN_MAG_POINT_MIP_LINEAR                    = 0x1;
        const MIN_POINT_MAG_LINEAR_MIP_POINT              = 0x4;
        const MIN_POINT_MAG_MIP_LINEAR                    = 0x5;
        const MIN_LINEAR_MAG_MIP_POINT                    = 0x10;
        const MIN_LINEAR_MAG_POINT_MIP_LINEAR             = 0x11;
        const MIN_MAG_LINEAR_MIP_POINT                    = 0x14;
        const MIN_MAG_MIP_LINEAR                          = 0x15;
        const ANISOTROPIC                                 = 0x55;
        const COMPARISON_MIN_MAG_MIP_POINT                = 0x80;
        const COMPARISON_MIN_MAG_POINT_MIP_LINEAR         = 0x81;
        const COMPARISON_MIN_POINT_MAG_LINEAR_MIP_POINT   = 0x84;
        const COMPARISON_MIN_POINT_MAG_MIP_LINEAR         = 0x85;
        const COMPARISON_MIN_LINEAR_MAG_MIP_POINT         = 0x90;
        const COMPARISON_MIN_LINEAR_MAG_POINT_MIP_LINEAR  = 0x91;
        const COMPARISON_MIN_MAG_LINEAR_MIP_POINT         = 0x94;
        const COMPARISON_MIN_MAG_MIP_LINEAR               = 0x95;
        const COMPARISON_ANISOTROPIC                      = 0xd5;
        const MINIMUM_MIN_MAG_MIP_POINT                   = 0x100;
        const MINIMUM_MIN_MAG_POINT_MIP_LINEAR            = 0x101;
        const MINIMUM_MIN_POINT_MAG_LINEAR_MIP_POINT      = 0x104;
        const MINIMUM_MIN_POINT_MAG_MIP_LINEAR            = 0x105;
        const MINIMUM_MIN_LINEAR_MAG_MIP_POINT            = 0x110;
        const MINIMUM_MIN_LINEAR_MAG_POINT_MIP_LINEAR     = 0x111;
        const MINIMUM_MIN_MAG_LINEAR_MIP_POINT            = 0x114;
        const MINIMUM_MIN_MAG_MIP_LINEAR                  = 0x115;
        const MINIMUM_ANISOTROPIC                         = 0x155;
        const MAXIMUM_MIN_MAG_MIP_POINT                   = 0x180;
        const MAXIMUM_MIN_MAG_POINT_MIP_LINEAR            = 0x181;
        const MAXIMUM_MIN_POINT_MAG_LINEAR_MIP_POINT      = 0x184;
        const MAXIMUM_MIN_POINT_MAG_MIP_LINEAR            = 0x185;
        const MAXIMUM_MIN_LINEAR_MAG_MIP_POINT            = 0x190;
        const MAXIMUM_MIN_LINEAR_MAG_POINT_MIP_LINEAR     = 0x191;
        const MAXIMUM_MIN_MAG_LINEAR_MIP_POINT            = 0x194;
        const MAXIMUM_MIN_MAG_MIP_LINEAR                  = 0x195;
        const MAXIMUM_ANISOTROPIC                         = 0x1d5;
    }
}

bitflags!{
    /// texture addressing modes when coordinates outside normalized boundary
    #[repr(C)]
    pub struct TextureAddressMode: u32 {
        /// tile at every integer junction, essentially repeating the textures
        const WRAP = 1;
        /// filp at every integer junction
        const MIRROR = 2;
        /// clamp to values at normalized boundary
        const CLAMP = 3;
        /// set to a "border color"
        const BORDER = 4;
        /// take the absolution value of texture coordinates, then clamp to the boundary
        const MIRROR_ONCE = 5;
    }
}

impl Default for TextureAddressMode {
    #[inline]
    fn default() -> Self {
        TextureAddressMode::WRAP
    }
}

bitflags!{
    /// border colors
    #[repr(C)]
    pub struct BorderColor: u32 {
        const TRANSPARENT_BLACK = 0;
        const OPAQUE_BLACK = BorderColor::TRANSPARENT_BLACK.bits + 1;
        const OPAQUE_WHITE = BorderColor::OPAQUE_BLACK.bits + 1;
    }
}

impl Default for BorderColor {
    #[inline]
    fn default() -> BorderColor {
        BorderColor::TRANSPARENT_BLACK
    }
}
