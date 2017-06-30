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
    //  get raw ID3D12CommandList pointer
    fn as_raw_ptr(&mut self) -> &mut ComPtr<ID3D12GraphicsCommandList>;

    /// get type of this command list
    fn get_type(&mut self) -> CommandListType {
        unsafe { ::std::mem::transmute(self.as_raw_ptr().GetType()) }
    }
}

macro_rules! impl_common_commands {
    ($Type: ident, $($T: tt),+) => {
        impl<$($T),+> CommandList for $Type<$($T),+> {
            //  get raw ID3D12CommandList pointer
            #[inline]
            fn as_raw_ptr(&mut self) -> &mut ComPtr<ID3D12GraphicsCommandList> {
                &mut self.ptr
            }
        }
    }
}

/// common methods for a graphics command list
pub trait GraphicsCommandList: CommandList {
    /// set the graphics root signature. TODO: distince root signatures
    #[inline]
    fn set_rootsig(&mut self, rootsig: &::pipeline::rootsig::RootSig) {
        unsafe { self.as_raw_ptr().SetGraphicsRootSignature(rootsig.ptr.as_mut_ptr())}
    }

    /// set a constant in the graphics root signature
    #[inline]
    fn set_root_constant(
        &mut self, param_index: u32, value: u32, param_offset: u32
    ) {
        unsafe { self.as_raw_ptr().SetGraphicsRoot32BitConstant(param_index, value, param_offset)}
    }

    /// set a group of constants in the graphics root signature
    #[inline]
    fn set_root_constants(
        &mut self, param_index: u32, values: &[u32], param_offset: u32
    ) {
        unsafe { self.as_raw_ptr().SetGraphicsRoot32BitConstants(
            param_index, values.len() as u32, values.as_ptr() as *const _, param_offset
        )}
    }

    /// set a cbv in the graphics root signature
    #[inline]
    fn set_root_cbv(
        &mut self, param_index: u32, resource: &mut RawResource
    ) {
        unsafe { self.as_raw_ptr().SetGraphicsRootConstantBufferView(
            param_index, resource.get_gpu_vaddress().into()
        )}
    }

    /// set a srv in the graphics root signature
    #[inline]
    fn set_root_srv(
        &mut self, param_index: u32, resource: &mut RawResource
    ) {
        unsafe { self.as_raw_ptr().SetGraphicsRootShaderResourceView(
            param_index, resource.get_gpu_vaddress().into()
        )}
    }

    /// set a uav in the graphics root signature
    #[inline]
    fn set_root_uav(
        &mut self, param_index: u32, resource: &mut RawResource
    ) {
        unsafe { self.as_raw_ptr().SetGraphicsRootUnorderedAccessView(
            param_index, resource.get_gpu_vaddress().into()
        )}
    }

    /// set primitive topology
    #[inline]
    fn ia_set_primitive_topology(&mut self, topology: ::pipeline::ia::PrimitiveTopology) {
        unsafe { self.as_raw_ptr().IASetPrimitiveTopology(::std::mem::transmute(topology))}
    }

    /// set the index buffer
    #[inline]
    fn ia_set_ibv(&mut self, ibv: &::pipeline::ia::IndexBufferView) {
        unsafe { self.as_raw_ptr().IASetIndexBuffer(ibv as *const _ as *const _)}
    }

    /// set the vertex buffer
    #[inline]
    fn ia_set_vbvs(
        &mut self, start_slot: u32, vbvs: &[::pipeline::ia::VertexBufferView]
    ) {
        unsafe {
            self.as_raw_ptr().IASetVertexBuffers(
                start_slot, vbvs.len() as u32, vbvs.as_ptr() as *const _
            )
        }
    }

    /// set the blend factor
    #[inline]
    fn om_set_blend_factor(&mut self, factors: [f32; 4]) {
        unsafe { self.as_raw_ptr().OMSetBlendFactor(factors.as_ptr() as *const _)}
    }

    /// set the stencil reference
    #[inline]
    fn om_set_stencil_ref(&mut self, stencil_ref: u32) {
        unsafe { self.as_raw_ptr().OMSetStencilRef(stencil_ref)}
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
            self.as_raw_ptr().OMSetRenderTargets(
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
            self.as_raw_ptr().OMSetRenderTargets(
                rtvs.len() as u32, 
                rtvs.as_ptr() as *const _,
                ::winapi::TRUE, &dsv as *const _ as *const _
            )
        }
    }

