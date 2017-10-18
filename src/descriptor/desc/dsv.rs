// Copyright 2017 Dasein Phaos aka. Luxko
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! depth stencil view description

use format::DxgiFormat;

/// describes a depth stencil view
#[derive(Copy, Clone, Debug)]
pub struct DsvDesc {
    pub format: DxgiFormat,
    pub flags: DsvFlags,
    pub dimension: DsvDimension,
}

impl DsvDesc {
    #[inline]
    pub(crate) fn into_cstruct(self) -> DsvDescBindHelper {
        self.into()
    }
}

#[derive(Copy, Clone, Debug)]
pub enum DsvDimension {
    Tex1D(DsvTex1DDesc),
    Tex1DArray(DsvTex1DArrayDesc),
    Tex2D(DsvTex2DDesc),
    Tex2DArray(DsvTex2DArrayDesc),
    Tex2DMs,
    Tex2DMsArray(DsvTex2DMsArrayDesc),
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DsvTex1DDesc {
    /// index of the mipmap slice to use
    pub mip_slice: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DsvTex1DArrayDesc{
    /// index of the mipmap slice to use
    pub mip_slice: u32,
    /// first array slice to use
    pub first_slice: u32,
    /// number of slices in the array
    pub array_size: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DsvTex2DDesc{
    /// index of the mipmap slice to use
    pub mip_slice: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DsvTex2DArrayDesc{
    /// index of the mipmap slice to use
    pub mip_slice: u32,
    /// first array slice to use
    pub first_slice: u32,
    /// number of slices in the array
    pub array_size: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DsvTex2DMsArrayDesc{
    /// first array slice to use
    pub first_slice: u32,
    /// number of slices in the array
    pub array_size: u32,
}

bitflags!{
    /// dsv misc flags. specifying read only would allow more than one dsv to be
    /// bound to the pipeline (on a same resource) simultaneously
    #[repr(C)]
    pub struct DsvFlags: u32 {
        const NONE = 0;
        const READ_ONLY_DEPTH = 0x1;
        const READ_ONLY_STENCIL = 0x2;
    }
}

impl Default for DsvFlags {
    #[inline]
    fn default() -> Self {
        DsvFlags::NONE
    }
}

/// helper struct for ffi, not intended for application user
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub(crate) struct DsvDescBindHelper {
    format: DxgiFormat,
    view_dimension: ::winapi::D3D12_DSV_DIMENSION,
    flags: DsvFlags,
    a: [u32; 3],
}

impl From<DsvDesc> for DsvDescBindHelper{
    #[inline]
    fn from(desc: DsvDesc) -> DsvDescBindHelper {
        unsafe {
            let mut ret: DsvDescBindHelper = ::std::mem::zeroed();
            ret.format = desc.format;
            ret.flags = desc.flags;
            match desc.dimension {
                DsvDimension::Tex1D(content) => {
                    ret.view_dimension = ::winapi::D3D12_DSV_DIMENSION_TEXTURE1D;
                    ret.a = ::std::mem::transmute_copy(&content);
                },
                DsvDimension::Tex1DArray(content) => {
                    ret.view_dimension = ::winapi::D3D12_DSV_DIMENSION_TEXTURE1DARRAY;
                    ret.a = ::std::mem::transmute_copy(&content);
                },
                DsvDimension::Tex2D(content) => {
                    ret.view_dimension = ::winapi::D3D12_DSV_DIMENSION_TEXTURE2D;
                    ret.a = ::std::mem::transmute_copy(&content);
                },
                DsvDimension::Tex2DArray(content) => {
                    ret.view_dimension = ::winapi::D3D12_DSV_DIMENSION_TEXTURE2DARRAY;
                    ret.a = ::std::mem::transmute_copy(&content);
                },
                DsvDimension::Tex2DMs =>
                    ret.view_dimension = ::winapi::D3D12_DSV_DIMENSION_TEXTURE2DMS,
                DsvDimension::Tex2DMsArray(content) => {
                    ret.view_dimension = ::winapi::D3D12_DSV_DIMENSION_TEXTURE2DMSARRAY;
                    ret.a = ::std::mem::transmute_copy(&content);
                },
            }
            ret
        }
    }
}
