// Copyright 2017 Dasein Phaos aka. Luxko
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[macro_use]
extern crate bitflags;
extern crate winapi;
extern crate d3d12;
extern crate dxguid;
extern crate dxgi;
extern crate d3dcompiler;
extern crate smallvec;

mod comptr;
pub mod error;
pub mod format;
pub mod swapchain;
pub mod resource;
pub mod device;
pub mod factory;
pub mod command;
pub mod pipeline;
pub mod shader;
pub mod fence;
pub mod descriptor;

#[repr(C)]
pub struct InputElementBuilder<'a> {
    pub semantic_name: &'a str,
    pub semantic_index: u32,
    pub format: format::DxgiFormat,
    pub input_slot: u32,
    pub aligned_byte_offset: u32,
    pub input_slot_class: winapi::d3d12::D3D12_INPUT_CLASSIFICATION,
    pub instance_data_step_rate: u32,
}

// impl<'a> InputElementBuilder<'a> {
//     pub fn new(semantic_name: &'a str, format: format::DxgiFormat) -> InputElementBuilder<'a> {
//         InputElementBuilder{
//             semantic_name,
//             semantic_index: 0,
//             format,
//             0, 0, 
//         }
//     }
// }
