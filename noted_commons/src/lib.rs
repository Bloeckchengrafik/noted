use std::time::Instant;

use winit::{
    dpi::PhysicalSize,
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::{Theme, Window, WindowBuilder},
};

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct NotedRenderingEngine {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: PhysicalSize<u32>,
    window: Window,
    mouse_pos: (f64, f64),


    hue: f32,
    sat: f32
}

impl NotedRenderingEngine {
    async fn new(window: Window) -> Self {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        let surface = unsafe { instance.create_surface(&window) }.unwrap();

        let all_adapters = instance.enumerate_adapters(wgpu::Backends::all());

        let adapter = all_adapters
            .filter(|adapter| adapter.is_surface_supported(&surface))
            .next()
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                    label: None,
                },
                None, // Trace path
            )
            .await
            .unwrap();

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .filter(|it| it.is_srgb())
            .next()
            .unwrap_or(surface_caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };

        surface.configure(&device, &config);

        NotedRenderingEngine {
            surface,
            device,
            queue,
            config,
            size,
            window: window,
            mouse_pos: (0.0, 0.0),
            hue: 0.0,
            sat: 0.0
        }
    }

    fn window(&self) -> &Window {
        &self.window
    }

    fn resize(&mut self, new_size: PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    fn input(&mut self, event: &WindowEvent) -> bool {
         match event {
            WindowEvent::CursorMoved { position, .. } => {
                self.mouse_pos = (position.x, position.y);
                true
            },
             _ => false,
         }
    }

    fn update(&mut self) {
        // Hue is 0-360 (mouse x)
        // Saturation is 0-1 (mouse y)
        self.hue = (self.mouse_pos.0 / self.size.width as f64) as f32 * 360.0;
        self.sat = (self.mouse_pos.1 / self.size.height as f64) as f32;
    }

    fn redraw(&self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let h = self.hue as f64;
            let s = self.sat as f64;
            let v = 0.5;

            let c = v * s;
            let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());

            let (r, g, b) = match h {
                h if h < 60.0 => (c, x, 0.0),
                h if h < 120.0 => (x, c, 0.0),
                h if h < 180.0 => (0.0, c, x),
                h if h < 240.0 => (0.0, x, c),
                h if h < 300.0 => (x, 0.0, c),
                h if h < 360.0 => (c, 0.0, x),
                _ => (0.0, 0.0, 0.0),
            };

            let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r,
                            g,
                            b,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}

pub async fn run_noted() {
    env_logger::init();

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title(format!("Noted! {}", VERSION))
        .with_min_inner_size(winit::dpi::LogicalSize::new(640.0, 480.0))
        .with_theme(Some(Theme::Dark))
        .build(&event_loop)
        .unwrap();

    let mut engine = NotedRenderingEngine::new(window).await;

    event_loop.run(move |base_event, _, control_flow| {
        *control_flow = winit::event_loop::ControlFlow::WaitUntil(
            Instant::now() + std::time::Duration::from_millis(32), // 30 FPS if we're not doing anything
        );

        match base_event {
            Event::WindowEvent {
                event, window_id, ..
            } if engine.window().id() == window_id => {
                if !engine.input(&event) {
                    // Prioritize input
                    match event {
                        WindowEvent::CloseRequested => {
                            *control_flow = winit::event_loop::ControlFlow::Exit;
                        }
                        WindowEvent::Resized(new_size) => {
                            engine.resize(new_size);
                        }
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            engine.resize(*new_inner_size);
                        }
                        _ => {}
                    }
                }
            }
            Event::RedrawRequested(window_id) if engine.window().id() == window_id => {
                engine.update();
                match engine.redraw() {
                    Ok(_) => (),
                    Err(wgpu::SurfaceError::Lost) => {
                        engine.resize(engine.size);
                    }
                    Err(wgpu::SurfaceError::OutOfMemory) => {
                        *control_flow = winit::event_loop::ControlFlow::Exit;
                    }
                    Err(e) => eprintln!("{:?}", e),
                }
            },
            Event::MainEventsCleared => {
                engine.window().request_redraw();
            }
            _ => (),
        }
    });
}
