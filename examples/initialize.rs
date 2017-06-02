// Copyright 2017 Dasein Phaos aka. Luxko
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! demonstrates initialization boilerplates

extern crate redirect;
// extern crate winit;

fn main() {
    // // initialize `winit` events loop
    // let events_loop = winit::EventsLoop::new();
    // // initialize `winit` window, get `hwnd` from it
    // let window = winit::Window::new(&events_loop);
    // let hwnd = <winit::Window as winit::os::windows::WindowExt>::get_hwnd(&window);

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
}