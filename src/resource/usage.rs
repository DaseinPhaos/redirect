// Copyright 2017 Dasein Phaos aka. Luxko
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! flags for resource usage and access patterns

bitflags! {
    /// resource usage flags
    #[repr(C)]
    pub struct Usage: u32 {
        /// cpu don't have access to this resource
        const CPU_NONE = 0;
        /// cpu have write only access to this resource 
        const CPU_DYNAMIC = 1;
        /// cpu can read or write this resource
        const CPU_READ_WRITE = 2;
        /// FIXME: wth?
        const CPU_SCRATCH = 3;
        /// resource can be used as shader input
        const SHADER_INPUT = 1<<(0+4);
        /// resource can be used as render target output
        const RENDER_TARGET_OUTPUT = 1<<(1+4);
        /// resource is used as a back buffer. This flag don't need to be passed when creating a swapchain.
        const BACK_BUFFER = 1<<(2+4);
        /// FIXME: resource can be shared across different adapters?
        const SHARED = 1<<(3+4);
        /// resource is read only for the gpu
        const READ_ONLY = 1<<(4+4);
        /// FIXME: resource content might be discarded after present?
        const DISCARD_ON_PRESENT = 1<<(5+4);
        /// resource can be unordered accessed
        const UNORDERED_ACCESS = 1<<(6+4);
    }
}

impl From<Usage> for ::winapi::DXGI_USAGE {
    fn from(usage: Usage) -> Self {
        ::winapi::DXGI_USAGE(usage.bits())
    }
}
