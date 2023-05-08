use super::texture::Texture;

pub struct Atlas {
    pub texture: Texture,
    pub bind_group: wgpu::BindGroup,
    pub bind_group_layout: wgpu::BindGroupLayout,
}

impl Atlas {
    pub fn new(texture: &[u8], device: &wgpu::Device, queue: &wgpu::Queue) -> Self {
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Atlas bind group layout"),
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
        let texture: Texture = Texture::from_bytes(device, queue, texture, "atlas.png");

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Atlas bind group"),
            layout: &bind_group_layout,
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
        Self {
            texture,
            bind_group,
            bind_group_layout,
        }
    }
}

const ATLAS_SIZE: f32 = 256.0;
const SUB_TEXTURE_SIZE: f32 = 16.0;

pub fn atlas_uv_mapping(row: f32, col: f32, x: f32, y: f32) -> [f32; 2] {
    let mut offset_x = row * SUB_TEXTURE_SIZE;
    let mut offset_y = col * SUB_TEXTURE_SIZE;

    if x == 1.0 {
        offset_x += 16.0;
    }

    if y == 1.0 {
        offset_y += 16.0;
    }

    return [offset_x / ATLAS_SIZE, offset_y / ATLAS_SIZE];
}
