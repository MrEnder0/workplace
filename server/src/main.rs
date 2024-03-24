#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod heartbeat;
mod web_socket;

use eframe::egui::{self, ScrollArea};
use once_cell::sync::Lazy;
use scorched::*;
use std::{
    sync::{atomic::{AtomicBool, Ordering}, RwLock},
    thread,
    time::Duration,
};

static STATUS: AtomicBool = AtomicBool::new(false);
static UPDATED: Lazy<RwLock<Vec<u8>>> = Lazy::new(|| RwLock::new(vec![]));

fn main() -> Result<(), eframe::Error> {
    scorched::set_logging_path(workplace_common::LOGGING_PATH);

    log_this(LogData {
        importance: LogImportance::Info,
        message: format!(
            "Launching Workplace-Server version {}",
            env!("CARGO_PKG_VERSION")
        ),
    });

    thread::spawn(|| {
        web_socket::server();
    });

    thread::spawn(|| {
        heartbeat::heartbeat_thread();
    });

    eframe::run_native(
        "WorkPlace-Server",
        eframe::NativeOptions {
            centered: true,
            ..Default::default()
        },
        Box::new(move |_cc| Box::<MyApp>::default()),
    )
}

struct MyApp {
    frame_limit: u8,
    dark_mode: bool,
    next_frame: Duration,
    status: bool,
    clients: Vec<u8>,
    updated: Vec<u8>,
    last_refresh: std::time::Instant,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            frame_limit: 60,
            dark_mode: true,
            next_frame: Duration::from_secs(0),
            status: false,
            clients: vec![],
            updated: vec![],
            last_refresh: std::time::Instant::now(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Options");

            if ui
                .add(egui::Checkbox::new(&mut self.status, "Deny Games")).on_hover_text("Denys games from running; recomended disable durring breaktime and leave on otherwise").changed()
            {
                STATUS.store(self.status, Ordering::Relaxed);
            }
            if ui
                .checkbox(&mut self.dark_mode, "Darkmode")
                .on_hover_text("Enables darkmode for the UI")
                .changed()
            {
                if self.dark_mode {
                    ctx.set_visuals(egui::Visuals::dark());
                } else {
                    ctx.set_visuals(egui::Visuals::light());
                }
            }
            ui.add(egui::Slider::new(&mut self.frame_limit, 15..=120).text("UI FPS Limit")).on_hover_text("Limits the FPS of the UI, higher values may increase the smoothness of the UI but may also increase CPU usage");

            ui.separator();

            ui.heading("Clients Info");

            ui.horizontal(|ui| {
                ui.label(format!("Number of clients connected: {}", self.clients.len()));
                if ui
                    .button("Refresh")
                    .on_hover_text("Refreshes the number of clients connected to the server")
                    .clicked()
                {
                    self.clients = heartbeat::get_clients();
                    self.updated = web_socket::get_updated();
                    self.last_refresh = std::time::Instant::now();
                }
            });

            if self.last_refresh.elapsed().as_secs() > 3 {
                self.clients = heartbeat::get_clients();
                self.updated = web_socket::get_updated();
                self.last_refresh = std::time::Instant::now();
            }

            // Client info and management
            let mut removed_clients: Vec<u8> = vec![];
            ScrollArea::vertical().show(ui, |ui| {
                for client in self.clients.iter() {
                    ui.horizontal(|ui| {
                        ui.label(format!("Client ID: {}", client));
                        if ui
                            .button("Shutdown")
                            .on_hover_text("Shuts down the client")
                            .clicked()
                        {
                            web_socket::request_shutdown(*client);
                            removed_clients.push(*client);
                        }
                        if ui
                            .button("Restart")
                            .on_hover_text("Restarts the client")
                            .clicked()
                        {
                            web_socket::request_restart(*client);
                            removed_clients.push(*client);
                        }
                        if self.updated.contains(client) {
                            ui.label("Updated").on_hover_text("Client has been updated and is pending a restart");
                        }
                    });
                }
            });

            for client in removed_clients.iter() {
                self.clients.remove((*client).into());
            }
        });

        egui::TopBottomPanel::bottom("footer").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.hyperlink_to(
                    format!("Workplace-Server {}", env!("CARGO_PKG_VERSION")),
                    format!(
                        "https://github.com/MrEnder0/workplace/releases/tag/{}",
                        env!("CARGO_PKG_VERSION")
                    ),
                );
            });
        });

        self.next_frame =
            Duration::from_millis(((1.0 / self.frame_limit as f32) * 1000.0).round() as u64);
        std::thread::sleep(self.next_frame - Duration::from_millis(1));
        ctx.request_repaint()
    }
}
