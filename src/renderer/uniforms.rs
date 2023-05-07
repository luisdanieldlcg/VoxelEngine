use vek::Mat4;

use super::camera::Camera;

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    pub transform: [[f32; 4]; 4],
}
impl CameraUniform {
    pub fn new(mat: vek::Mat4<f32>) -> Self {
        Self {
            transform: mat.into_col_arrays(),
        }
    }
    pub fn update(&mut self, camera: &Camera) {
        self.transform = camera.update_proj().into_col_arrays();
    }

    pub fn empty() -> Self {
        Self {
            transform: vek::Mat4::identity().into_col_arrays(),
        }
    }
    pub fn to_mat(&mut self) -> Mat4<f32> {
        Mat4::from_col_arrays(self.transform)
    }
}
