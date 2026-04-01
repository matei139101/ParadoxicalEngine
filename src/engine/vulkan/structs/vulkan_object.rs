use std::sync::Arc;

use vulkano::{buffer::Subbuffer, descriptor_set::DescriptorSet};

use crate::{engine::utils::structs::transform::Transform, prelude::*};

#[derive(Debug)]
pub struct VulkanObject {
    vertex_buffer: Subbuffer<[Vertex]>,
    index_buffer: Subbuffer<[u32]>,
    object_transform: Transform,
    texture_descriptor_set: Arc<DescriptorSet>
}

impl VulkanObject {
    pub fn new(vertex_buffer: vulkano::buffer::Subbuffer<[Vertex]>, index_buffer: Subbuffer<[u32]>, object_transform: Transform, texture_descriptor_set: Arc<DescriptorSet>) -> Self {
        return VulkanObject { vertex_buffer, index_buffer, object_transform, texture_descriptor_set}
    }

    pub fn get_transform(&self) -> &Transform {
        return &self.object_transform;
    }

    pub fn get_vertex_buffer(&self) -> &vulkano::buffer::Subbuffer<[Vertex]> {
        return &self.vertex_buffer;
    }

    pub fn get_index_buffer(&self) -> &vulkano::buffer::Subbuffer<[u32]> {
        return &self.index_buffer;
    }

    pub fn get_descriptor_set(&self) -> Arc<DescriptorSet> {
        return self.texture_descriptor_set.clone();
    }
}
