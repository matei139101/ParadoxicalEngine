use std::{
    any::Any,
    thread::{self},
};
use tokio::{
    self,
    sync::mpsc::{self, UnboundedReceiver, UnboundedSender},
};
use winit::event_loop::ControlFlow;
use winit::event_loop::EventLoop;

use crate::engine::{
    app::App,
    event_bus::event_bus::EventBus,
    services::{
        entity_service::entity_service::EntityService,
        vulkan_service::vulkan_service::VulkanService,
    },
};
use crate::prelude::*;
mod engine;
mod prelude;

fn main() {
    let (async_sender, async_receiver) = mpsc::unbounded_channel::<Box<dyn Any + Send + Sync>>();

    make_async_runner(async_sender.clone(), async_receiver);

    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    //[TO-DO]: Create the synchronizer.

    let mut app = App::new(async_sender);
    let _ = event_loop.run_app(&mut app);
}

fn make_async_runner(
    async_sender: UnboundedSender<Box<dyn Any + Send + Sync>>,
    async_receiver: UnboundedReceiver<Box<dyn Any + Send + Sync>>,
) {
    thread::spawn(move || {
        log!(High, "Starting async runtime");
        let async_runtime = tokio::runtime::Runtime::new().unwrap();

        let event_bus = EventBus::new();
        let _vulkan_component = VulkanService::new(event_bus.clone());
        let _entity_component = EntityService::new(event_bus.clone(), async_sender.clone());

        async_runtime.block_on(async {
            EventBus::run(event_bus.clone(), async_receiver).await;
        })
    });
}
