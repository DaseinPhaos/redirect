// Copyright 2017 Dasein Phaos aka. Luxko
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! demonstrates how to draw a triangle on the screen

extern crate redirect;
extern crate winit;

use redirect::descriptor::DescriptorHeap;
use redirect::command::GraphicsCommandList;

#[repr(C)]
#[derive(Copy, Debug, Clone)]
struct Vertex {
    position: [f32; 3],
    color: [f32; 4],
}

fn main() {
    // initialize factory
    let mut factory = redirect::factory::Factory::new().unwrap();

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
    let (width, height) = window.get_inner_size().expect(
        "can get window size"
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

    // create dsv heap and dsv
    let mut dsv_heap = redirect::descriptor::DescriptorHeapBuilder::new(
        1
    ).build_dsv_heap(&mut device).expect(
        "dsv heap creation failed"
    );
    let mut ds_buffer = device.create_committed_resource(
        &Default::default(), Default::default(),
        &redirect::resource::ResourceDesc::tex2d(
            width as u64, height, 1, 1,
            redirect::format::DXGI_FORMAT_D24_UNORM_S8_UINT,
            redirect::resource::RESOURCE_FLAG_ALLOW_DEPTH_STENCIL,
            Default::default()
        ),
        redirect::resource::RESOURCE_STATE_DEPTH_WRITE
    ).expect("ds buffer creation failed");
    dsv_heap.create_dsv(&mut device, Some(ds_buffer.as_raw()), None, 0);

    // create rtvs on this heap for the two back buffers
    let mut backbuffers = [
        swapchain.get_buffer(0).expect("failed to get buffer 0"), 
        swapchain.get_buffer(1).expect("failed to get buffer 1"),
    ];
    rtv_heap.create_rtv(&mut device, Some(&mut backbuffers[0]), None, 0);
    rtv_heap.create_rtv(&mut device, Some(&mut backbuffers[1]), None, 1);

    // create vertex buffer
    let mut vertex_buffer = device.create_committed_resource(
        &redirect::resource::HeapProperties::new(
            redirect::resource::HEAP_TYPE_UPLOAD
        ),
        Default::default(),
        &redirect::resource::ResourceDesc::buffer(
            256, Default::default(), Default::default()
        ),
        Default::default()
    ).expect("vertex buffer creation failed");

    // upload to vertex buffer
    unsafe {
        let triangle = [
            Vertex{position: [0.0, 0.7, 0.2], color: [0.7, 0.6, 0.5, 0.5]},
            Vertex{position: [-0.5, -0.1, 0.2], color: [0.2, 0.3, 0.4, 1.0]},
            Vertex{position: [0.5, -0.7, 0.2], color: [0.1, 0.3, 0.1, 1.0]},
        ];
        let ptr = vertex_buffer.as_raw().map(0, None).expect("mapping failed") as *mut [Vertex; 3];
        std::ptr::copy(&triangle, ptr, 1);
        vertex_buffer.as_raw().unmap(0, None);
    }

    // create vertex buffer view
    let vbv = redirect::pipeline::ia::VertexBufferView{
        location: vertex_buffer.as_raw().get_gpu_vaddress(),
        size: std::mem::size_of::<Vertex>() as u32 * 3,
        stride: std::mem::size_of::<Vertex>() as u32,
    };

    // create input layout description for the vertex
    let pos_cstr = std::ffi::CString::new("POS").unwrap();
    let color_cstr = std::ffi::CString::new("COLOR").unwrap();

    // create view port and scissor rects
    let viewport = redirect::format::Viewport::new(
        width as f32, height as f32
    );

    let scissor = redirect::format::Rect{
        left: 0, top: 0, right: width as _, bottom: height as _
    };

    // create shaders
    let mut file = std::fs::File::open("examples/triangle.hlsl").expect("shader file doesn't exist");
    let mut buffer = Vec::new();
    <_ as std::io::Read>::read_to_end(&mut file, &mut buffer).expect("reading failed");
    let ps_entry = std::ffi::CString::new("PSMain").unwrap();
    let vs_entry = std::ffi::CString::new("VSMain").unwrap();
    let mut builder = redirect::shader::ShaderBuilder::new(buffer.as_ref(), vs_entry.as_ref());
    let vs = builder.build_vs().expect("VS creation failed");
    builder.entry_point = ps_entry.as_ref();
    let ps = builder.build_ps().expect("PS creation failed");

    // create root signature and pso
    let rootsig = redirect::pipeline::rootsig::RootSigBuilder::new().build(&mut device, 0).expect("root signature creation failed");
    let mut psod = redirect::pipeline::GraphicsPipelineStateBuilder::new(&rootsig);
    psod.vs = Some(vs);
    psod.ps = Some(ps);
    psod.input_layout.elements.push(redirect::pipeline::ia::InputElementDesc::new(
        &pos_cstr, redirect::format::DXGI_FORMAT_R32G32B32_FLOAT
    ));
    psod.input_layout.elements.push(redirect::pipeline::ia::InputElementDesc::new(
        &color_cstr, redirect::format::DXGI_FORMAT_R32G32B32A32_FLOAT
    ));
    psod.depth_stencil_state.depth = true.into();
    psod.rtv_formats[0] = backbuffers[0].get_desc().format;
    let pso = psod.build(&mut device).expect("PSO creation failed");

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

    // create fence and event for synchronization
    let mut fence_count = 2;
    let mut fence = device.create_fence(fence_count, Default::default()).expect(
        "failed to create the fence"
    );
    let fence_event = redirect::event::Event::default();

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
        while fence.get_completed_value() < fence_count {
            fence_event.wait().expect("wait event failed");
        }
        allocator.reset().expect("command allocator resetting failed");
        let subsec = (start_time.elapsed().subsec_nanos() as f32)/1.0e9f32;
        let color = (0.5 - subsec).abs();
        let colors = [color, color, color, 1.0];
        let mut recording = cmdlist.start_graphics(&mut allocator, Some(&pso)).expect(
            "command list start recording failed"
        );
        recording.ia_set_primitive_topology(
            Default::default()
        );
        recording.ia_set_vbvs(0, &[vbv]);
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
        recording.clear_dsv(
            dsv_heap.get_cpu_handle(0),
            Default::default(),
            1.0, 0, None
        );
        recording.om_set_rtv_dsv_continuous(
            &mut rtv_heap, backbuffer_idx, 1, dsv_heap.get_cpu_handle(0)
        );
        recording.rs_set_viewports(&[viewport]);
        recording.rs_set_scissors(&[scissor]);
        recording.draw(3, 1, 0, 0);
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
        unsafe { command_queue.execute_command_list(&cmdlist); }
        fence_count+=1;
        command_queue.signal(&fence, fence_count).expect("signaling failed");
        
        // present next frame
        swapchain.present(0, Default::default()).expect(
            "presentation failed."
        );
        fence.set_event_on(fence_count, &fence_event).expect(
            "Set event failed."
        );
        // update back buffer count
        backbuffer_idx = (backbuffer_idx + 1)%2;
    }
}
