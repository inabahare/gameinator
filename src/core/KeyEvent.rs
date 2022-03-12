use sdl2::keyboard::Keycode;

pub trait KeyEvent {
  fn is_active(&mut self, code: Keycode) -> bool;
}

pub struct KeyDown {
  keys: Vec<Keycode>,
}

impl KeyEvent for KeyDown {
  fn is_active(&mut self, code: Keycode) -> bool {
    for key in &self.keys {
      if key == &code {
        return true;
      }
    }

    false
  }
}

impl KeyDown {
  pub fn new() -> Self {
    Self { keys: Vec::new() }
  }

  pub fn add_key(&mut self, code: Keycode) {
    println!("{}", code);
    self.keys.push(code);
  }

  pub fn clear(&mut self) {
    self.keys.clear();
  }
}
