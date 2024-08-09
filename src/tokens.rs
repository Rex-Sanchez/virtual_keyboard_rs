use uinput::event::keyboard::{Key, KeyPad};

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum KeyCodeGroup {
    Kp(KeyPad),
    Key(Key),
}
#[allow(unused)]
#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) enum Token {
    Space,
    Enter,

    Shift,
    LShift,
    RShift,

    Super,
    RSuper,
    LSuper,

    Alt,
    RAlt,
    LAlt,

    Ctrl,
    RCtrl,
    LCtrl,

    Plus,
    Minus,
    Equals,
    Backspace,
    Tab,
    Escape,

    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
    F21,
    F22,
    F23,
    F24,

    Q,
    W,
    E,
    R,
    T,
    Y,
    U,
    I,
    O,
    P,

    A,
    S,
    D,
    F,
    G,
    H,
    J,
    K,
    L,

    Z,
    X,
    C,
    V,
    B,
    N,
    M,

    _1,
    _2,
    _3,
    _4,
    _5,
    _6,
    _7,
    _8,
    _9,
    _0,

    LeftBrace,
    RightBrace,

    SemiColon,
    Apostrophe,
    Grave,
    BackSlash,
    Comma,
    Dot,
    Slash,
    CapsLock,

    NumLock,
    ScrollLock,
    SysRq,
    LineFeed,

    Up,
    Down,
    Left,
    Right,

    End,
    Insert,
    Home,
    Delete,

    PageDown,
    PageUp,

    LeftMeta,
    RightMeta,
    ScrollUp,
    ScrollDown,

    Deliminator,
}

