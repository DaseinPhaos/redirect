// Copyright 2017 Dasein Phaos aka. Luxko
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! common command list methods implementation boilerplate

use super::*;

pub trait CommandList {
    /// get type of this command list
    fn get_type(&mut self) -> CommandListType;

    /// set the graphics root signature.
    fn set_graphics_rootsig(&mut self, rootsig: &::pipeline::rootsig::RootSig);

    /// set a constant in the graphics root signature
    fn set_graphics_root_constant(
        &mut self, param_index: u32, value: u32, param_offset: u32
    );

    /// set a group of constants in the graphics root signature
    fn set_graphics_root_constants(
        &mut self, param_index: u32, values: &[u32], param_offset: u32
    ); 

    /// set a cbv in the graphics root signature
    fn set_graphics_root_cbv(
        &mut self, param_index: u32, resource: &mut RawResource
    );

    /// set a srv in the graphics root signature
    fn set_graphics_root_srv(
        &mut self, param_index: u32, resource: &mut RawResource
    );

    /// set a uav in the graphics root signature
    fn set_graphics_root_uav(
        &mut self, param_index: u32, resource: &mut RawResource
    );

    /// set a decriptor table in the graphics root signature TODO: double check
    fn set_graphics_root_dt<H: Into<::winapi::D3D12_GPU_DESCRIPTOR_HANDLE>>
    (
        &mut self, param_index: u32, base_descriptor: H
    );

    /// set the compute root signature. TODO: distince root signatures
    fn set_compute_rootsig(&mut self, rootsig: &::pipeline::rootsig::RootSig);


    /// set a constant in the compute root signature
    fn set_compute_root_constant(
        &mut self, param_index: u32, value: u32, param_offset: u32
    );

    /// set a group of constants in the compute root signature
    fn set_compute_root_constants(
        &mut self, param_index: u32, values: &[u32], param_offset: u32
    );

    /// set a cbv in the compute root signature
    fn set_compute_root_cbv(
        &mut self, param_index: u32, resource: &mut RawResource
    );

    /// set a srv in the compute root signature
    fn set_compute_root_srv(
        &mut self, param_index: u32, resource: &mut RawResource
    );

    /// set a uav in the compute root signature
    fn set_compute_root_uav(
        &mut self, param_index: u32, resource: &mut RawResource
    );

    /// set a decriptor table in the compute root signature TODO: double check
    fn set_compute_root_dt<H: Into<::winapi::D3D12_GPU_DESCRIPTOR_HANDLE>>
    (
        &mut self, param_index: u32, base_descriptor: H
    );

    /// execute a command list from a thread group. [more info](https://msdn.microsoft.com/zh-cn/library/windows/desktop/dn903871(v=vs.85).aspx)
    fn dispatch(&mut self, x: u32, y: u32, z: u32);

    /// set primitive topology
    fn ia_set_primitive_topology(&mut self, topology: ::pipeline::ia::PrimitiveTopology);

    /// set the index buffer
    fn ia_set_ibv(&mut self, ibv: &::pipeline::ia::IndexBufferView);

    /// set the vertex buffer
    fn ia_set_vbvs(
        &mut self, start_slot: u32, vbvs: &[::pipeline::ia::VertexBufferView]
    );

    /// set the blend factor
    fn om_set_blend_factor(&mut self, factors: [f32; 4]);

    /// set the stencil reference
    fn om_set_stencil_ref(&mut self, stencil_ref: u32);

    /// set rtv and dsvs
    fn om_set_rtv_dsv_continuous(
        &mut self, rtv_heap: &mut RtvHeap, rtv_offset: u32, 
        num_rtvs: u32, dsv: CpuDsvHandle
    );

    /// set rtv and dsvs
    fn om_set_rtv_dsv_discontinuous(
        &mut self, rtvs: &[CpuRtvHandle], dsv: CpuDsvHandle
    );

    /// set the pipeline state
    fn set_pipeline_state(
        &mut self, state: Option<&PipelineState>
    );
}

