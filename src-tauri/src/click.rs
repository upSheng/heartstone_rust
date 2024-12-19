use std::{thread, time};

use rdev::{listen, simulate, Event, EventType, SimulateError};

pub fn main() {
    println!("Hello, world!");
    let (w, h) = rdev::display_size().unwrap();
    println!("My screen size : {:?}x{:?}", w, h);
    if let Err(error) = listen(callback) {
        println!("Error: {:?}", error)
    }
}

fn send(event_type: &EventType) {
    let delay = time::Duration::from_millis(20);
    match simulate(event_type) {
        Ok(()) => (),
        Err(SimulateError) => {
            println!("We could not send {:?}", event_type);
        }
    }
    // Let ths OS catchup (at least MacOS)
    thread::sleep(delay);
}


fn callback(event: Event) {
    println!("My1aaa callback {:?}", event);
    match event.name {
        Some(string) => {
            if event.event_type == EventType::KeyPress(rdev::Key::KeyA) {
                send(&EventType::MouseMove { x: 0.0, y: 0.0 });
            }
            if event.event_type == EventType::KeyPress(rdev::Key::KeyD) {
                send(&EventType::MouseMove { x: 800.0, y: 1000.0 });
            }
            println!("User wrote {:?}", string)
        }
        None => (),
    }
}