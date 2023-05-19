use super::pipelines::voxel::VoxelPipeline;

pub struct WorldRenderer {
    pub pipeline: VoxelPipeline,
    pub pipeline_wireframe: VoxelPipeline,
    pub wireframe: bool,
}

impl WorldRenderer {
    pub fn new(
        device: &wgpu::Device,
        shader: &wgpu::ShaderModule,
        cfg: &wgpu::SurfaceConfiguration,
        bg_layouts: &[&wgpu::BindGroupLayout],
    ) -> Self {
        let pipeline = VoxelPipeline::new(
            device,
            shader,
            cfg,
            bg_layouts,
            wgpu::PolygonMode::Fill,
        );

        let pipeline_wireframe = VoxelPipeline::new(
            device,
            shader,
            cfg,
            bg_layouts,
            wgpu::PolygonMode::Line,
        );

        Self {
            pipeline,
            pipeline_wireframe,
            wireframe: false,
        }
    }
}