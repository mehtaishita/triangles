use gfx_hal::Instance;
use std::borrow::Cow;
use wgpu::TextureFormat;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

pub async fn run(event_loop: EventLoop<()>, window: Window) {
    let size = window.inner_size();
    let instance = wgpu::Instance::new(wgpu::Backends::METAL); // Apple M1 Chip
    let surface = unsafe { instance.create_surface(&window).unwrap() };
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        })
        .await
        .expect("Failed to find appropriate adapter");

    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                features: wgpu::Features::empty(), // simple triangle, so not much else needed
                limits: wgpu::Limits::default(),   // default supports most
            },
            None,
        )
        .await
        .expect("Failed to create device");

    // let format = surface.texture;
    let mut config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT, // how the surface texture will be used
        format: TextureFormat::Bgra8Unorm,             // not sure what to use here yet
        width: size.width,                             // size defined as window size earlier
        height: size.height,                           // not set to 0, otherwise will crash
        present_mode: wgpu::PresentMode::Mailbox, // presentation frames in a single-frame queue, same display until the next one - which replaces old frame
        alpha_mode: wgpu::CompositeAlphaMode::Auto, // chooses either Opaque or Inherit automatically
        view_formats: (),                           //??
    };
    surface.configure(&device, &config);
}

fn main() {
    let event_loop = EventLoop::new();
    let window = Window::new(&event_loop).unwrap();
    window.set_title("my window");
    env_logger::init();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => {}
        }
    });
}
