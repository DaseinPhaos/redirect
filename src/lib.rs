// Copyright 2017 Dasein Phaos aka. Luxko
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![cfg(windows)]

//! `redirect` aims to be a lightweight and type-safe wrapper around the DirectX12 API for the Rust Programming Language.

#[macro_use]
extern crate bitflags;
extern crate winapi;
extern crate d3d12;
extern crate dxguid;
extern crate dxgi;
extern crate d3dcompiler;
extern crate kernel32;
extern crate smallvec;

mod comptr;
pub mod error;
pub mod format;
pub mod swapchain;
pub mod resource;
pub mod device;
pub mod factory;
pub mod command;
pub mod pipeline;
pub mod shader;
pub mod fence;
pub mod descriptor;
pub mod event;
