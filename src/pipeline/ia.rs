// Copyright 2017 Dasein Phaos aka. Luxko
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! input element layout

use std::os::raw::c_char;
use format::DxgiFormat;

/// a single input element
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct InputElementDesc {
    pub semantic_name: *const c_char,
    pub semantic_index: u32,
    pub format: DxgiFormat,
    pub input_slot: u32,
    pub aligned_byte_offset: u32,
    pub input_slot_class: InputClassification,
    pub instance_data_step_rate: u32,
}

impl InputElementDesc{
    #[inline]
    pub unsafe fn new(
        semantic_name: *const c_char, semantic_index: u32, format: DxgiFormat
    ) -> Self {
        InputElementDesc{
            semantic_name, semantic_index, format,
            input_slot: 0, aligned_byte_offset: ::winapi::D3D12_APPEND_ALIGNED_ELEMENT,
            input_slot_class: INPUT_CLASSIFICATION_PER_VERTEX,
            instance_data_step_rate: 0
        }
    }
}

bitflags!{
    /// identifies the type of input data
    #[repr(C)]
    pub struct InputClassification: u32 {
        const INPUT_CLASSIFICATION_PER_VERTEX = 0;
        const INPUT_CLASSIFICATION_PER_INSTANCE = 1;
    }
}

bitflags!{
    #[repr(C)]
    pub struct StripCutValue: u32 {
        const STRIP_CUT_VALUE_DISABLED = 0;
        const STRIP_CUT_VALUE_0XFFFF = 1;
        const STRIP_CUT_VALUE_0XFFFFFFFF = 2;
    }
}

bitflags!{
    #[repr(C)]
    pub struct PrimitiveTopologyType: u32 {
        const PRIMITIVE_TOPOLOGY_TYPE_UNDEFINED  = 0;
        const PRIMITIVE_TOPOLOGY_TYPE_POINT      = 1;
        const PRIMITIVE_TOPOLOGY_TYPE_LINE       = 2;
        const PRIMITIVE_TOPOLOGY_TYPE_TRIANGLE   = 3;
        const PRIMITIVE_TOPOLOGY_TYPE_PATCH      = 4;
    }
}
