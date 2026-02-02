/*
use crate::prelude::*;

use tokio::sync::mpsc::UnboundedSender;

use crate::engine::event_bus::event_bus::EventBus;

pub struct Synchronizer {
    event_bus_ptr: Arc<EventBus>,
    async_sender: UnboundedSender<Box<dyn Any + Send + Sync>>,
}

impl Synchronizer {
    pub fn new(        event_bus_ptr: Arc<EventBus>,
        async_sender: UnboundedSender<Box<dyn Any + Send + Sync>>,)
}
*/
