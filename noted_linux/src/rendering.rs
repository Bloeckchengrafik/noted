use std::time::Instant;

use noted_commons::rendering_engine::{RenderingEngine, SoftwareRenderer};
use winit::{
    event::{Event, WindowEvent::*}, event_loop::EventLoop, window::{Theme, WindowBuilder}
};

pub(crate) struct LinuxRenderingEngineImpl;

impl LinuxRenderingEngineImpl {
    pub fn new() -> Self {
        Self
    }
}

impl <Renderer : SoftwareRenderer> RenderingEngine<Renderer> for LinuxRenderingEngineImpl {
    fn start_loop(&self, renderer: &Renderer) -> Result<(), Box<dyn std::error::Error>>{
        let event_loop = EventLoop::new().unwrap();
        let window = WindowBuilder::new()
            .with_title("Noted! on Linux")
            .with_min_inner_size(winit::dpi::LogicalSize::new(640.0, 480.0))
            .with_theme(Some(Theme::Dark))
            .build(&event_loop)
            .unwrap();

        event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);

        event_loop.run(move |event, target| {
            target.set_control_flow(winit::event_loop::ControlFlow::WaitUntil(
                Instant::now() + std::time::Duration::from_millis(32), // 30 FPS if
            ));

            match event {
                Event::WindowEvent { event, .. } => match event {
                    CloseRequested => {
                        target.exit();
                    }
                    RedrawRequested => {
                        renderer.redraw().unwrap();
                    }
                    _ => (),
                },
                Event::AboutToWait => window.request_redraw(),
                _ => (),
            }
        })?;

        Ok(())
    }
}
