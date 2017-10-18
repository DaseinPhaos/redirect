// Copyright 2017 Dasein Phaos aka. Luxko
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! blend state

use format::Bool;

/// describes the blend state for the whole pipeline
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct BlendDesc {
    /// whether to use [alpha-to-coverage](https://msdn.microsoft.com/library/windows/desktop/bb205072(v=vs.85).aspx#Alpha_To_Coverage) as a multisampling technique
    pub alpha_to_coverage: Bool,
    /// whether to enable independent blending in simultaneous rendering targets
    pub independent_blend: Bool,
    /// array of render targets
    pub render_targets: [RenderTargetBlendDesc; 8],
}

impl Default for BlendDesc {
    fn default() -> Self {
        BlendDesc{
            alpha_to_coverage: false.into(),
            independent_blend: false.into(),
            render_targets: [Default::default(); 8],
        }
    }
}

/// describes the blend state for a render target
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct RenderTargetBlendDesc {
    pub blend_enabled: Bool,
    pub logic_op_enabled: Bool,
    pub src_blend: Blend,
    pub dst_blend: Blend,
    pub blend_op: BlendOp,
    pub src_alpha: Blend,
    pub dst_alpha: Blend,
    pub blend_op_alpha: BlendOp,
    pub logic_op: LogicOp,
    pub write_mask: ColorWriteMask,
}

impl RenderTargetBlendDesc {
    #[inline]
    pub fn new() -> Self {
        Default::default()
    }

    #[inline]
    pub fn set_blend(&mut self, src: Blend, dst: Blend, op: BlendOp) {
        self.blend_enabled = true.into();
        self.src_blend = src;
        self.dst_blend = dst;
        self.blend_op = op;
    }

    #[inline]
    pub fn set_blend_alpha(&mut self, src: Blend, dst: Blend, op: BlendOp) {
        self.blend_enabled = true.into();
        self.src_alpha = src;
        self.dst_alpha = dst;
        self.blend_op_alpha = op;
    }
}

impl Default for RenderTargetBlendDesc {
    fn default() -> Self {
        RenderTargetBlendDesc{
            blend_enabled: false.into(),
            logic_op_enabled: false.into(),
            src_blend: Blend::ONE,
            dst_blend: Blend::ZERO,
            blend_op: BlendOp::ADD,
            src_alpha: Blend::ONE,
            dst_alpha: Blend::ZERO,
            blend_op_alpha: BlendOp::ADD,
            logic_op: LogicOp::NOOP,
            write_mask: ColorWriteMask::ENABLE_ALL,
        }
    }
}

bitflags!{
    /// a blend factor
    #[repr(C)]
    pub struct Blend: u32 {
        const ZERO              = 1;
        const ONE               = 2;
        const SRC_COLOR         = 3;
        const INV_SRC_COLOR     = 4;
        const SRC_ALPHA         = 5;
        const INV_SRC_ALPHA     = 6;
        const DEST_ALPHA        = 7;
        const INV_DEST_ALPHA    = 8;
        const DEST_COLOR        = 9;
        const INV_DEST_COLOR    = 10;
        const SRC_ALPHA_SAT     = 11;
        const FACTOR      = 14;
        const INV_FACTOR  = 15;
        const SRC1_COLOR        = 16;
        const INV_SRC1_COLOR    = 17;
        const SRC1_ALPHA        = 18;
        const INV_SRC1_ALPHA    = 19;
    }
}

bitflags!{
    /// a blend operation
    #[repr(C)]
    pub struct BlendOp: u32 {
        const ADD           = 1;
        const SUBTRACT      = 2;
        const REV_SUBTRACT  = 3;
        const MIN           = 4;
        const MAX           = 5;
    }
}

bitflags!{
    /// a logic operation
    #[repr(C)]
    pub struct LogicOp: u32 {
        const CLEAR          = 0;
        const SET            = LogicOp::CLEAR.bits +1;
        const COPY           = LogicOp::SET.bits +1;
        const COPY_INVERTED  = LogicOp::COPY.bits +1;
        const NOOP           = LogicOp::COPY_INVERTED.bits +1;
        const INVERT         = LogicOp::NOOP.bits +1;
        const AND            = LogicOp::INVERT.bits +1;
        const NAND           = LogicOp::AND.bits +1;
        const OR             = LogicOp::NAND.bits +1;
        const NOR            = LogicOp::OR.bits +1;
        const XOR            = LogicOp::NOR.bits +1;
        const EQUIV          = LogicOp::XOR.bits +1;
        const AND_REVERSE    = LogicOp::EQUIV.bits +1;
        const AND_INVERTED   = LogicOp::AND_REVERSE.bits +1;
        const OR_REVERSE     = LogicOp::AND_INVERTED.bits +1;
        const OR_INVERTED    = LogicOp::OR_REVERSE.bits +1;
    }
}

bitflags!{
    /// color write mask of a blend operation
    #[repr(C)]
    pub struct ColorWriteMask: u8 {
        const ENABLE_RED = 1;
        const ENABLE_GREEN = 2;
        const ENABLE_BLUE = 4;
        const ENABLE_ALPHA = 8;
        const ENABLE_ALL = ColorWriteMask::ENABLE_RED.bits |
                           ColorWriteMask::ENABLE_GREEN.bits |
                           ColorWriteMask::ENABLE_BLUE.bits |
                           ColorWriteMask::ENABLE_ALPHA.bits;
    }
}
