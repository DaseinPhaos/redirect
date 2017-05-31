// Copyright 2017 Dasein Phaos aka. Luxko
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! GPU command encapsulation.

use comptr::ComPtr;
use winapi::ID3D12CommandAllocator;
use error::WinError;

mod queue;
pub use self::queue::*;

mod list;
pub use self::list::*;

/// An allocator for GPU commands
#[derive(Debug, Clone)]
pub struct CommandAllocator {
    pub ptr: ComPtr<ID3D12CommandAllocator>,
}

impl CommandAllocator {
    /// indicates that the associated memory would be recycled by the allocator.
    #[inline]
    pub fn reset(&mut self) -> Result<(), WinError> {
        let hr = unsafe {self.ptr.Reset()};
        WinError::from_hresult(hr)
    }
}
