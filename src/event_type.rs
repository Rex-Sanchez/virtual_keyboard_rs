use uinput::Device;

use crate::error::Result;
use crate::tokens::{KeyCodeGroup, Token};
use crate::Tokonizer;

#[derive(Debug)]
pub(crate) enum EventType {
    Hold { token: Vec<KeyCodeGroup> },
    Click { token: Vec<KeyCodeGroup> },
}

impl EventType {
    pub(crate) fn from_tokens(tokens: Tokonizer) -> Vec<EventType> {
        tokens
            .get()
            .into_iter()
            .map(|token_group| {
                if token_group.contains(&Token::Plus) {
                    let tokens = token_group
                        .into_iter()
                        .map(|token| {
                            if token != Token::Plus {
                                return Some(token.to_keycode());
                            } else {
                                return None;
                            }
                        })
                        .flatten()
                        .collect::<Vec<KeyCodeGroup>>();

                    return EventType::Hold { token: tokens };
                } else {
                    return EventType::Click {
                        token: token_group
                            .iter()
                            .map(|f| f.to_keycode())
                            .collect::<Vec<KeyCodeGroup>>(),
                    };
                }
            })
            .collect::<Vec<EventType>>()
    }
    pub(crate) fn run(&self, dev: &mut Device) -> Result<()> {
        match self {
            EventType::Hold { token } => {
                token.iter().for_each(|t| match t {
                    KeyCodeGroup::Kp(val) => {
                        dev.press(val).ok();
                    }
                    KeyCodeGroup::Key(val) => {
                        dev.press(val).ok();
                    }
                });
                token.iter().for_each(|t| match t {
                    KeyCodeGroup::Kp(val) => {
                        dev.release(val).ok();
                    }
                    KeyCodeGroup::Key(val) => {
                        dev.release(val).ok();
                    }
                });
                dev.synchronize()?;
            }
            EventType::Click { token } => {
                token.iter().for_each(|k| match k {
                    KeyCodeGroup::Kp(val) => {
                        dev.click(val).ok();
                    }
                    KeyCodeGroup::Key(val) => {
                        dev.click(val).ok();
                    }
                });
                dev.synchronize()?;
            }
        }
        Ok(())
    }
}
