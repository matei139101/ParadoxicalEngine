use crate::prelude::*;

pub trait Entity: Send + Sync {
    fn load(
        &self,
        repository: Arc<EntityRepository>,
        async_sender: UnboundedSender<Box<dyn Any + Send + Sync>>,
    );
}
