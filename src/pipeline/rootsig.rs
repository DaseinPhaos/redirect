// Copyright 2017 Dasein Phaos aka. Luxko
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! describes what kind of resources are to be bound to the pipeline.

use device::Device;
use smallvec::SmallVec;
use comptr::ComPtr;
use winapi::{ID3D12RootSignature, ID3DBlob};
use error::WinError;
use super::sampler::StaticSamplerDesc;

/// a root signature
#[derive(Clone, Debug)]
pub struct RootSig {
    pub ptr: ComPtr<ID3D12RootSignature>,
}

/// a serialized root signature description blob
#[derive(Clone, Debug)]
pub struct RootSigDescBlob {
    pub ptr: ComPtr<ID3DBlob>,
}

/// builder for a root signature
#[derive(Clone, Debug, Default)]
pub struct RootSigBuilder {
    pub root_params: SmallVec<[RootParam; 8]>,
    pub static_samplers: SmallVec<[StaticSamplerDesc; 8]>,
    pub flags: RootSigFlags,
}

impl RootSigBuilder {
    /// construct a new builder
    pub fn new() -> Self {
        Default::default()
    }

    /// build a root signature with description in this builder using `device`
    pub fn build(&self, device: &mut Device, node_mask: u32) -> Result<RootSig, WinError> {
        let blob = self.serialize()?;
        device.create_root_sig(node_mask, &blob)
    }

    /// serialize the description into a blob
    pub fn serialize(&self) -> Result<RootSigDescBlob, WinError> {
        let mut root_params: SmallVec<[_; 8]> = Default::default();
        for root_param in self.root_params.iter() {
            root_params.push(root_param.into());
        }
        
        let desc = ::winapi::D3D12_ROOT_SIGNATURE_DESC{
            NumParameters: root_params.len() as u32,
            pParameters: root_params.as_ptr(),
            NumStaticSamplers: self.static_samplers.len() as u32,
            pStaticSamplers: self.static_samplers.as_ptr() as *const _,
            Flags: self.flags.into()
        };

        unsafe {
            let mut ptr = ::std::mem::uninitialized();
            let hr = ::d3d12::D3D12SerializeRootSignature(
                &desc, ::winapi::D3D_ROOT_SIGNATURE_VERSION_1, // TODO: support more signature versions?
                &mut ptr,
                ::std::ptr::null_mut() // TODO: support error blob?
            );
            WinError::from_hresult_or_ok(hr, || RootSigDescBlob{
                ptr: ComPtr::new(ptr)
            })
        }
    }
}

/// describes a root parameter
#[derive(Clone, Debug)]
pub struct RootParam {
    /// shader visibility
    pub visibility: ShaderVisibility,
    pub param_type: RootParamType,
}

impl<'a> From<&'a RootParam> for ::winapi::D3D12_ROOT_PARAMETER {
    fn from(param: &'a RootParam) -> Self {
        let (t, d) = match param.param_type {
            RootParamType::DescriptorTable{
                ref descriptor_ranges
            } => (
                ::winapi::D3D12_ROOT_PARAMETER_TYPE_DESCRIPTOR_TABLE,
                ::winapi::D3D12_ROOT_DESCRIPTOR_TABLE {
                    NumDescriptorRanges: descriptor_ranges.len() as u32,
                    pDescriptorRanges: descriptor_ranges.as_ptr() as *const _,
                }
            ),
            RootParamType::Constant{
                shader_register, register_space, num_32bit_values,
            } => (
                ::winapi::D3D12_ROOT_PARAMETER_TYPE_32BIT_CONSTANTS,
                ParamTypeHelper::new(shader_register, register_space, num_32bit_values).into()
            ),
            RootParamType::Cbv{
                shader_register, register_space,
            } => (
                ::winapi::D3D12_ROOT_PARAMETER_TYPE_CBV,
                ParamTypeHelper::new(shader_register, register_space, 0).into()
            ),
            RootParamType::Srv{
                shader_register, register_space,
            } => (
                ::winapi::D3D12_ROOT_PARAMETER_TYPE_SRV,
                ParamTypeHelper::new(shader_register, register_space, 0).into()
            ),
            RootParamType::Uav{
                shader_register, register_space,
            } => (
                ::winapi::D3D12_ROOT_PARAMETER_TYPE_UAV,
                ParamTypeHelper::new(shader_register, register_space, 0).into()
            ),
        };
        ::winapi::D3D12_ROOT_PARAMETER{
            ParameterType: t,
            u: d,
            ShaderVisibility: unsafe{::std::mem::transmute(param.visibility)}
        }
    }
}