    /// set the pipeline state
    #[inline]
    fn set_pipeline_state(
        &mut self, state: Option<&GraphicsPipelineState>
    ) {
        let p_state = if let Some(state) = state {
            state.ptr.as_mut_ptr()
        } else {
            ::std::ptr::null_mut()
        };
        unsafe { self.as_raw_ptr().SetPipelineState(p_state)}
    }

    /// draw non-indexed, instanced primitives
    #[inline]
    fn draw(
        &mut self, vertex_per_instance: u32, instance_count: u32, 
        first_vertex_index: u32, first_instance_index: u32
    ) {
        unsafe { self.as_raw_ptr().DrawInstanced(
            vertex_per_instance, instance_count, 
            first_vertex_index, first_instance_index
        )}
    }

    /// draw indexed, instanced primitives
    fn draw_indexed(
        &mut self, index_per_instance: u32, instance_count: u32,
        first_index_index: u32, vertex_index_offset: i32,
        first_instance_index: u32
    ) {
        unsafe { self.as_raw_ptr().DrawIndexedInstanced(
            index_per_instance, instance_count,
            first_index_index, vertex_index_offset,
            first_instance_index
        )}
    }

    // TODO: predication

    // TODO: queries

    // TODO: execute indirect

    // TODO: tiled resource copying

    // TODO: add method to discard resource

    // TODO: should drawing methods be delegated or not?
}

/// common methods for a compute command list
pub trait ComputeCommandList: CommandList {
    /// set the compute root signature. TODO: distince root signatures
    #[inline]
    fn set_rootsig(&mut self, rootsig: &::pipeline::rootsig::RootSig) {
        unsafe { self.as_raw_ptr().SetComputeRootSignature(rootsig.ptr.as_mut_ptr())}
    }


    /// set a constant in the compute root signature
    #[inline]
    fn set_root_constant(
        &mut self, param_index: u32, value: u32, param_offset: u32
    ) {
        unsafe { self.as_raw_ptr().SetComputeRoot32BitConstant(param_index, value, param_offset)}
    }

    /// set a group of constants in the compute root signature
    #[inline]
    fn set_root_constants(
        &mut self, param_index: u32, values: &[u32], param_offset: u32
    ) {
        unsafe { self.as_raw_ptr().SetComputeRoot32BitConstants(
            param_index, values.len() as u32, values.as_ptr() as *const _, param_offset
        )}
    }

    /// set a cbv in the compute root signature
    #[inline]
    fn set_root_cbv(
        &mut self, param_index: u32, resource: &mut RawResource
    ) {
        unsafe { self.as_raw_ptr().SetComputeRootConstantBufferView(
            param_index, resource.get_gpu_vaddress().into()
        )}
    }

    /// set a srv in the compute root signature
    #[inline]
    fn set_root_srv(
        &mut self, param_index: u32, resource: &mut RawResource
    ) {
        unsafe { self.as_raw_ptr().SetComputeRootShaderResourceView(
            param_index, resource.get_gpu_vaddress().into()
        )}
    }

    /// set a uav in the compute root signature
    #[inline]
    fn set_root_uav(
        &mut self, param_index: u32, resource: &mut RawResource
    ) {
        unsafe { self.as_raw_ptr().SetComputeRootUnorderedAccessView(
            param_index, resource.get_gpu_vaddress().into()
        )}
    }
    
    /// execute a command list from a thread group. [more info](https://msdn.microsoft.com/zh-cn/library/windows/desktop/dn903871(v=vs.85).aspx)
    #[inline]
    fn dispatch(&mut self, x: u32, y: u32, z: u32) {
        unsafe { self.as_raw_ptr().Dispatch(x, y, z) }
    }

    /// set the pipeline state
    #[inline]
    fn set_pipeline_state(
        &mut self, state: Option<&ComputePipelineState>
    ) {
        let p_state = if let Some(state) = state {
            state.ptr.as_mut_ptr()
        } else {
            ::std::ptr::null_mut()
        };
        unsafe { self.as_raw_ptr().SetPipelineState(p_state)}
    }
}

