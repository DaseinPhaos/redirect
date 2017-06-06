// Copyright 2017 Dasein Phaos aka. Luxko
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! direct command lists

use super::*;


/// An allocator for GPU commands
#[derive(Debug)]
pub struct DirectCommandAllocator {
    pub ptr: ComPtr<ID3D12CommandAllocator>,
}

impl DirectCommandAllocator {
    /// indicates that the associated memory would be recycled by the allocator.
    #[inline]
    pub fn reset(&mut self) -> Result<(), WinError> {
        let hr = unsafe {self.ptr.Reset()};
        WinError::from_hresult(hr)
    }
}

/// a direct command list
#[derive(Clone, Debug)]
pub struct DirectCommandList {
    pub ptr: ComPtr<ID3D12GraphicsCommandList>,
}

impl DirectCommandList {
    /// start command recording. [more](https://msdn.microsoft.com/zh-cn/library/windows/desktop/dn903895(v=vs.85).aspx)
    pub fn start_graphics<'b>(
        mut self, alloc: &'b mut DirectCommandAllocator, 
        initial_state: Option<&'b GraphicsPipelineState>
    ) -> Result<DirectCommandListRecording<'b, GraphicsPipelineState>, (WinError, Self)> {
        let p_initial_state = if let Some(initial_state) = initial_state {
            initial_state.ptr.as_mut_ptr()
        } else {
            ::std::ptr::null_mut()
        };
        unsafe {
            let result = WinError::from_hresult(self.ptr.Reset(alloc.ptr.as_mut_ptr(), p_initial_state));
            if result.is_ok() {
                Ok(DirectCommandListRecording{ ptr: self.ptr, alloc, initial_state})
            } else {
                Err((result.unwrap_err(), self))
            }
        }
    }

    /// start command recording. [more](https://msdn.microsoft.com/zh-cn/library/windows/desktop/dn903895(v=vs.85).aspx)
    pub fn start_compute<'b>(
        mut self, alloc: &'b mut DirectCommandAllocator, 
        initial_state: Option<&'b ComputePipelineState>
    ) -> Result<DirectCommandListRecording<'b, ComputePipelineState>, (WinError, Self)> {
        let p_initial_state = if let Some(initial_state) = initial_state {
            initial_state.ptr.as_mut_ptr()
        } else {
            ::std::ptr::null_mut()
        };
        unsafe {
            let result = WinError::from_hresult(self.ptr.Reset(alloc.ptr.as_mut_ptr(), p_initial_state));
            if result.is_ok() {
                Ok(DirectCommandListRecording{ ptr: self.ptr, alloc, initial_state})
            } else {
                Err((result.unwrap_err(), self))
            }
        }
    }
}

/// a direct command list on recording state
#[derive(Debug)]
pub struct DirectCommandListRecording<'a, P: 'a> {
    pub ptr: ComPtr<ID3D12GraphicsCommandList>,
    /// command allocator used to back up command recording
    pub alloc: &'a mut DirectCommandAllocator,
    /// initial state of this command list
    pub initial_state: Option<&'a P>,
}

impl<'a, P: 'a + PipelineState> DirectCommandListRecording<'a, P> {
    /// record a command to clear the dsv. [more info](https://msdn.microsoft.com/zh-cn/library/windows/desktop/dn903840(v=vs.85).aspx)
    pub fn clear_dsv(
        &mut self, dsv: CpuDsvHandle,
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
                dsv.into(),
                ::std::mem::transmute(flags), 
                depth, stencil, numrects, prects
            );
        }
    }

    /// record a command to clear uav. [more info](https://msdn.microsoft.com/zh-cn/library/windows/desktop/dn903849(v=vs.85).aspx)
    pub fn clear_uav_f32<T: CsuHeap>(
        &mut self, heap: &mut T, index: u32,
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
    pub fn clear_uav_u32<T: CsuHeap>(
        &mut self, heap: &mut T, index: u32,
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
        &mut self, rtv: CpuRtvHandle,
        values: &[f32; 4], rects: Option<&[::format::Rect]>
    ) {
        let (numrects, prects) = if let Some(rects) = rects {
            (rects.len() as u32, rects.as_ptr())
        } else {
            (0, ::std::ptr::null())
        };
        unsafe {
            self.ptr.ClearRenderTargetView(
                rtv.into(),
                values.as_ptr() as *const _,
                numrects, prects
            )
        }
    }

    /// execute a bundle. [more](https://msdn.microsoft.com/zh-cn/library/windows/desktop/dn903882(v=vs.85).aspx)
    #[inline]
    pub fn execute_bundle(&mut self, bundle: &Bundle) {
        unsafe { self.ptr.ExecuteBundle(bundle.ptr.as_mut_ptr())}
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
    pub fn resource_barriers(&mut self, barriers: &ResourceBarriersBuilder) {
        let barriers = barriers.as_ffi_slice();
        unsafe {
            self.ptr.ResourceBarrier(
                barriers.len() as u32,
                barriers.as_ptr()
            )
        }
    }

    /// reset the state of a direct command list back to the state it was in when created.
    /// initial_state has to match this state
    pub fn clear_state(&mut self) {
        let p_initial_state = if let Some(initial_state) = self.initial_state {
            initial_state.as_raw_ptr().as_mut_ptr()
        } else {
            ::std::ptr::null_mut()
        };
        unsafe { self.ptr.ClearState(p_initial_state)}
    }

    // TODO: double check descriptor heap settings
    #[inline]
    pub fn set_descriptor_heaps<T: CsuHeap, S: SamplerHeap>(
        &mut self, cbv_srv_uav_heap: Option<&mut T>,
        sampler_heap: Option<&mut S>
    ) {
        let mut heaps = [
            ::std::ptr::null_mut(), ::std::ptr::null_mut(),
        ];
        if let Some(heap) = cbv_srv_uav_heap {
            heaps[1] = heap.as_raw_ptr().as_mut_ptr();
        }
        if let Some(heap) = sampler_heap {
            heaps[0] = heap.as_raw_ptr().as_mut_ptr();
        }
        unsafe {
            self.ptr.SetDescriptorHeaps(2, heaps.as_mut_ptr())
        }
    }

    /// reset a command list back to the initial state. [more](https://msdn.microsoft.com/zh-cn/library/windows/desktop/dn903895(v=vs.85).aspx)
    pub fn reset<'b, T: PipelineState+'b>(
        mut self, alloc: &'b mut DirectCommandAllocator, 
        initial_state: Option<&'b T>
    ) -> Result<DirectCommandListRecording<'b, T>, (WinError, Self)> {
        let p_initial_state = if let Some(initial_state) = initial_state {
            initial_state.as_raw_ptr().as_mut_ptr()
        } else {
            ::std::ptr::null_mut()
        };
        unsafe {
            let result = WinError::from_hresult(self.ptr.Reset(alloc.ptr.as_mut_ptr(), p_initial_state));
            if result.is_ok() {
                Ok(DirectCommandListRecording{ ptr: self.ptr, alloc, initial_state})
            } else {
                Err((result.unwrap_err(), self))
            }
        }
    }

    /// close the current recording
    #[inline]
    pub fn close(mut self) -> Result<DirectCommandList, WinError> {
        unsafe{
            WinError::from_hresult_or_ok(self.ptr.Close(), move || DirectCommandList{
                ptr: self.ptr
            })
        }
    }
}
