// Copyright 2017 Dasein Phaos aka. Luxko
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! defines the rasterizer state

use format::Bool;

/// describes the rasterizer state. [more info](https://msdn.microsoft.com/library/windows/desktop/dn770387(v=vs.85).aspx)
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct RasterizerDesc {
    pub fill: FillMode,
    pub cull: CullMode,
    pub front_ccw: Bool,
    pub depth_bias: i32,
    pub depth_bias_clamp: f32,
    pub slope_scaled_depth_bias: f32,
    pub depth_clip: Bool,
    pub multisample: Bool,
    pub antialiased_line: Bool,
    pub forced_sample_count: u32,
    pub conservative_raster: ConservativeMode,
}

impl Default for RasterizerDesc {
    #[inline]
    fn default() -> RasterizerDesc{
        RasterizerDesc{
            fill: FillMode::SOLID,
            cull: CullMode::BACK,
            front_ccw: true.into(),
            depth_bias: 0,
            depth_bias_clamp: 0.0f32,
            slope_scaled_depth_bias: 0.0f32,
            depth_clip: false.into(),
            multisample: false.into(),
            antialiased_line: false.into(),
            forced_sample_count: 0,
            conservative_raster: ConservativeMode::OFF,
        }
    }
}

bitflags!{
    #[repr(C)]
    pub struct FillMode: u32 {
        const WIREFRAME = 2;
        const SOLID = 3;
    }
}

bitflags!{
    #[repr(C)]
    pub struct CullMode: u32 {
        const NONE = 1;
        const FRONT = 2;
        const BACK = 3;
    }
}

bitflags!{
    #[repr(C)]
    pub struct ConservativeMode: u32 {
        const OFF = 0;
        const ON = 1;
    }
}