/// common methods for a copy command list
pub trait CopyCommandList: CommandList {
    /// record a resource copy operation.
    ///
    /// # restrictions
    ///
    /// - must be different resources
    /// - must be the same type
    /// - must have identical dimensions
    /// - must have compatible formats
    /// - [more](https://msdn.microsoft.com/zh-cn/library/windows/desktop/dn903859(v=vs.85).aspx)
    #[inline]
    fn copy_resource(
        &mut self, dst: &mut RawResource, src: &mut RawResource
    ) {
        unsafe {
            self.as_raw_ptr().CopyResource(dst.ptr.as_mut_ptr(), src.ptr.as_mut_ptr())
        }
    }

    /// record a buffer copy operation.
    /// Linear subresource copying must be aligned to 512 bytes
    /// constant data reads must be a multiple of 256 bytes from the beginning of the heap
    #[inline]
    fn copy_buffer_region(
        &mut self, dst: &mut RawResource, dst_offset: u64,
        src: &mut RawResource, src_offset: u64, size: u64
    ) {
        unsafe {
            self.as_raw_ptr().CopyBufferRegion(
                dst.ptr.as_mut_ptr(), dst_offset,
                src.ptr.as_mut_ptr(), src_offset, size
            )
        }
    }

    /// record a texture copy operation. [more info](https://msdn.microsoft.com/zh-cn/library/windows/desktop/dn903862(v=vs.85).aspx)
    #[inline]
    fn copy_texture_region(
        &mut self, dst: TextureCopyLocation, dstx: u32, dsty: u32, 
        dstz: u32, src: TextureCopyLocation, src_box: &::format::Box3u
    ) {
        let dst = dst.into();
        let src = src.into();
        unsafe {
            self.as_raw_ptr().CopyTextureRegion(
                &dst, dstx, dsty, dstz, 
                &src, src_box as *const _ as *const _
            )
        }
    }
}

/// common methods for a command list with bounded descriptor heaps
pub trait CommandListWithHeap {
    /// set a decriptor table in the graphics root signature TODO: double check
    fn set_graphics_root_dt<H: Into<::winapi::D3D12_GPU_DESCRIPTOR_HANDLE>>
    (
        &mut self, param_index: u32, base_descriptor: H
    );

    /// set a decriptor table in the compute root signature TODO: double check
    fn set_compute_root_dt<H: Into<::winapi::D3D12_GPU_DESCRIPTOR_HANDLE>>
    (
        &mut self, param_index: u32, base_descriptor: H
    );
}

macro_rules! impl_common_with_heap_commands{
    ($Type: ident, $($T: tt),+) => {
impl<$($T),+> CommandListWithHeap for $Type<$($T),+> {
    /// set a decriptor table in the graphics root signature
    #[inline]
    fn set_graphics_root_dt<H: Into<::winapi::D3D12_GPU_DESCRIPTOR_HANDLE>>
    (
        &mut self, param_index: u32, base_descriptor: H
    ) {
        unsafe { self.as_raw_ptr().SetGraphicsRootDescriptorTable(
            param_index, base_descriptor.into()
        )}
    }

    /// set a decriptor table in the compute root signature
    #[inline]
    fn set_compute_root_dt<H: Into<::winapi::D3D12_GPU_DESCRIPTOR_HANDLE>>
    (
        &mut self, param_index: u32, base_descriptor: H
    ) {
        unsafe { self.as_raw_ptr().SetComputeRootDescriptorTable(
            param_index, base_descriptor.into()
        )}
    }
}}}

impl_common_commands!(DirectCommandListRecording, 'a, T);
impl_common_commands!(BundleRecording, 'a);
impl_common_commands!(BundleRecordingWithHeap, 'a);
impl_common_with_heap_commands!(DirectCommandListRecording, 'a, T);
impl_common_with_heap_commands!(BundleRecordingWithHeap, 'a);

impl<'a> GraphicsCommandList for DirectCommandListRecording<'a, GraphicsPipelineState> {}

impl<'a> ComputeCommandList for DirectCommandListRecording<'a, ComputePipelineState> {}

impl<'a, P: PipelineState+'a> CopyCommandList for DirectCommandListRecording<'a, P> {}

impl<'a> GraphicsCommandList for BundleRecording<'a> {}

impl<'a> ComputeCommandList for BundleRecording<'a> {}

impl<'a> GraphicsCommandList for BundleRecordingWithHeap<'a> {}

impl<'a> ComputeCommandList for BundleRecordingWithHeap<'a> {}
