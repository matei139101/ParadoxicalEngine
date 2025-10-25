use crate::engine::utils::structs::model::Model;

pub trait RenderedModel {
    fn get_model(&self) -> &Model;
    fn get_texture(&self) -> String;
}
