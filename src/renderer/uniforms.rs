use vek::Mat4;

#[repr(C)]
// This is so we can store this in a buffer
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct TransformUniform {
    pub transform: [[f32; 4]; 4],
}
impl TransformUniform {
    pub fn new(mat: vek::Mat4<f32>) -> Self {
        Self {
            transform: mat.into_col_arrays(),
        }
    }
    pub fn update(&mut self, mat: vek::Mat4<f32>) {
        self.transform = mat.into_col_arrays();
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