impl Token {
    pub(crate) fn new(s: &str) -> Self {
        match s {
            "Space" => Self::Space,
            "Enter" => Self::Enter,

            "Super" => Self::Super,
            "RSuper" => Self::RSuper,
            "LSuper" => Self::LSuper,

            "Ctrl" => Self::Ctrl,
            "RCtrl" => Self::RCtrl,
            "LCtrl" => Self::LCtrl,

            "Alt" => Self::Alt,
            "RAlt" => Self::RAlt,
            "LAlt" => Self::LAlt,

            "Shift" => Self::Shift,
            "LShift" => Self::LShift,
            "TShift" => Self::RShift,

            "+" => Self::Plus,
            "-" => Self::Minus,

            "i" => Self::I,
            "n" => Self::N,
            "1" => Self::_1,
            "2" => Self::_2,
            _ => Self::Deliminator,
        }
    }
    pub(crate) fn to_keycode(&self) -> KeyCodeGroup {
        match self {
            Token::Space => KeyCodeGroup::Key(Key::Space),
            Token::Enter => KeyCodeGroup::Key(Key::Enter),
            Token::Shift => KeyCodeGroup::Key(Key::LeftShift),
            Token::LShift => KeyCodeGroup::Key(Key::LeftShift),
            Token::RShift => KeyCodeGroup::Key(Key::RightShift),
            Token::Super => KeyCodeGroup::Key(Key::LeftShift),
            Token::RSuper => KeyCodeGroup::Key(Key::RightMeta),
            Token::LSuper => KeyCodeGroup::Key(Key::LeftMeta),
            Token::Alt => KeyCodeGroup::Key(Key::LeftAlt),
            Token::RAlt => KeyCodeGroup::Key(Key::RightAlt),
            Token::LAlt => KeyCodeGroup::Key(Key::LeftAlt),
            Token::Ctrl => KeyCodeGroup::Key(Key::LeftControl),
            Token::RCtrl => KeyCodeGroup::Key(Key::RightControl),
            Token::LCtrl => KeyCodeGroup::Key(Key::LeftControl),

            Token::Plus => KeyCodeGroup::Kp(KeyPad::Plus),
            Token::Minus => KeyCodeGroup::Kp(KeyPad::Minus),

            Token::Equals => KeyCodeGroup::Key(Key::Equal),
            Token::Backspace => KeyCodeGroup::Key(Key::BackSpace),
            Token::Tab => KeyCodeGroup::Key(Key::Tab),
            Token::Escape => KeyCodeGroup::Key(Key::Esc),

            Token::F1 => KeyCodeGroup::Key(Key::F1),
            Token::F2 => KeyCodeGroup::Key(Key::F2),
            Token::F3 => KeyCodeGroup::Key(Key::F3),
            Token::F4 => KeyCodeGroup::Key(Key::F4),
            Token::F5 => KeyCodeGroup::Key(Key::F5),
            Token::F6 => KeyCodeGroup::Key(Key::F6),
            Token::F7 => KeyCodeGroup::Key(Key::F7),
            Token::F8 => KeyCodeGroup::Key(Key::F8),
            Token::F9 => KeyCodeGroup::Key(Key::F9),
            Token::F10 => KeyCodeGroup::Key(Key::F10),
            Token::F11 => KeyCodeGroup::Key(Key::F11),
            Token::F12 => KeyCodeGroup::Key(Key::F12),
            Token::F13 => KeyCodeGroup::Key(Key::F13),
            Token::F14 => KeyCodeGroup::Key(Key::F14),
            Token::F15 => KeyCodeGroup::Key(Key::F15),
            Token::F16 => KeyCodeGroup::Key(Key::F16),
            Token::F17 => KeyCodeGroup::Key(Key::F17),
            Token::F18 => KeyCodeGroup::Key(Key::F18),
            Token::F19 => KeyCodeGroup::Key(Key::F19),
            Token::F20 => KeyCodeGroup::Key(Key::F20),
            Token::F21 => KeyCodeGroup::Key(Key::F21),
            Token::F22 => KeyCodeGroup::Key(Key::F22),
            Token::F23 => KeyCodeGroup::Key(Key::F23),
            Token::F24 => KeyCodeGroup::Key(Key::F24),

            Token::I => KeyCodeGroup::Key(Key::I),
            Token::N => KeyCodeGroup::Key(Key::N),
            Token::Q => KeyCodeGroup::Key(Key::Q),
            Token::W => KeyCodeGroup::Key(Key::W),
            Token::E => KeyCodeGroup::Key(Key::E),
            Token::R => KeyCodeGroup::Key(Key::R),
            Token::T => KeyCodeGroup::Key(Key::T),
            Token::Y => KeyCodeGroup::Key(Key::Y),
            Token::U => KeyCodeGroup::Key(Key::U),
            Token::O => KeyCodeGroup::Key(Key::O),
            Token::P => KeyCodeGroup::Key(Key::P),
            Token::A => KeyCodeGroup::Key(Key::A),
            Token::S => KeyCodeGroup::Key(Key::S),
            Token::D => KeyCodeGroup::Key(Key::D),
            Token::F => KeyCodeGroup::Key(Key::F),
            Token::G => KeyCodeGroup::Key(Key::G),
            Token::H => KeyCodeGroup::Key(Key::H),
            Token::J => KeyCodeGroup::Key(Key::J),
            Token::K => KeyCodeGroup::Key(Key::K),
            Token::L => KeyCodeGroup::Key(Key::L),
            Token::Z => KeyCodeGroup::Key(Key::Z),
            Token::X => KeyCodeGroup::Key(Key::X),
            Token::C => KeyCodeGroup::Key(Key::C),
            Token::V => KeyCodeGroup::Key(Key::V),
            Token::B => KeyCodeGroup::Key(Key::B),
            Token::M => KeyCodeGroup::Key(Key::M),

            Token::_1 => KeyCodeGroup::Key(Key::_1),
            Token::_2 => KeyCodeGroup::Key(Key::_2),
            Token::_3 => KeyCodeGroup::Key(Key::_3),
            Token::_4 => KeyCodeGroup::Key(Key::_4),
            Token::_5 => KeyCodeGroup::Key(Key::_5),
            Token::_6 => KeyCodeGroup::Key(Key::_6),
            Token::_7 => KeyCodeGroup::Key(Key::_7),
            Token::_8 => KeyCodeGroup::Key(Key::_8),
            Token::_9 => KeyCodeGroup::Key(Key::_9),
            Token::_0 => KeyCodeGroup::Key(Key::_0),

            Token::LeftBrace => KeyCodeGroup::Key(Key::LeftBrace),
            Token::RightBrace => KeyCodeGroup::Key(Key::RightBrace),
            Token::SemiColon => KeyCodeGroup::Key(Key::SemiColon),
            Token::Apostrophe => KeyCodeGroup::Key(Key::Apostrophe),
            Token::Grave => KeyCodeGroup::Key(Key::Grave),
            Token::BackSlash => KeyCodeGroup::Key(Key::BackSlash),
            Token::Comma => KeyCodeGroup::Key(Key::Comma),
            Token::Dot => KeyCodeGroup::Key(Key::Dot),
            Token::Slash => KeyCodeGroup::Key(Key::Slash),
            Token::CapsLock => KeyCodeGroup::Key(Key::CapsLock),
            Token::NumLock => KeyCodeGroup::Key(Key::NumLock),
            Token::ScrollLock => KeyCodeGroup::Key(Key::ScrollLock),
            Token::SysRq => KeyCodeGroup::Key(Key::SysRq),
            Token::LineFeed => KeyCodeGroup::Key(Key::LineFeed),
            Token::Up => KeyCodeGroup::Key(Key::Up),
            Token::Down => KeyCodeGroup::Key(Key::Down),
            Token::Left => KeyCodeGroup::Key(Key::Left),
            Token::Right => KeyCodeGroup::Key(Key::Right),
            Token::End => KeyCodeGroup::Key(Key::End),
            Token::Insert => KeyCodeGroup::Key(Key::Insert),
            Token::Home => KeyCodeGroup::Key(Key::Home),
            Token::Delete => KeyCodeGroup::Key(Key::Delete),
            Token::PageDown => KeyCodeGroup::Key(Key::PageDown),
            Token::PageUp => KeyCodeGroup::Key(Key::PageUp),
            Token::LeftMeta => KeyCodeGroup::Key(Key::LeftMeta),
            Token::RightMeta => KeyCodeGroup::Key(Key::RightMeta),
            Token::ScrollUp => KeyCodeGroup::Key(Key::ScrollUp),
            Token::ScrollDown => KeyCodeGroup::Key(Key::ScrollDown),

            Token::Deliminator => unreachable!(),
        }
    }
}
