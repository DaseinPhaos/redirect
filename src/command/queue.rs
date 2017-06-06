// Copyright 2017 Dasein Phaos aka. Luxko
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use comptr::ComPtr;
use winapi::ID3D12CommandQueue;
use error::WinError;
use super::list::*;
use smallvec::SmallVec;
use fence::Fence;

/// A GPU command queue, providing methods for command submission,
/// execution synchronization, etc.
#[derive(Debug, Clone)]
pub struct CommandQueue {
    pub ptr: ComPtr<ID3D12CommandQueue>,
}

impl CommandQueue {
    // TODO: add method for `CopyTileMapping, UpdateTileMappings`, block on resources

    /// determine the rate the GPU timestamp counter increments
    #[inline]
    pub fn get_timestamp_frequency(&mut self) -> Result<u64, WinError> {
        let mut ret = 0;
        let hr = unsafe { self.ptr.GetTimestampFrequency(&mut ret)};
        WinError::from_hresult_or_ok(hr, || ret)
    }

    /// samples the CPU and GPU timestamp counters at the same moment in time.
    /// return (CPUTimestamp, GPUTimestamp) on success
    #[inline]
    pub fn get_clock_calibration(&mut self) -> Result<(u64, u64), WinError> {
        let mut gpu = 0;
        let mut cpu = 0;
        let hr = unsafe { self.ptr.GetClockCalibration(&mut gpu, &mut cpu)};
        WinError::from_hresult_or_ok(hr, || (cpu, gpu))
    }

    /// add one command list to the GPU execution queue
    ///
    /// # Safety
    /// This method is `unsafe` because it is up to the caller to ensure
    /// that th list along with the underlying allocator and
    /// all the resources it is referencing is ready for use by the GPU
    #[inline]
    pub unsafe fn execute_command_list(&mut self, list: &DirectCommandList) {
        let mut ptr = list.ptr.as_mut_ptr() as *mut ::winapi::ID3D12CommandList;
        self.ptr.ExecuteCommandLists(1, &mut ptr);
    }

    /// add a sequence of command lists to the GPU execution queue
    ///
    /// # Safety
    /// This method is `unsafe` because it is up to the caller to ensure
    /// that these lists along with the underlying allocators, and all
    /// the resources they are referencing is ready for use by the GPU
    pub unsafe fn execute_command_lists(&mut self, lists: &[DirectCommandList]) {
        let mut raw_lists: SmallVec<[*mut ::winapi::ID3D12CommandList; 8]> = Default::default();
        for list in lists {
            raw_lists.push(list.ptr.as_mut_ptr() as *mut _);
        }
        let ptr = raw_lists.as_mut_ptr();
        self.ptr.ExecuteCommandLists(lists.len() as u32, ptr);
    }

    /// use GPU to update a fence to a specified value
    #[inline]
    pub fn signal(&mut self, fence: &Fence, value: u64) -> Result<(), WinError> {
        let raw_fence = fence.ptr.as_mut_ptr();
        unsafe {
            WinError::from_hresult(self.ptr.Signal(raw_fence, value))
        }
    }

    /// wait until the specified fence reaches or exceeds the specified value
    #[inline]
    pub fn wait(&mut self, fence: &Fence, value: u64) -> Result<(), WinError> {
        let raw_fence = fence.ptr.as_mut_ptr();
        unsafe {
            WinError::from_hresult(self.ptr.Wait(raw_fence, value))
        }
    }

    // TODO: add method for PIX events?

    /// get description of this queue
    pub fn get_desc(&mut self) -> CommandQueueDesc {
        unsafe{
            let mut ret = ::std::mem::uninitialized();
            self.ptr.GetDesc(&mut ret);
            ::std::mem::transmute(ret)
        }
    }
}

/// describes a command queue.
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct CommandQueueDesc {
    /// type of the contained lists
    pub list_type: CommandListType,
    /// priority levels for the queue
    pub priority: CommandQueuePriority,
    /// misc flags
    pub flags: CommandQueueFlags,
    /// GPU node mask. 0 means single GPU operation
    pub node_mask: u32,
}

impl From<CommandQueueDesc> for ::winapi::D3D12_COMMAND_QUEUE_DESC {
    #[inline]
    fn from(desc: CommandQueueDesc) -> Self {
        unsafe { ::std::mem::transmute(desc) }
    }
}

impl Default for CommandQueueDesc {
    #[inline]
    fn default() -> CommandQueueDesc {
        CommandQueueDesc{
            list_type: COMMAND_LIST_TYPE_DIRECT,
            priority: COMMAND_QUEUE_PRIOIRITY_NORMAL,
            flags: COMMAND_QUEUE_FLAG_NONE,
            node_mask: 0,
        }
    }
}

// TODO: double check types maybe?
bitflags!{
    /// priority levels for a command queue
    #[repr(C)]
    pub struct CommandQueuePriority: u32 {
        const COMMAND_QUEUE_PRIOIRITY_NORMAL = 0;
        const COMMAND_QUEUE_PRIORITY_HIGH = 100;
        const COMMAND_QUEUE_PRIOIRITY_GLOBAL_REALTIME = 1000;
    }
}

impl Default for CommandQueuePriority {
    #[inline]
    fn default() -> Self {
        COMMAND_QUEUE_PRIOIRITY_NORMAL
    }
}

bitflags!{
    /// misc flags for a command queue
    #[repr(C)]
    pub struct CommandQueueFlags: u32 {
        const COMMAND_QUEUE_FLAG_NONE = 0;
        const COMMAND_QUEUE_FLAG_DISABLE_GPU_TIMEOUT = 0x1;
    }
}

impl Default for CommandQueueFlags {
    #[inline]
    fn default() -> Self {
        COMMAND_QUEUE_FLAG_NONE
    }
}
