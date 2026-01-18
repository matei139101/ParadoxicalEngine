use std::{
    any::Any,
    sync::{Arc, Mutex},
    thread::{self},
};
use tokio::{
    self,
    sync::mpsc::{self, UnboundedReceiver, UnboundedSender},
};
use winit::event_loop::EventLoop;
use winit::{event::DeviceEvent, event_loop::ControlFlow};

use crate::engine::{
    app::App,
    event_bus::event_bus::EventBus,
    services::{
        entity_service::entity_service::EntityService, input_service::input_service::InputService,
        vulkan_service::vulkan_service::VulkanService,
    },
    utils::structs::repositories::Repositories,
};
use crate::prelude::*;
mod engine;
mod prelude;

fn main() {
    let (async_sender, async_receiver) = mpsc::unbounded_channel::<Box<dyn Any + Send + Sync>>();
    let (input_sender, input_receiver) = mpsc::unbounded_channel::<DeviceEvent>();

    let event_bus = EventBus::new();

    let repositories: Arc<Repositories> = Arc::new(Repositories::new());

    make_services(
        repositories.clone(),
        event_bus.clone(),
        async_sender.clone(),
        input_receiver,
    );

    let app = make_app(async_sender.clone(), input_sender.clone());

    start_event_bus_thread(event_bus, async_receiver);
    start_window_thread(app);
}

type ThreadSafe<T> = Arc<Mutex<T>>;
fn make_services(
    repositories: Arc<Repositories>,
    event_bus_ptr: Arc<EventBus>,
    async_sender: UnboundedSender<Box<dyn Any + Send + Sync>>,
    input_receiver: UnboundedReceiver<DeviceEvent>,
) -> (
    ThreadSafe<VulkanService>,
    ThreadSafe<EntityService>,
    ThreadSafe<InputService>,
) {
    (
        VulkanService::new(event_bus_ptr.clone()),
        EntityService::new(
            repositories.clone(),
            event_bus_ptr.clone(),
            async_sender.clone(),
        ),
        InputService::new(repositories.clone(), input_receiver),
    )
}

fn make_app(
    async_sender: UnboundedSender<Box<dyn Any + Send + Sync>>,
    input_sender: UnboundedSender<DeviceEvent>,
) -> (App, EventLoop<()>) {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let app = App::new(async_sender, input_sender);
    (app, event_loop)
}

fn start_event_bus_thread(
    event_bus_ptr: Arc<EventBus>,
    async_receiver: UnboundedReceiver<Box<dyn Any + Send + Sync>>,
) {
    thread::spawn(move || {
        log!(High, "Starting async runtime");
        let async_runtime = tokio::runtime::Runtime::new().unwrap();

        async_runtime.block_on(async {
            EventBus::run(event_bus_ptr.clone(), async_receiver).await;
        })
    });
}

fn start_window_thread(mut app: (App, EventLoop<()>)) {
    let _ = app.1.run_app(&mut app.0);
}
