use crate::graphics::{IntRect, Sprite, Texture};
use sfml::graphics::{Drawable, RenderStates, RenderTarget};

#[derive(Debug)]
pub struct Animation {
    texture: Texture,

    /// The frames in the animation.
    frames: Vec<IntRect>,

    /// The length of a frame, in ticks.
    frame_length: i32,

    /// The index of the current animation frame.
    current_frame: usize,
    timer: i32,
}

impl Animation {
    pub fn new(texture: Texture, frames: Vec<IntRect>, frame_length: i32) -> Self {
        Self {
            texture,
            frames,
            frame_length,
            current_frame: 0,
            timer: 0,
        }
    }

    pub fn tick(&mut self) {
        self.timer += 1;

        if self.timer >= self.frame_length {
            self.current_frame = (self.current_frame + 1) % self.frames.len();
            self.timer = 0;
        }
    }

    /// Restarts the animation from the beginning.
    pub fn restart(&mut self) {
        self.current_frame = 0;
        self.timer = 0;
    }
}

impl Drawable for Animation {
    fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture>(
        &'a self,
        target: &mut RenderTarget,
        states: RenderStates<'texture, 'shader, 'shader_texture>,
    ) {
        let mut sprite = Sprite::with_texture(&self.texture);
        sprite.set_texture_rect(&self.frames[self.current_frame]);

        target.draw(&sprite);
    }
}