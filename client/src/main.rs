#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod web_socket;

use std::{sync::Mutex, thread};

use sysinfo::System;

static STATUS: Mutex<bool> = Mutex::new(false);

const PROCESS_NAMES: [&str; 3] = ["RobloxPlayerBeta", "RobloxPlayer", "Minecraft.Windows"];

fn main() {
    thread::spawn(|| {
        web_socket::client();
    });

    loop {
        thread::sleep(std::time::Duration::from_secs(10));
        if !*STATUS.lock().unwrap() {
            let s = System::new_all();
            for process_name in PROCESS_NAMES.iter() {
                for process in s.processes_by_name(process_name) {
                    println!("{} {}", process.pid(), process.name());
                    process.kill();
                }
            }
        }
    }
}
