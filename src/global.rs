use crate::renderer::Renderer;

pub struct GlobalState {
    pub renderer: Renderer,
    pub locked_input: bool,
}

impl GlobalState {
    pub fn new(renderer: Renderer) -> Self {
        Self {
            renderer,
            locked_input: false,
        }
    }
}
