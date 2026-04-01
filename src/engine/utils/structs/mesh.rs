use std::panic;

use gltf::{image, mesh::util::ReadIndices};
use vulkano::{buffer::BufferContents, pipeline::graphics::vertex_input::Vertex as VulkanoVertex};

use crate::prelude::*;

#[derive(BufferContents, VulkanoVertex, Clone, Debug)]
#[repr(C)]
pub struct Vertex {
    #[format(R32G32B32_SFLOAT)]
    pub position: [f32; 3],
    
    #[format(R32G32_SFLOAT)]
    pub uv: [f32; 2],

    #[format(R32G32B32_SFLOAT)]
    pub normal: [f32; 3],
}

#[derive(Clone)]
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    pub image: image::Data,
}

impl Mesh {
    pub fn from_gltf(path: &str) -> Vec<Mesh> {
        if let Ok((document, buffers, images)) = gltf::import(path) {
            let mut meshes = Vec::new();

            for mesh in document.meshes() {
                for primitive in mesh.primitives() {
                    let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));

                    let positions: Vec<[f32; 3]> = reader
                        .read_positions()
                        .unwrap_or_else(|| {log!(Self, Critical, "No positions found in primitive in glTF..."); panic!()})
                        .collect();

                    let normals: Vec<[f32; 3]> = reader
                        .read_normals()
                        .map(|n| n.collect())
                        .unwrap_or_else(|| vec![[0.0, 1.0, 0.0]; positions.len()]);

                    let uvs: Vec<[f32; 2]> = reader
                        .read_tex_coords(0)
                        .map(|uv| uv.into_f32().collect())
                        .unwrap_or_else(|| vec![[0.0, 0.0]; positions.len()]);

                    let vertices = positions
                        .iter()
                        .enumerate()
                        .map(|(i, pos)| Vertex {
                            position: *pos,
                            normal: normals[i],
                            uv: uvs[i],
                        })
                    .collect();

                    let indices = match reader.read_indices() {
                        Some(ReadIndices::U32(iter)) => iter.collect(),
                        Some(ReadIndices::U16(iter)) => iter.map(|i| i as u32).collect(),
                        Some(ReadIndices::U8(iter)) => iter.map(|i| i as u32).collect(),
                        None => (0..positions.len() as u32).collect(),
                    };

                    let image = primitive
                        .material()
                        .pbr_metallic_roughness()
                        .base_color_texture()
                        .map(|t| images[t.texture().source().index()].clone())
                        .unwrap_or_else(|| {log!(Self, Critical, "No image found by index for primitive..."); panic!()});

                    meshes.push(Mesh { vertices, indices, image});
                }
            }

            meshes
        } else {
            log!(Self, Critical, "Failed to import glTF...");
            panic!()
        }
    }
}
