// Copyright 2017 Dasein Phaos aka. Luxko
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! describes an unordered access view

use format::DxgiFormat;

/// describes an unordered access view
#[derive(Copy, Clone, Debug)]
pub struct UavDesc {
    pub format: DxgiFormat,
    pub dimension: UavDimension,
}

impl UavDesc {
    #[inline]
    pub fn into_cstruct(self) -> UavDescBindHelper {
        self.into()
    }
}

#[derive(Copy, Clone, Debug)]
pub enum UavDimension {
    Unknown,
    Buffer(UavBufferDesc),
    Tex1D(UavTex1DDesc),
    Tex1DArray(UavTex1DArrayDesc),
    Tex2D(UavTex2DDesc),
    Tex2DArray(UavTex2DArrayDesc),
    Tex3D(UavTex3DDesc),
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct UavBufferDesc {
    /// first element to be accessed by the view
    pub offset: u64,
    /// number of elements
    pub num_elements: u32,
    /// size of each element in the buffer
    pub byte_stride: u32,
    /// counter offset, if no counter presented, set zero
    pub counter_offset: u64,
    /// whether to view it as a raw buffer, 1 means raw, 0 means not
    pub raw: u32,

}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct UavTex1DDesc {
    /// mipmap slice to use
    pub mip_slice: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct UavTex1DArrayDesc{
    /// mipmap slice to use
    pub mip_slice: u32,
    /// first array slice to use
    pub first_slice: u32,
    /// number of slices in the array
    pub array_size: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct UavTex2DDesc{
    /// mipmap slice to use
    pub mip_slice: u32,
    /// index of the plane slice to use
    pub plane_slice: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct UavTex2DArrayDesc{
    /// mipmap slice to use
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
pub struct UavTex3DDesc{
    /// index of the mipmap slice to use
    pub mip_slice: u32,
    /// first depth level to use
    pub first_slice: u32,
    /// number of depth levels in the array
    pub num_slices: u32,
}

/// helper struct for ffi, not intended for application user
/// TODO: remove from public interface
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct UavDescBindHelper {
    format: DxgiFormat,
    view_dimension: ::winapi::D3D12_UAV_DIMENSION,
    component_mapping: u32,
    a: UavBufferDesc,
}

impl From<UavDesc> for UavDescBindHelper{
    #[inline]
    fn from(desc: UavDesc) -> UavDescBindHelper {
        unsafe {
            let mut ret: UavDescBindHelper = ::std::mem::zeroed();
            ret.format = desc.format;
            match desc.dimension {
                UavDimension::Unknown =>
                    ret.view_dimension = ::winapi::D3D12_UAV_DIMENSION_UNKNOWN,
                UavDimension::Buffer(content) => {
                    ret.view_dimension = ::winapi::D3D12_UAV_DIMENSION_BUFFER;
                    ret.a = ::std::mem::transmute_copy(&content);
                },
                UavDimension::Tex1D(content) => {
                    ret.view_dimension = ::winapi::D3D12_UAV_DIMENSION_TEXTURE1D;
                    ret.a = ::std::mem::transmute_copy(&content);
                },
                UavDimension::Tex1DArray(content) => {
                    ret.view_dimension = ::winapi::D3D12_UAV_DIMENSION_TEXTURE1DARRAY;
                    ret.a = ::std::mem::transmute_copy(&content);
                },
                UavDimension::Tex2D(content) => {
                    ret.view_dimension = ::winapi::D3D12_UAV_DIMENSION_TEXTURE2D;
                    ret.a = ::std::mem::transmute_copy(&content);
                },
                UavDimension::Tex2DArray(content) => {
                    ret.view_dimension = ::winapi::D3D12_UAV_DIMENSION_TEXTURE2DARRAY;
                    ret.a = ::std::mem::transmute_copy(&content);
                },
                UavDimension::Tex3D(content) => {
                    ret.view_dimension = ::winapi::D3D12_UAV_DIMENSION_TEXTURE3D;
                    ret.a = ::std::mem::transmute_copy(&content);
                },
            }
            ret
        }
    }
}
