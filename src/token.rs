use std::{ops::Deref, process};

use uinput::event::keyboard::{Key, KeyPad};

#[derive(Debug)]
//pub(crate) struct Token(pub Option<Key>, pub Option<KeyPad>);
pub(crate) enum Token {
    Kp(KeyPad),
    Key(Key),
}

impl From<Key> for Token {
    fn from(value: Key) -> Self {
        Self::Key(value)
    }
}
impl From<KeyPad> for Token {
    fn from(value: KeyPad) -> Self {
        Self::Kp(value)
    }
}

impl Token {
    pub(crate) fn new(s: &str) -> Self {
        match s {
            "Lctrl" | "ctrl" => Key::LeftControl.into(),
            "Rctrl" => Key::RightControl.into(),
            "Latl" | "alt" => Key::LeftAlt.into(),
            "Ralt" => Key::RightAlt.into(),
            "Lshift" | "shift" => Key::LeftShift.into(),
            "Rshift" => Key::RightShift.into(),
            "Lsuper" | "super" => Key::LeftMeta.into(),
            "Rsuper" => Key::RightMeta.into(),

            "a" | "A" => Key::A.into(),
            "b" | "B" => Key::B.into(),
            "c" | "C" => Key::C.into(),
            "d" | "D" => Key::D.into(),
            "e" | "E" => Key::E.into(),
            "f" | "F" => Key::F.into(),
            "g" | "G" => Key::G.into(),
            "h" | "H" => Key::H.into(),
            "i" | "I" => Key::I.into(),
            "j" | "J" => Key::J.into(),
            "k" | "K" => Key::K.into(),
            "l" | "L" => Key::L.into(),
            "m" | "M" => Key::M.into(),
            "n" | "N" => Key::N.into(),
            "o" | "O" => Key::O.into(),
            "p" | "P" => Key::P.into(),
            "q" | "Q" => Key::Q.into(),
            "r" | "R" => Key::R.into(),
            "s" | "S" => Key::S.into(),
            "t" | "T" => Key::T.into(),
            "u" | "U" => Key::U.into(),
            "v" | "V" => Key::V.into(),
            "w" | "W" => Key::W.into(),
            "x" | "X" => Key::X.into(),
            "y" | "Y" => Key::Y.into(),
            "z" | "Z" => Key::Z.into(),

            "0" => Key::_0.into(),
            "1" => Key::_1.into(),
            "2" => Key::_2.into(),
            "3" => Key::_3.into(),
            "4" => Key::_4.into(),
            "5" => Key::_5.into(),
            "6" => Key::_6.into(),
            "7" => Key::_7.into(),
            "8" => Key::_8.into(),
            "9" => Key::_9.into(),

            "Minus" => Key::Minus.into(),
            "Equal" => Key::Equal.into(),
            "BackSpace" => Key::BackSpace.into(),
            "Tab" => Key::Tab.into(),
            "LeftBrace" => Key::LeftBrace.into(),
            "RightBrace" => Key::RightBrace.into(),
            "Enter" => Key::Enter.into(),
            "SemiColon" => Key::SemiColon.into(),
            "SingleQuote" => Key::Apostrophe.into(),
            "Backtick" => Key::Grave.into(),
            "Backslash" => Key::BackSlash.into(),
            "Comma" => Key::Comma.into(),
            "Dot" => Key::Dot.into(),
            "Space" => Key::Space.into(),
            "CapsLock" => Key::CapsLock.into(),
            "NumLock" => Key::NumLock.into(),
            "ScrollLock" => Key::ScrollLock.into(),

            "SysRq" => Key::SysRq.into(),
            "LineFeed" => Key::LineFeed.into(),

            "Home" => Key::Home.into(),
            "End" => Key::End.into(),
            "Insert" => Key::Insert.into(),
            "Delete" => Key::Delete.into(),

            "PgUp" => Key::PageUp.into(),
            "PgDn" => Key::PageDown.into(),

            "ScrollUp" => Key::ScrollUp.into(),
            "ScrollDown" => Key::ScrollDown.into(),

            "Up" => Key::Up.into(),
            "Down" => Key::Down.into(),
            "Left" => Key::Left.into(),
            "Right" => Key::Right.into(),

            "F1" => Key::F1.into(),
            "F2" => Key::F2.into(),
            "F3" => Key::F3.into(),
            "F4" => Key::F4.into(),
            "F5" => Key::F5.into(),
            "F6" => Key::F6.into(),
            "F7" => Key::F7.into(),
            "F8" => Key::F8.into(),
            "F9" => Key::F9.into(),
            "F10" => Key::F10.into(),
            "F11" => Key::F11.into(),
            "F12" => Key::F12.into(),
            "F13" => Key::F13.into(),
            "F14" => Key::F14.into(),
            "F15" => Key::F15.into(),
            "F16" => Key::F16.into(),
            "F17" => Key::F17.into(),
            "F18" => Key::F18.into(),
            "F19" => Key::F19.into(),
            "F20" => Key::F20.into(),
            "F21" => Key::F21.into(),
            "F22" => Key::F22.into(),
            "F23" => Key::F23.into(),
            "F24" => Key::F24.into(),

            _ => {
                println!("[!] Unknown Key: {s}");
                process::exit(1)
            }
        }
    }
}
