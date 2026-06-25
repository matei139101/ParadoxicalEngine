use crate::prelude::*;

pub struct LevelService {
    metadata: RwLock<HashMap<String, String>>,
}

pub trait Level {
    fn get_name(&self) -> String;
    fn get_entities(&self) -> [Box<dyn Entity>];
}
