mod scene_object;

pub use self::scene_object::*;

pub trait Updatable {
    fn update(&self, dt: f64);
}
