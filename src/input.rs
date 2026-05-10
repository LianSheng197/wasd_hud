use rdev::{Button, Key};

pub fn map_key(k: Key) -> Option<&'static str> {
    Some(match k {
        Key::BackQuote => "BACKQUOTE",
        Key::Num1 => "1",
        Key::Num2 => "2",
        Key::Num3 => "3",
        Key::Num4 => "4",
        Key::Tab => "TAB",
        Key::KeyW => "W",
        Key::KeyE => "E",
        Key::KeyR => "R",
        Key::KeyA => "A",
        Key::KeyS => "S",
        Key::KeyD => "D",
        Key::KeyF => "F",
        Key::KeyX => "X",
        Key::ShiftLeft => "LSHIFT",
        Key::Alt => "LALT",
        Key::Space => "SPACE",
        _ => return None,
    })
}

pub fn map_button(b: Button) -> Option<&'static str> {
    Some(match b {
        Button::Left => "LMB",
        Button::Right => "RMB",
        Button::Middle => "MMB",
        _ => return None,
    })
}
