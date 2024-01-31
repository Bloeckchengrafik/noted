use raw_window_handle::WindowHandle;
use rendering_engine::Size;

pub mod rendering_engine;

pub struct MyRenderer<'a> {
    surface: wgpu::Surface<'a>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: Size,
    handle: WindowHandle<'a>,
}

impl <'a> MyRenderer<'a> {
    fn new(handle: WindowHandle<'a>) -> Self {
        
    }
}

impl rendering_engine::SoftwareRenderer for MyRenderer<'_> {
    fn redraw(&self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}


pub struct NotedApplication<Renderer: rendering_engine::SoftwareRenderer, Engine: rendering_engine::RenderingEngine<Renderer>> {
    pub engine: Engine,
    pub renderer: Renderer,
}

impl <Renderer: rendering_engine::SoftwareRenderer, Engine: rendering_engine::RenderingEngine<Renderer>> NotedApplication<Renderer, Engine> {
    pub fn new(engine: Engine, renderer: Renderer) -> Self {
        Self {
            engine,
            renderer,
        }
    }

    pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.engine.start_loop(&self.renderer)
    }
}


pub fn init() {
    env_logger::init();
}