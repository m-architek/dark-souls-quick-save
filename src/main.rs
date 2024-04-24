use std::{env, fs, thread};
use std::path::PathBuf;
use std::time::Duration;

use inputbot::KeybdKey;

fn main() {
    let save_path = build_save_path();
    if !save_path.exists() {
        panic!("Cannot find save file: {save_path:?}");
    }

    KeybdKey::F1Key.bind(|| println!("-- F5 - Quick Save; F6 - Quick Load"));
    KeybdKey::F5Key.bind(|| quick_save());
    KeybdKey::F6Key.bind(|| quick_load());

    println!("Initialization done");
    beep();
    inputbot::handle_input_events();
}

fn build_save_path() -> PathBuf {
    return build_path( "DRAKS0005.sl2");
}

fn build_quick_save_path() -> PathBuf {
    return build_path("QUICK_SAVE.sl2");
}

fn build_backup_path(name: &str) -> PathBuf {
    return build_path(&format!("BACKUP_{name}.sl2"));
}

fn quick_save() {
    let save_path = build_save_path();
    let quick_save_path = build_quick_save_path();
    let backup_path = build_backup_path("QUICK_SAVE");

    if quick_save_path.exists() {
        fs::copy(&quick_save_path, &backup_path).unwrap();
    }
    fs::copy(&save_path, &quick_save_path).unwrap();

    println!("Quick Save done successfully");
    beep();
}

fn quick_load() {
    let save_path = build_save_path();
    let quick_save_path = build_quick_save_path();
    let backup_path = build_backup_path("QUICK_LOAD");

    if !quick_save_path.exists() {
        println!("Cannot find quick save file: {quick_save_path:?}");
        return;
    }
    fs::copy(&save_path, &backup_path).unwrap();
    fs::copy(&quick_save_path, &save_path).unwrap();

    println!("Quick Load done successfully");
    beep();
    thread::sleep(Duration::from_millis(500));
    beep();
}

fn build_path(file_name: &str) -> PathBuf {
    let mut path = env::current_dir().unwrap();
    path.push(file_name);
    return path
}

fn beep() {
    println!("{}", '\x07'.to_string());
}
