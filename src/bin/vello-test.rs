use vello::{peniko::*, wgpu, Renderer, RendererOptions, Scene, Surface};
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};

fn main() {
    colog::init();

    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);
    event_loop.set_control_flow(ControlFlow::Wait);

    let mut app = App::default();
    let _ = event_loop.run_app(&mut app);
}


#[derive(Default)]
struct App {
    window: Option<Window>,
    renderer: Option<vello::Renderer>,
    surface: Option<vello::Surface>,
    scene: Option<vello::Scene>,
}

impl ApplicationHandler for App {

    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let mut attribs = Window::default_attributes();
        attribs.title = "Vello Font Test".to_string();
        let window = Some(event_loop.create_window(attribs).unwrap());


// STEP 1: Initialize the GPU context and Vello renderer

        // Initialize GPU context and Vello renderer
        let instance = wgpu::Instance::default();
        let surface = unsafe { instance.create_surface(&window) }?;

        let mut renderer = Renderer::new(&device, &queue, RendererOptions::default());
        let size = window.inner_size();
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_supported_formats(&adapter)[0],
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            view_formats: vec![],
        };
        surface.configure(&device, &config);


// STEP 2: Create a scene

        let mut scene = Scene::default();
        scene.append(&Draw::Fill(Fill::new(
            FillStyle::default(),
            Transform::identity(),
            Rect::new(100.0, 100.0, 300.0, 300.0).into_path(),
            None,
        )));

        self.window = Some(window);
        self.renderer = Some(renderer);
        self.surface = Some(surface);
        self.scene = Some(scene);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            },
            WindowEvent::RedrawRequested => {

// STEP 3: Render the scene to the window when needed?

                // self.window.as_ref().unwrap().request_redraw();
                // renderer.render_to_surface(&scene, &mut surface).unwrap();
                // surface.present();

                /// I wnat to render a scene to the window here.. where do I define the scene, and where do I initialize vello?
            },
            _ => ()
        }
    }
}


// STEP 4: How to add texts.. (we figure out when we actually can render stuff)
