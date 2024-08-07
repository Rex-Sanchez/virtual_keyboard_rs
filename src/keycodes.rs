use std::ops::Deref;

use uinput::event::keyboard::{Key, KeyPad};


pub struct KeyCode(pub u8);

impl Deref for KeyCode {
    type Target = u8;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl KeyCode {
    pub fn from_key(token: Key) -> Self {
        match token{
            Key::Reserved => KeyCode(0xa5),
            Key::Esc => KeyCode(0x29),

            Key::_1 => KeyCode(0x1e),
            Key::_2 => KeyCode(0x1f),
            Key::_3 => KeyCode(0x20),
            Key::_4 => KeyCode(0x21),
            Key::_5 => KeyCode(0x22),
            Key::_6 => KeyCode(0x23),
            Key::_7 => KeyCode(0x24),
            Key::_8 => KeyCode(0x25),
            Key::_9 => KeyCode(0x26),
            Key::_0 => KeyCode(0x27),

            Key::Minus => KeyCode(0x2d),
            Key::Equal => KeyCode(0x2e),
            Key::BackSpace => KeyCode(0x2a),
            Key::Tab => KeyCode(0x2b),

            Key::Q => KeyCode(0x14),
            Key::W => KeyCode(0x1a),
            Key::E => KeyCode(0x08),
            Key::R => KeyCode(0x15),
            Key::T => KeyCode(0x17),
            Key::Y => KeyCode(0x1c),
            Key::U => KeyCode(0x18),
            Key::I => KeyCode(0x0c),
            Key::O => KeyCode(0x12),
            Key::P => KeyCode(0x13),

            Key::A => KeyCode(0x04),
            Key::S => KeyCode(0x16),
            Key::D => KeyCode(0x07),
            Key::F => KeyCode(0x09),
            Key::G => KeyCode(0x0a),
            Key::H => KeyCode(0x0b),
            Key::J => KeyCode(0x0d),
            Key::K => KeyCode(0x0e),
            Key::L => KeyCode(0x0F),

            Key::Z => KeyCode(0x1d),
            Key::X => KeyCode(0x1b),
            Key::C => KeyCode(0x06),
            Key::V => KeyCode(0x19),
            Key::B => KeyCode(0x05),
            Key::N => KeyCode(0x11),
            Key::M => KeyCode(0x10),

            Key::LeftBrace => KeyCode(0x2f),
            Key::RightBrace => KeyCode(0x30),
            Key::Enter => KeyCode(0x28),
            Key::LeftControl => KeyCode(0x01),

            Key::SemiColon => KeyCode(0x33),
            Key::Apostrophe => KeyCode(0x34),
            Key::Grave => KeyCode(0x35),
            Key::LeftShift => KeyCode(0x02),
            Key::BackSlash => KeyCode(0x31),
            Key::Comma => KeyCode(0x36),
            Key::Dot => KeyCode(0x37),
            Key::Slash => KeyCode(0x38),
            Key::RightShift => KeyCode(0x40),
            Key::LeftAlt => KeyCode(0x08),
            Key::Space => KeyCode(0x2c),
            Key::CapsLock => KeyCode(0x39),

            Key::F1 => KeyCode(0x3a),
            Key::F2 => KeyCode(0x3b),
            Key::F3 => KeyCode(0x3c),
            Key::F4 => KeyCode(0x3d),
            Key::F5 => KeyCode(0x3e),
            Key::F6 => KeyCode(0x3f),
            Key::F7 => KeyCode(0x40),
            Key::F8 => KeyCode(0x41),
            Key::F9 => KeyCode(0x42),
            Key::F10 => KeyCode(0x43),
            Key::F11 => KeyCode(0x44),
            Key::F12 => KeyCode(0x45),
            Key::F13 => KeyCode(0x68),
            Key::F14 => KeyCode(0x69),
            Key::F15 => KeyCode(0x6a),
            Key::F16 => KeyCode(0x6b),
            Key::F17 => KeyCode(0x6c),
            Key::F18 => KeyCode(0x6d),
            Key::F19 => KeyCode(0x6e),
            Key::F20 => KeyCode(0x6f),
            Key::F21 => KeyCode(0x70),
            Key::F22 => KeyCode(0x71),
            Key::F23 => KeyCode(0x72),
            Key::F24 => KeyCode(0x73),

            Key::NumLock => KeyCode(0x83),
            Key::ScrollLock => KeyCode(0x84),
            Key::RightControl => KeyCode(0x10),
            Key::SysRq => KeyCode(0x9a),
            Key::RightAlt => KeyCode(0x40),
            Key::LineFeed => KeyCode(0xa5),

            Key::Up => KeyCode(0x52),
            Key::Down => KeyCode(0x51),
            Key::Left => KeyCode(0x50),
            Key::Right => KeyCode(0x4f),

            Key::End => KeyCode(0x4d),
            Key::Insert => KeyCode(0x49),
            Key::Home => KeyCode(0x4a),
            Key::Delete => KeyCode(0x4c),

            Key::PageDown => KeyCode(0x4e),
            Key::PageUp => KeyCode(0x4b),

            Key::LeftMeta => KeyCode(0x08),
            Key::RightMeta => KeyCode(0x80),
            Key::ScrollUp => KeyCode(0x80),
            Key::ScrollDown => KeyCode(0x81),
        }
    }

    pub fn from_keypad(token: KeyPad) -> Self {
        match token {
            KeyPad::Asterisk => KeyCode(0x55),
            KeyPad::_7 => KeyCode(0x5f),
            KeyPad::_8 => KeyCode(0x60),
            KeyPad::_9 => KeyCode(0x61),
            KeyPad::Minus => KeyCode(0x56),
            KeyPad::_4 => KeyCode(0x5c),
            KeyPad::_5 => KeyCode(0x5d),
            KeyPad::_6 => KeyCode(0x5e),
            KeyPad::Plus => KeyCode(0x57),
            KeyPad::_1 => KeyCode(0x59),
            KeyPad::_2 => KeyCode(0x5a),
            KeyPad::_3 => KeyCode(0x5b),
            KeyPad::_0 => KeyCode(0x62),
            KeyPad::Dot => KeyCode(0x63),
            KeyPad::AltComma => KeyCode(0x5a),
            KeyPad::Enter => KeyCode(0x58),
            KeyPad::Slash => KeyCode(0x54),
            KeyPad::Equal => KeyCode(0x5a),
            KeyPad::PlusMinus => KeyCode(0x5a),
            KeyPad::Comma => KeyCode(0x5a),
            KeyPad::LeftParen => KeyCode(0x5a),
            KeyPad::RightParen => KeyCode(0x5a),
        }
    }
}
