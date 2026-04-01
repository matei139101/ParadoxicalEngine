use crate::{
    prelude::*,
    resources::events::vulkan_events::VulkanCreateObjectEvent,
};

pub struct BaseCube {
    name: String,
    transform: Transform,
    mesh: String,
}

impl BaseCube {
    pub fn new(name: String, transform: Transform) -> BaseCube {
        BaseCube {
            name,
            transform,
            mesh: "src/resources/entities/monkey.glb".to_string(),
        }
    }
}

impl Entity for BaseCube {
    fn load(
        &self,
        repository: Arc<EntityRepository>,
        async_sender: Sender<Box<dyn Any + Send + Sync>>,
    ) {
        if let Some(id) = repository.get_id() {
            repository.add_entity(id, self.name.clone());
            repository.set_transform(id, self.transform.clone());

            let vulkan_event = VulkanCreateObjectEvent {
                object_id: id,
                mesh: self.mesh.clone(),
                object_transform: self.transform.clone(),
            };

            let _ = async_sender.send(Box::new(vulkan_event));
        } else {
            log!(Self, Critical, "Failed to get id for entity...");
        }
    }
}
