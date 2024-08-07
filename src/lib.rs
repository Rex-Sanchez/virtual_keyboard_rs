
use std::{thread::sleep, time::Duration};

use event_type::EventType;
use uinput::Device;

pub mod error;
mod event_type;
mod keycodes;
mod token;

use error::Result;

pub type KeyCodes = Vec<[u8; 8]>;

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
        map.split(" ").for_each(|f| {
            EventType::new(f).run(&mut self.device).ok();
        });
    }

    /// Translate virtual keypresses to keyboard bytes;
    pub fn to_keycodes(&mut self, map: &str) -> KeyCodes {
        map.split(" ")
            .map(|x| EventType::new(x).as_keycodes())
            .collect::<KeyCodes>()
    }
}
