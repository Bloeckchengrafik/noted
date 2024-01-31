use noted_commons::MyRenderer;

mod rendering;

fn main() {
    let engine = rendering::LinuxRenderingEngineImpl::new();
    let renderer = MyRenderer;

    let app = noted_commons::NotedApplication::new(engine, renderer);
    app.run().unwrap();
}
