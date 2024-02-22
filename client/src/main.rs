#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod web_socket;

use std::{
    sync::atomic::{AtomicBool, Ordering},
    thread,
};
use sysinfo::System;

static STATUS: AtomicBool = AtomicBool::new(false);

const PROCESS_NAMES: [&str; 8] = [
    "RobloxPlayerBeta",
    "RobloxPlayer",
    "Minecraft.Windows",
    "EpicGamesLauncher",
    "steam",
    "XboxPcApp",
    "Discord",
    "RiotClientUx",
];

fn main() {
    scorched::set_logging_path(workplace_common::LOGGING_PATH);

    thread::spawn(|| {
        web_socket::client();
    });

    loop {
        thread::sleep(std::time::Duration::from_secs(2));
        if STATUS.load(Ordering::Relaxed) {
            let s = System::new_all();
            for process_name in PROCESS_NAMES.iter() {
                for process in s.processes_by_name(process_name) {
                    process.kill();
                }
            }
        }
    }
}

pub(crate) fn update_client(version: &str) {
    let url = format!(
        "https://github.com/MrEnder0/workplace/releases/download/{}/workplace-client.exe",
        version
    );
    let response = reqwest::blocking::get(url).unwrap();

    std::fs::write("workplace-client.exe.update", response.bytes().unwrap()).unwrap();

    self_replace::self_replace("workplace-client.exe.update").unwrap();
}
