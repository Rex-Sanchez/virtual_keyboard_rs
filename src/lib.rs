use std::{thread::sleep, time::Duration};

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

#[cfg(test)]
mod test {

    use crate::tokens::Token;
    use crate::{tokonizer::Tokonizer, VirtKeyboard};

    #[test]
    fn send_keystrokes() {
        let mut kb = VirtKeyboard::new("yeh").unwrap();

        let a = vec![
            vec![Token::Shift, Token::Plus, Token::_9],
            vec![Token::Shift, Token::Plus, Token::H],
            vec![Token::E, Token::L, Token::L, Token::O],
            vec![Token::Space],
            vec![Token::W, Token::O, Token::R, Token::L, Token::D],
            vec![Token::Shift, Token::Plus, Token::_1],
            vec![Token::Shift, Token::Plus, Token::_0],
        ];

        let map = "<Shift>+9 <Shift>+h ello <Space> world <Shift>+1 <Shift>+0";

        kb.send_keystrokes(map);

        assert_eq!(a, Tokonizer::new(map).get());
    }
}
