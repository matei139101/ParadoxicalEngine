use crate::{
    prelude::*,
    resources::entities::{base_controller::BaseController, base_cube::BaseCube},
};
use glam::vec3;
use std::{
    any::Any,
    sync::{Arc, Mutex},
};
use tokio::sync::{mpsc::UnboundedSender, oneshot};
use winit::{
    application::ApplicationHandler,
    event::{DeviceEvent, DeviceId, WindowEvent},
    event_loop::ActiveEventLoop,
    keyboard::PhysicalKey,
    window::{Window, WindowId},
};

use crate::engine::{
    services::{
        entity_service::entity_events::CreateEntityEvent,
        vulkan_service::vulkan_events::{CreateVulkanInstanceEvent, VulkanDrawEvent},
    },
    utils::structs::transform::Transform,
    vulkan::{structs::viewport::ViewportInfo, vulkan_container::VulkanContainer},
};

pub struct App {
    window: Option<Arc<Window>>,
    viewport_info: Option<ViewportInfo>,
    async_sender: UnboundedSender<Box<dyn Any + Send + Sync>>,
    input_sender: UnboundedSender<DeviceEvent>,
}

impl App {
    pub fn new(
        async_sender: UnboundedSender<Box<dyn Any + Send + Sync>>,
        input_sender: UnboundedSender<DeviceEvent>,
    ) -> Self {
        App {
            window: Default::default(),
            viewport_info: Default::default(),
            async_sender,
            input_sender,
        }
    }
}

impl ApplicationHandler for App {
    //[TO-DO]: This needs to be cleaned up and have dev stuff removed from it.
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attributes = Window::default_attributes();
        self.window = Some(event_loop.create_window(window_attributes).unwrap().into());
        self.window.as_ref().unwrap().request_redraw();
        self.viewport_info = Some(ViewportInfo::new(
            [0.0, 0.0],
            [
                self.window.as_ref().unwrap().inner_size().width as f32,
                self.window.as_ref().unwrap().inner_size().height as f32,
            ],
        ));

        let vulkan_container = Arc::new(Mutex::new(VulkanContainer::new(
            event_loop,
            self.window.as_ref().unwrap().clone(),
            self.viewport_info.as_ref().unwrap(),
        )));

        let create_vk_container_message = CreateVulkanInstanceEvent {
            vulkan_container: vulkan_container.clone(),
        };

        let _ = self
            .async_sender
            .send(Box::new(create_vk_container_message));

        //For testing purposes
        let cube1_transform = Transform::new(vec3(-1.0, 0.0, 0.0), vec3(0.0, 0.0, 0.0));
        let cube2_transform = Transform::new(vec3(1.0, 0.0, 0.0), vec3(0.0, 0.0, 0.0));
        let controller_transform = Transform::new(vec3(0.0, 0.0, -5.0), vec3(0.0, 0.0, 0.0));

        let _ = self.async_sender.send(Box::new(CreateEntityEvent {
            entity: Box::new(BaseCube::new("Base cube 1".to_string(), cube1_transform)),
        }));

        let _ = self.async_sender.send(Box::new(CreateEntityEvent {
            entity: Box::new(BaseCube::new("Base cube 2".to_string(), cube2_transform)),
        }));

        let _ = self.async_sender.send(Box::new(CreateEntityEvent {
            entity: Box::new(BaseController::new(
                "Player 1".to_string(),
                controller_transform,
                1,
            )),
        }));

        //[TO-DO]: Locking the mouse for now. Needs to be thought over if it's meant to be here or elsewhere.
        /*
        self.window
            .as_mut()
            .unwrap()
            .set_cursor_grab(winit::window::CursorGrabMode::Locked)
            .unwrap();
        */
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                log!(Self, High, "The close button was pressed; stopping");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                let (confirmation_sender, confirmation_receiver) = oneshot::channel::<()>();
                let draw_message = Box::new(VulkanDrawEvent {
                    player_id: 1,
                    confirmation_sender: Arc::new(Mutex::new(Some(confirmation_sender))),
                });

                let _ = self.async_sender.send(draw_message);
                confirmation_receiver.blocking_recv().unwrap();

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
        event: DeviceEvent,
    ) {
        let _ = self.input_sender.send(event);
    }
}
