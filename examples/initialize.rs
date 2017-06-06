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

use redirect::descriptor::DescriptorHeap;

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

    // create the rtv heap for our backbuffers
    let mut rtv_heap = redirect::descriptor::DescriptorHeapBuilder::new(
        2
    ).build_rtv_heap(&mut device).expect(
        "rtv heap creation failed"
    );

    // create rtvs on this heap for the two back buffers
    let mut backbuffers = [
        swapchain.get_buffer(0).expect("failed to get buffer 0"), 
        swapchain.get_buffer(1).expect("failed to get buffer 1"),
    ];
    rtv_heap.create_rtv(&mut device, Some(&mut backbuffers[0]), None, 0);
    rtv_heap.create_rtv(&mut device, Some(&mut backbuffers[1]), None, 1);

    // create a command allocator for direct command list
    let mut allocator = device.create_direct_command_allocator(
    ).expect("command allocator creation failed");

    // create a direct command list and start recording
    let mut cmdlist = device.create_direct_command_list::<
        redirect::pipeline::GraphicsPipelineState
    >(
        0, &mut allocator, None
    ).expect("command list creation failed").close().expect(
        "command list initial close failed"
    );

    // create fence for synchronization
    let mut fence_count = 2;
    let mut fence = device.create_fence(fence_count, Default::default()).expect(
        "failed to create the fence"
    );

    println!("Render loop started..");
    let start_time = std::time::Instant::now();
    let mut backbuffer_idx = swapchain.get_current_back_buffer_index();
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

        // every frame we clear the render target to a diffrent gray scale color,
        // depending on time
        while fence.get_completed_value() < fence_count { }
        allocator.reset().expect("command allocator resetting failed");
        let subsec = (start_time.elapsed().subsec_nanos() as f32)/1.0e9f32;
        let color = (0.5 - subsec).abs();
        let colors = [color, color, color, 1.0];
        let mut recording = cmdlist.start_graphics(&mut allocator, None).expect(
            "command list start recording failed"
        );
        let mut barriers = redirect::resource::ResourceBarriersBuilder::new();
        barriers.push(redirect::resource::ResourceBarrier::transition(
            &backbuffers[backbuffer_idx as usize], 0,
            redirect::resource::RESOURCE_STATE_PRESENT,
            redirect::resource::RESOURCE_STATE_RENDER_TARGET
        ));
        recording.resource_barriers(&barriers);
        recording.clear_rtv(
            rtv_heap.get_cpu_handle(backbuffer_idx),
            &colors, None
        );
        let mut barriers = redirect::resource::ResourceBarriersBuilder::new();
        barriers.push(redirect::resource::ResourceBarrier::transition(
            &backbuffers[backbuffer_idx as usize], 0,
            redirect::resource::RESOURCE_STATE_RENDER_TARGET,
            redirect::resource::RESOURCE_STATE_PRESENT
        ));
        recording.resource_barriers(&barriers);
        cmdlist = recording.close().expect(
            "command list close recording failed"
        );
        // command_queue.wait(&fence, fence_count-1).expect("waiting failed");
        command_queue.execute_command_list(&cmdlist);
        fence_count+=1;
        command_queue.signal(&fence, fence_count).expect("signaling failed");
        
        // present next frame
        swapchain.present(0, Default::default()).expect(
            "presentation failed."
        );
        // update back buffer count
        backbuffer_idx = (backbuffer_idx + 1)%2;
    }
}
