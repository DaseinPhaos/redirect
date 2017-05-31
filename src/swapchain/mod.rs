// Copyright 2017 Dasein Phaos aka. Luxko
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! link between the graphics API and the target surface

use comptr::ComPtr;
use winapi::IDXGISwapChain3;
use format::*;
use resource::usage::*;
use error::WinError;

/// link between the graphics API and the target surface
#[derive(Debug)]
pub struct SwapChain {
    ptr: ComPtr<IDXGISwapChain3>,
}

impl SwapChain {
    /// gets the index of this swapchain's current back buffer
    #[inline]
    pub fn get_current_back_buffer_index(&mut self) -> u32 {
        unsafe{
            self.ptr.GetCurrentBackBufferIndex()
        }
    }

    /// attemp to resize the back buffers with given parameters
    #[inline]
    pub fn resize_buffers(&mut self, params: SwapChainResizeDesc) -> Result<(), WinError> {
        let hr = unsafe {
            self.ptr.ResizeBuffers1(
                params.buffer_count,
                params.width,
                params.height,
                params.format,
                params.flags.bits(),
                ::std::ptr::null(),
                ::std::ptr::null_mut()
            )
        };
        WinError::from_hresult(hr)
    }

    // TODO: methods for color spaces

    // TODO: methods for frame latency waitable object

    // TODO: methods for composition swap chain

    /// get the source region size for the swap chain
    #[inline]
    pub fn get_source_size(&mut self) -> Result<(u32, u32), WinError> {
        let mut width = 0;
        let mut height = 0;
        let hr = unsafe {
            self.ptr.GetSourceSize(&mut width, &mut height)
        };
        WinError::from_hresult_or_ok(hr, || (width, height))
    }

    /// set the source region size for the swap chain
    #[inline]
    pub fn set_source_size(&mut self, width: u32, height: u32) -> Result<(), WinError> {
        WinError::from_hresult(unsafe {
            self.ptr.SetSourceSize(width, height)
        })
    }

    /// get the background color for the next `present` method of this swapchain
    #[inline]
    pub fn get_background_color(&mut self) -> Result<[f32; 4], WinError> {
        unsafe {
            let mut ret = ::std::mem::uninitialized();
            let hr = self.ptr.GetBackgroundColor(&mut ret);
            WinError::from_hresult_or_ok(hr, || {
                [ret.r, ret.g, ret.b, ret.a]
            })
        }
    }

    /// change the background color for the next frame
    #[inline]
    pub fn set_background_color(&mut self, r: f32, g: f32, b: f32, a: f32) -> Result<(), WinError> {
        let rgba = ::winapi::DXGI_RGBA{r, g, b, a};
        WinError::from_hresult(unsafe {
            self.ptr.SetBackgroundColor(&rgba)
        })
    }

    // TODO: add `get_core_window`?

    /// get description
    #[inline]
    pub fn get_desc(&mut self) -> Result<SwapChainDesc, WinError> {
        unsafe {
            let mut ret = ::std::mem::uninitialized();
            let hr = self.ptr.GetDesc1(&mut ret);
            WinError::from_hresult_or_ok(hr, || ::std::mem::transmute(ret))
        }
    }

    // TODO: add method for fullscreen description

    /// get the underlying `HWMD` handle for the swapchain object
    #[inline]
    pub fn get_hwnd(&mut self) -> Result<::winapi::HWND, WinError> {
        unsafe {
            let mut ret = ::std::mem::uninitialized();
            let hr = self.ptr.GetHwnd(&mut ret);
            WinError::from_hresult_or_ok(hr, || ret)
        }
    }

    // TODO: add method `get_restrict_to_output`

    // TODO: add method `get_buffer`, `get_containing_output`, `get_fullscreen_state`


    // TODO: add method to get performance statistics about the last render frame

    /// get the number of times that `Present` or `Present1` has been called.
    #[inline]
    pub fn get_last_present_count(&mut self) -> Result<u32, WinError> {
        unsafe {
            let mut ret = 0;
            let hr = self.ptr.GetLastPresentCount(&mut ret);
            WinError::from_hresult_or_ok(hr, || ret)
        }
    }

    /// present a rendered back buffer to the target output.
    /// `sync_interval` specifies how to synchronize presentation of a frame
    /// with the verticle blank. valid values include [0..4].
    // TODO: support dirty rectangles
    #[inline]
    pub fn present(
        &mut self, sync_interval: u32, flags: PresentFlags
    ) -> Result<(), WinError> {
        debug_assert!(sync_interval<=4);
        WinError::from_hresult(unsafe {
            self.ptr.Present(sync_interval, flags.bits())
        })
    }
}

