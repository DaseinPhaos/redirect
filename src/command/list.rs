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
use resource::RawResource;

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
pub struct GraphicsCommandList {
    pub ptr: ComPtr<ID3D12GraphicsCommandList>,
}

// TODO: distinct bundle from direct command lists at type level?

impl GraphicsCommandList {
    // TODO: add methods for queries. blocker: Query interface

    /// record a command to clear the dsv. [more info](https://msdn.microsoft.com/zh-cn/library/windows/desktop/dn903840(v=vs.85).aspx)
    pub fn clear_dsv(
        &mut self, heap: &mut DsvHeap, index: u32,
        flags: DepthStencilClearFlags, depth: f32, stencil: u8,
        rects: Option<&[::format::Rect]>
    ) {
        let handle = heap.get_cpu_handle(index);
        let (numrects, prects) = if let Some(rects) = rects {
            (rects.len() as u32, rects.as_ptr())
        } else {
            (0, ::std::ptr::null())
        };
        unsafe {
            self.ptr.ClearDepthStencilView(
                ::std::mem::transmute(handle), 
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
        let cpuh = heap.get_cpu_handle(index);
        let gpuh = heap.get_gpu_handle(index);
        let (numrects, prects) = if let Some(rects) = rects {
            (rects.len() as u32, rects.as_ptr())
        } else {
            (0, ::std::ptr::null())
        };
        unsafe {
            self.ptr.ClearUnorderedAccessViewFloat(
                ::std::mem::transmute(gpuh),
                ::std::mem::transmute(cpuh),
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
        let cpuh = heap.get_cpu_handle(index);
        let gpuh = heap.get_gpu_handle(index);
        let (numrects, prects) = if let Some(rects) = rects {
            (rects.len() as u32, rects.as_ptr())
        } else {
            (0, ::std::ptr::null())
        };
        unsafe {
            self.ptr.ClearUnorderedAccessViewFloat(
                ::std::mem::transmute(gpuh),
                ::std::mem::transmute(cpuh),
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
        let cpuh = heap.get_cpu_handle(index);
        let (numrects, prects) = if let Some(rects) = rects {
            (rects.len() as u32, rects.as_ptr())
        } else {
            (0, ::std::ptr::null())
        };
        unsafe {
            self.ptr.ClearRenderTargetView(
                ::std::mem::transmute(cpuh),
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
    pub fn copy_resource(
        &mut self, dst: &mut RawResource, src: &mut RawResource
    ) {
        unsafe {
            self.ptr.CopyResource(dst.ptr.as_mut_ptr(), src.ptr.as_mut_ptr())
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
