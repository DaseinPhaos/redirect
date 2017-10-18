// Copyright 2017 Dasein Phaos aka. Luxko
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! depth stencil states
use super::*;
use format::Bool;

/// describes the depth-stencil state
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DepthStencilDesc{
    pub depth: Bool,
    pub depth_write_mask: DepthWriteMask,
    pub depth_func: ComparisonFunc,
    pub stencil: Bool,
    pub stencil_read_mask: u8,
    pub stencil_write_mask: u8,
    pub front_op: StencilOpDesc,
    pub back_op: StencilOpDesc,
}

impl Default for DepthStencilDesc {
    #[inline]
    fn default() -> DepthStencilDesc{
        DepthStencilDesc{
            depth: true.into(),
            depth_write_mask: DepthWriteMask::ALL,
            depth_func: ComparisonFunc::LESS,
            stencil: false.into(),
            stencil_read_mask: 0xff,
            stencil_write_mask: 0xff,
            front_op: Default::default(),
            back_op: Default::default(),
        }
    }
}

/// operations based on the stencil test
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct StencilOpDesc {
    /// operation when stencil test fails
    pub fail: StencilOp,
    /// operation when stencil test pass, but depth test fails
    pub depth_fail: StencilOp,
    /// operation when both test pass
    pub pass: StencilOp,
    /// stencil test func
    pub func: ComparisonFunc,
}

impl Default for StencilOpDesc{
    fn default() -> StencilOpDesc{
        StencilOpDesc{
            fail: StencilOp::KEEP,
            depth_fail: StencilOp::KEEP,
            pass: StencilOp::KEEP,
            func: ComparisonFunc::ALWAYS,
        }
    }
}

bitflags!{
    #[repr(C)]
    pub struct DepthWriteMask: u32 {
        const ZERO = 0;
        const ALL = 1;
    }
}

bitflags!{
    #[repr(C)]
    pub struct StencilOp: u32 {
        const KEEP      = 1;
        const ZERO      = 2;
        const REPLACE   = 3;
        /// increment and clamp
        const INCR_SAT  = 4;
        /// decrement and clamp
        const DECR_SAT  = 5;
        const INVERT    = 6;
        /// increment and wrap
        const INCR      = 7;
        /// decrement and wrap
        const DECR      = 8;
    }
}
