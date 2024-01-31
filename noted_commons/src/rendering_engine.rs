pub trait SoftwareRenderer {
    fn redraw(&self) -> Result<(), Box<dyn std::error::Error>>;
}

pub trait RenderingEngine<T: SoftwareRenderer> {
    fn start_loop(&self, software_renderer: &T) -> Result<(), Box<dyn std::error::Error>>;
}

pub struct Size(pub u32, pub u32);