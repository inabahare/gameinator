use crate::GameObject;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub fn new(title: &'static str, width: u32, height: u32) -> Game {
  Game {
    title,
    width,
    height,
    canvas: None,
    context: None,
    objects: Vec::new(),
  }
}

pub struct Game {
  pub title: &'static str,
  pub width: u32,
  pub height: u32,
  canvas: Option<Canvas<Window>>,
  context: Option<sdl2::Sdl>,
  objects: Vec<Box<dyn GameObject>>,
}

impl Game {
  pub fn add_object(&mut self, object: impl GameObject + 'static) {
    self.objects.push(Box::new(object));
  }

  pub fn setup(&mut self) -> Result<&mut Game, String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
      .window(self.title, self.width, self.height)
      .resizable()
      .build()
      .map_err(|e| e.to_string())?;

    let canvas = window
      .into_canvas()
      .present_vsync()
      .build()
      .map_err(|e| e.to_string())?;

    self.canvas = Some(canvas);
    self.context = Some(sdl_context);

    Ok(self)
  }

  pub fn start(&mut self) -> Result<&mut Game, String> {
    let context = self.context.as_ref().unwrap();
    let canvas = self.canvas.as_mut().unwrap();

    let mut event_pump = context.event_pump().map_err(|e| e.to_string())?;

    'running: loop {
      for event in event_pump.poll_iter() {
        match event {
          Event::Quit { .. }
          | Event::KeyDown {
            keycode: Some(Keycode::Escape),
            ..
          } => break 'running,
          _ => {}
        }
      }

      canvas.clear();

      // for o in &mut objects {
      //   o.update();
      // }

      // for o in &mut objects {
      //   o.render(&mut canvas);
      // }

      canvas.set_draw_color(Color::RGB(0, 0, 0));
      canvas.present();
    }

    Ok(self)
  }
}
