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
use winapi::{ID3D12PipelineState, ID3DBlob};
use error::WinError;
use comptr::ComPtr;

pub mod so;
pub mod blend;
pub mod rootsig;
pub mod rasterizer;
pub mod ds;
pub mod ia;
pub mod sampler;

pub type SampleDesc = ::swapchain::SampleDesc;

/// a pipeline state object
#[derive(Clone, Debug)]
pub struct PipelineState {
    pub ptr: ComPtr<ID3D12PipelineState>,
}

/// a pipeline state cached blob
#[derive(Clone, Debug)]
pub struct PipelineStateCache {
    pub ptr: ComPtr<ID3DBlob>,
}

impl PipelineState {
    /// get the cached blob
    #[inline]
    pub fn cached(&mut self) -> Result<PipelineStateCache, WinError> {
        unsafe {
            let mut ret = ::std::mem::uninitialized();
            let hr = self.ptr.GetCachedBlob(&mut ret);
            WinError::from_hresult_or_ok(hr, || PipelineStateCache{
                ptr: ComPtr::new(ret)
            })
        }
    }
}

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

// #[derive(Clone, Debug)]
// pub struct GraphicsPipelineStateBuilder {
//     // TODO: shader byte codes
//     pub stream_output: so::DescBuilder,
//     pub blend_state: blend::BlendDesc,
//     pub sample_mask: u32,
//     pub rasterizer_state: rasterizer::RasterizerDesc,
//     pub depth_stencil_state: ds::DepthStencilDesc,
//     pub input_layout: ia::InputLayoutBuilder,
//     pub strip_cut_value: ia::StripCutValue,
//     pub primitive_topology_type: ia::PrimitiveTopologyType,
//     pub num_render_targets: u32,
//     pub rtv_formats: [DxgiFormat; 8],
//     pub dsv_format: DxgiFormat,
//     pub sample_desc: SampleDesc,
//     pub node_mask: u32,
//     pub cache: Option<PipelineStateCache>,
//     pub flags: PipelineStateFlags,
// }

// TODO: add methods for the builder

bitflags!{
    #[repr(C)]
    pub struct PipelineStateFlags: u32 {
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

bitflags!{
    /// comparison options, specifying under which circumstance the comparison passes
    #[repr(C)]
    pub struct ComparisonFunc: u32 {
        const COMPARISON_FUNC_NEVER          = 1;
        const COMPARISON_FUNC_LESS           = 2;
        const COMPARISON_FUNC_EQUAL          = 3;
        const COMPARISON_FUNC_LESS_EQUAL     = 4;
        const COMPARISON_FUNC_GREATER        = 5;
        const COMPARISON_FUNC_NOT_EQUAL      = 6;
        const COMPARISON_FUNC_GREATER_EQUAL  = 7;
        const COMPARISON_FUNC_ALWAYS         = 8;
    }
}
