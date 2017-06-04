// Copyright 2017 Dasein Phaos aka. Luxko
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! command lists

use comptr::ComPtr;
use winapi::ID3D12GraphicsCommandList;
use error::WinError;
use descriptor::heap::*;
use super::CommandAllocator;
use pipeline::PipelineState;
use resource::{RawResource, ResourceBarrierBuilder, TextureCopyLocation};

bitflags!{
    /// type of a `CommandList`
    #[repr(C)]
    pub struct CommandListType: u32 {
        /// direct list doesn't inherit any GPU state.
        const COMMAND_LIST_TYPE_DIRECT = 0;
        /// bundle inherits all GPU state (except PSO and primitive topology)
        /// from the direct list, where it must be opearated on
        const COMMAND_LIST_TYPE_BUNDLE = 1;
        /// computing command list
        const COMMAND_LIST_TYPE_COMPUTE = 2;
        /// copying(drawing) command list
        const COMMAND_LIST_TYPE_COPY = 3;
    }
}

impl Default for CommandListType {
    #[inline]
    fn default() -> Self {
        COMMAND_LIST_TYPE_DIRECT
    }
}

/// a list of graphics commands, including execution intrumenting APIs etc.
/// TODO: distinct bundle from direct command lists at type level?
pub struct GraphicsCommandList {
    pub ptr: ComPtr<ID3D12GraphicsCommandList>,
}

impl GraphicsCommandList {
    /// set the graphics root signature. TODO: distince root signatures
    #[inline]
    pub fn set_graphics_rootsig(&mut self, rootsig: &::pipeline::rootsig::RootSig) {
        unsafe { self.ptr.SetGraphicsRootSignature(rootsig.ptr.as_mut_ptr())}
    }

    /// set a constant in the graphics root signature
    #[inline]
    pub fn set_graphics_root_constant(
        &mut self, param_index: u32, value: u32, param_offset: u32
    ) {
        unsafe { self.ptr.SetGraphicsRoot32BitConstant(param_index, value, param_offset)}
    }

    /// set a group of constants in the graphics root signature
    #[inline]
    pub fn set_graphics_root_constants(
        &mut self, param_index: u32, values: &[u32], param_offset: u32
    ) {
        unsafe { self.ptr.SetGraphicsRoot32BitConstants(
            param_index, values.len() as u32, values.as_ptr() as *const _, param_offset
        )}
    }

    /// set a cbv in the graphics root signature
    #[inline]
    pub fn set_graphics_root_cbv(
        &mut self, param_index: u32, resource: &mut RawResource
    ) {
        unsafe { self.ptr.SetGraphicsRootConstantBufferView(
            param_index, resource.get_gpu_vaddress().into()
        )}
    }

    /// set a srv in the graphics root signature
    #[inline]
    pub fn set_graphics_root_srv(
        &mut self, param_index: u32, resource: &mut RawResource
    ) {
        unsafe { self.ptr.SetGraphicsRootShaderResourceView(
            param_index, resource.get_gpu_vaddress().into()
        )}
    }

    /// set a uav in the graphics root signature
    #[inline]
    pub fn set_graphics_root_uav(
        &mut self, param_index: u32, resource: &mut RawResource
    ) {
        unsafe { self.ptr.SetGraphicsRootUnorderedAccessView(
            param_index, resource.get_gpu_vaddress().into()
        )}
    }

    /// set a decriptor table in the graphics root signature
    #[inline]
    pub fn set_graphics_root_dt(
        &mut self, param_index: u32, base_descriptor: GpuDescriptorHandle
    ) {
        unsafe { self.ptr.SetGraphicsRootDescriptorTable(
            param_index, base_descriptor.into()
        )}
    }

    /// set the compute root signature. TODO: distince root signatures
    #[inline]
    pub fn set_compute_rootsig(&mut self, rootsig: &::pipeline::rootsig::RootSig) {
        unsafe { self.ptr.SetComputeRootSignature(rootsig.ptr.as_mut_ptr())}
    }


    /// set a constant in the compute root signature
    #[inline]
    pub fn set_compute_root_constant(
        &mut self, param_index: u32, value: u32, param_offset: u32
    ) {
        unsafe { self.ptr.SetComputeRoot32BitConstant(param_index, value, param_offset)}
    }

    /// set a group of constants in the compute root signature
    #[inline]
    pub fn set_compute_root_constants(
        &mut self, param_index: u32, values: &[u32], param_offset: u32
    ) {
        unsafe { self.ptr.SetComputeRoot32BitConstants(
            param_index, values.len() as u32, values.as_ptr() as *const _, param_offset
        )}
    }

