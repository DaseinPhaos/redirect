// Copyright 2017 Dasein Phaos aka. Luxko
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! stream output descriptions

use smallvec::SmallVec;
use std::os::raw::c_char;
use std::marker::PhantomData;
use resource::{RawResource, GpuVAddress};

/// stream output buffer view
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct StreamOutputBufferView {
    pub loc: GpuVAddress,
    pub size: u64,
    pub filled_size: u64, // TODO: change type to GPUVAddress?
}

// TODO: find out a nicer way to deal with resources
impl StreamOutputBufferView {
    pub fn new(resource: &mut RawResource, size: u64, filled_size: u64) -> Self {
        StreamOutputBufferView{
            loc: resource.get_gpu_vaddress(), size, filled_size,
        }
    }
}

impl Default for StreamOutputBufferView {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}

/// stream output description
#[derive(Clone, Debug)]
pub struct DescBuilder {
    pub entries: SmallVec<[DeclarationEntry; 8]>,
    /// buffer strides
    pub strides: SmallVec<[u32; 8]>,
    /// index of the stream to be sent to the rasterizer stage
    pub rasterized_stream: u32,
}

impl DescBuilder {
    /// construction
    #[inline]
    pub fn new(rasterized_stream: u32) -> DescBuilder {
        DescBuilder{
            entries: Default::default(),
            strides: Default::default(),
            rasterized_stream,
        }
    }

    /// finalization
    #[inline]
    pub fn build(&self) -> (::winapi::D3D12_STREAM_OUTPUT_DESC, PhantomData<&DescBuilder>) {
        (::winapi::D3D12_STREAM_OUTPUT_DESC{
            pSODeclaration: self.entries.as_ptr() as *const _,
            NumEntries: self.entries.len() as u32,
            pBufferStrides: self.strides.as_ptr(),
            NumStrides: self.strides.len() as u32,
            RasterizedStream: self.rasterized_stream,
        },
        Default::default())
    }
}

/// describes an entry in a stream output slot
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DeclarationEntry {
    /// zero based stream index
    pub stream: u32,
    /// `0` ended semantic name of the element
    semantic_name: *const c_char, // TODO: deal with lifetimes
    /// zero based element index
    pub semantic_index: u32,
    /// component of the entry to begin writing to. valid in [0..3]
    pub start_component: u8,
    /// number of components of the entry to writing to. valid in [1..4]
    pub component_count: u8,
    /// associated stream output buffer that is bound to the pipeline. valid in [0..3]
    pub output_slot: u8,
}
