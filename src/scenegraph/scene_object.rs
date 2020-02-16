use crate::Context;

pub struct SceneObject {
    x: f32,
    y: f32,
    children: Vec<SceneObject>,
}

impl SceneObject {
    pub fn new() -> SceneObject {
        Self {
            x: 0.0,
            y: 0.0,
            children: Vec::new(),
        }
    }
    // TODO: using traits?
    pub fn update(&self, _ctx: &Context) {}
    pub fn draw(&self, _ctx: &Context, _dt: f64) {}
}
