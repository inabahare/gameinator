use crate::core::KeyEvent::KeyDown;
use crate::GameObject;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub struct Game {
  pub title: &'static str,
  pub width: u32,
  pub height: u32,
  pub canvas: Canvas<Window>,
  context: sdl2::Sdl,
  objects: Vec<Box<dyn GameObject>>,
  pub keyDown: KeyDown,
}

impl Game {
  pub fn new(title: &'static str, width: u32, height: u32) -> Result<Self, String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
      .window(title, width, height)
      .resizable()
      .build()
      .map_err(|e| e.to_string())?;

    let canvas = window
      .into_canvas()
      .present_vsync()
      .build()
      .map_err(|e| e.to_string())?;

    let res = Self {
      title,
      width,
      height,
      canvas,
      context: sdl_context,
      objects: Vec::new(),
      keyDown: KeyDown::new(),
    };

    Ok(res)
  }

  pub fn add_object(&mut self, object: impl GameObject + 'static) {
    self.objects.push(Box::new(object));
  }

  pub fn start(&mut self) -> Result<&mut Game, String> {
    let mut event_pump = self.context.event_pump().map_err(|e| e.to_string())?;

    'running: loop {
      for event in event_pump.poll_iter() {
        match event {
          Event::Quit { .. } => break 'running,
          Event::KeyDown { keycode, .. } => match keycode {
            Some(code) => self.keyDown.add_key(code),
            None => {}
          },
          _ => {}
        }
      }

      self.canvas.clear();

      for o in &self.objects {
        o.update(self);
      }

      for o in &self.objects {
        o.draw(self);
      }

      self.canvas.set_draw_color(Color::RGB(0, 0, 0));
      self.canvas.present();
      self.keyDown.clear();
    }

    Ok(self)
  }
}