impl From<ComPtr<IDXGISwapChain3>> for SwapChain {
    #[inline]
    fn from(ptr: ComPtr<IDXGISwapChain3>) -> SwapChain {
        SwapChain{ptr}
    }
}

/// description of a swapchain
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct SwapChainDesc {
    /// the resolution width, 0 to use the width of the CA of the target window
    pub width: u32,
    /// the resolution height, 0 to use the height of the CA of the target window
    pub height: u32,
    /// the display format
    pub format: DxgiFormat,
    /// whether the full-screen display mode or the back buffer is stereo
    // TODO: note the relationship with swapchain flip mode
    pub stereo: Bool,
    /// multi-sampling scheme description
    pub sample_desc: SampleDesc,
    /// surface usage and CPU access options for the back buffer.
    /// the back buffer can be used for shader input or target output.
    pub buffer_usage: Usage,
    /// number of buffers in the swap chain
    pub buffer_count: u32,
    /// scaling behavior when the back buffer is presented
    pub scaling: Scaling,
    /// presentation model, as well as how the back buffer would be
    /// handled after calling `swapchain.present()`.
    pub swap_effect: SwapEffect,
    /// transparency behavior
    pub alpha_mode: AlphaMode,
    /// misc flags
    pub flags: SwapChainFlags,
}

impl SwapChainDesc {
    /// create a new `SwapChainDesc` with default parameters
    #[inline]
    pub fn new(format: DxgiFormat) -> SwapChainDesc {
        SwapChainDesc{
            width: 0,
            height: 0, 
            format,
            stereo: false.into(),
            sample_desc: Default::default(),
            buffer_usage: USAGE_RENDER_TARGET_OUTPUT,
            buffer_count: 2,
            scaling: Default::default(),
            swap_effect: Default::default(),
            alpha_mode: Default::default(),
            flags: Default::default(),
        }
    }
}

/// parameters for swapchain resizing
#[derive(Clone, Copy, Debug)]
pub struct SwapChainResizeDesc {
    /// new resolution width, 0 to use the width of the CA of the target window
    pub width: u32,
    /// new resolution height, 0 to use the height of the CA of the target window
    pub height: u32,
    /// new format, `DXGI_FORMAT_UNKNOWN` to preserve exisiting format
    pub format: DxgiFormat,
    /// new buffer counts, 0 to preserve existing counts
    pub buffer_count: u32,
    /// new flags
    pub flags: SwapChainFlags,
    // TODO: add nodes support
    // TODO: add present queue support
}

impl SwapChainResizeDesc {
    /// construct a new resize description with `flags` and default paramters
    #[inline]
    pub fn new(flags: SwapChainFlags) -> SwapChainResizeDesc {
        SwapChainResizeDesc{
            width: 0, height: 0, format: DXGI_FORMAT_UNKNOWN,
            buffer_count: 0, flags
        }
    }
}

/// multi-sampling scheme description
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct SampleDesc {
    /// the number of multisamples per pixel
    pub count: u32,
    /// the image quality level
    pub quality: u32,
}

impl Default for SampleDesc {
    fn default() -> SampleDesc {
        SampleDesc{count: 1, quality: 0}
    }
}

bitflags!{
    /// scaling behavor when the back buffer got presented. 
    #[repr(C)]
    pub struct Scaling: u32 {
        /// back buffer content would be scaled to fill the presentation target
        const SCALING_STRETCH = 0;
        /// back buffer content would appear without scaling, with top edge
        /// aligned with the presentation target.
        const SCALING_NONE = 1;
        /// back buffer content would be scaled to fit the presentation target,
        /// while preserving the aspect ratio, centered with black borders
        const SCALING_ASPECT_RATIO_STRETCH = 2;
    }
}

impl From<Scaling> for ::winapi::DXGI_SCALING {
    fn from(scaling: Scaling) -> Self {
        ::winapi::DXGI_SCALING(scaling.bits())
    }
}

impl Default for Scaling {
    #[inline]
    fn default() -> Scaling {
        SCALING_STRETCH
    }
}

bitflags!{
    /// presentation model, as well as how the back buffer would be
    /// handled after calling `swapchain.present()`.
    /// [more info](https://msdn.microsoft.com/en-us/library/windows/desktop/bb173077%28v=vs.85%29.aspx?f=255&MSPPError=-2147217396)
    #[repr(C)]
    pub struct SwapEffect: u32 {
        /// bitblt, back buffer content would be discarded after presented
        const SWAP_EFFECT_DISCARD = 0;
        /// bitblt, back buffer content would persist after presented,
        /// cannot be used with multisampling
        const SWAP_EFFECT_SEQUENTIAL = 1;
        /// flip, back buffer content would persist after presented,
        /// cannot be used with multisampling
        const SWAP_EFFECT_FLIP_SEQUENTIAL = 3;
        /// flip, back buffer content would be discared after presented,
        /// cannot be used with multisampling and partial presentation
        const SWAP_EFFECT_FLIP_DISCARD = 4;
    }
}

