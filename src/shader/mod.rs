// Copyright 2017 Dasein Phaos aka. Luxko
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! executable code snippet for GPU

use comptr::ComPtr;
use winapi::ID3DBlob;
use std::os::raw::c_char;
use std::ffi::CStr;
use std::marker::PhantomData;
use error::WinError;
use smallvec::SmallVec;

/// a compiled piece of VS shader byte code
#[derive(Debug, Clone)]
pub struct VsShaderBytecode {
    pub ptr: ComPtr<ID3DBlob>,
}

/// a compiled piece of PS shader byte code
#[derive(Debug, Clone)]
pub struct PsShaderBytecode {
    pub ptr: ComPtr<ID3DBlob>,
}

/// a compiled piece of CS shader byte code
#[derive(Debug, Clone)]
pub struct CsShaderBytecode {
    pub ptr: ComPtr<ID3DBlob>,
}

/// a compiled piece of HS shader byte code
#[derive(Debug, Clone)]
pub struct HsShaderBytecode {
    pub ptr: ComPtr<ID3DBlob>,
}

/// a compiled piece of GS shader byte code
#[derive(Debug, Clone)]
pub struct GsShaderBytecode {
    pub ptr: ComPtr<ID3DBlob>,
}

/// a compiled piece of DS shader byte code
#[derive(Debug, Clone)]
pub struct DsShaderBytecode {
    pub ptr: ComPtr<ID3DBlob>,
}

macro_rules! impl_shader_bytecode {
    ($Shader: ty) => {
        impl $Shader {
            #[inline]
            pub fn to_shader_bytecode(&mut self) -> ::winapi::D3D12_SHADER_BYTECODE {
                let mut ret: ::winapi::D3D12_SHADER_BYTECODE = unsafe {
                    ::std::mem::uninitialized()
                };
                ret.pShaderBytecode = unsafe {
                    self.ptr.GetBufferPointer() as *const _
                };
                ret.BytecodeLength = unsafe {
                    self.ptr.GetBufferSize()
                };
                ret
            }
        }
    }
}

impl_shader_bytecode!(VsShaderBytecode);
impl_shader_bytecode!(PsShaderBytecode);
impl_shader_bytecode!(DsShaderBytecode);
impl_shader_bytecode!(CsShaderBytecode);
impl_shader_bytecode!(HsShaderBytecode);
impl_shader_bytecode!(GsShaderBytecode);

/// shader builder
#[derive(Debug)]
pub struct ShaderBuilder<'a> {
    pub src_data: &'a [u8],
    // TODO: source names for error info?
    pub shader_macros: SmallVec<[ShaderMacro<'a>; 8]>,
    // TODO: custom include handlers?
    pub entry_point: &'a CStr,
    // TODO: custom shader levels?
    pub flags: ShaderCompileFlags,
    // TODO: effect flags?
    // TODO: secondary data?
}

macro_rules! impl_build {
    ($func: ident, $Ret: tt, $Target: tt) => {
    #[inline]
    pub fn $func(&mut self) -> Result<$Ret, WinError> {
        self.shader_macros.push(Default::default());
        unsafe {
            let mut ret = ::std::mem::uninitialized();
            let hr = ::d3dcompiler::D3DCompile(
                self.src_data.as_ptr() as *const _ as *const _,
                self.src_data.len() as _,
                ::std::ptr::null(),
                self.shader_macros.as_ptr() as *const _,
                ::winapi::D3D_COMPILE_STANDARD_FILE_INCLUDE,
                self.entry_point.as_ptr() as _,
                $Target.as_ptr() as *const _,
                ::std::mem::transmute(self.flags),
                0,
                &mut ret,
                ::std::ptr::null_mut()
            );

            self.shader_macros.pop();
            WinError::from_hresult_or_ok(hr, || $Ret{
                ptr: ComPtr::new(ret)
            })
        }
    }}
}

impl<'a> ShaderBuilder<'a> {
    #[inline]
    pub fn new(
        src_data: &'a [u8], entry_point: &'a CStr
    ) -> Self {
        ShaderBuilder{
            src_data, entry_point,
            shader_macros: Default::default(),
            flags: Default::default(),
        }
    }

    impl_build!(build_vs, VsShaderBytecode, "vs_5_0\0");
    impl_build!(build_ps, PsShaderBytecode, "ps_5_0\0");
    impl_build!(build_hs, HsShaderBytecode, "hs_5_0\0");
    impl_build!(build_cs, CsShaderBytecode, "cs_5_0\0");
    impl_build!(build_gs, GsShaderBytecode, "gs_5_0\0");
    impl_build!(build_ds, DsShaderBytecode, "ds_5_0\0");
}

/// shader macros
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ShaderMacro<'a> {
    name: *const c_char,
    definition: *const c_char,
    _pd: PhantomData<&'a CStr>,
}

