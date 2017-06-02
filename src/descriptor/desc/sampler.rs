// Copyright 2017 Dasein Phaos aka. Luxko
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! sampler description
use pipeline::sampler::*;
pub type ComparisonFunc = ::pipeline::ComparisonFunc;

/// describes a sampler
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct SamplerDesc {
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
}