impl Default for SwapEffect {
    fn default() -> SwapEffect {
        SWAP_EFFECT_DISCARD
    }
}

bitflags!{
    /// transparency behavior of a surface
    #[repr(C)]
    pub struct AlphaMode: u32 {
        /// transparency behavior is not specified
        const ALPHA_MODE_UNSPECIFIED = 0;
        /// each color channel is premultiplied by the alpha value
        const ALPHA_MODE_PREMULTIPLIED = 1;
        /// each color channel is not premultiplied by the alpha value
        const ALPHA_MODE_STRAIGHT = 2;
        /// alpha channel would be ignored
        const ALPHA_MODE_IGNORE = 3;
    }
}

impl Default for AlphaMode {
    fn default() -> AlphaMode {
        ALPHA_MODE_UNSPECIFIED
    }
}

bitflags!{
    /// misc flags for swapchain behavior
    #[repr(C)]
    pub struct SwapChainFlags: u32 {
        const SWAP_CHAIN_FLAG_NONE = 0;
        /// turn off fullscreen automatic rotation
        const SWAP_CHAIN_FLAG_NONPREROTATED = 1;
        /// allow switch between fullscreen and windowed with `resize_target`
        const SWAP_CHAIN_FLAG_ALLOW_MODE_SWITCH = 2;
        /// allow `get_dc` on the 0th back buffer
        const SWAP_CHAIN_FLAG_GDI_COMPATIBLE = 4;
        /// OS would support creation only when driver and hardware protection is used?
        const SWAP_CHAIN_FLAG_RESTRICTED_CONTENT = 8;
        const SWAP_CHAIN_FLAG_RESTRICT_SHARED_RESOURCE_DRIVER = 16;
        /// the presented content would only be avaiable for local display
        const SWAP_CHAIN_FLAG_DISPLAY_ONLY = 32;
        /// ensure rendering does not begin while a frame is still being resented
        const SWAP_CHAIN_FLAG_FRAME_LATENCY_WAITABLE_OBJECT = 64;
        /// create a swapchain in the foreground layer for multi-plane rendering
        const SWAP_CHAIN_FLAG_FOREGROUND_LAYER = 128;
        const SWAP_CHAIN_FLAG_FULLSCREEN_VIDEO = 256;
        const SWAP_CHAIN_FLAG_YUV_VIDEO = 512;
        const SWAP_CHAIN_FLAG_HW_PROTECTED = 1024;
        /// enable displays that support variable refresh rates to function
        /// properly when the application presents a swapchain tied to a full
        /// screen borderless window.
        const SWAP_CHAIN_FLAG_ALLOW_TEARING = 2048;
    }
}

impl Default for SwapChainFlags {
    #[inline]
    fn default() -> Self {
        SWAP_CHAIN_FLAG_NONE
    }
}

bitflags!{
    /// options for frame presentation
    pub struct PresentFlags: u32 {
        /// present a frame from each buffer (starting from the current one)
        /// to the output
        const PRESENT_FLAG_NONE = 0;
        /// present a frame from current buffer to the output.
        /// this flag allows vsync instead of typical sequencing
        const PRESENT_FLAG_DO_NOT_SEQUENCE = 0x2;
        /// don't present to the output. intended for use only when switching from idle
        const PRESENT_FLAG_TEST = 0x1;
        /// make the runtime discard outstanding queued frames
        const PRESENT_FLAG_RESTART = 0x4;
        /// make the invocation fail if the calling thread would be blocked
        const PRESENT_FLAG_DO_NOT_WAIT = 0x8;
        /// indicates that presentation content will be shown only on the particular output. The content will not be visible on other outputs.
        const PRESENT_FLAG_RESTRICT_TO_OUTPUT = 0x10;
        /// stereo prefers right-eye viewing instead of right
        const PRESENT_FLAG_STEREO_PREFER_RIGHT = 0x20;
        /// Indicates that the presentation should use the left buffer as a mono buffer.
        const PRESENT_FLAG_STEREO_TEMPORARY_MONO = 0x40;
        // TODO: const PRESENT_USE_DURATION = 0x100;
        /// allow tearing for variable refresh rate displays.
        ///
        /// this flag can be used when:
        /// - the swapchain was reated with the `ALLOW_TEARING` flag
        /// - the `sync_interval` is `0`
        /// - fullscreen borderless window, disabling automatic Alt+Enter...
        const PRESENT_FLAG_ALLOW_TEARING = 0x200;
    }
}

impl Default for PresentFlags {
    #[inline]
    fn default() -> Self {
        PRESENT_FLAG_NONE
    }
}
