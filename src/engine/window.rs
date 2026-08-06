use crate::prelude::*;
use winit::{
    application::ApplicationHandler,
    event::{DeviceEvent, DeviceId, WindowEvent},
    event_loop::ActiveEventLoop,
    keyboard::PhysicalKey,
};

pub struct Window {
    window: Option<Arc<winit::window::Window>>,
    service_locator: Arc<ServiceLocator>,
}

impl Window {
    pub fn new(service_locator: Arc<ServiceLocator>) -> Self {
        Self {
            window: Default::default(),
            service_locator,
        }
    }
}

impl ApplicationHandler for Window {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attributes = winit::window::Window::default_attributes();
        self.window = Some(event_loop.create_window(window_attributes).unwrap().into());
        self.window.as_ref().unwrap().request_redraw();

        let viewport_info = ViewportInfo::new(
            [0.0, 0.0],
            [
                self.window.as_ref().unwrap().inner_size().width as f32,
                self.window.as_ref().unwrap().inner_size().height as f32,
            ],
        );
        let vulkan = Vulkan::new(
            event_loop,
            self.window.as_ref().unwrap().clone(),
            &viewport_info,
        );

        self.service_locator
            .get_render_service()
            .set_graphics_api(Box::new(vulkan));
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                log!(Self, High, "The close button was pressed; stopping");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                self.window.as_ref().unwrap().request_redraw();
            }

            WindowEvent::Resized(_size) => {
                log!(Self, High, "Window Resized");
            }
            WindowEvent::KeyboardInput {
                device_id: _,
                event,
                is_synthetic: _,
            } => {
                if event.physical_key == PhysicalKey::Code(winit::keyboard::KeyCode::KeyQ) {
                    event_loop.exit();
                }
            }
            _ => (),
        }
    }

    //[TO-DO]: For camera turning, will need to be cleaned up later.
    fn device_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _device_id: DeviceId,
        _event: DeviceEvent,
    ) {
    }
}
