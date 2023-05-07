mod buffer;
pub mod camera;
mod cube;
mod mesh;
mod texture;
mod ui;
mod uniforms;

use std::time::Duration;

use crate::ui::EguiInstance;

use self::{
    buffer::Buffer,
    camera::{Camera, CameraController},
    cube::CubePipeline,
    mesh::{vertex::Vertex, Mesh, Quad, Triangle},
    texture::Texture,
    ui::UIRenderer,
    uniforms::CameraUniform,
};
use vek::Mat4;
use wgpu::BindGroupEntry;
use winit::event::WindowEvent;
use winit::window::Window;

pub struct Renderer {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    cube_pipeline: CubePipeline,
    quad_buffer: Buffer<Vertex>,
    quad_index_buffer: Buffer<u16>,
    mesh: mesh::Mesh<Vertex>,
    texture_bind_group: wgpu::BindGroup,
    size: winit::dpi::PhysicalSize<u32>,
    camera_uniform: CameraUniform,
    transform_buffer: Buffer<CameraUniform>,
    transform_bind_group: wgpu::BindGroup,
    camera_controller: camera::CameraController,
    egui_render_pass: egui_wgpu_backend::RenderPass,
    camera: camera::Camera,
    pub gui: EguiInstance,
    cursor_pos: (f32, f32),
}

impl Renderer {
    // Creating some of the wgpu types requires async code
    pub async fn new(window: &Window, gui: EguiInstance) -> Self {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            dx12_shader_compiler: Default::default(),
        });
        // The surface needs to live as long as the window that created it.
        // State owns the window so this should be safe.
        let surface =
            unsafe { instance.create_surface(&window) }.expect("Failed to create surface");

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .expect("Failed to create adapter");

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                    label: None,
                },
                None, // Trace path
            )
            .await
            .unwrap();
        let surface_caps = surface.get_capabilities(&adapter);
        // Shader code in this tutorial assumes an sRGB surface texture. Using a different
        // one will result all the colors coming out darker. If you want to support non
        // sRGB surfaces, you'll need to account for that when drawing to the frame.
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .filter(|f| f.is_srgb())
            .next()
            .unwrap_or(surface_caps.formats[0]);

        let size = window.inner_size();

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };

        let shader =
            device.create_shader_module(wgpu::include_wgsl!("../assets/shaders/vertex.wgsl"));

        // Texture bind group

        let file = include_bytes!("../assets/stone.jpg");
        let texture = Texture::new(&device, &queue, file);
        let texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Texture bind group layout"),
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        // This should match the filterable field of the
                        // corresponding Texture entry above.
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
            });

        let texture_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Texture bind group"),
            layout: &texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&texture.sampler),
                },
            ],
        });
        let camera = Camera::new(size.width as f32, size.height as f32);
        let mut camera_uniform = CameraUniform::empty();
        camera_uniform.update(&camera);

        let transform_buffer = Buffer::new(
            &device,
            wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            &[camera_uniform],
        );

        let transform_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: None,
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });
        let transform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &transform_bind_group_layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: transform_buffer.buf.as_entire_binding(),
            }],
        });

        let pipeline = CubePipeline::new(
            &device,
            &shader,
            &config,
            &[&texture_bind_group_layout, &transform_bind_group_layout],
        );
        let cube = Mesh::<Vertex>::cube();
        let quad_buffer = Buffer::new(&device, wgpu::BufferUsages::VERTEX, cube.vertices());
        let quad_index_buffer = create_quad_index_buffer(&device);
        let egui_render_pass = egui_wgpu_backend::RenderPass::new(&device, surface_format, 1);

        surface.configure(&device, &config);

        Self {
            surface,
            device,
            queue,
            config,
            cube_pipeline: pipeline,
            quad_buffer,
            quad_index_buffer,
            texture_bind_group,
            size,
            transform_buffer,
            transform_bind_group,
            camera_uniform,
            camera_controller: CameraController::new(),
            egui_render_pass,
            gui,
            camera,
            mesh: cube,
            cursor_pos: (0.0, 0.0),
        }
    }

    /// Support resizing the surface
    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);

            self.camera.width = new_size.width as f32;
            self.camera.height = new_size.height as f32;
        }
    }

    pub fn on_key_pressed(&mut self, input: &winit::event::KeyboardInput) {
        self.camera_controller.handle_keyboard_events(input);
    }

    pub fn on_cursor_moved(&mut self, pos: (f32, f32)) {
        self.cursor_pos = pos;
    }
    pub fn on_mouse_motion(&mut self, delta: (f64, f64)) {
        self.camera_controller.handle_mouse_events(delta.0, delta.1);
    }

    pub fn update(&mut self, dt: Duration) {
        self.camera_controller
            .update(&mut self.camera, dt, self.cursor_pos);
        self.camera_uniform.update(&self.camera);
        self.queue.write_buffer(
            &self.transform_buffer.buf,
            0,
            bytemuck::cast_slice(&[self.camera_uniform]),
        );
    }

    pub fn render(&mut self, window: &Window) -> Result<(), wgpu::SurfaceError> {
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render enconder"),
            });

        let surface_texture = self.surface.get_current_texture()?;

        let view = surface_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        {
            let mut render = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.3,
                            b: 0.5,
                            a: 1.0,
                        }),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });
            render.set_pipeline(&self.cube_pipeline.pipeline);
            render.set_bind_group(0, &self.texture_bind_group, &[]);
            render.set_bind_group(1, &self.transform_bind_group, &[]);
            render.set_vertex_buffer(0, self.quad_buffer.buf.slice(..));
            render.set_index_buffer(
                self.quad_index_buffer.buf.slice(..),
                wgpu::IndexFormat::Uint16,
            );
            render.draw_indexed(0..self.quad_index_buffer.len() as u32, 0, 0..1)
            // render.draw(0..self.mesh.vertices().len() as u32, 0..1);
        }
        let mut ui_renderer = UIRenderer::new(&mut encoder, self);
        ui_renderer.draw_egui(&surface_texture, window.scale_factor() as f32);

        self.queue.submit(std::iter::once(encoder.finish()));
        surface_texture.present();
        Ok(())
    }
}

pub fn create_quad_index_buffer(device: &wgpu::Device) -> Buffer<u16> {
    let vert_length = 24;
    let indices = [0, 1, 2, 2, 1, 3]
        .iter()
        .cycle()
        .copied()
        .take(vert_length / 4 * 6)
        .enumerate()
        .map(|(i, b)| (i / 6 * 4 + b) as u16)
        .collect::<Vec<_>>();
    Buffer::new(&device, wgpu::BufferUsages::INDEX, &indices)
}
