extern crate sdl2;

use crate::core::Game;
use crate::core::GameObject::GameObject;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

mod core;

// struct object {
//     pub x: i32,
//     pub y: i32,
// }

// impl GameObject for object {
//     fn update(&mut self) {
//         self.y += 4;
//     }
//     fn render(&mut self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
//         let r = Rect::new(self.x, self.y, 100, 100);
//         canvas.set_draw_color(Color::RGB(255, 0, 255));
//         canvas.fill_rect(r);
//     }
// }

struct Player {
    pub x: i32,
    pub y: i32,
}

impl GameObject for Player {
    fn update(&mut self) {
        self.y += 4;
    }
    fn draw(&mut self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        let r = Rect::new(self.x, self.y, 100, 100);
        canvas.set_draw_color(Color::RGB(255, 0, 255));
        canvas.fill_rect(r);
    }
}

fn main() -> Result<(), String> {
    let player = Player { x: 10, y: 10 };

    let mut game = Game::new("Test", 800, 600);

    game.setup()?;

    game.add_object(player);

    game.start()?;
    Ok(())
}
