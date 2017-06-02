// Copyright 2017 Dasein Phaos aka. Luxko
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! describes a shader resource view

use format::DxgiFormat;
use super::Shader4ComponentMapping;

/// describes a shader resource view
#[derive(Copy, Clone, Debug)]
pub struct SrvDesc {
    pub format: DxgiFormat,
    pub dimension: SrvDimension,
    pub component_mapping: Shader4ComponentMapping,
}

impl SrvDesc {
    #[inline]
    pub fn into_cstruct(self) -> SrvDescBindHelper {
        self.into()
    }
}

#[derive(Copy, Clone, Debug)]
pub enum SrvDimension {
    Unknown,
    Buffer(SrvBufferDesc),
    Tex1D(SrvTex1DDesc),
    Tex1DArray(SrvTex1DArrayDesc),
    Tex2D(SrvTex2DDesc),
    Tex2DArray(SrvTex2DArrayDesc),
    Tex2DMs,
    Tex2DMsArray(SrvTex2DMsArrayDesc),
    Tex3D(SrvTex3DDesc),
    TexCube(SrvTexCubeDesc),
    TexCubeArray(SrvTexCubeArrayDesc),
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct SrvBufferDesc {
    /// first element to be accessed by the view
    pub offset: u64,
    /// number of elements
    pub num_elements: u32,
    /// size of each element in the buffer
    pub byte_stride: u32,
    /// whether to view it as a raw buffer, 1 means raw, 0 means not
    pub raw: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct SrvTex1DDesc {
    /// index of the most detailed mipmap to use
    pub most_detailed_mip: u32,
    /// levels of mipmap to use, `-1` means up until the least detailed
    pub mip_levels: i32,
    /// minimum sampled lod clamp value
    pub mip_lod_clamp: f32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct SrvTex1DArrayDesc{
    /// index of the most detailed mipmap to use
    pub most_detailed_mip: u32,
    /// levels of mipmap to use, `-1` means up until the least detailed
    pub mip_levels: i32,
    /// first array slice to use
    pub first_slice: u32,
    /// number of slices in the array
    pub array_size: u32,
    /// minimum sampled lod clamp value
    pub mip_lod_clamp: f32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct SrvTex2DDesc{
    /// index of the most detailed mipmap to use
    pub most_detailed_mip: u32,
    /// levels of mipmap to use, `-1` means up until the least detailed
    pub mip_levels: i32,
    /// index of the plane slice to use
    pub plane_slice: u32,
    /// minimum sampled lod clamp value
    pub mip_lod_clamp: f32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct SrvTex2DArrayDesc{
    /// index of the most detailed mipmap to use
    pub most_detailed_mip: u32,
    /// levels of mipmap to use, `-1` means up until the least detailed
    pub mip_levels: i32,
    /// first array slice to use
    pub first_slice: u32,
    /// number of slices in the array
    pub array_size: u32,
    /// index of the plane slice to use
    pub plane_slice: u32,
    /// minimum sampled lod clamp value
    pub mip_lod_clamp: f32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct SrvTex2DMsArrayDesc{
    /// first array slice to use
    pub first_slice: u32,
    /// number of slices in the array
    pub array_size: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct SrvTex3DDesc{
    /// index of the most detailed mipmap to use
    pub most_detailed_mip: u32,
    /// levels of mipmap to use, `-1` means up until the least detailed
    pub mip_levels: i32,
    /// minimum sampled lod clamp value
    pub mip_lod_clamp: f32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct SrvTexCubeDesc{
    /// index of the most detailed mipmap to use
    pub most_detailed_mip: u32,
    /// levels of mipmap to use, `-1` means up until the least detailed
    pub mip_levels: i32,
    /// minimum sampled lod clamp value
    pub mip_lod_clamp: f32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct SrvTexCubeArrayDesc{
    /// index of the most detailed mipmap to use
    pub most_detailed_mip: u32,
    /// levels of mipmap to use, `-1` means up until the least detailed
    pub mip_levels: i32,
    /// first 2D slice to use
    pub first_slice: u32,
    /// number of cube textures to use
    pub num_cubes: u32,
    /// minimum sampled lod clamp value
    pub mip_lod_clamp: f32,
}

/// helper struct for ffi, not intended for application user
/// TODO: remove from public interface
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct SrvDescBindHelper {
    format: DxgiFormat,
    view_dimension: ::winapi::D3D12_SRV_DIMENSION,
    component_mapping: u32,
    a: [u32; 6],
}

impl From<SrvDesc> for SrvDescBindHelper{
    #[inline]
    fn from(desc: SrvDesc) -> SrvDescBindHelper {
        unsafe {
            let mut ret: SrvDescBindHelper = ::std::mem::zeroed();
            ret.format = desc.format;
            ret.component_mapping = ::std::mem::transmute(desc.component_mapping);
            match desc.dimension {
                SrvDimension::Unknown =>
                    ret.view_dimension = ::winapi::D3D12_SRV_DIMENSION_UNKNOWN,
                SrvDimension::Buffer(content) => {
                    ret.view_dimension = ::winapi::D3D12_SRV_DIMENSION_BUFFER;
                    ret.a = ::std::mem::transmute_copy(&content);
                },
                SrvDimension::Tex1D(content) => {
                    ret.view_dimension = ::winapi::D3D12_SRV_DIMENSION_TEXTURE1D;
                    ret.a = ::std::mem::transmute_copy(&content);
                },
                SrvDimension::Tex1DArray(content) => {
                    ret.view_dimension = ::winapi::D3D12_SRV_DIMENSION_TEXTURE1DARRAY;
                    ret.a = ::std::mem::transmute_copy(&content);
                },
                SrvDimension::Tex2D(content) => {
                    ret.view_dimension = ::winapi::D3D12_SRV_DIMENSION_TEXTURE2D;
                    ret.a = ::std::mem::transmute_copy(&content);
                },
                SrvDimension::Tex2DArray(content) => {
                    ret.view_dimension = ::winapi::D3D12_SRV_DIMENSION_TEXTURE2DARRAY;
                    ret.a = ::std::mem::transmute_copy(&content);
                },
                SrvDimension::Tex2DMs =>
                    ret.view_dimension = ::winapi::D3D12_SRV_DIMENSION_TEXTURE2DMS,
                SrvDimension::Tex2DMsArray(content) => {
                    ret.view_dimension = ::winapi::D3D12_SRV_DIMENSION_TEXTURE2DMSARRAY;
                    ret.a = ::std::mem::transmute_copy(&content);
                },
                SrvDimension::Tex3D(content) => {
                    ret.view_dimension = ::winapi::D3D12_SRV_DIMENSION_TEXTURE3D;
                    ret.a = ::std::mem::transmute_copy(&content);
                },
                SrvDimension::TexCube(content) => {
                    ret.view_dimension = ::winapi::D3D12_SRV_DIMENSION_TEXTURECUBE;
                    ret.a = ::std::mem::transmute_copy(&content);
                },
                SrvDimension::TexCubeArray(content) => {
                    ret.view_dimension = ::winapi::D3D12_SRV_DIMENSION_TEXTURECUBEARRAY;
                    ret.a = ::std::mem::transmute_copy(&content);
                },
            }
            ret
        }
    }
}
