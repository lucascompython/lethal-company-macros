use std::cell::RefCell;

use enigo::{Enigo, Keyboard, Settings};
use rdev::{Event, EventType, Key, grab};

thread_local! {
    static ENIGO: RefCell<Enigo> = RefCell::new(unsafe {
        Enigo::new(&Settings::default()).unwrap_unchecked()
    });
}

fn main() {
    println!("Listening");

    println!("Press NUMPAD 6 to switch, NUMPAD 5 to view monitor, or NUMBER 5 to transmit.");

    if let Err(e) = grab(callback) {
        eprintln!("Error listening for events: {:?}", e);
        std::process::exit(1);
    }
}

fn callback(event: Event) -> Option<Event> {
    unsafe {
        match event.event_type {
            EventType::KeyPress(Key::Kp6) => {
                ENIGO.with(|enigo| {
                    let mut enigo = enigo.borrow_mut();
                    enigo.text("switch").unwrap_unchecked();
                    enigo
                        .key(enigo::Key::Return, enigo::Direction::Click)
                        .unwrap_unchecked();
                });
                return None;
            }
            EventType::KeyPress(Key::Kp5) => {
                ENIGO.with(|enigo| {
                    let mut enigo = enigo.borrow_mut();
                    enigo.text("view monitor").unwrap_unchecked();
                    enigo
                        .key(enigo::Key::Return, enigo::Direction::Click)
                        .unwrap_unchecked();
                });
                return None;
            }
            EventType::KeyPress(Key::Num5) => {
                ENIGO.with(|enigo| {
                    let mut enigo = enigo.borrow_mut();
                    enigo.text("transmit ").unwrap_unchecked();
                });
                return None;
            }
            _ => (),
        }
    }
    Some(event)
}
