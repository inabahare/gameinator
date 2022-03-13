use crate::Game;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub trait GameObject {
  fn update(&mut self, game: &mut Game);
  fn draw(&mut self, game: &mut Game);
}
