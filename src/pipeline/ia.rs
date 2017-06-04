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
use smallvec::SmallVec;
use std::marker::PhantomData;
use std::ffi::CStr;

/// a index buffer view
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct IndexBufferView {
    pub location: ::resource::GpuVAddress,
    pub size: u32,
    pub format: ::format::DxgiFormat,
}

/// a vertex buffer view
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct VertexBufferView {
    pub location: ::resource::GpuVAddress,
    pub size: u32,
    pub stride: u32,
}

/// a input layout constructor
#[derive(Clone, Debug, Default)]
pub struct InputLayoutBuilder<'a> {
    pub elements: SmallVec<[InputElementDesc<'a>; 8]>,
}

impl<'a> InputLayoutBuilder<'a>{
    #[inline]
    pub fn new() -> Self {
        Default::default()
    }
}


/// a single input element
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct InputElementDesc<'a> {
    pub semantic_name: *const c_char,
    pub semantic_index: u32,
    pub format: DxgiFormat,
    pub input_slot: u32,
    pub aligned_byte_offset: u32,
    pub input_slot_class: InputClassification,
    pub instance_data_step_rate: u32,
    _pd: PhantomData<&'a CStr>, // TODO: check if legit
}

impl<'a> InputElementDesc<'a>{
    #[inline]
    pub unsafe fn new(
        semantic_name: &'a CStr, semantic_index: u32, format: DxgiFormat
    ) -> Self {
        InputElementDesc{
            semantic_name: semantic_name.as_ptr(), 
            semantic_index, format, input_slot: 0, 
            aligned_byte_offset: ::winapi::D3D12_APPEND_ALIGNED_ELEMENT,
            input_slot_class: INPUT_CLASSIFICATION_PER_VERTEX,
            instance_data_step_rate: 0,
            _pd: Default::default(),
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

impl Default for StripCutValue {
    #[inline]
    fn default() -> StripCutValue {
        STRIP_CUT_VALUE_DISABLED
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

bitflags!{
    #[repr(C)]
    pub struct PrimitiveTopology: u32 {
        const PRIMITIVE_TOPOLOGY_UNDEFINED                     = 0;
        const PRIMITIVE_TOPOLOGY_POINTLIST                     = 1;
        const PRIMITIVE_TOPOLOGY_LINELIST                      = 2;
        const PRIMITIVE_TOPOLOGY_LINESTRIP                     = 3;
        const PRIMITIVE_TOPOLOGY_TRIANGLELIST                  = 4;
        const PRIMITIVE_TOPOLOGY_TRIANGLESTRIP                 = 5;
        const PRIMITIVE_TOPOLOGY_LINELIST_ADJ                  = 10;
        const PRIMITIVE_TOPOLOGY_LINESTRIP_ADJ                 = 11;
        const PRIMITIVE_TOPOLOGY_TRIANGLELIST_ADJ              = 12;
        const PRIMITIVE_TOPOLOGY_TRIANGLESTRIP_ADJ             = 13;
        const PRIMITIVE_TOPOLOGY_1_CONTROL_POINT_PATCHLIST     = 33;
        const PRIMITIVE_TOPOLOGY_2_CONTROL_POINT_PATCHLIST     = 34;
        const PRIMITIVE_TOPOLOGY_3_CONTROL_POINT_PATCHLIST     = 35;
        const PRIMITIVE_TOPOLOGY_4_CONTROL_POINT_PATCHLIST     = 36;
        const PRIMITIVE_TOPOLOGY_5_CONTROL_POINT_PATCHLIST     = 37;
        const PRIMITIVE_TOPOLOGY_6_CONTROL_POINT_PATCHLIST     = 38;
        const PRIMITIVE_TOPOLOGY_7_CONTROL_POINT_PATCHLIST     = 39;
        const PRIMITIVE_TOPOLOGY_8_CONTROL_POINT_PATCHLIST     = 40;
        const PRIMITIVE_TOPOLOGY_9_CONTROL_POINT_PATCHLIST     = 41;
        const PRIMITIVE_TOPOLOGY_10_CONTROL_POINT_PATCHLIST    = 42;
        const PRIMITIVE_TOPOLOGY_11_CONTROL_POINT_PATCHLIST    = 43;
        const PRIMITIVE_TOPOLOGY_12_CONTROL_POINT_PATCHLIST    = 44;
        const PRIMITIVE_TOPOLOGY_13_CONTROL_POINT_PATCHLIST    = 45;
        const PRIMITIVE_TOPOLOGY_14_CONTROL_POINT_PATCHLIST    = 46;
        const PRIMITIVE_TOPOLOGY_15_CONTROL_POINT_PATCHLIST    = 47;
        const PRIMITIVE_TOPOLOGY_16_CONTROL_POINT_PATCHLIST    = 48;
        const PRIMITIVE_TOPOLOGY_17_CONTROL_POINT_PATCHLIST    = 49;
        const PRIMITIVE_TOPOLOGY_18_CONTROL_POINT_PATCHLIST    = 50;
        const PRIMITIVE_TOPOLOGY_19_CONTROL_POINT_PATCHLIST    = 51;
        const PRIMITIVE_TOPOLOGY_20_CONTROL_POINT_PATCHLIST    = 52;
        const PRIMITIVE_TOPOLOGY_21_CONTROL_POINT_PATCHLIST    = 53;
        const PRIMITIVE_TOPOLOGY_22_CONTROL_POINT_PATCHLIST    = 54;
        const PRIMITIVE_TOPOLOGY_23_CONTROL_POINT_PATCHLIST    = 55;
        const PRIMITIVE_TOPOLOGY_24_CONTROL_POINT_PATCHLIST    = 56;
        const PRIMITIVE_TOPOLOGY_25_CONTROL_POINT_PATCHLIST    = 57;
        const PRIMITIVE_TOPOLOGY_26_CONTROL_POINT_PATCHLIST    = 58;
        const PRIMITIVE_TOPOLOGY_27_CONTROL_POINT_PATCHLIST    = 59;
        const PRIMITIVE_TOPOLOGY_28_CONTROL_POINT_PATCHLIST    = 60;
        const PRIMITIVE_TOPOLOGY_29_CONTROL_POINT_PATCHLIST    = 61;
        const PRIMITIVE_TOPOLOGY_30_CONTROL_POINT_PATCHLIST    = 62;
        const PRIMITIVE_TOPOLOGY_31_CONTROL_POINT_PATCHLIST    = 63;
        const PRIMITIVE_TOPOLOGY_32_CONTROL_POINT_PATCHLIST    = 64;
    }
}
