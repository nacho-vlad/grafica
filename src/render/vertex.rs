use cgmath::Vector3;


#[repr(C)]
#[derive(Copy,Clone,Debug)]
pub struct Vertex {
    position: Vector3<f32>,
    color: Vector3<f32>,
}

unsafe impl bytemuck::Pod for Vertex {}
unsafe impl bytemuck::Zeroable for Vertex {}

impl Vertex {

    pub fn new(position: [f32;3], color: [f32;3] ) -> Self {
        Vertex {
            position: Vector3::new(position[0],position[1],position[2]),
            color: Vector3::new(color[0],color[1],color[2]),
        }
    }

    pub fn desc<'a>() -> wgpu::VertexBufferDescriptor<'a> {
        use std::mem;
        wgpu::VertexBufferDescriptor {
            stride: mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::InputStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttributeDescriptor {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float3,
                },
                wgpu::VertexAttributeDescriptor {
                    offset: mem::size_of::<Vector3<f32>>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float3,
                }
            ],
        }
    }
}
