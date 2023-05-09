use wgpu::{CommandEncoder, SurfaceTexture};

use crate::{
    renderer::Renderer,
    ui::{self, EguiInstance},
};

use super::camera::{Camera, CameraController};

pub struct RendererBorrow<'a> {
    device: &'a wgpu::Device,
    encoder: &'a mut wgpu::CommandEncoder,
    queue: &'a wgpu::Queue,
    egui_render_pass: &'a mut egui_wgpu_backend::RenderPass,
    surface_config: &'a wgpu::SurfaceConfiguration,
    gui: &'a mut EguiInstance,
    camera: &'a mut Camera,
    camera_controller: &'a mut CameraController,
    wireframe: &'a mut bool,
    delta_time: f32,
}

pub struct UIRenderer<'frame> {
    renderer: RendererBorrow<'frame>,
}

impl<'a> RendererBorrow<'a> {
    pub fn new(encoder: &'a mut wgpu::CommandEncoder, renderer: &'a mut Renderer, dt: f32) -> Self {
        Self {
            encoder,
            queue: &renderer.queue,
            device: &renderer.device,
            egui_render_pass: &mut renderer.egui_render_pass,
            surface_config: &renderer.config,
            gui: &mut renderer.gui,
            camera: &mut renderer.camera,
            camera_controller: &mut renderer.camera_controller,
            wireframe: &mut renderer.wireframe,
            delta_time: dt,
        }
    }
}
impl<'frame> UIRenderer<'frame> {
    pub fn new(
        enconder: &'frame mut CommandEncoder,
        renderer: &'frame mut Renderer,
        dt: f32,
    ) -> Self {
        let renderer: RendererBorrow = RendererBorrow::new(enconder, renderer, dt);
        Self { renderer }
    }

    pub fn draw_egui(&mut self, tex: &SurfaceTexture, scale_factor: f32) {
        self.renderer.gui.platform.begin_frame();
        // Draw UI

        ui::draw_camera_settings(
            &mut self.renderer.gui.platform,
            &mut self.renderer.camera,
            &mut self.renderer.camera_controller,
        );
        ui::draw_debugging_settings(
            &mut self.renderer.gui.platform,
            self.renderer.delta_time,
            &mut self.renderer.wireframe,
        );
        let output = self.renderer.gui.platform.end_frame(None);

        let paint_jobs = self
            .renderer
            .gui
            .platform
            .context()
            .tessellate(output.shapes);

        let screen_descriptor = egui_wgpu_backend::ScreenDescriptor {
            physical_width: self.renderer.surface_config.width,
            physical_height: self.renderer.surface_config.height,
            scale_factor,
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
