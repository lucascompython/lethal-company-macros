use rdev::{Event, EventType, Key, grab, simulate};

fn main() {
    println!("Listening");

    println!("Press NUMPAD 6 to switch, NUMPAD 5 to view monitor, or NUMBER 5 to transmit.");

    if let Err(e) = grab(callback) {
        eprintln!("Error listening for events: {:?}", e);
        std::process::exit(1);
    }
}

fn text(text: &str) {
    for c in text.chars() {
        unsafe {
            let key = match c {
                'a' => Key::KeyA,
                'b' => Key::KeyB,
                'c' => Key::KeyC,
                'd' => Key::KeyD,
                'e' => Key::KeyE,
                'f' => Key::KeyF,
                'g' => Key::KeyG,
                'h' => Key::KeyH,
                'i' => Key::KeyI,
                'j' => Key::KeyJ,
                'k' => Key::KeyK,
                'l' => Key::KeyL,
                'm' => Key::KeyM,
                'n' => Key::KeyN,
                'o' => Key::KeyO,
                'p' => Key::KeyP,
                'q' => Key::KeyQ,
                'r' => Key::KeyR,
                's' => Key::KeyS,
                't' => Key::KeyT,
                'u' => Key::KeyU,
                'v' => Key::KeyV,
                'w' => Key::KeyW,
                'x' => Key::KeyX,
                'y' => Key::KeyY,
                'z' => Key::KeyZ,
                '0' => Key::Num0,
                '1' => Key::Num1,
                '2' => Key::Num2,
                '3' => Key::Num3,
                '4' => Key::Num4,
                '5' => Key::Num5,
                '6' => Key::Num6,
                '7' => Key::Num7,
                '8' => Key::Num8,
                '9' => Key::Num9,
                ' ' => Key::Space,
                '\n' => Key::Return,
                _ => continue,
            };
            simulate(&EventType::KeyPress(key)).unwrap_unchecked();
        }
    }
}

fn callback(event: Event) -> Option<Event> {
    unsafe {
        match event.event_type {
            EventType::KeyPress(Key::Kp6) => {
                text("switch");
                simulate(&EventType::KeyPress(Key::Return)).unwrap_unchecked();
                return None;
            }
            EventType::KeyPress(Key::Kp5) => {
                text("view monitor");
                simulate(&EventType::KeyPress(Key::Return)).unwrap_unchecked();
                return None;
            }
            EventType::KeyPress(Key::Num5) => {
                text("transmit ");
                return None;
            }
            _ => (),
        }
    }
    Some(event)
}