    /// set a cbv in the compute root signature
    #[inline]
    pub fn set_compute_root_cbv(
        &mut self, param_index: u32, resource: &mut RawResource
    ) {
        unsafe { self.ptr.SetComputeRootConstantBufferView(
            param_index, resource.get_gpu_vaddress().into()
        )}
    }

    /// set a srv in the compute root signature
    #[inline]
    pub fn set_compute_root_srv(
        &mut self, param_index: u32, resource: &mut RawResource
    ) {
        unsafe { self.ptr.SetComputeRootShaderResourceView(
            param_index, resource.get_gpu_vaddress().into()
        )}
    }

    /// set a uav in the compute root signature
    #[inline]
    pub fn set_compute_root_uav(
        &mut self, param_index: u32, resource: &mut RawResource
    ) {
        unsafe { self.ptr.SetComputeRootUnorderedAccessView(
            param_index, resource.get_gpu_vaddress().into()
        )}
    }

    /// set a decriptor table in the compute root signature
    #[inline]
    pub fn set_compute_root_dt(
        &mut self, param_index: u32, base_descriptor: GpuDescriptorHandle
    ) {
        unsafe { self.ptr.SetComputeRootDescriptorTable(
            param_index, base_descriptor.into()
        )}
    }

    /// record a command to clear the dsv. [more info](https://msdn.microsoft.com/zh-cn/library/windows/desktop/dn903840(v=vs.85).aspx)
    pub fn clear_dsv(
        &mut self, heap: &mut DsvHeap, index: u32,
        flags: DepthStencilClearFlags, depth: f32, stencil: u8,
        rects: Option<&[::format::Rect]>
    ) {
        let (numrects, prects) = if let Some(rects) = rects {
            (rects.len() as u32, rects.as_ptr())
        } else {
            (0, ::std::ptr::null())
        };
        unsafe {
            self.ptr.ClearDepthStencilView(
                heap.get_cpu_handle(index).into(),
                ::std::mem::transmute(flags), 
                depth, stencil, numrects, prects
            );
        }
    }

    /// record a command to clear uav. [more info](https://msdn.microsoft.com/zh-cn/library/windows/desktop/dn903849(v=vs.85).aspx)
    pub fn clear_uav_f32(
        &mut self, heap: &mut CbvSrvUavHeap, index: u32,
        resource: &mut RawResource, values: &[f32; 4],
        rects: Option<&[::format::Rect]>
    ) {
        let (numrects, prects) = if let Some(rects) = rects {
            (rects.len() as u32, rects.as_ptr())
        } else {
            (0, ::std::ptr::null())
        };
        unsafe {
            self.ptr.ClearUnorderedAccessViewFloat(
                heap.get_gpu_handle(index).into(),
                heap.get_cpu_handle(index).into(),
                resource.ptr.as_mut_ptr(),
                values.as_ptr() as *const _,
                numrects, prects
            )
        }
    }

    /// record a command to clear uav. [more info](https://msdn.microsoft.com/zh-cn/library/windows/desktop/dn903849(v=vs.85).aspx)
    pub fn clear_uav_u32(
        &mut self, heap: &mut CbvSrvUavHeap, index: u32,
        resource: &mut RawResource, values: &[u32; 4],
        rects: Option<&[::format::Rect]>
    ) {
        let (numrects, prects) = if let Some(rects) = rects {
            (rects.len() as u32, rects.as_ptr())
        } else {
            (0, ::std::ptr::null())
        };
        unsafe {
            self.ptr.ClearUnorderedAccessViewFloat(
                heap.get_gpu_handle(index).into(),
                heap.get_cpu_handle(index).into(),
                resource.ptr.as_mut_ptr(),
                values.as_ptr() as *const _,
                numrects, prects
            )
        }
    }

    /// record clearing a rtv
    pub fn clear_rtv(
        &mut self, heap: &mut RtvHeap, index: u32,
        values: &[f32; 4], rects: Option<&[::format::Rect]>
    ) {
        let (numrects, prects) = if let Some(rects) = rects {
            (rects.len() as u32, rects.as_ptr())
        } else {
            (0, ::std::ptr::null())
        };
        unsafe {
            self.ptr.ClearRenderTargetView(
                heap.get_cpu_handle(index).into(),
                values.as_ptr() as *const _,
                numrects, prects
            )
        }
    }

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
    pub fn copy_resource(
        &mut self, dst: &mut RawResource, src: &mut RawResource
    ) {
        unsafe {
            self.ptr.CopyResource(dst.ptr.as_mut_ptr(), src.ptr.as_mut_ptr())
        }
    }

    /// record a buffer copy operation
    #[inline]
    pub fn copy_buffer_region(
        &mut self, dst: &mut RawResource, dst_offset: u64,
        src: &mut RawResource, src_offset: u64, size: u64
    ) {
        unsafe {
            self.ptr.CopyBufferRegion(
                dst.ptr.as_mut_ptr(), dst_offset,
                src.ptr.as_mut_ptr(), src_offset, size
            )
        }
    }

