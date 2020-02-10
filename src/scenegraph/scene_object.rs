use crate::graphics::Drawable;
use crate::scenegraph::Updatable;
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
}

impl Updatable for SceneObject {
    fn update(&self, dt: f64) {}
}

impl Drawable for SceneObject {
    fn draw(&self, ctx: &mut Context) {}
}
