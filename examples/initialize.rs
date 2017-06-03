// Copyright 2017 Dasein Phaos aka. Luxko
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! demonstrates initialization boilerplates

extern crate redirect;
extern crate winit;

fn main() {
    // initialize factory
    let mut factory = redirect::factory::Factory::new().unwrap();

    // enumerate adapters
    for mut adapter in factory.enumerate_adapters() {
        // get and print adapter descriptions
        if let Ok(desc) = adapter.get_desc() {
            println!("{}", desc);
        } else {
            println!("can't get adapter description.");
        }

        // enumerate avaiable outputs for this adapter
        for mut output in adapter.enumerate_outputs() {
            // get and print output descriptions
            if let Ok(desc) = output.get_desc() {
                println!("\t{}", desc);
            } else {
                println!("can't get adapter description.");
            }
        }
    }

    // initialize a device with the default adapter
    let mut device = redirect::device::Device::new(
        None, redirect::device::FEATURE_LEVEL_11_0
    ).expect("device initialization failed.");

    // initialize a command queue from this device with default options
    let mut command_queue = device.create_command_queue(
        &Default::default()
    ).expect("command queue creation failed.");

    // initialize `winit` events loop
    let events_loop = winit::EventsLoop::new();
    // initialize `winit` window, get `hwnd` from it
    let window = winit::Window::new(&events_loop).expect(
        "window initialization failed"
    );
    let hwnd = <winit::Window as winit::os::windows::WindowExt>::get_hwnd(&window);
    let hwnd = unsafe { ::std::mem::transmute(hwnd)};

    // create the swapchain from this hwnd
    let mut swapchain = factory.create_swapchain_for_hwnd(
        &command_queue, hwnd, &redirect::swapchain::SwapChainDesc::new(
            redirect::format::DXGI_FORMAT_R8G8B8A8_UNORM
        ), None, None
    ).expect("swap chain creation failed");

    println!("Render loop started..");
    loop {
        let mut interruptted = false;
        events_loop.poll_events(|event| {
            match event {
                winit::Event::WindowEvent { event: winit::WindowEvent::Closed, ..} => {
                    println!("window closed; stopping..");
                    events_loop.interrupt();
                    interruptted = true;
                },
                _ => ()
            }
        });
        if interruptted { break; }
        let _ = swapchain.present(0, Default::default()).expect(
            "presentation failed."
        );
    }
}