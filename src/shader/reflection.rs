//! Contains definitions for various shader reflection structures

use super::{ShaderCompileFlags, ShaderBytecode};
use pipeline::ia::PrimitiveTopology;
use error::WinError;
use winapi::ID3D12ShaderReflection;
use std::os::raw::c_char;
use comptr::ComPtr;
use std::marker::PhantomData;
use std::ffi::CStr;
use device::FeatureLevel;
use format::Bool;
use dxguid::IID_ID3D12ShaderReflection;
use d3dcompiler::D3DReflect;

bitflags!{
    /// enumrates the types of shaders
    #[repr(C)]
    pub struct VersionType: u32 {
        const PIXEL = 0;
        const VERTEX = 1;
        const GEOMETRY = 2;
        const HULL = 3;
        const DOMAIN = 4;
        const COMPUTE = 5;
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub struct Version {
    version: u32,
}

impl Version {
    /// Get the type of the shader
    #[inline]
    pub fn get_type(&self) -> VersionType {
        let bits = (self.version & 0xffff0000) >> 16;
        VersionType::from_bits_truncate(bits)
    }

    /// Get the major version number of the shader
    #[inline]
    pub fn get_major(&self) -> u32 {
        (self.version & 0x000000f0) >> 4
    }

    /// Get the minor version number of the shader
    #[inline]
    pub fn get_minor(&self) -> u32 {
        self.version & 0xf
    }

    #[inline]
    pub fn new(t: VersionType, major: u32, minor: u32) -> Version {
        let mut version: u32 = 0;
        version += t.bits << 16;
        version += (major & 0xf) << 4;
        version += minor & 0xf;
        Version{version}
    }
}

impl From<Version> for u32 {
    #[inline]
    fn from(v: Version) -> u32 {
        v.version
    }
}

/// Description of a shader
#[repr(C)]
#[derive(Clone, Debug)]
pub struct ShaderDesc<'a> {
    /// version info
    pub version: Version,
    // TODO: creator
    pub creator: *const c_char,
    /// compilation flags
    pub flags: ShaderCompileFlags,
    /// number of shader-constant buffers
    pub constant_buffers: u32,
    /// number of resources bound to
    pub bound_resources: u32,
    /// number of parameters in the input signature
    pub input_parameters: u32,
    /// number of parameters in the output signature
    pub output_parameters: u32,
    /// number of IL instructions in the compiled shader
    pub instruction_count: u32,
    /// number of temporary registers in the compiled shader
    pub temp_register_count: u32,
    /// number of temporary arrays used
    pub temp_array_count: u32,
    /// number of constant definitions
    pub def_count: u32,
    /// number of declarations (input + output)
    pub dcl_count: u32,
    /// number of non-categorized texture instructions
    pub texture_normal_instructions: u32,
    /// number of texture load instructions
    pub texture_load_instructions: u32,
    /// number of texture comparison instructions
    pub texture_comp_instructions: u32,
    /// number of texture bias instructions
    pub texture_bias_instructions: u32,
    /// number of texture gradient instructions
    pub texture_gradient_instructions: u32,
    /// number of floating point arithmetic instructions
    pub float_instructions: u32,
    /// number of signed integer arithmetic instructions
    pub int_instructions: u32,
    /// number of unsigned integer arithmetic instructions
    pub uint_instructions: u32,
    /// number of static flow control instructions
    pub static_flow_controls: u32,
    /// number of dynamic flow control instructions
    pub dynamic_flow_controls: u32,
    /// number of macro instructions
    pub macro_instructions: u32,
    /// number of array instructions
    pub array_instructions: u32,
    /// number of cut instructions
    pub cut_instructions: u32,
    /// number of emit instructions
    pub emit_instructions: u32,
    /// GS specific: output topology
    pub gs_output_topology: PrimitiveTopology,
    /// GS specific: shader maximum output vertex count
    pub gs_max_output_vertex_count: u32,
    /// GS/HS: the input primitive type
    pub input_primitive: u32, // FIXME: D3D_PRIMITIVE
    /// number of parameters in the patch-constant signature
    pub patch_constant_parameters: u32,
    /// number of geometry shader instances (?)
    pub c_gs_instance_count: u32,
    /// number of control points in HS/DS
    pub c_control_points: u32,
    /// tessellator output primitive type
    pub hs_output_primitive: u32, // FIXME: D3D_TESSELLATOR_OUTPUT_PRIMITIVE
    /// tessellator partitioning mode
    pub hs_partitioning: u32, // FIXME: D3D_TESSELLATOR_PARTITIONING
    /// tessellator domain
    pub tessellator_domain: u32, // FIXME: D3D_TESSELLATOR_DOMAIN
    /// CS specific: number of barrier instructions
    pub c_barrier_instructions: u32,
    /// CS specific: number of interlocked instructions
    pub c_interlocked_instructions: u32,
    /// CS specific: number of texture writes
    pub c_texture_store_instructions: u32,
    _pd: PhantomData<&'a CStr>,
}

bitflags! {
    /// a system defined value type [name](https://msdn.microsoft.com/en-us/library/windows/desktop/ff728724(v=vs.85).aspx)
    #[repr(C)]
    pub struct SystemValueName: u32 {
        const UNDEFINED                        = 0;
        const POSITION                         = 1;
        const CLIP_DISTANCE                    = 2;
        const CULL_DISTANCE                    = 3;
        const RENDER_TARGET_ARRAY_INDEX        = 4;
        const VIEWPORT_ARRAY_INDEX             = 5;
        const VERTEX_ID                        = 6;
        const PRIMITIVE_ID                     = 7;
        const INSTANCE_ID                      = 8;
        const IS_FRONT_FACE                    = 9;
        const SAMPLE_INDEX                     = 10;
        const FINAL_QUAD_EDGE_TESSFACTOR       = 11;
        const FINAL_QUAD_INSIDE_TESSFACTOR     = 12;
        const FINAL_TRI_EDGE_TESSFACTOR        = 13;
        const FINAL_TRI_INSIDE_TESSFACTOR      = 14;
        const FINAL_LINE_DETAIL_TESSFACTOR     = 15;
        const FINAL_LINE_DENSITY_TESSFACTOR    = 16;
        const BARYCENTRICS                     = 23;
        const TARGET                           = 64;
        const DEPTH                            = 65;
        const COVERAGE                         = 66;
        const DEPTH_GREATER_EQUAL              = 67;
        const DEPTH_LESS_EQUAL                 = 68;
        const STENCIL_REF                      = 69;
        const INNER_COVERAGE                   = 70;
    }
}

bitflags! {
    /// data types that can be stored in a register
    #[repr(C)]
    pub struct RegisterComponentType: u32 {
        const UNKNOWN = 0;
        const UINT32 = 1;
        const SINT32 = 2;
        const FLOAT32 = 3;
    }
}

bitflags! {
    /// the minimum desired [interpolation precision](https://msdn.microsoft.com/en-us/library/windows/desktop/jj247572%28v=vs.85%29.aspx?f=255&MSPPError=-2147217396)
    #[repr(C)]
    pub struct MinPrecison: u32 {
        const DEFAULT = 0;
        const FLOAT_16 = 1;
        const FLOAT_2_8 = 2;
        const SINT_16 = 4;
        const UINT_16 = 5;
        const ANY_16 = 0xf0;
        const ANY_10 = 0xf1;
    }
}

/// descritpion of a signature parameter
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct SignatureParameterDesc<'a> {
    pub semantic_name: *const c_char,
    pub semantic_index: u32,
    pub register: u32,
    pub system_value_type: SystemValueName,
    pub component_type: RegisterComponentType,
    pub mask: u8,
    pub rw_mask: u8,
    pub stream: u32,
    pub min_precision: MinPrecison,
    _pd: PhantomData<&'a CStr>,
}

