
use std::{
    thread::sleep,
    time::Duration,
};

use event_type::EventType;
use tokonizer::Tokonizer;
use uinput::Device;

pub mod error;
mod event_type;
mod keycodes;
mod tokens;
mod tokonizer;

use error::Result;


/// Virtual keyboard struct;
pub struct VirtKeyboard {
    device: Device,
}

impl VirtKeyboard {
    /// Create a New Virtual keyboard with a given name;
    pub fn new(name: &str) -> Result<Self> {
        let dev = uinput::default()?
            .name(name)?
            .event(uinput::event::Keyboard::All)?
            .create()?;

        sleep(Duration::from_secs(1));
        Ok(Self { device: dev })
    }

    /// Send keystrokes to the virtual keyboard;
    pub fn send_keystrokes(&mut self, map: &str) {
        let tokens = Tokonizer::new(map);
        let events = EventType::from_tokens(tokens);
        events.into_iter().for_each(|event| {
            event.run(&mut self.device).ok();
        });
    }
}

#[test]
fn convert_from_file() {
    let mut kb = VirtKeyboard::new("yeh").unwrap();

    let s = "<Shift>+12 <Space> <Shift>+i <Space>n2";
    kb.send_keystrokes(s);
}


