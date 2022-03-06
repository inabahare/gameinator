use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub trait GameObject {
  fn update(&mut self);
  pub fn draw(&mut self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>);
}
