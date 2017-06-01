// Copyright 2017 Dasein Phaos aka. Luxko
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use comptr::ComPtr;
use winapi::{ID3D12CommandList, ID3D12GraphicsCommandList};
use error::WinError;

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
