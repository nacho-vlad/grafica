use winit::{
    window::Window,
};
use crate::render::vertex::Vertex;
use super::uniforms::Uniforms;


fn shader_module<P: AsRef<std::path::Path> + Clone>(path: P, kind: shaderc::ShaderKind, device: &wgpu::Device) -> wgpu::ShaderModule {
    let src = &std::fs::read_to_string(path.clone()).unwrap();

    let mut compiler = shaderc::Compiler::new().unwrap();

    let spirv = compiler.compile_into_spirv(src, kind, path.as_ref().file_name().unwrap().to_str().unwrap(), "main", None).unwrap();

    let data = wgpu::read_spirv(std::io::Cursor::new(spirv.as_binary_u8())).unwrap();

    device.create_shader_module(&data)
}

pub struct GraphicsState { 
    pub surface: wgpu::Surface,
    pub adapter: wgpu::Adapter,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub sc_desc: wgpu::SwapChainDescriptor,
    pub swap_chain: wgpu::SwapChain, 

    pub render_pipeline: wgpu::RenderPipeline,

    pub uniforms: Uniforms,
    pub uniform_buffer: wgpu::Buffer,
    pub uniform_bind_group: wgpu::BindGroup,


    pub size: winit::dpi::PhysicalSize<u32>,
}


impl GraphicsState {
    pub async fn new(window: &Window) -> Self {
        let size = window.inner_size();

        let surface = wgpu::Surface::create(window);

        let adapter = wgpu::Adapter::request(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::Default,
                compatible_surface: Some(&surface),
            },
            wgpu::BackendBit::PRIMARY,
        ).await.unwrap();

        let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor {
            extensions: wgpu::Extensions {
                anisotropic_filtering: false,
            },
            limits: Default::default(),
        }).await;

        let sc_desc = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };

        let swap_chain = device.create_swap_chain(&surface, &sc_desc);

        let mut uniforms = Uniforms::new();

        let uniform_buffer = uniforms.buffer(&device);
        let uniform_bind_group_layout = uniforms.bind_group_layout(&device);
        let uniform_bind_group= device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &uniform_bind_group_layout,
            bindings: &[
                wgpu::Binding {
                    binding: 0,
                    resource: wgpu::BindingResource::Buffer {
                        buffer: &uniform_buffer,
                        range: 0..std::mem::size_of::<Uniforms>() as wgpu::BufferAddress,
                    }
                }
            ],
            label: Some("uniform_bind_group"),
        });



        let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            bind_group_layouts: &[&uniform_bind_group_layout],
        });

        let vs_module = shader_module("shaders/shader.vert", shaderc::ShaderKind::Vertex, &device);
        let fs_module = shader_module("shaders/shader.frag", shaderc::ShaderKind::Fragment, &device);

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            layout: &render_pipeline_layout,
            vertex_stage: wgpu::ProgrammableStageDescriptor {
                module: &vs_module,
                entry_point: "main",
            },
            fragment_stage: Some(wgpu::ProgrammableStageDescriptor {
                module: &fs_module,
                entry_point: "main",
            }),
            rasterization_state: Some(wgpu::RasterizationStateDescriptor {
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: wgpu::CullMode::Back,
                depth_bias: 0,
                depth_bias_slope_scale: 0.0,
                depth_bias_clamp: 0.0,
            }),
            color_states: &[
                wgpu::ColorStateDescriptor {
                    format: sc_desc.format,
                    color_blend: wgpu::BlendDescriptor::REPLACE,
                    alpha_blend: wgpu::BlendDescriptor::REPLACE,
                    write_mask: wgpu::ColorWrite::ALL,
                },
            ],
            primitive_topology: wgpu::PrimitiveTopology::TriangleList,
            depth_stencil_state: None,
            vertex_state: wgpu::VertexStateDescriptor {
                index_format: wgpu::IndexFormat::Uint16,
                vertex_buffers: &[
                    Vertex::desc(),
                ],
            },
            sample_count: 1,
            sample_mask: !0,
            alpha_to_coverage_enabled: false,
        });


        Self {
            surface,
            adapter,
            device,
            queue,
            sc_desc,
            swap_chain,
            uniforms,
            uniform_buffer,
            uniform_bind_group,
            render_pipeline,
            size,
        }

    }
    
    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.size = new_size;
        self.sc_desc.width = new_size.width;
        self.sc_desc.height = new_size.height;
        self.swap_chain = self.device.create_swap_chain(&self.surface, &self.sc_desc);
    }


    // pub fn render(&mut self) {
    //     let frame = self.swap_chain.get_next_texture()
    //         .expect("Timeout getting texture");

    //     let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
    //         label: Some("Render Encoder"),
    //     });

    //     {
    //         let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
    //             color_attachments: &[
    //                 wgpu::RenderPassColorAttachmentDescriptor {
    //                     attachment: &frame.view,
    //                     resolve_target: None,
    //                     load_op: wgpu::LoadOp::Clear,
    //                     store_op: wgpu::StoreOp::Store,
    //                     clear_color: wgpu::Color {
    //                         r: 0.1,
    //                         g: 0.2,
    //                         b: 0.3,
    //                         a: 1.0,
    //                     },
    //                 }
    //             ],
    //             depth_stencil_attachment: None,
    //         });

    //         render_pass.set_pipeline(&self.render_pipeline);
    //         render_pass.set_vertex_buffer(0, &self.vertex_buffer, 0, 0);
    //         render_pass.draw(0..3,0..1);
    //     }
    //     self.queue.submit(&[
    //         encoder.finish()
    //     ]);

    //     }
}
