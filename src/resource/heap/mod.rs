// Copyright 2017 Dasein Phaos aka. Luxko
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! a continous GPU memory region.

pub mod description;
pub use self::description::*;

pub mod raw;
pub use self::raw::*;

pub mod traits;
pub use self::traits::*;

use device::Device;
use error::WinError;

/// a safe heap with all properties set to default
#[derive(Debug, Clone)]
pub struct DefaultHeap {
    pub(crate) raw: RawHeap
}

impl DefaultHeap {
    #[inline]
    pub fn new(device: &mut Device, size: u64) -> Result<Self, WinError> {
        let desc = HeapDesc::new(
            size,
            HeapProperties::new(HeapType::DEFAULT),
            Default::default()
        );

        let raw = device.create_heap(&desc)?;
        Ok(DefaultHeap{raw})
    }
}

/// an upload heap with all properties set to default
#[derive(Debug, Clone)]
pub struct UploadHeap {
    pub(crate) raw: RawHeap
}

impl UploadHeap {
    #[inline]
    pub fn new(device: &mut Device, size: u64) -> Result<Self, WinError> {
        let desc = HeapDesc::new(
            size,
            HeapProperties::new(HeapType::UPLOAD),
            Default::default()
        );

        let raw = device.create_heap(&desc)?;
        Ok(UploadHeap{raw})
    }
}

/// an readback heap with all properties set to default
#[derive(Debug, Clone)]
pub struct ReadbackHeap {
    pub(crate) raw: RawHeap
}

impl ReadbackHeap {
    #[inline]
    pub fn new(device: &mut Device, size: u64) -> Result<Self, WinError> {
        let desc = HeapDesc::new(
            size,
            HeapProperties::new(HeapType::READBACK),
            Default::default()
        );

        let raw = device.create_heap(&desc)?;
        Ok(ReadbackHeap{raw})
    }
}
