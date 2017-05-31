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

/// A GPU command queue, providing methods for command submission,
/// execution synchronization, etc.
#[derive(Debug, Clone)]
pub struct CommandQueue {
    pub ptr: ComPtr<ID3D12CommandQueue>,
}

impl CommandQueue {
    // TODO: add method for `CopyTileMapping, UpdateTileMappings`, block on resources

    // TODO: add method for command list execution

    // TODO: add method for time stamp calibration

    // TODO: add method for PIX events?

    // TODO: add method for `Signal, Wait`

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