impl<'a> ShaderMacro<'a> {
    #[inline]
    pub fn new<A, B>(name: &'a A, definition: &'a B) -> Self 
        where A: AsRef<CStr>, B: AsRef<CStr>,
    {
        ShaderMacro{
            name: name.as_ref().as_ptr() as *const _, 
            definition: definition.as_ref().as_ptr() as *const _,
            _pd: Default::default(),
        }
    }
}

impl<'a> Default for ShaderMacro<'a> {
    #[inline]
    fn default() -> Self {
        ShaderMacro{
            name: ::std::ptr::null(),
            definition: ::std::ptr::null(),
            _pd: Default::default(),
        }
    }
}

bitflags!{
    /// shader compile flags. [more](https://msdn.microsoft.com/en-us/library/windows/desktop/gg615083(v=vs.85).aspx)
    pub struct ShaderCompileFlags: u32 {
        const SHADER_COMPILE_DEBUG = ::winapi::D3DCOMPILE_DEBUG;
        const SHADER_COMPILE_SKIP_VALIDATION = ::winapi::D3DCOMPILE_SKIP_VALIDATION;
        const SHADER_COMPILE_SKIP_OPTIMIZATION = ::winapi::D3DCOMPILE_SKIP_OPTIMIZATION;
        const SHADER_COMPILE_PACK_MATRIX_ROW_MAJOR = ::winapi::D3DCOMPILE_PACK_MATRIX_ROW_MAJOR;
        const SHADER_COMPILE_PACK_MATRIX_COLUMN_MAJOR = ::winapi::D3DCOMPILE_PACK_MATRIX_COLUMN_MAJOR;
        const SHADER_COMPILE_PARTIAL_PRECISION = ::winapi::D3DCOMPILE_PARTIAL_PRECISION;
        const SHADER_COMPILE_FORCE_VS_SOFTWARE_NO_OPT = ::winapi::D3DCOMPILE_FORCE_VS_SOFTWARE_NO_OPT;
        const SHADER_COMPILE_FORCE_PS_SOFTWARE_NO_OPT = ::winapi::D3DCOMPILE_FORCE_PS_SOFTWARE_NO_OPT;
        const SHADER_COMPILE_NO_PRESHADER = ::winapi::D3DCOMPILE_NO_PRESHADER;
        const SHADER_COMPILE_AVOID_FLOW_CONTROL = ::winapi::D3DCOMPILE_AVOID_FLOW_CONTROL;
        const SHADER_COMPILE_PREFER_FLOW_CONTROL = ::winapi::D3DCOMPILE_PREFER_FLOW_CONTROL;
        const SHADER_COMPILE_ENABLE_STRICTNESS = ::winapi::D3DCOMPILE_ENABLE_STRICTNESS;
        const SHADER_COMPILE_ENABLE_BACKWARDS_COMPATIBILITY = ::winapi::D3DCOMPILE_ENABLE_BACKWARDS_COMPATIBILITY;
        const SHADER_COMPILE_IEEE_STRICTNESS = ::winapi::D3DCOMPILE_IEEE_STRICTNESS;
        const SHADER_COMPILE_OPTIMIZATION_LEVEL0 = ::winapi::D3DCOMPILE_OPTIMIZATION_LEVEL0;
        const SHADER_COMPILE_OPTIMIZATION_LEVEL1 = ::winapi::D3DCOMPILE_OPTIMIZATION_LEVEL1;
        const SHADER_COMPILE_OPTIMIZATION_LEVEL2 = ::winapi::D3DCOMPILE_OPTIMIZATION_LEVEL2;
        const SHADER_COMPILE_OPTIMIZATION_LEVEL3 = ::winapi::D3DCOMPILE_OPTIMIZATION_LEVEL3;
        const SHADER_COMPILE_WARNINGS_ARE_ERRORS = ::winapi::D3DCOMPILE_WARNINGS_ARE_ERRORS;
        const SHADER_COMPILE_RESOURCES_MAY_ALIAS = ::winapi::D3DCOMPILE_RESOURCES_MAY_ALIAS;
        const SHADER_COMPILE_ENABLE_UNBOUNDED_DESCRIPTOR_TABLES = ::winapi::D3DCOMPILE_ENABLE_UNBOUNDED_DESCRIPTOR_TABLES;
        const SHADER_COMPILE_ALL_RESOURCES_BOUND = ::winapi::D3DCOMPILE_ALL_RESOURCES_BOUND;
    }
}

impl Default for ShaderCompileFlags {
    fn default() -> Self {
        SHADER_COMPILE_OPTIMIZATION_LEVEL0 | SHADER_COMPILE_WARNINGS_ARE_ERRORS
    }
}
