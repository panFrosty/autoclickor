use device_query::{DeviceEvents, DeviceEventsHandler, Keycode, MouseButton, MousePosition};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use enigo::{Enigo, Key, Keyboard, Settings};

const SINGLE_SPAM_KEY: char = 'o';
const AOE_SPAM_KEY: char = 'p';
const AOE_SPAM_BUTTON: MouseButton = 4;
const SINGLE_SPAM_BUTTON: Keycode = Keycode::Grave;

struct SpamState {
    spam_single: bool,
    spam_aoe: bool,
}

fn spam_thread(spam_state: Arc<Mutex<SpamState>>) {
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    loop {
        let state = spam_state.lock().unwrap();
        if state.spam_aoe {
            let _ = enigo.key(Key::Unicode(AOE_SPAM_KEY), enigo::Direction::Click);
        } else if state.spam_single {
            let _ = enigo.key(Key::Unicode(SINGLE_SPAM_KEY), enigo::Direction::Click);
        }
        drop(state);
        thread::sleep(Duration::from_millis(100));
    }
}

fn listener_thread(spam_state: Arc<Mutex<SpamState>>) {
    let event_handler = DeviceEventsHandler::new(Duration::from_millis(10))
        .expect("Could not initialize event loop");
    let spam_state_for_mouse = spam_state.clone();
    let _mouse_button_detect_guard =
        event_handler.on_mouse_down(move |key_pressed: &MouseButton| {
            if *key_pressed == AOE_SPAM_BUTTON {
                let mut state = spam_state_for_mouse.lock().unwrap();
                state.spam_aoe = !state.spam_aoe;
                state.spam_single = false;
                println!("Pressed Mouse key {}", key_pressed);
            }
        });
    let spam_state_for_keyboard = spam_state.clone();
    let _key_detect_guard = event_handler.on_key_down(move |key_pressed: &Keycode| {
        if *key_pressed == SINGLE_SPAM_BUTTON {
            let mut state = spam_state_for_keyboard.lock().unwrap();
            println!("Pressed keyboard key {}", key_pressed);
            state.spam_aoe = false;
            state.spam_single = !state.spam_single;
            drop(state);
        }
    });
    loop {
        thread::sleep(Duration::from_millis(20));
    }
}

fn main() {
    println!("Hello, world!");
    let spam_state = Arc::new(Mutex::new(SpamState {
        spam_single: false,
        spam_aoe: false,
    }));
    let spam_state_clone_for_output = spam_state.clone();
    let spam_state_clone_for_input = spam_state.clone();

    let spam_thread = thread::spawn(|| spam_thread(spam_state_clone_for_output));
    let listener_thread = thread::spawn(|| listener_thread(spam_state_clone_for_input));

    listener_thread.join().unwrap();
    spam_thread.join().unwrap();
}