/// specifies a type of root parameter
#[derive(Clone, Debug)]
pub enum RootParamType {
    /// a collection of descriptor ranges, appearing in sequence in a descriptor heap
    DescriptorTable{
        /// an array of descriptor ranges
        descriptor_ranges: SmallVec<[DescriptorRange; 4]>,
    },
    /// cbv descriptor inlined in the signature
    Cbv{
        /// the shader register
        shader_register: u32,
        /// the register space
        register_space: u32,
    },
    /// srv descriptor inlined in the signature
    Srv{
        /// the shader register
        shader_register: u32,
        /// the register space
        register_space: u32,
    },
    /// uav descriptor inlined in the signature
    Uav{
        /// the shader register
        shader_register: u32,
        /// the register space
        register_space: u32,
    },
    /// constants inlined in the signature that appear in shaders as one constant buffer
    Constant{
        /// shader register of this constant
        shader_register: u32,
        /// register space of this constant
        register_space: u32,
        /// number of 32bit values in this constant slot
        num_32bit_values: u32,
    },
}

#[repr(C)]
struct ParamTypeHelper {
    shader_register: u32,
    register_space: u32,
    num_32bit_values: u32,
    _pad: u32,
}

impl ParamTypeHelper {
    #[inline]
    fn new(shader_register: u32, register_space: u32, num_32bit_values: u32) -> Self {
        ParamTypeHelper{shader_register, register_space, num_32bit_values, _pad: 0}
    }
}

impl From<ParamTypeHelper> for ::winapi::D3D12_ROOT_DESCRIPTOR_TABLE {
    fn from(helper: ParamTypeHelper) -> Self {
        unsafe {::std::mem::transmute(helper)}
    }
}

/// descriptor range
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DescriptorRange {
    pub range_type: DescriptorRangeType,
    pub num_descriptors: u32,
    pub base_shader_register: u32,
    pub register_space: u32,
    pub offset_from_table_start: u32,
}

bitflags!{
    /// type of a descriptor range
    pub struct DescriptorRangeType: u32 {
        const DESCRIPTOR_RANGE_TYPE_SRV      = 0;
        const DESCRIPTOR_RANGE_TYPE_UAV      = DESCRIPTOR_RANGE_TYPE_SRV.bits + 1;
        const DESCRIPTOR_RANGE_TYPE_CBV      = DESCRIPTOR_RANGE_TYPE_UAV.bits + 1;
        const DESCRIPTOR_RANGE_TYPE_SAMPLER  = DESCRIPTOR_RANGE_TYPE_CBV.bits + 1;
    }
}

bitflags!{
    /// specifies which shader can access content of a given root parameter
    #[repr(C)]
    pub struct ShaderVisibility: u32 {
        const SHADER_VISIBILITY_ALL       = 0;
        const SHADER_VISIBILITY_VERTEX    = 1;
        const SHADER_VISIBILITY_HULL      = 2;
        const SHADER_VISIBILITY_DOMAIN    = 3;
        const SHADER_VISIBILITY_GEOMETRY  = 4;
        const SHADER_VISIBILITY_PIXEL     = 5;
    }
}

impl Default for ShaderVisibility {
    #[inline]
    fn default() -> ShaderVisibility {
        SHADER_VISIBILITY_ALL
    }
}

bitflags!{
    /// misc flags for a root signature
    #[repr(C)]
    pub struct RootSigFlags: u32 {
        const ROOT_SIGNATURE_FLAG_NONE                                = 0;
        const ROOT_SIGNATURE_FLAG_ALLOW_INPUT_ASSEMBLER_INPUT_LAYOUT  = 0x1;
        const ROOT_SIGNATURE_FLAG_DENY_VERTEX_SHADER_ROOT_ACCESS      = 0x2;
        const ROOT_SIGNATURE_FLAG_DENY_HULL_SHADER_ROOT_ACCESS        = 0x4;
        const ROOT_SIGNATURE_FLAG_DENY_DOMAIN_SHADER_ROOT_ACCESS      = 0x8;
        const ROOT_SIGNATURE_FLAG_DENY_GEOMETRY_SHADER_ROOT_ACCESS    = 0x10;
        const ROOT_SIGNATURE_FLAG_DENY_PIXEL_SHADER_ROOT_ACCESS       = 0x20;
        const ROOT_SIGNATURE_FLAG_ALLOW_STREAM_OUTPUT                 = 0x40;
    }
}

impl Default for RootSigFlags {
    #[inline]
    fn default() -> Self {
        ROOT_SIGNATURE_FLAG_ALLOW_INPUT_ASSEMBLER_INPUT_LAYOUT
    }
}

impl From<RootSigFlags> for ::winapi::D3D12_ROOT_SIGNATURE_FLAGS {
    #[inline]
    fn from(flags: RootSigFlags) -> Self {
        unsafe{ ::std::mem::transmute(flags)}
    }
}
