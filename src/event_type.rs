use uinput::event::keyboard::Key;
use uinput::Device;

use crate::error::Result;
use crate::keycodes::KeyCode;
use crate::token::Token;

#[derive(Debug)]
pub(crate) enum EventType {
    Hold { token: Vec<Token> },
    Click { token: Vec<Token> },
}

impl EventType {
    pub(crate) fn new(map: &str) -> Self {
        if map.contains("+") {
            return EventType::Hold {
                token: map
                    .split("+")
                    .map(|s| Token::new(s))
                    .collect::<Vec<Token>>(),
            };
        } else {
            return EventType::Click {
                token: map.chars().map(|s| Token::new(&s.to_string())).collect(),
            };
        }
    }
    pub(crate) fn run(&self, dev: &mut Device) -> Result<()> {
        match self {
            EventType::Hold { token } => {
                token.iter().for_each(|t| match t {
                    Token::Kp(val) => {
                        dev.press(val).ok();
                    }
                    Token::Key(val) => {
                        dev.press(val).ok();
                    }
                });
                token.iter().for_each(|t| match t {
                    Token::Kp(val) => {
                        dev.release(val).ok();
                    }
                    Token::Key(val) => {
                        dev.release(val).ok();
                    }
                });
                dev.synchronize()?;
            }
            EventType::Click { token } => {
                token.iter().for_each(|k| match k {
                    Token::Kp(val) => {
                        dev.click(val).ok();
                    }
                    Token::Key(val) => {
                        dev.click(val).ok();
                    }
                });
                dev.synchronize()?;
            }
        }
        Ok(())
    }
    pub(crate) fn as_keycodes(&self) -> [u8; 8] {
        let mut codes: [u8; 8] = [0; 8];

        let get_codes = |codes: &mut [u8; 8], token: &Vec<Token>| {
            for t in token {
                match t {
                    Token::Kp(k) => match k {
                        _ => {
                            *KeyCode::from_keypad(*k);
                        }
                    },
                    Token::Key(k) => {
                        match k {
                            Key::LeftAlt
                            | Key::LeftShift
                            | Key::LeftControl
                            | Key::LeftMeta
                            | Key::RightMeta
                            | Key::RightControl
                            | Key::RightShift
                            | Key::RightAlt => codes[0] |= *KeyCode::from_key(*k),
                            _ => codes[2] = *KeyCode::from_key(*k),
                        };
                    }
                }
            }
        };

        match self {
            EventType::Hold { token } => {
                get_codes(&mut codes, token);
            }
            EventType::Click { token } => {
                get_codes(&mut codes, token);
            }
        }

        codes
    }
}
