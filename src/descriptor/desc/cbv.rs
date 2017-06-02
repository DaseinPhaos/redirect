// Copyright 2017 Dasein Phaos aka. Luxko
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! constant buffer view description

use resource::GpuVAddress;

/// describes constant buffer view
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct CbvDesc {
    /// the gpu virtual address of the viewed virtual buffer
    pub buffer_location: GpuVAddress,
    /// size of the viewed buffer in bytes
    pub size: u32,
}