/// descritpion of a bound shader input resource
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct ShaderInputBindDesc<'a> {
    pub name: *const c_char,
    pub input_type: u32, // TODO: ShaderInputType
    pub bind_point: u32,
    pub bind_count: u32,
    pub bind_flags: u32, // TODO: ShaderInputFlags
    pub return_type: u32, // TODO: ShaderReturnType
    pub dimension: u32, // TODO: SrvDimension?
    /// The number of samples for a multisampled texture;
    /// when a texture isn't multisampled, the value is
    /// set to -1 (0xFFFFFFFF). This is zero if the shader
    /// resource is not a recognized texture.
    pub num_samples: u32,
    pub space: u32,
    pub uid: u32,
    _pd: PhantomData<&'a CStr>,
}


#[derive(Debug, Clone)]
pub struct ShaderReflection {
    pub(crate) ptr: ComPtr<ID3D12ShaderReflection>,
}

impl ShaderReflection {
    #[inline]
    pub fn new<SB: ShaderBytecode+?Sized>(
        source: &mut SB
    ) -> Result<ShaderReflection, WinError> {unsafe{
        let mut ret: *mut ID3D12ShaderReflection = ::std::ptr::null_mut();
        let sbc = source.to_shader_bytecode();
        let hr = D3DReflect(
            sbc.pShaderBytecode, sbc.BytecodeLength,
            &IID_ID3D12ShaderReflection,
            &mut ret as *mut *mut _ as *mut *mut ::winapi::c_void
        );
        WinError::from_hresult_or_ok(hr, || ShaderReflection{
            ptr: ComPtr::new(ret)
        })
    }}

