// Copyright 2017 Dasein Phaos aka. Luxko
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! describes graphics and compute pipelines

// TODO: add graphic pipeline state description structure
// blockers: shader, rootsignature

use format::DxgiFormat;

pub mod so;
pub mod blend;
pub mod rootsig;
pub mod rasterizer;
pub mod ds;
pub mod ia;

pub type SampleDesc = ::swapchain::SampleDesc;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct GraphicsPipelineStateDesc {
    pub root_sig: *const ::winapi::ID3D12RootSignature, // TODO: ?
    // TODO: shader byte codes
    // TODO: pub stream_output: so::StreamOutputDesc?
    pub blend_state: blend::BlendDesc,
    pub sample_mask: u32,
    pub rasterizer_state: rasterizer::RasterizerDesc,
    pub depth_stencil_state: ds::DepthStencilDesc,
    // TODO: pub input_layout: 
    pub strip_cut_value: ia::StripCutValue,
    pub primitive_topology_type: ia::PrimitiveTopologyType,
    pub num_render_targets: u32,
    pub rtv_formats: [DxgiFormat; 8],
    pub dsv_format: DxgiFormat,
    pub sample_desc: SampleDesc,
    pub node_mask: u32,
    // TODO: caches?
    pub flags: PipelineStateFlags,
}

#[derive(Clone, Debug)]
pub struct GraphicsPipelineStateBuilder {
    // TODO: shader byte codes
    pub stream_output: so::DescBuilder,
    pub blend_state: blend::BlendDesc,
    pub sample_mask: u32,
    pub rasterizer_state: rasterizer::RasterizerDesc,
    pub depth_stencil_state: ds::DepthStencilDesc,
    pub input_layout: ia::InputLayoutBuilder,
    pub strip_cut_value: ia::StripCutValue,
    pub primitive_topology_type: ia::PrimitiveTopologyType,
    pub num_render_targets: u32,
    pub rtv_formats: [DxgiFormat; 8],
    pub dsv_format: DxgiFormat,
    pub sample_desc: SampleDesc,
    pub node_mask: u32,
    // TODO: caches?
    pub flags: PipelineStateFlags,
}

// TODO: add methods for the builder

bitflags!{
    pub struct PipelineStateFlags {
        const PIPELINE_STATE_FLAG_NONE        = 0;
        const PIPELINE_STATE_FLAG_TOOL_DEBUG  = 0x1;
    }
}

impl Default for PipelineStateFlags{
    #[inline]
    fn default() -> Self {
        PIPELINE_STATE_FLAG_NONE
    }
}
