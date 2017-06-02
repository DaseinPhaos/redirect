// Copyright 2017 Dasein Phaos aka. Luxko
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! descriptor description boilerplates

mod srv;
pub use self::srv::*;

mod cbv;
pub use self::cbv::*;

mod rtv;
pub use self::rtv::*;

mod dsv;
pub use self::dsv::*;

mod sampler;
pub use self::sampler::*;

mod uav;
pub use self::uav::*;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Shader4ComponentMapping{inner: u32}

impl Shader4ComponentMapping {
    #[inline]
    pub fn new(
        r: ShaderComponentMapping, 
        g: ShaderComponentMapping, 
        b: ShaderComponentMapping, 
        a: ShaderComponentMapping)
     -> Self {
        Shader4ComponentMapping{
            // TODO: double check
            inner: r.bits() + 
             g.bits()<< ::winapi::D3D12_SHADER_COMPONENT_MAPPING_SHIFT + 
             b.bits()<< ::winapi::D3D12_SHADER_COMPONENT_MAPPING_SHIFT*2 + 
             a.bits()<< ::winapi::D3D12_SHADER_COMPONENT_MAPPING_SHIFT*3
        }
    }
}

impl Default for Shader4ComponentMapping {
    #[inline]
    fn default() -> Self {
        Shader4ComponentMapping::new(
            SHADER_COMPONENT_MAPPING_FROM_MEMORY_COMPONENT_0,
            SHADER_COMPONENT_MAPPING_FROM_MEMORY_COMPONENT_1,
            SHADER_COMPONENT_MAPPING_FROM_MEMORY_COMPONENT_2,
            SHADER_COMPONENT_MAPPING_FROM_MEMORY_COMPONENT_3
        )
    }
}

bitflags!{
    /// specifies how memory gets routed by a srv
    #[repr(C)]
    pub struct ShaderComponentMapping: u32 {
        /// indicates return component 0, i.e. R in RGBA
        const SHADER_COMPONENT_MAPPING_FROM_MEMORY_COMPONENT_0 = 0;
        /// indicates return component 1, i.e. G in RGBA
        const SHADER_COMPONENT_MAPPING_FROM_MEMORY_COMPONENT_1 = 1;
        /// indicates return component 2, i.e. B in RGBA
        const SHADER_COMPONENT_MAPPING_FROM_MEMORY_COMPONENT_2 = 2;
        /// indicates return component 3, i.e. A in RGBA
        const SHADER_COMPONENT_MAPPING_FROM_MEMORY_COMPONENT_3 = 3;
        /// indicates forcing the resulting value to 0
        const SHADER_COMPONENT_MAPPING_FORCE_VALUE_0 = 4;
        /// indicates forcing the resulting value to 0x1 or 1.0f
        const SHADER_COMPONENT_MAPPING_FORCE_VALUE_1 = 5;
    }
}
