use crate::prelude::*;

pub struct Repositories {
    input_repository: Arc<InputRepository>,
    entity_repository: Arc<EntityRepository>,
}

impl Repositories {
    pub fn new() -> Repositories {
        Repositories {
            input_repository: Arc::from(InputRepository::new()),
            entity_repository: Arc::from(EntityRepository::new()),
        }
    }

    pub fn get_input_repository(&self) -> Arc<InputRepository> {
        self.input_repository.clone()
    }

    pub fn get_entity_repository(&self) -> Arc<EntityRepository> {
        self.entity_repository.clone()
    }
}
