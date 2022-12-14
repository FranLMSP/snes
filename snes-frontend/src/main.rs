extern crate imgui_winit_support;

use rfd::FileDialog;
use imgui::*;
use imgui_wgpu::{Renderer, RendererConfig};
use pollster::block_on;

use std::time::Instant;
use winit::{
    dpi::LogicalSize,
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

use snes_frontend::state::State;
extern crate snes_core;
use snes_core::emulator::Emulator;
use snes_frontend::ppu as ppu_render;

fn main() {
    // Windowing state
    let mut state = State::new();
    let mut emulator = Emulator::new();


    // Set up window and GPU
    let event_loop = EventLoop::new();

    let instance = wgpu::Instance::new(wgpu::Backends::PRIMARY);

    let (window, size, surface) = {
        let version = env!("CARGO_PKG_VERSION");

        let window = Window::new(&event_loop).unwrap();
        window.set_inner_size(LogicalSize {
            width: 1280.0,
            height: 720.0,
        });
        window.set_title(&format!("SNES {}", version));
        let size = window.inner_size();

        let surface = unsafe { instance.create_surface(&window) };

        (window, size, surface)
    };

    let hidpi_factor = window.scale_factor();

    let adapter = block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::HighPerformance,
        compatible_surface: Some(&surface),
        force_fallback_adapter: false,
    }))
    .unwrap();

    let (device, queue) =
        block_on(adapter.request_device(&wgpu::DeviceDescriptor::default(), None)).unwrap();

    // Set up swap chain
    let surface_desc = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: wgpu::TextureFormat::Bgra8UnormSrgb,
        width: size.width as u32,
        height: size.height as u32,
        present_mode: wgpu::PresentMode::Mailbox,
    };

    surface.configure(&device, &surface_desc);

    // Set up dear imgui
    let mut imgui = imgui::Context::create();
    let mut platform = imgui_winit_support::WinitPlatform::init(&mut imgui);
    platform.attach_window(
        imgui.io_mut(),
        &window,
        imgui_winit_support::HiDpiMode::Default,
    );
    imgui.set_ini_filename(None);

    let font_size = (12.0 * hidpi_factor) as f32;
    imgui.io_mut().font_global_scale = (1.0 / hidpi_factor) as f32;

    imgui.fonts().add_font(&[FontSource::DefaultFontData {
        config: Some(imgui::FontConfig {
            oversample_h: 1,
            pixel_snap_h: true,
            size_pixels: font_size,
            ..Default::default()
        }),
    }]);

    //
    // Set up dear imgui wgpu renderer
    //
    let clear_color = wgpu::Color {
        r: 0.1,
        g: 0.2,
        b: 0.3,
        a: 1.0,
    };

    let renderer_config = RendererConfig {
        texture_format: surface_desc.format,
        ..Default::default()
    };

    let mut renderer = Renderer::new(&mut imgui, &device, &queue, renderer_config);

    let mut last_frame = Instant::now();

    let mut last_cursor = None;

    // Generate dummy texture
    const WIDTH: usize = 400;
    const HEIGHT: usize = 400;
    let mut data = Vec::with_capacity(WIDTH * HEIGHT);
    for i in 0..WIDTH {
        for j in 0..HEIGHT {
            // Insert RGB values
            data.push(i as u8);
            data.push(j as u8);
            data.push((i + j) as u8);
        }
    }

    let texture = imgui_wgpu::Texture::new(
        &device,
        &renderer,
        imgui_wgpu::TextureConfig {
            label: Some("framebuffer texture"),
            size: wgpu::Extent3d {
                width: WIDTH as u32,
                height: HEIGHT as u32,
                depth_or_array_layers: 1,
            },
            format: Some(wgpu::TextureFormat::Rgba8Unorm),
            ..imgui_wgpu::TextureConfig::default()
        },
    );
    let texture_id = renderer.textures.insert(texture);

    for bgdebug in state.ppudebug.backgrounds.iter_mut() {
        bgdebug.texture_id = Some(
            ppu_render::background_texture(&device, &mut renderer, bgdebug.background)
        );
    }
    

    // Event loop
    event_loop.run(move |event, _, control_flow| {
        *control_flow = if cfg!(feature = "metal-auto-capture") {
            ControlFlow::Exit
        } else {
            ControlFlow::Poll
        };
        match event {
            Event::WindowEvent {
                event: WindowEvent::Resized(_),
                ..
            } => {
                let size = window.inner_size();

                let surface_desc = wgpu::SurfaceConfiguration {
                    usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                    format: wgpu::TextureFormat::Bgra8UnormSrgb,
                    width: size.width as u32,
                    height: size.height as u32,
                    present_mode: wgpu::PresentMode::Mailbox,
                };

                surface.configure(&device, &surface_desc);
            }
            Event::WindowEvent {
                event:
                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                state: ElementState::Pressed,
                                ..
                            },
                        ..
                    },
                ..
            }
            | Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = ControlFlow::Exit;
            }
            Event::MainEventsCleared => window.request_redraw(),
            Event::RedrawEventsCleared => {
                let now = Instant::now();
                imgui.io_mut().update_delta_time(now - last_frame);
                last_frame = now;

                let frame = match surface.get_current_texture() {
                    Ok(frame) => frame,
                    Err(e) => {
                        eprintln!("dropped frame: {:?}", e);
                        return;
                    }
                };
                platform
                    .prepare_frame(imgui.io_mut(), &window)
                    .expect("Failed to prepare frame");
                let ui = imgui.frame();

                {
                    ui.main_menu_bar(|| {
                        ui.menu("Emulator", || {
                            if imgui::MenuItem::new("Load ROM")
                                .build(&ui)
                            {
                                if let Some(path) = FileDialog::new()
                                    .pick_file()
                                {
                                    match emulator.rom.load(&String::from(path.to_str().unwrap())) {
                                        Ok(_) => {},
                                        Err(err) => {
                                            state.error_message.show = true;
                                            state.error_message.message = format!(
                                                "Could not load rom: {}", err,
                                            );
                                        }
                                    }
                                }
                            }
                            ui.separator();
                        });
                        ui.menu("Debug", || {
                            if imgui::MenuItem::new("Show Debug menu")
                                .build(&ui)
                            {
                                state.debug_options.show_debug_window = true;
                            }
                        });
                    });

                    if state.debug_options.show_debug_window {
                        let window = imgui::Window::new("Debug window");
                        window
                            .size([300.0, 100.0], Condition::FirstUseEver)
                            .opened(&mut state.debug_options.show_debug_window)
                            .build(&ui, || {
                                ui.checkbox(
                                    "Enable debugging", 
                                    &mut state.debug_options.is_enabled,
                                );
                                ui.separator();
                                ui.checkbox(
                                    "Show PPU debugging options", 
                                    &mut state.ppudebug.is_enabled,
                                );
                                ui.checkbox(
                                    "Show CPU registers", 
                                    &mut state.debug_options.show_cpu_registers,
                                );
                            });
                    }

                    // Render all debugging stuff
                    if state.debug_options.is_enabled {
                        if state.ppudebug.is_enabled {
                            let window = imgui::Window::new("PPU Debugging options");
                            window
                                .size([300.0, 400.0], Condition::FirstUseEver)
                                .build(&ui, || {
                                    ui.text("Backgrounds:");

                                    for bgdebug in state.ppudebug.backgrounds.iter_mut() {
                                        ui.checkbox(
                                            format!("Show {:?}", bgdebug.background),
                                            &mut bgdebug.is_enabled,
                                        );
                                    }
                                });
                            for bgdebug in state.ppudebug.backgrounds.iter_mut() {
                                ppu_render::background_window(
                                    bgdebug,
                                    &emulator.bus.ppu.registers,
                                    &ui,
                                    &mut renderer,
                                    &queue,
                                )
                            }
                        }
                    }

                    // Render emulator framebuffer
                    {
                        let tex = renderer.textures.get_mut(texture_id).unwrap();
                        tex.write(&queue, &vec![0xAA; 400 * 400 * 4], 400, 400);
                        let game_window = imgui::Window::new("Game");
                        game_window
                            .size([600.0, 600.0], Condition::FirstUseEver)
                            .collapsible(false)
                            .build(&ui, || {
                                let game_image = imgui::Image::new(texture_id, [400.0, 400.0]);
                                game_image.build(&ui);
                            });
                    }

                    if state.error_message.show {
                        let window = imgui::Window::new("Error");
                        window
                            .size([300.0, 100.0], Condition::Always)
                            .collapsible(false)
                            .build(&ui, || {
                                ui.text(&state.error_message.message);
                                if ui.button("Ok") {
                                    state.error_message.show = false
                                }
                            });
                    }

                }

                let mut encoder: wgpu::CommandEncoder =
                    device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

                if last_cursor != Some(ui.mouse_cursor()) {
                    last_cursor = Some(ui.mouse_cursor());
                    platform.prepare_render(&ui, &window);
                }

                let view = frame
                    .texture
                    .create_view(&wgpu::TextureViewDescriptor::default());
                let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: None,
                    color_attachments: &[wgpu::RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(clear_color),
                            store: true,
                        },
                    }],
                    depth_stencil_attachment: None,
                });

                renderer
                    .render(ui.render(), &queue, &device, &mut rpass)
                    .expect("Rendering failed");

                drop(rpass);

                queue.submit(Some(encoder.finish()));

                frame.present();
            }
            _ => (),
        }

        platform.handle_event(imgui.io_mut(), &window, &event);
    });
}