macro_rules! impl_common_commands {
    ($Type: ident) => {
    impl<'a> CommandList for $Type<'a> {
    #[inline]
    fn get_type(&mut self) -> CommandListType {
        unsafe { ::std::mem::transmute(self.ptr.GetType()) }
    }

    /// set the graphics root signature. TODO: distince root signatures
    #[inline]
    fn set_graphics_rootsig(&mut self, rootsig: &::pipeline::rootsig::RootSig) {
        unsafe { self.ptr.SetGraphicsRootSignature(rootsig.ptr.as_mut_ptr())}
    }

    /// set a constant in the graphics root signature
    #[inline]
    fn set_graphics_root_constant(
        &mut self, param_index: u32, value: u32, param_offset: u32
    ) {
        unsafe { self.ptr.SetGraphicsRoot32BitConstant(param_index, value, param_offset)}
    }

    /// set a group of constants in the graphics root signature
    #[inline]
    fn set_graphics_root_constants(
        &mut self, param_index: u32, values: &[u32], param_offset: u32
    ) {
        unsafe { self.ptr.SetGraphicsRoot32BitConstants(
            param_index, values.len() as u32, values.as_ptr() as *const _, param_offset
        )}
    }

    /// set a cbv in the graphics root signature
    #[inline]
    fn set_graphics_root_cbv(
        &mut self, param_index: u32, resource: &mut RawResource
    ) {
        unsafe { self.ptr.SetGraphicsRootConstantBufferView(
            param_index, resource.get_gpu_vaddress().into()
        )}
    }

    /// set a srv in the graphics root signature
    #[inline]
    fn set_graphics_root_srv(
        &mut self, param_index: u32, resource: &mut RawResource
    ) {
        unsafe { self.ptr.SetGraphicsRootShaderResourceView(
            param_index, resource.get_gpu_vaddress().into()
        )}
    }

    /// set a uav in the graphics root signature
    #[inline]
    fn set_graphics_root_uav(
        &mut self, param_index: u32, resource: &mut RawResource
    ) {
        unsafe { self.ptr.SetGraphicsRootUnorderedAccessView(
            param_index, resource.get_gpu_vaddress().into()
        )}
    }

    /// set a decriptor table in the graphics root signature
    #[inline]
    fn set_graphics_root_dt<H: Into<::winapi::D3D12_GPU_DESCRIPTOR_HANDLE>>
    (
        &mut self, param_index: u32, base_descriptor: H
    ) {
        unsafe { self.ptr.SetGraphicsRootDescriptorTable(
            param_index, base_descriptor.into()
        )}
    }

    /// set the compute root signature. TODO: distince root signatures
    #[inline]
    fn set_compute_rootsig(&mut self, rootsig: &::pipeline::rootsig::RootSig) {
        unsafe { self.ptr.SetComputeRootSignature(rootsig.ptr.as_mut_ptr())}
    }


    /// set a constant in the compute root signature
    #[inline]
    fn set_compute_root_constant(
        &mut self, param_index: u32, value: u32, param_offset: u32
    ) {
        unsafe { self.ptr.SetComputeRoot32BitConstant(param_index, value, param_offset)}
    }

    /// set a group of constants in the compute root signature
    #[inline]
    fn set_compute_root_constants(
        &mut self, param_index: u32, values: &[u32], param_offset: u32
    ) {
        unsafe { self.ptr.SetComputeRoot32BitConstants(
            param_index, values.len() as u32, values.as_ptr() as *const _, param_offset
        )}
    }

    /// set a cbv in the compute root signature
    #[inline]
    fn set_compute_root_cbv(
        &mut self, param_index: u32, resource: &mut RawResource
    ) {
        unsafe { self.ptr.SetComputeRootConstantBufferView(
            param_index, resource.get_gpu_vaddress().into()
        )}
    }

    /// set a srv in the compute root signature
    #[inline]
    fn set_compute_root_srv(
        &mut self, param_index: u32, resource: &mut RawResource
    ) {
        unsafe { self.ptr.SetComputeRootShaderResourceView(
            param_index, resource.get_gpu_vaddress().into()
        )}
    }

    /// set a uav in the compute root signature
    #[inline]
    fn set_compute_root_uav(
        &mut self, param_index: u32, resource: &mut RawResource
    ) {
        unsafe { self.ptr.SetComputeRootUnorderedAccessView(
            param_index, resource.get_gpu_vaddress().into()
        )}
    }

    /// set a decriptor table in the compute root signature
    #[inline]
    fn set_compute_root_dt<H: Into<::winapi::D3D12_GPU_DESCRIPTOR_HANDLE>>
    (
        &mut self, param_index: u32, base_descriptor: H
    ) {
        unsafe { self.ptr.SetComputeRootDescriptorTable(
            param_index, base_descriptor.into()
        )}
    }
    
    /// execute a command list from a thread group. [more info](https://msdn.microsoft.com/zh-cn/library/windows/desktop/dn903871(v=vs.85).aspx)
    #[inline]
    fn dispatch(&mut self, x: u32, y: u32, z: u32) {
        unsafe { self.ptr.Dispatch(x, y, z) }
    }

    /// set primitive topology
    #[inline]
    fn ia_set_primitive_topology(&mut self, topology: ::pipeline::ia::PrimitiveTopology) {
        unsafe { self.ptr.IASetPrimitiveTopology(::std::mem::transmute(topology))}
    }

    /// set the index buffer
    #[inline]
    fn ia_set_ibv(&mut self, ibv: &::pipeline::ia::IndexBufferView) {
        unsafe { self.ptr.IASetIndexBuffer(ibv as *const _ as *const _)}
    }

    /// set the vertex buffer
    #[inline]
    fn ia_set_vbvs(
        &mut self, start_slot: u32, vbvs: &[::pipeline::ia::VertexBufferView]
    ) {
        unsafe {
            self.ptr.IASetVertexBuffers(
                start_slot, vbvs.len() as u32, vbvs.as_ptr() as *const _
            )
        }
    }

    /// set the blend factor
    #[inline]
    fn om_set_blend_factor(&mut self, factors: [f32; 4]) {
        unsafe { self.ptr.OMSetBlendFactor(factors.as_ptr() as *const _)}
    }

    /// set the stencil reference
    #[inline]
    fn om_set_stencil_ref(&mut self, stencil_ref: u32) {
        unsafe { self.ptr.OMSetStencilRef(stencil_ref)}
    }

    /// set rtv and dsvs
    #[inline]
    fn om_set_rtv_dsv_continuous(
        &mut self, rtv_heap: &mut RtvHeap, rtv_offset: u32, 
        num_rtvs: u32, dsv: CpuDsvHandle
    ) {
        debug_assert!(rtv_offset + num_rtvs <= rtv_heap.len());
        let rtv_handle = rtv_heap.get_cpu_handle(rtv_offset);
        unsafe {
            self.ptr.OMSetRenderTargets(
                num_rtvs, &rtv_handle as *const _ as *const _,
                ::winapi::TRUE, &dsv as *const _ as *const _
            )
        }
    }

    /// set rtv and dsvs
    #[inline]
    fn om_set_rtv_dsv_discontinuous(
        &mut self, rtvs: &[CpuRtvHandle], dsv: CpuDsvHandle
    ) {
        unsafe {
            self.ptr.OMSetRenderTargets(
                rtvs.len() as u32, 
                rtvs.as_ptr() as *const _,
                ::winapi::TRUE, &dsv as *const _ as *const _
            )
        }
    }

    /// set the pipeline state
    #[inline]
    fn set_pipeline_state(
        &mut self, state: Option<&PipelineState>
    ) {
        let p_state = if let Some(state) = state {
            state.ptr.as_mut_ptr()
        } else {
            ::std::ptr::null_mut()
        };
        unsafe { self.ptr.SetPipelineState(p_state)}
    }

    // TODO: predication

    // TODO: queries

    // TODO: execute indirect

    // TODO: tiled resource copying

    // TODO: add method to discard resource

    // TODO: should drawing methods be delegated or not?
}}}

impl_common_commands!(DirectCommandListRecording);
impl_common_commands!(BundleRecording);