    /// record a texture copy operation. [more info](https://msdn.microsoft.com/zh-cn/library/windows/desktop/dn903862(v=vs.85).aspx)
    #[inline]
    pub fn copy_texture_region(
        &mut self, dst: TextureCopyLocation, dstx: u32, dsty: u32, 
        dstz: u32, src: TextureCopyLocation, src_box: &::format::Box3u
    ) {
        let dst = dst.into();
        let src = src.into();
        unsafe {
            self.ptr.CopyTextureRegion(
                &dst, dstx, dsty, dstz, 
                &src, src_box as *const _ as *const _
            )
        }
    }

    /// execute a command list from a thread group. [more info](https://msdn.microsoft.com/zh-cn/library/windows/desktop/dn903871(v=vs.85).aspx)
    #[inline]
    pub fn dispatch(&mut self, x: u32, y: u32, z: u32) {
        unsafe { self.ptr.Dispatch(x, y, z) }
    }

    /// execute a bundle. [more](https://msdn.microsoft.com/zh-cn/library/windows/desktop/dn903882(v=vs.85).aspx)
    #[inline]
    pub fn execute_bundle(&mut self, bundle: &mut Self) {
        unsafe { self.ptr.ExecuteBundle(bundle.ptr.as_mut_ptr())}
    }

    /// set primitive topology
    #[inline]
    pub fn ia_set_primitive_topology(&mut self, topology: ::pipeline::ia::PrimitiveTopology) {
        unsafe { self.ptr.IASetPrimitiveTopology(::std::mem::transmute(topology))}
    }

    /// set the index buffer
    #[inline]
    pub fn ia_set_ibv(&mut self, ibv: &::pipeline::ia::IndexBufferView) {
        unsafe { self.ptr.IASetIndexBuffer(ibv as *const _ as *const _)}
    }

    /// set the vertex buffer
    #[inline]
    pub fn ia_set_vbvs(
        &mut self, start_slot: u32, vbvs: &[::pipeline::ia::VertexBufferView]
    ) {
        unsafe {
            self.ptr.IASetVertexBuffers(
                start_slot, vbvs.len() as u32, vbvs.as_ptr() as *const _
            )
        }
    }

    /// set the stream output buffer views
    #[inline]
    pub fn so_set_targets(
        &mut self, start_slot: u32, sovs: &[::pipeline::so::StreamOutputBufferView]
    ) {
        unsafe {
            self.ptr.SOSetTargets(
                start_slot, sovs.len() as u32, sovs.as_ptr() as *const _
            )
        }
    }

    /// set scissor rectangles
    #[inline]
    pub fn rs_set_scissors(&mut self, scissors: &[::format::Rect]) {
        unsafe {
            self.ptr.RSSetScissorRects(
                scissors.len() as u32,
                scissors.as_ptr() as *const _
            )
        }
    }

    /// set viewports
    #[inline]
    pub fn rs_set_viewports(&mut self, viewports: &[::format::Viewport]) {
        unsafe {
            self.ptr.RSSetViewports(
                viewports.len() as u32,
                viewports.as_ptr() as *const _
            )
        }
    }

    /// set the blend factor
    #[inline]
    pub fn om_set_blend_factor(&mut self, factors: [f32; 4]) {
        unsafe { self.ptr.OMSetBlendFactor(factors.as_ptr() as *const _)}
    }

    /// set the stencil reference
    #[inline]
    pub fn om_set_stencil_ref(&mut self, stencil_ref: u32) {
        unsafe { self.ptr.OMSetStencilRef(stencil_ref)}
    }

    /// set rtv and dsvs
    #[inline]
    pub fn om_set_rtv_dsv_continuous(
        &mut self, rtv_heap: &mut RtvHeap, rtv_offset: u32, 
        num_rtvs: u32, dsv_heap: &mut DsvHeap, dsv_offset: u32
    ) {
        debug_assert!(rtv_offset + num_rtvs <= rtv_heap.len());
        let rtv_handle = rtv_heap.get_cpu_handle(rtv_offset);
        let dsv_handle = dsv_heap.get_cpu_handle(dsv_offset);
        unsafe {
            self.ptr.OMSetRenderTargets(
                num_rtvs, &rtv_handle as *const _ as *const _,
                ::winapi::TRUE, &dsv_handle as *const _ as *const _
            )
        }
    }

    /// set rtv and dsvs
    #[inline]
    pub fn om_set_rtv_dsv_discontinuous(
        &mut self, rtvs: &[CpuDescriptorHandle],
        dsv_heap: &mut DsvHeap, dsv_offset: u32
    ) {
        let dsv_handle = dsv_heap.get_cpu_handle(dsv_offset);
        unsafe {
            self.ptr.OMSetRenderTargets(
                rtvs.len() as u32, 
                rtvs.as_ptr() as *const _,
                ::winapi::TRUE, &dsv_handle as *const _ as *const _
            )
        }
    }

