// Copyright 2017 Dasein Phaos aka. Luxko
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! resource state


bitflags!{
    /// the state of a resource regarding how it is being used. [more](https://msdn.microsoft.com/library/windows/desktop/dn986744%28v=vs.85%29.aspx?f=255&MSPPError=-2147217396)
    #[repr(C)]
    pub struct ResourceStates: u32 {
        /// resource should be in this state when
        ///
        /// - being translated across `COPY` queue to/from `DIRECT/COMPUTE` queues
        /// - for CPU accessing
        const COMMON                      = 0;
        /// a subresource should be in this state when accessed as a vertex buffer or constant buffer
        const VERTEX_AND_CONSTANT_BUFFER  = 0x1;
        /// a subresource should be in this state when accessed as a index buffer
        const INDEX_BUFFER                = 0x2;
        /// a subresource should be in this state when used as a render target
        const RENDER_TARGET               = 0x4;
        /// a subresource should be in this state when accessed via an UAV.
        /// when in this state, a resource can be accessed for RW from multiple
        /// command queues simultaneously.
        const UNORDERED_ACCESS            = 0x8;
        /// a subresource should be in this state when used for depth write. mutual exclusive
        const DEPTH_WRITE                 = 0x10;
        /// a subresource should be in this state when used for depth read.
        const DEPTH_READ                  = 0x20;
        /// a subresource should be in this state when accessed as a SRV from any stage other than PS
        const NON_PIXEL_SHADER_RESOURCE   = 0x40;
        /// a subresource should be in this state when accessed as a SRV from PS
        const PIXEL_SHADER_RESOURCE       = 0x80;
        /// a subresource is used with stream output
        const STREAM_OUT                  = 0x100;
        /// the resource is used as indirect argument
        const INDIRECT_ARGUMENT           = 0x200;
        /// used as the destination in a copy operation
        const COPY_DEST                   = 0x400;
        /// used as the src in a copy operation
        const COPY_SOURCE                 = 0x800;
        /// used as the destination in a resolve operation
        const RESOLVE_DEST                = 0x1000;
        /// used as the src in a resolve operation
        const RESOLVE_SOURCE              = 0x2000;
        /// required starting state for upload heaps.
        /// when in this state, a resource can be accessed for reading from
        /// multiple command queues simultaneously.
        const GENERIC_READ = ((((0x1|0x2)|0x40)|0x80)|0x200)|0x800;
        /// alias for `COMMON`
        const PRESENT                     = 0;
        /// used for [predication](https://msdn.microsoft.com/library/windows/desktop/dn903927(v=vs.85).aspx)
        const PREDICATION                 = 0x200;
    }
}

impl Default for ResourceStates {
    #[inline]
    fn default() -> Self {
        ResourceStates::GENERIC_READ
    }
}
