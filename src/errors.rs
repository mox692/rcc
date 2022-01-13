use once_cell::sync::Lazy;
use std::{ops::Deref, sync::Mutex};

pub static INPUT: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new(String::new()));

pub fn init_error(input: String) {
    INPUT.lock().unwrap().push_str(input.as_str());
}

// ここのposは, inputからのoffsetとして変換ずみ
pub fn display_around_pos(pos: usize) -> String {
    let mut p1 = INPUT.lock().unwrap().deref().clone();
    // p1 -> "aaa.../"

    if pos + 10 < INPUT.lock().unwrap().deref().len() {
        let _ = p1.split_off(pos + 10);
    } else {
        let _ = p1.split_off(INPUT.lock().unwrap().deref().len());
    }

    if pos < 9 {
        return p1;
    }
    let p2 = p1.split_off(pos - 10);
    return p2;
}
