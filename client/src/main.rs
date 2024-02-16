#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod web_socket;

use std::{sync::Mutex, thread};
use sysinfo::System;

static STATUS: Mutex<bool> = Mutex::new(false);

const PROCESS_NAMES: [&str; 7] = [
    "RobloxPlayerBeta",
    "RobloxPlayer",
    "Minecraft.Windows",
    "EpicGamesLauncher",
    "steam",
    "XboxPcApp",
    "Discord",
];

fn main() {
    scorched::set_logging_path(workplace_common::LOGGING_PATH);

    thread::spawn(|| {
        web_socket::client();
    });

    loop {
        thread::sleep(std::time::Duration::from_secs(3));
        if *STATUS.lock().unwrap() {
            let s = System::new_all();
            for process_name in PROCESS_NAMES.iter() {
                for process in s.processes_by_name(process_name) {
                    process.kill();
                }
            }
        }
    }
}
