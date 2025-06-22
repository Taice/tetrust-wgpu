mod tetris;

use super::vertex::Vertex;
use std::{cmp::Ordering, iter, sync::Arc};
use tetris::{Tetris, action::Action};
use wgpu::util::DeviceExt;
use winit::{event_loop::ActiveEventLoop, keyboard::KeyCode, window::Window};

pub struct State {
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    is_surface_configured: bool,
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    num_indices: u32,
    pub window: Arc<Window>,

    soft: bool,
    pause: bool,
    tetris: Tetris,
}

impl State {
    pub async fn new(window: Arc<Window>) -> anyhow::Result<State> {
        let size = window.inner_size();

        // The instance is a handle to our GPU
        // BackendBit::PRIMARY => Vulkan + Metal + DX12 + Browser WebGPU
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            ..Default::default()
        });

        // # Safety
        //
        // The surface needs to live as long as the window that created it.
        // State owns the window so this should be safe.
        let surface = instance.create_surface(window.clone()).unwrap();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await?;

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                // WebGL doesn't support all of wgpu's features, so if
                // we're building for the web we'll have to disable some.
                required_limits: if cfg!(target_arch = "wasm32") {
                    wgpu::Limits::downlevel_webgl2_defaults()
                } else {
                    wgpu::Limits::default()
                },
                memory_hints: Default::default(),
                trace: wgpu::Trace::Off, // Trace path
            })
            .await?;

        let surface_caps = surface.get_capabilities(&adapter);
        // Shader code in this tutorial assumes an Srgb surface texture. Using a different
        // one will result all the colors comming out darker. If you want to support non
        // Srgb surfaces, you'll need to account for that when drawing to the frame.
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[Vertex::desc()],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState {
                        color: wgpu::BlendComponent::REPLACE,
                        alpha: wgpu::BlendComponent::REPLACE,
                    }),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                // Setting this to anything other than Fill requires Features::POLYGON_MODE_LINE
                // or Features::POLYGON_MODE_POINT
                polygon_mode: wgpu::PolygonMode::Fill,
                // Requires Features::DEPTH_CLIP_CONTROL
                unclipped_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            // If the pipeline will be used with a multiview render pass, this
            // indicates how many array layers the attachments will have.
            multiview: None,
            // Useful for optimizing shader compilation on Android
            cache: None,
        });

        let vertices = [Vertex::default(); 10 * 20 * 4];
        let mut indices = [0u16; 10 * 20 * 6 + 1];
        let mut vi = 0;
        let mut ii = 0;
        for _ in 0..(20 * 10) {
            let v = vi as u16;

            // i literaly have no clue why the fuck its like this this is genuinely mind fick
            // behaviour i have no idea whhy it does this itts likea  weird fucking 90 degree
            // rotation left for whatever reason this is absolutely out of my scope as a human
            // like what is this genuine mind fuck but whatever it actually fucking works i
            // cant be more hppy about that fact this fucking shit took way too fucking long
            // fucking cunt this is not even a joke i genuinely want to thank my family to be
            // able to do this i genuinely used all of my brain power to try to understand how
            // to make this work and it fukcing works now how? i dont know but i dont care
            // either im jus china make a tetris clone for fuck ssake why does this work dont
            // axe me
            indices[ii] = v;
            indices[ii + 1] = v + 1;
            indices[ii + 2] = v + 2;
            indices[ii + 3] = v + 3;
            indices[ii + 4] = v + 2;
            indices[ii + 5] = v + 1;

            vi += 4;
            ii += 6;
        }

        // top lfet, top right, bot left, bot right
        // 0, 3, 1, 0, 2, 3

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(&indices),
            usage: wgpu::BufferUsages::INDEX,
        });
        let num_indices = indices.len() as u32;

        let mut state = Self {
            surface,
            device,
            queue,
            config,
            is_surface_configured: false,
            render_pipeline,
            vertex_buffer,
            index_buffer,
            num_indices,
            window,

            pause: false,
            soft: false,
            tetris: Tetris::new(),
        };

        state.resize(size.width, size.height);

        Ok(state)
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.config.width = width;
            self.config.height = height;
            self.surface.configure(&self.device, &self.config);
            self.is_surface_configured = true;
        }
    }

    pub fn update(&mut self) {
        if !self.pause && self.tetris.update(self.soft) {
            self.new_vertices();
        }
    }

    #[rustfmt::skip]
    pub fn handle_key(&mut self, event_loop: &ActiveEventLoop, key: KeyCode, pressed: bool) {
        let mut done = true;
        #[allow(clippy::single_match)]
        match (key, pressed) {
            (KeyCode::Space, true) => self.tetris.process_action(Action::HardDrop),
            (KeyCode::ArrowLeft, true) => self.tetris.process_action(Action::Move(-1)),
            (KeyCode::ArrowRight, true) => self.tetris.process_action(Action::Move(1)),
            (KeyCode::ArrowUp, true) => self.tetris.process_action(Action::Rotate(90)),
            (KeyCode::ArrowDown, true) => self.tetris.process_action(Action::Rotate(-90)),
            (KeyCode::KeyP, true) => self.pause = !self.pause,
            (KeyCode::KeyH, true) => self.tetris.process_action(Action::Hold),
            (KeyCode::KeyA, true) => self.tetris.toggle_autoplay(),

            (KeyCode::Escape, true) => event_loop.exit(),
            (KeyCode::ShiftLeft, true) => self.soft = true,
            (KeyCode::ShiftLeft, false) => self.soft = false,
            _ => done = false,
        }
        if done {
            self.new_vertices();
        }
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        self.window.request_redraw();

        // We can't render unless the surface is configured
        if !self.is_surface_configured {
            return Ok(());
        }

        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.0,
                            g: 0.0,
                            b: 0.0,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
        }

        self.queue.submit(iter::once(encoder.finish()));
        output.present();

        Ok(())
    }

    pub fn new_vertices(&mut self) {
        if !self.is_surface_configured {
            return;
        }
        let board = self.tetris.get_full_board();
        let mut vertices = [Vertex::default(); 10 * 20 * 4];
        let ratio = self.config.width as f32 / self.config.height as f32;
        let (width, height, startx, starty) = match ratio.total_cmp(&0.5) {
            Ordering::Equal => (1. / 5., 1. / 10., -1., -1.),
            Ordering::Less => {
                let width = 1. / 5.;
                let startx = -1.0;
                let height = width * ratio;
                let starty = -(10. * height);

                (width, height, startx, starty)
            }
            Ordering::Greater => {
                let height = 1. / 10.;
                let starty = -1.0;
                let width = height / ratio;
                let startx = -(5. * width);

                (width, height, startx, starty)
            }
        };

        let color = [0.01; 3];
        let mut vi = 0;
        for (y, row) in board.iter().rev().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                let color = if let tetris::cell::Cell::Filled(c) = *cell {
                    c
                } else {
                    color
                };
                let fx = startx + width * x as f32;
                let fy = starty + height * y as f32;

                vertices[vi] = Vertex {
                    // top left
                    position: [fx, fy, 0.0],
                    color,
                };
                vertices[vi + 1] = Vertex {
                    // top right
                    position: [fx + width, fy, 0.0],
                    color,
                };
                vertices[vi + 2] = Vertex {
                    // bottom left
                    position: [fx, fy + height, 0.0],
                    color,
                };
                vertices[vi + 3] = Vertex {
                    // bottom right
                    position: [fx + width, fy + height, 0.0],
                    color,
                };

                vi += 4;
            }
        }
        self.vertex_buffer = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(&vertices),
                usage: wgpu::BufferUsages::VERTEX,
            });
    }
}
