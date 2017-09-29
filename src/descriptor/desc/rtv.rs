// Copyright 2017 Dasein Phaos aka. Luxko
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! render target view description

use format::DxgiFormat;

/// describes a render target view
#[derive(Copy, Clone, Debug)]
pub struct RtvDesc {
    pub format: DxgiFormat,
    pub dimension: RtvDimension,
}

impl RtvDesc {
    #[inline]
    pub(crate) fn into_cstruct(self) -> RtvDescBindHelper {
        self.into()
    }
}

#[derive(Copy, Clone, Debug)]
pub enum RtvDimension {
    Buffer(RtvBufferDesc),
    Tex1D(RtvTex1DDesc),
    Tex1DArray(RtvTex1DArrayDesc),
    Tex2D(RtvTex2DDesc),
    Tex2DArray(RtvTex2DArrayDesc),
    Tex2DMs,
    Tex2DMsArray(RtvTex2DMsArrayDesc),
    Tex3D(RtvTex3DDesc),
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct RtvBufferDesc {
    /// first element to be accessed by the view
    pub offset: u64,
    /// number of elements
    pub num_elements: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct RtvTex1DDesc {
    /// index of the mipmap slice to use
    pub mip_slice: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct RtvTex1DArrayDesc{
    /// index of the mipmap slice to use
    pub mip_slice: u32,
    /// first array slice to use
    pub first_slice: u32,
    /// number of slices in the array
    pub array_size: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct RtvTex2DDesc{
    /// index of the mipmap slice to use
    pub mip_slice: u32,
    /// index of the plane slice to use
    pub plane_slice: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct RtvTex2DArrayDesc{
    /// index of the mipmap slice to use
    pub mip_slice: u32,
    /// first array slice to use
    pub first_slice: u32,
    /// number of slices in the array
    pub array_size: u32,
    /// index of the plane slice to use
    pub plane_slice: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct RtvTex2DMsArrayDesc{
    /// first array slice to use
    pub first_slice: u32,
    /// number of slices in the array
    pub array_size: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct RtvTex3DDesc{
    /// index of the mipmap slice to use
    pub mip_slice: u32,
    /// first depth level to use
    pub first_slice: u32,
    /// number of depth levels in the array
    pub num_slices: u32,
}

/// helper struct for ffi, not intended for application user
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub(crate) struct RtvDescBindHelper {
    format: DxgiFormat,
    view_dimension: ::winapi::D3D12_RTV_DIMENSION,
    a: [u32; 4],
}

impl From<RtvDesc> for RtvDescBindHelper{
    #[inline]
    fn from(desc: RtvDesc) -> RtvDescBindHelper {
        unsafe {
            let mut ret: RtvDescBindHelper = ::std::mem::zeroed();
            ret.format = desc.format;
            match desc.dimension {
                RtvDimension::Buffer(content) => {
                    ret.view_dimension = ::winapi::D3D12_RTV_DIMENSION_BUFFER;
                    ret.a = ::std::mem::transmute_copy(&content);
                },
                RtvDimension::Tex1D(content) => {
                    ret.view_dimension = ::winapi::D3D12_RTV_DIMENSION_TEXTURE1D;
                    ret.a = ::std::mem::transmute_copy(&content);
                },
                RtvDimension::Tex1DArray(content) => {
                    ret.view_dimension = ::winapi::D3D12_RTV_DIMENSION_TEXTURE1DARRAY;
                    ret.a = ::std::mem::transmute_copy(&content);
                },
                RtvDimension::Tex2D(content) => {
                    ret.view_dimension = ::winapi::D3D12_RTV_DIMENSION_TEXTURE2D;
                    ret.a = ::std::mem::transmute_copy(&content);
                },
                RtvDimension::Tex2DArray(content) => {
                    ret.view_dimension = ::winapi::D3D12_RTV_DIMENSION_TEXTURE2DARRAY;
                    ret.a = ::std::mem::transmute_copy(&content);
                },
                RtvDimension::Tex2DMs =>
                    ret.view_dimension = ::winapi::D3D12_RTV_DIMENSION_TEXTURE2DMS,
                RtvDimension::Tex2DMsArray(content) => {
                    ret.view_dimension = ::winapi::D3D12_RTV_DIMENSION_TEXTURE2DMSARRAY;
                    ret.a = ::std::mem::transmute_copy(&content);
                },
                RtvDimension::Tex3D(content) => {
                    ret.view_dimension = ::winapi::D3D12_RTV_DIMENSION_TEXTURE3D;
                    ret.a = ::std::mem::transmute_copy(&content);
                },
            }
            ret
        }
    }
}
