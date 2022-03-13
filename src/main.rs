extern crate sdl2;

use crate::core::Game::Game;
use crate::core::GameObject::GameObject;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

mod core;
struct Player {
    pub x: i32,
    pub y: i32,
}

impl GameObject for Player {
    fn update(&mut self, game: &mut Game) {}
    fn draw(&mut self, game: &mut Game) {
        let r = Rect::new(self.x, self.y, 100, 100);
        game.canvas.set_draw_color(Color::RGB(255, 0, 255));

        game.canvas.fill_rect(r);
    }
}

fn main() -> Result<(), String> {
    let player = Player {
        x: 800 / 2,
        y: 600 / 2,
    };

    let mut game = Game::new("Test", 800, 600)?;

    game.add_object(player);

    game.start()?;
    Ok(())
}
