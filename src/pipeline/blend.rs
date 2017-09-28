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
            src_blend: BLEND_ONE,
            dst_blend: BLEND_ZERO,
            blend_op: BLEND_OP_ADD,
            src_alpha: BLEND_ONE,
            dst_alpha: BLEND_ZERO,
            blend_op_alpha: BLEND_OP_ADD,
            logic_op: LOGIC_OP_NOOP,
            write_mask: COLOR_WRITE_ENABLE_ALL,
        }
    }
}

bitflags!{
    /// a blend factor
    #[repr(C)]
    pub struct Blend: u32 {
        const BLEND_ZERO              = 1;
        const BLEND_ONE               = 2;
        const BLEND_SRC_COLOR         = 3;
        const BLEND_INV_SRC_COLOR     = 4;
        const BLEND_SRC_ALPHA         = 5;
        const BLEND_INV_SRC_ALPHA     = 6;
        const BLEND_DEST_ALPHA        = 7;
        const BLEND_INV_DEST_ALPHA    = 8;
        const BLEND_DEST_COLOR        = 9;
        const BLEND_INV_DEST_COLOR    = 10;
        const BLEND_SRC_ALPHA_SAT     = 11;
        const BLEND_BLEND_FACTOR      = 14;
        const BLEND_INV_BLEND_FACTOR  = 15;
        const BLEND_SRC1_COLOR        = 16;
        const BLEND_INV_SRC1_COLOR    = 17;
        const BLEND_SRC1_ALPHA        = 18;
        const BLEND_INV_SRC1_ALPHA    = 19;
    }
}

bitflags!{
    /// a blend operation
    #[repr(C)]
    pub struct BlendOp: u32 {
        const BLEND_OP_ADD           = 1;
        const BLEND_OP_SUBTRACT      = 2;
        const BLEND_OP_REV_SUBTRACT  = 3;
        const BLEND_OP_MIN           = 4;
        const BLEND_OP_MAX           = 5;
    }
}

bitflags!{
    /// a logic operation
    #[repr(C)]
    pub struct LogicOp: u32 {
        const LOGIC_OP_CLEAR          = 0;
        const LOGIC_OP_SET            = LOGIC_OP_CLEAR.bits +1;
        const LOGIC_OP_COPY           = LOGIC_OP_SET.bits +1;
        const LOGIC_OP_COPY_INVERTED  = LOGIC_OP_COPY.bits +1;
        const LOGIC_OP_NOOP           = LOGIC_OP_COPY_INVERTED.bits +1;
        const LOGIC_OP_INVERT         = LOGIC_OP_NOOP.bits +1;
        const LOGIC_OP_AND            = LOGIC_OP_INVERT.bits +1;
        const LOGIC_OP_NAND           = LOGIC_OP_AND.bits +1;
        const LOGIC_OP_OR             = LOGIC_OP_NAND.bits +1;
        const LOGIC_OP_NOR            = LOGIC_OP_OR.bits +1;
        const LOGIC_OP_XOR            = LOGIC_OP_NOR.bits +1;
        const LOGIC_OP_EQUIV          = LOGIC_OP_XOR.bits +1;
        const LOGIC_OP_AND_REVERSE    = LOGIC_OP_EQUIV.bits +1;
        const LOGIC_OP_AND_INVERTED   = LOGIC_OP_AND_REVERSE.bits +1;
        const LOGIC_OP_OR_REVERSE     = LOGIC_OP_AND_INVERTED.bits +1;
        const LOGIC_OP_OR_INVERTED    = LOGIC_OP_OR_REVERSE.bits +1;
    }
}

bitflags!{
    /// color write mask of a blend operation
    #[repr(C)]
    pub struct ColorWriteMask: u8 {
        const COLOR_WRITE_ENABLE_RED = 1;
        const COLOR_WRITE_ENABLE_GREEN = 2;
        const COLOR_WRITE_ENABLE_BLUE = 4;
        const COLOR_WRITE_ENABLE_ALPHA = 8;
        const COLOR_WRITE_ENABLE_ALL = COLOR_WRITE_ENABLE_RED.bits |
                                        COLOR_WRITE_ENABLE_GREEN.bits |
                                        COLOR_WRITE_ENABLE_BLUE.bits |
                                        COLOR_WRITE_ENABLE_ALPHA.bits;
    }
}
