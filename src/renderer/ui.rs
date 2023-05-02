use egui_winit_platform::Platform;
use wgpu::{CommandEncoder, SurfaceTexture};

use crate::renderer::Renderer;

pub struct RendererBorrow<'a> {
    device: &'a wgpu::Device,
    encoder: &'a mut wgpu::CommandEncoder,
    queue: &'a wgpu::Queue,
    egui_render_pass: &'a mut egui_wgpu_backend::RenderPass,
    surface_config: &'a wgpu::SurfaceConfiguration,
}

pub struct UIRenderer<'frame> {
    renderer: RendererBorrow<'frame>,
}
impl<'a> RendererBorrow<'a> {
    pub fn new(encoder: &'a mut wgpu::CommandEncoder, renderer: &'a mut Renderer) -> Self {
        Self {
            encoder,
            queue: &renderer.queue,
            device: &renderer.device,
            egui_render_pass: &mut renderer.egui_render_pass,
            surface_config: &renderer.config,
        }
    }
}
impl<'frame> UIRenderer<'frame> {
    pub fn new(enconder: &'frame mut CommandEncoder, renderer: &'frame mut Renderer) -> Self {
        let renderer: RendererBorrow = RendererBorrow::new(enconder, renderer);
        Self { renderer }
    }

    pub fn draw_egui(&mut self, tex: &SurfaceTexture, platform: &mut Platform, scale_factor: f32) {
        platform.begin_frame();
        egui::Window::new("EGUI Instance")
            .default_size([340.0, 700.0])
            .show(&platform.context(), |ui| {
                ui.label("Camera Settings");
            });
        let output = platform.end_frame(None);

        let paint_jobs = platform.context().tessellate(output.shapes);

        let screen_descriptor = egui_wgpu_backend::ScreenDescriptor {
            physical_width: self.renderer.surface_config.width,
            physical_height: self.renderer.surface_config.height,
            scale_factor: scale_factor,
        };

        let texture_delta = output.textures_delta;
        self.renderer
            .egui_render_pass
            .add_textures(self.renderer.device, self.renderer.queue, &texture_delta)
            .unwrap();

        self.renderer.egui_render_pass.update_buffers(
            self.renderer.device,
            self.renderer.queue,
            &paint_jobs,
            &screen_descriptor,
        );
        self.renderer
            .egui_render_pass
            .execute(
                self.renderer.encoder,
                &tex.texture
                    .create_view(&wgpu::TextureViewDescriptor::default()),
                &paint_jobs,
                &screen_descriptor,
                None,
            )
            .unwrap();
        self.renderer
            .egui_render_pass
            .remove_textures(texture_delta)
            .expect("Failed to remove texture");
    }
}
