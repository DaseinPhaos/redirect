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
use resource::{RawResource, ResourceBarriersBuilder, TextureCopyLocation};

mod common;
pub use self::common::*;

mod direct;
pub use self::direct::*;

mod bundle;
pub use self::bundle::*;

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
