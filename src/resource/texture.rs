// Copyright 2017 Dasein Phaos aka. Luxko
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Various types of safe textures

use super::raw::*;
use super::traits::*;
use device::Device;
use error::WinError;
use format::DxgiFormat;

#[derive(Debug)]
pub struct DefaultTex2D {
    raw: RawResource,
    width: u64,
    height: u32,
    mip_levels: u16,
    format: DxgiFormat,
}

impl DefaultTex2D {
    /// intial state is generic read
    #[inline]
    pub fn new(
        device: &mut Device, width: u64, height: u32,
        mip_levels: u16, format: DxgiFormat
    ) -> Result<DefaultTex2D, WinError> {
        let raw = device.create_committed_resource(
            &Default::default(),
            Default::default(), // TODO: check if additional denies helps?
            &super::description::ResourceDesc::tex2d(
                width, height, 1, mip_levels, format,
                Default::default(), Default::default()
            ),
            Default::default() // TODO: other initial states?
        )?;
        Ok(DefaultTex2D{raw, width, height, mip_levels, format})
    }
}

impl_as_raw!(Resource, DefaultTex2D, RawResource);
unsafe impl Texture for DefaultTex2D {
    #[inline]
    fn get_format(&mut self) -> ::format::DxgiFormat {
        self.format
    }
}
unsafe impl Tex2D for DefaultTex2D {
    #[inline]
    fn get_width(&mut self) -> u64 {
        self.width
    }

    #[inline]
    fn get_height(&mut self) -> u32 {
        self.height
    }
}

unsafe impl GpuOnly for DefaultTex2D {}
unsafe impl AllowShaderResource for DefaultTex2D {}

#[derive(Debug)]
pub struct DsableTex2D {
    raw: RawResource,
    width: u64,
    height: u32,
    mip_levels: u16,
    format: DxgiFormat,
}

impl DsableTex2D {
    /// intial state is depth write
    #[inline]
    pub fn new(
        device: &mut Device, width: u64, height: u32,
        mip_levels: u16, format: DxgiFormat
    ) -> Result<DsableTex2D, WinError> {
        debug_assert!(
            format == ::format::DXGI_FORMAT_D16_UNORM ||
            format == ::format::DXGI_FORMAT_D32_FLOAT ||
            format == ::format::DXGI_FORMAT_D24_UNORM_S8_UINT ||
            format == ::format::DXGI_FORMAT_D32_FLOAT_S8X24_UINT
        );
        let raw = device.create_committed_resource(
            &Default::default(),
            Default::default(), // TODO: check if additional denies helps?
            &super::description::ResourceDesc::tex2d(
                width, height, 1, mip_levels, format,
                super::description::RESOURCE_FLAG_ALLOW_DEPTH_STENCIL,
                Default::default()
            ),
            super::RESOURCE_STATE_DEPTH_WRITE // TODO: other initial states?
        )?;
        Ok(DsableTex2D{raw, width, height, mip_levels, format})
    }
}

impl_as_raw!(Resource, DsableTex2D, RawResource);
unsafe impl Texture for DsableTex2D {
    #[inline]
    fn get_format(&mut self) -> ::format::DxgiFormat {
        self.format
    }
}
unsafe impl Tex2D for DsableTex2D {
    #[inline]
    fn get_width(&mut self) -> u64 {
        self.width
    }

    #[inline]
    fn get_height(&mut self) -> u32 {
        self.height
    }
}

unsafe impl GpuOnly for DsableTex2D {}
unsafe impl AllowShaderResource for DsableTex2D {}
unsafe impl AllowDepthStencil for DsableTex2D {}

#[derive(Debug)]
pub struct RenderableTex2D {
    raw: RawResource,
    width: u64,
    height: u32,
    mip_levels: u16,
    format: DxgiFormat,
}

impl RenderableTex2D {
    /// intial state is generic read
    #[inline]
    pub fn new(
        device: &mut Device, width: u64, height: u32,
        mip_levels: u16, format: DxgiFormat
    ) -> Result<RenderableTex2D, WinError> {
        let raw = device.create_committed_resource(
            &Default::default(),
            Default::default(), // TODO: check if additional denies helps?
            &super::description::ResourceDesc::tex2d(
                width, height, 1, mip_levels, format,
                super::description::RESOURCE_FLAG_ALLOW_RENDER_TARGET,
                Default::default()
            ),
            Default::default() // TODO: other initial states?
        )?;
        Ok(RenderableTex2D{raw, width, height, mip_levels, format})
    }
}

impl_as_raw!(Resource, RenderableTex2D, RawResource);
unsafe impl Texture for RenderableTex2D {
    #[inline]
    fn get_format(&mut self) -> ::format::DxgiFormat {
        self.format
    }
}
unsafe impl Tex2D for RenderableTex2D {
    #[inline]
    fn get_width(&mut self) -> u64 {
        self.width
    }

    #[inline]
    fn get_height(&mut self) -> u32 {
        self.height
    }
}

unsafe impl GpuOnly for RenderableTex2D {}
unsafe impl AllowShaderResource for RenderableTex2D {}
unsafe impl AllowRenderTarget for RenderableTex2D {}

// TODO: placed textures, see https://msdn.microsoft.com/en-us/library/windows/desktop/dn788680(v=vs.85).aspx
