use crate::render::{
    state::GraphicsState,
    vertex::Vertex,
    Render,
};

pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u16>,
    pub vertex_buffer: Option<wgpu::Buffer>,
    pub index_buffer: Option<wgpu::Buffer>
}


impl Mesh {
    pub fn new(vertices: Vec<Vertex>, indices: Vec<u16>) -> Self {
        Mesh {
            vertices,
            indices,
            vertex_buffer: None,
            index_buffer: None,
        }
    }
}

impl Render for Mesh {
    fn render(&mut self, graphics: &mut GraphicsState) {
        self.vertex_buffer.get_or_insert( 
            graphics.device.create_buffer_with_data(
                bytemuck::cast_slice(&self.vertices),
                wgpu::BufferUsage::VERTEX
            )
        );     
        
        self.index_buffer.get_or_insert( 
            graphics.device.create_buffer_with_data(
                bytemuck::cast_slice(&self.indices),
                wgpu::BufferUsage::INDEX
            )
        );     

        let frame = graphics.swap_chain.get_next_texture()
            .expect("Timeout getting texture");

        let mut encoder = graphics.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                color_attachments: &[
                    wgpu::RenderPassColorAttachmentDescriptor {
                        attachment: &frame.view,
                        resolve_target: None,
                        load_op: wgpu::LoadOp::Clear,
                        store_op: wgpu::StoreOp::Store,
                        clear_color: wgpu::Color {
                            r: 0.01,
                            g: 0.01,
                            b: 0.01,
                            a: 1.0,
                        },
                    }
                ],
                depth_stencil_attachment: None,
            });

            render_pass.set_pipeline(&graphics.render_pipeline);
            render_pass.set_bind_group(0, &graphics.uniform_bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.as_ref().unwrap(), 0, 0);
            render_pass.set_index_buffer(self.index_buffer.as_ref().unwrap(), 0, 0);
            render_pass.draw_indexed(0..self.indices.len() as u32,0, 0..1);
        }
        graphics.queue.submit(&[
            encoder.finish()
        ]);

    }
}