    #[inline]
    pub fn get_bitwise_instruction_count(&mut self) -> u32 {unsafe{
        self.ptr.GetBitwiseInstructionCount()
    }}

    // TODO: GetConstantBufferBy

    #[inline]
    pub fn get_conversion_instruction_count(&mut self) -> u32 {unsafe{
        self.ptr.GetConversionInstructionCount()
    }}

    #[inline]
    pub fn get_desc(&mut self) -> Result<ShaderDesc, WinError> {unsafe{
        let mut ret = ::std::mem::uninitialized();
        let hr = self.ptr.GetDesc(&mut ret as *mut _ as *mut _);
        WinError::from_hresult_or_ok(hr, || ret)
    }}

    // TODO: GetGSInputPrimitive

    #[inline]
    pub fn get_input_parameter_desc(
        &mut self, index: u32
    ) -> Result<SignatureParameterDesc, WinError> {unsafe{
        let mut ret = ::std::mem::uninitialized();
        let hr = self.ptr.GetInputParameterDesc(
            index, &mut ret as *mut _ as *mut _
        );
        WinError::from_hresult_or_ok(hr, || ret)
    }}

    #[inline]
    pub fn get_output_parameter_desc(
        &mut self, index: u32
    ) -> Result<SignatureParameterDesc, WinError> {unsafe{
        let mut ret = ::std::mem::uninitialized();
        let hr = self.ptr.GetOutputParameterDesc(
            index, &mut ret as *mut _ as *mut _
        );
        WinError::from_hresult_or_ok(hr, || ret)
    }}

    #[inline]
    pub fn get_patch_constant_parameter_desc(
        &mut self, index: u32
    ) -> Result<SignatureParameterDesc, WinError> {unsafe{
        let mut ret = ::std::mem::uninitialized();
        let hr = self.ptr.GetPatchConstantParameterDesc(
            index, &mut ret as *mut _ as *mut _
        );
        WinError::from_hresult_or_ok(hr, || ret)
    }}

    #[inline]
    pub fn get_min_feature_level(&mut self) -> Result<FeatureLevel, WinError> {unsafe{
        let mut ret = ::std::mem::uninitialized();
        let hr = self.ptr.GetMinFeatureLevel(&mut ret as *mut _ as *mut _);
        WinError::from_hresult_or_ok(hr, || ret)
    }}

    #[inline]
    pub fn get_movc_instruction_count(&mut self) -> u32 {unsafe{
        self.ptr.GetMovcInstructionCount()
    }}

    #[inline]
    pub fn get_mov_instruction_count(&mut self) -> u32 {unsafe{
        self.ptr.GetMovInstructionCount()
    }}

    #[inline]
    pub fn get_num_interface_slots(&mut self) -> u32 {unsafe{
        self.ptr.GetNumInterfaceSlots()
    }}

    // TODO: GetRequiresFlags

    #[inline]
    pub fn get_resource_binding_desc(
        &mut self, index: u32
    ) -> Result<ShaderInputBindDesc, WinError> {unsafe{
        let mut ret = ::std::mem::uninitialized();
        let hr = self.ptr.GetResourceBindingDesc(
            index, &mut ret as *mut _ as *mut _
        );
        WinError::from_hresult_or_ok(hr, || ret)
    }}

    // TODO: GetResourceBindingDescByName

    #[inline]
    pub fn get_thread_group_size(&mut self) -> (u32, u32, u32) {unsafe{
        let mut ret = (0, 0, 0);
        self.ptr.GetThreadGroupSize(
            &mut ret.0 as *mut _,
            &mut ret.1,
            &mut ret.2
        );
        ret
    }}

    // TODO: GetShaderReflectionVariable

    #[inline]
    pub fn is_sample_frequency(&mut self) -> Bool {unsafe{
        Bool::from_win_bool(self.ptr.IsSampleFrequencyShader())
    }}
}
