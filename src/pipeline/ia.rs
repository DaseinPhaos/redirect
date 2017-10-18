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

/// a vertex
pub trait Vertex: Sized {
    /// get the stride per vertex
    #[inline]
    fn get_stride(&self) -> u32 {
        ::std::mem::size_of::<Self>() as u32
    }

    /// get the input layout of this vertex
    fn get_input_layout(&self) -> InputLayoutBuilder;
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
    pub fn new(
        semantic_name: &'a CStr, format: DxgiFormat
    ) -> Self {
        InputElementDesc{
            semantic_name: semantic_name.as_ptr(), 
            semantic_index: 0, format, input_slot: 0, 
            aligned_byte_offset: ::winapi::D3D12_APPEND_ALIGNED_ELEMENT,
            input_slot_class: InputClassification::PER_VERTEX,
            instance_data_step_rate: 0,
            _pd: Default::default(),
        }
    }
}

bitflags!{
    /// identifies the type of input data
    #[repr(C)]
    pub struct InputClassification: u32 {
        const PER_VERTEX = 0;
        const PER_INSTANCE = 1;
    }
}

bitflags!{
    #[repr(C)]
    pub struct StripCutValue: u32 {
        const DISABLED = 0;
        const FFFF = 1;
        const FFFFFFFF = 2;
    }
}

impl Default for StripCutValue {
    #[inline]
    fn default() -> StripCutValue {
        StripCutValue::DISABLED
    }
}

bitflags!{
    #[repr(C)]
    pub struct PrimitiveTopologyType: u32 {
        const UNDEFINED  = 0;
        const POINT      = 1;
        const LINE       = 2;
        const TRIANGLE   = 3;
        const PATCH      = 4;
    }
}

bitflags!{
    #[repr(C)]
    pub struct PrimitiveTopology: u32 {
        const UNDEFINED                     = 0;
        const POINTLIST                     = 1;
        const LINELIST                      = 2;
        const LINESTRIP                     = 3;
        const TRIANGLELIST                  = 4;
        const TRIANGLESTRIP                 = 5;
        const LINELIST_ADJ                  = 10;
        const LINESTRIP_ADJ                 = 11;
        const TRIANGLELIST_ADJ              = 12;
        const TRIANGLESTRIP_ADJ             = 13;
        const CONTROL_POINT_PATCHLIST_1     = 33;
        const CONTROL_POINT_PATCHLIST_2     = 34;
        const CONTROL_POINT_PATCHLIST_3     = 35;
        const CONTROL_POINT_PATCHLIST_4     = 36;
        const CONTROL_POINT_PATCHLIST_5     = 37;
        const CONTROL_POINT_PATCHLIST_6     = 38;
        const CONTROL_POINT_PATCHLIST_7     = 39;
        const CONTROL_POINT_PATCHLIST_8     = 40;
        const CONTROL_POINT_PATCHLIST_9     = 41;
        const CONTROL_POINT_PATCHLIST_10    = 42;
        const CONTROL_POINT_PATCHLIST_11    = 43;
        const CONTROL_POINT_PATCHLIST_12    = 44;
        const CONTROL_POINT_PATCHLIST_13    = 45;
        const CONTROL_POINT_PATCHLIST_14    = 46;
        const CONTROL_POINT_PATCHLIST_15    = 47;
        const CONTROL_POINT_PATCHLIST_16    = 48;
        const CONTROL_POINT_PATCHLIST_17    = 49;
        const CONTROL_POINT_PATCHLIST_18    = 50;
        const CONTROL_POINT_PATCHLIST_19    = 51;
        const CONTROL_POINT_PATCHLIST_20    = 52;
        const CONTROL_POINT_PATCHLIST_21    = 53;
        const CONTROL_POINT_PATCHLIST_22    = 54;
        const CONTROL_POINT_PATCHLIST_23    = 55;
        const CONTROL_POINT_PATCHLIST_24    = 56;
        const CONTROL_POINT_PATCHLIST_25    = 57;
        const CONTROL_POINT_PATCHLIST_26    = 58;
        const CONTROL_POINT_PATCHLIST_27    = 59;
        const CONTROL_POINT_PATCHLIST_28    = 60;
        const CONTROL_POINT_PATCHLIST_29    = 61;
        const CONTROL_POINT_PATCHLIST_30    = 62;
        const CONTROL_POINT_PATCHLIST_31    = 63;
        const CONTROL_POINT_PATCHLIST_32    = 64;
    }
}

impl Default for PrimitiveTopology {
    fn default() -> Self {
        PrimitiveTopology::TRIANGLELIST
    }
}
