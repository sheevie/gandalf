use std::sync::Arc;

use wgpu::{
    Adapter, Device, InstanceDescriptor, Queue, RenderPassColorAttachment, RenderPassDescriptor,
    RenderPipeline, RenderPipelineDescriptor, Surface, TextureFormat, TextureViewDescriptor,
    wgt::CommandEncoderDescriptor,
};
use winit::{
    application::ApplicationHandler, event::WindowEvent, event_loop::ActiveEventLoop,
    window::Window,
};

struct State {
    window: Arc<Window>,
    adapter: Adapter,
    surface: Surface<'static>,
    device: Device,
    queue: Queue,
    render_pipeline: RenderPipeline,
    surface_format: TextureFormat,
}
impl State {
    fn new(event_loop: &ActiveEventLoop) -> anyhow::Result<State> {
        let window = Arc::new(event_loop.create_window(Window::default_attributes())?);
        let instance = wgpu::Instance::new(&InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });
        let surface = instance.create_surface(window.clone())?;
        let adapter =
            pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptionsBase {
                power_preference: wgpu::PowerPreference::default(),
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            }))?;
        let (device, queue) =
            pollster::block_on(adapter.request_device(&wgpu::DeviceDescriptor::default()))?;
        let window_size = window.inner_size();
        let config = surface
            .get_default_config(&adapter, window_size.width, window_size.height)
            .unwrap();
        surface.configure(&device, &config);
        let surface_format = config.format;

        let shader = &device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });

        let render_pipeline_layout =
            &device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor::default());

        let targets = &[Some(wgpu::ColorTargetState {
            format: surface_format, // needs to match your surface
            blend: Some(wgpu::BlendState::REPLACE),
            write_mask: wgpu::ColorWrites::ALL,
        })];
        let descriptor = &RenderPipelineDescriptor {
            label: Some("pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[], // no vertex buffers yet
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
            multiview: None,
            cache: None,
        };
        let render_pipeline = device.create_render_pipeline(&descriptor);

        Ok(State {
            window,
            adapter,
            surface,
            device,
            queue,
            render_pipeline,
            surface_format,
        })
    }
}

#[derive(Default)]
pub struct App {
    state: Option<State>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.state = Some(State::new(event_loop).unwrap());
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::RedrawRequested => {
                if let Some(state) = self.state.as_ref() {
                    let texture = state.surface.get_current_texture().unwrap();
                    let view = texture
                        .texture
                        .create_view(&TextureViewDescriptor::default());
                    let mut encoder = state
                        .device
                        .create_command_encoder(&CommandEncoderDescriptor::default());
                    let attachment = Some(RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color {
                                r: 0.0,
                                g: 1.0,
                                b: 0.5,
                                a: 1.0,
                            }),
                            store: wgpu::StoreOp::Store,
                        },
                        depth_slice: None,
                    });
                    {
                        let mut pass = encoder.begin_render_pass(&RenderPassDescriptor {
                            color_attachments: &[attachment],
                            ..Default::default()
                        });
                        pass.set_pipeline(&state.render_pipeline);
                        pass.draw(0..3, 0..1);
                    }
                    state.queue.submit([encoder.finish()]);
                    texture.present();
                }
            }
            _ => {}
        }
    }
}