    /// resolve a multisampled resource into a non-MS resource. [more](https://msdn.microsoft.com/zh-cn/library/windows/desktop/dn903897(v=vs.85).aspx)
    #[inline]
    pub fn resolve_ms(
        &mut self, dst: &mut RawResource, dst_sub: u32,
        src: &mut RawResource, src_sub: u32, format: ::format::DxgiFormat
    ) {
        unsafe {
            self.ptr.ResolveSubresource(
                dst.ptr.as_mut_ptr(), dst_sub,
                src.ptr.as_mut_ptr(), src_sub,
                format
            )
        }
    }

    /// synchronizaing multiple access to resources. [more](https://msdn.microsoft.com/zh-cn/library/windows/desktop/dn903898(v=vs.85).aspx)
    pub fn resource_barrier(&mut self, barriers: &ResourceBarrierBuilder) {
        let barriers = barriers.as_ffi_slice();
        unsafe {
            self.ptr.ResourceBarrier(
                barriers.len() as u32,
                barriers.as_ptr()
            )
        }
    }

    /// reset a command list back to the initial state. [more](https://msdn.microsoft.com/zh-cn/library/windows/desktop/dn903895(v=vs.85).aspx)
    pub fn reset(
        &mut self, alloc: &mut CommandAllocator, initial_state: Option<&PipelineState>
    ) -> Result<(), WinError> {
        let p_initial_state = if let Some(initial_state) = initial_state {
            initial_state.ptr.as_mut_ptr()
        } else {
            ::std::ptr::null_mut()
        };
        unsafe {
            WinError::from_hresult(self.ptr.Reset(alloc.ptr.as_mut_ptr(), p_initial_state))
        }
    }

    /// reset the state of a direct command list back to the state it was in when created.
    /// initial_state has to match this state
    pub fn clear_state(
        &mut self, initial_state: Option<&PipelineState>
    ) {
        let p_initial_state = if let Some(initial_state) = initial_state {
            initial_state.ptr.as_mut_ptr()
        } else {
            ::std::ptr::null_mut()
        };
        unsafe { self.ptr.ClearState(p_initial_state)}
    }

    /// set the pipeline state
    pub fn set_pipeline_state(
        &mut self, state: Option<&PipelineState>
    ) {
        let p_state = if let Some(state) = state {
            state.ptr.as_mut_ptr()
        } else {
            ::std::ptr::null_mut()
        };
        unsafe { self.ptr.SetPipelineState(p_state)}
    }

    /// close the current recording
    #[inline]
    pub fn close(&mut self) -> Result<(), WinError> {
        unsafe{
            WinError::from_hresult(self.ptr.Close())
        }
    }

    // TODO: double check descriptor heap settings
    #[inline]
    pub fn set_descriptor_heaps(
        &mut self, cbv_srv_uav_heap: Option<&CbvSrvUavHeap>,
        rtv_heap: Option<&RtvHeap>, dsv_heap: Option<&DsvHeap>,
        sampler_heap: Option<&SamplerHeap>
    ) {
        let mut heaps = [
            ::std::ptr::null_mut(), ::std::ptr::null_mut(),
            ::std::ptr::null_mut(), ::std::ptr::null_mut(),
        ];
        if let Some(heap) = cbv_srv_uav_heap {
            heaps[0] = heap.ptr.as_mut_ptr();
        }
        if let Some(heap) = rtv_heap {
            heaps[1] = heap.ptr.as_mut_ptr();
        }
        if let Some(heap) = dsv_heap {
            heaps[2] = heap.ptr.as_mut_ptr();
        }
        if let Some(heap) = sampler_heap {
            heaps[3] = heap.ptr.as_mut_ptr();
        }
        unsafe {
            self.ptr.SetDescriptorHeaps(4, heaps.as_mut_ptr())
        }
    }

    // TODO: predication

    // TODO: queries

    // TODO: execute indirect

    // TODO: tiled resource copying

    // TODO: add method to discard resource

    // TODO: should drawing methods be delegated or not?
}

bitflags!{
    /// depth stencil clear flags
    #[repr(C)]
    pub struct DepthStencilClearFlags: u32 {
        const DS_CLEAR_FLAG_DEPTH = 0x1;
        const DS_CLEAR_FLAG_STENCIL = 0x2;
    }
}

impl Default for DepthStencilClearFlags {
    #[inline]
    fn default() -> DepthStencilClearFlags {
        DS_CLEAR_FLAG_DEPTH | DS_CLEAR_FLAG_STENCIL
    }
}
