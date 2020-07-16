
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Uniforms {
    pub color: cgmath::Vector3<f32>
}

unsafe impl bytemuck::Pod for Uniforms {}
unsafe impl bytemuck::Zeroable for Uniforms {}

impl Uniforms {
    pub fn new() -> Self {
        Uniforms {
            color: cgmath::Vector3::new(0.0,0.0,0.0)
        }
    }

    pub fn buffer(&mut self, device: &wgpu::Device) -> wgpu::Buffer {
        device.create_buffer_with_data(
            bytemuck::cast_slice(&[*self]),
            wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST
        )
    }

    pub fn bind_group_layout(&mut self, device: &wgpu::Device) -> wgpu::BindGroupLayout {
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            bindings: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStage::FRAGMENT,
                    ty: wgpu::BindingType::UniformBuffer {
                        dynamic: false,
                    }
                }
            ],
            label: Some("uniform_bind_group_laout"),
        })
    }
}
