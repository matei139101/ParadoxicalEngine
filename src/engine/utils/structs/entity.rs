use std::{any::Any, sync::Arc};

use tokio::sync::mpsc::UnboundedSender;

use crate::engine::repositories::entity_repository::EntityRepository;

pub trait Entity: Clone {
    fn load(&self, repository: Arc<EntityRepository>, async_sender: UnboundedSender<Box<dyn Any + Send + Sync>>);
}

