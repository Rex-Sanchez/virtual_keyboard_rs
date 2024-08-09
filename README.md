# Virtual_keyboard_rs



Easy to use virtual keyboard for rust.
At this time this only works on linux.



```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut vk = VirtKeyboard::new("keyboard_name")?;
    vk.send_keystrokes("<Shift>+h ello <Space> world <Shift>+1")
    Ok(())
}


```


## Available tokens for now are..

```rust
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
        "[" => Self::LeftBrace,
        "]" => Self::RightBrace,
        "." => Self::Dot,
        "," => Self::Comma,
        "/" => Self::Slash,
        "\\" => Self::BackSlash,
        "'" => Self::Apostrophe,
        ";" => Self::SemiColon,

        "a" => Self::A,
        "b" => Self::B,
        "c" => Self::C,
        "d" => Self::D,
        "e" => Self::E,
        "f" => Self::F,
        "g" => Self::G,
        "h" => Self::H,
        "i" => Self::I,
        "j" => Self::J,
        "k" => Self::K,
        "l" => Self::L,
        "m" => Self::M,
        "n" => Self::N,
        "o" => Self::O,
        "p" => Self::P,
        "q" => Self::Q,
        "r" => Self::R,
        "s" => Self::S,
        "t" => Self::T,
        "u" => Self::U,
        "v" => Self::V,
        "w" => Self::W,
        "x" => Self::X,
        "y" => Self::Y,
        "z" => Self::Z,
        
        "1" => Self::_1,
        "2" => Self::_2,
        "3" => Self::_3,
        "4" => Self::_4,
        "5" => Self::_5,
        "6" => Self::_6,
        "7" => Self::_7,
        "8" => Self::_8,
        "9" => Self::_9,
        "0" => Self::_0,

        _ => Self::Deliminator,
        }
```
