use std::sync::Arc;

use wgpu::{
    Buffer, InstanceDescriptor, RenderPassColorAttachment, RenderPassDescriptor, RenderPipeline,
    VertexAttribute, VertexBufferLayout,
    util::{BufferInitDescriptor, DeviceExt},
    wgt::{CommandEncoderDescriptor, TextureViewDescriptor},
};
use winit::{event_loop::ActiveEventLoop, window::Window};

pub struct State {
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    render_pipeline: RenderPipeline,

    // FIXME move these somewhere else?
    buffer: Buffer,
    index_buffer: Buffer,
}

impl State {
    pub fn new(event_loop: &ActiveEventLoop) -> anyhow::Result<State> {
        let window = Arc::new(
            event_loop
                .create_window(Window::default_attributes())
                .unwrap(),
        );
        let instance = wgpu::Instance::new(InstanceDescriptor::new_with_display_handle(Box::new(
            event_loop.owned_display_handle(),
        )));
        let surface = instance.create_surface(window.clone())?;
        let adapter = pollster::block_on(
            instance.request_adapter(&wgpu::RequestAdapterOptionsBase::default()),
        )?;
        let (device, queue) =
            pollster::block_on(adapter.request_device(&wgpu::wgt::DeviceDescriptor::default()))?;

        let window_size = window.inner_size();
        let config = surface
            .get_default_config(&adapter, window_size.width, window_size.height)
            .unwrap();

        surface.configure(&device, &config);
        let shader = device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));
        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor::default());
        let targets = &[Some(wgpu::ColorTargetState {
            format: config.format, // needs to match your surface
            blend: Some(wgpu::BlendState::REPLACE),
            write_mask: wgpu::ColorWrites::ALL,
        })];

        let rad = 1.0;
        let vertex: [[f32; 2]; 4] = [[0.0, rad], [rad, 0.0], [0.0, -rad], [-rad, 0.0]];

        let index_bytes = bytemuck::cast_slice::<u16, u8>(&[0, 1, 2, 2, 3, 0]);
        let bytes = bytemuck::cast_slice::<[f32; 2], u8>(&vertex);

        let buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("vertices"),
            contents: bytes,
            usage: wgpu::BufferUsages::VERTEX,
        });
        let index_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("indices"),
            contents: index_bytes,
            usage: wgpu::BufferUsages::INDEX,
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[Some(VertexBufferLayout {
                    array_stride: 8,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &[VertexAttribute {
                        format: wgpu::VertexFormat::Float32x2,
                        offset: 0,
                        shader_location: 0,
                    }],
                })],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets,
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState::default(), // triangles
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview_mask: None,
            cache: None,
        });
        Ok(State {
            surface,
            queue,
            device,
            render_pipeline,
            buffer,
            index_buffer,
        })
    }
    pub fn redraw(&self) {
        let texture = match self.surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(texture) => texture,
            wgpu::CurrentSurfaceTexture::Suboptimal(texture) => texture,
            _ => return,
        };

        let view = texture
            .texture
            .create_view(&TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&CommandEncoderDescriptor::default());

        {
            let attatchment = Some(RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: (wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.2,
                        g: 0.3,
                        b: 0.2,
                        a: 1.0,
                    })),
                    store: (wgpu::StoreOp::Store),
                },
                depth_slice: None,
            });
            let mut pass = encoder.begin_render_pass(&RenderPassDescriptor {
                color_attachments: &[attatchment],
                ..Default::default()
            });
            pass.set_pipeline(&self.render_pipeline);
            pass.set_vertex_buffer(0, self.buffer.slice(..));
            pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            // pass.draw(0..6, 0..1);
            pass.draw_indexed(0..6, 0, 0..1);
        }

        self.queue.submit([encoder.finish()]);
        self.queue.present(texture);
    }
}
