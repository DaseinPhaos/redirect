// Copyright 2017 Dasein Phaos aka. Luxko
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! resource

macro_rules! impl_as_raw {
    ($Trait: ident, $Type: ident, $Raw: ident) => {
        impl $Trait for $Type {
            #[inline]
            fn as_raw(&self) -> & $Raw {
                &self.raw
            }

            #[inline]
            fn as_raw_mut(&mut self) ->&mut $Raw {
                &mut self.raw
            }
        }
    }
}

pub mod usage;
pub use self::usage::*;

pub mod description;
pub use self::description::*;

pub mod heap;
pub use self::heap::*;

pub mod raw;
pub use self::raw::*;

pub mod barrier;
pub use self::barrier::*;

pub mod state;
pub use self::state::*;

pub mod traits;
pub use self::traits::*;

pub mod buffer;
pub use self::buffer::*;

use format::*;

// TODO: find out a sound way to work with different types of resources

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct ResourceAllocInfo {
    /// consumed size of the resource on paged heap
    pub size: u64,
    pub alignment: ResourceAlignment,
}

/// describes a resource used for GPU texture copying
#[derive(Copy, Clone, Debug)]
pub struct TextureCopyLocation {
    ptr: *mut ::winapi::ID3D12Resource,
    pub copy_type: TextureCopyType,
}

impl From<TextureCopyLocation> for ::winapi::D3D12_TEXTURE_COPY_LOCATION {
    #[inline]
    fn from(loc: TextureCopyLocation) -> Self {
        unsafe {
            let mut ret: Self = ::std::mem::uninitialized();
            ret.pResource = loc.ptr;
            match loc.copy_type {
                TextureCopyType::SubresourceIndex(idx) => {
                    ret.Type = ::winapi::D3D12_TEXTURE_COPY_TYPE_SUBRESOURCE_INDEX;
                    ret.u = ::std::mem::transmute_copy(&idx);
                },
                TextureCopyType::PlacedFootprint(footprint) => {
                    ret.Type = ::winapi::D3D12_TEXTURE_COPY_TYPE_PLACED_FOOTPRINT;
                    ret.u = ::std::mem::transmute(footprint);
                },
            }

            ret
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum TextureCopyType {
    SubresourceIndex(u32),
    PlacedFootprint(PlacedSubresourceFootprint),
}

/// [more info](https://msdn.microsoft.com/library/windows/desktop/dn986749(v=vs.85).aspx)
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct PlacedSubresourceFootprint {
    /// offset within the parent resource
    pub offset: u64,
    pub format: DxgiFormat,
    pub width: u32,
    pub height: u32,
    pub depth: u32,
    pub row_pitch: u32,
}

// TODO: reserved resource?
