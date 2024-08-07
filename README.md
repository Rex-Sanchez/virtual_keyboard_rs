# Virtual_keyboard_rs





```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    

    let mut vk = VirtKeyboard::new("keyboard_name")?;

    vk.send_keystrokes("shift+1");
    vk.send_keystrokes("shift+0 shift+ctrl+9");
    vk.send_keystrokes("shift+h e l l o _ w o r l d")

    Ok(())
}


```
