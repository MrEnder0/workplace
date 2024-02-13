#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod heartbeat;
mod web_socket;

use eframe::egui;
use std::{sync::Mutex, thread, time::Duration};

static STATUS: Mutex<bool> = Mutex::new(false);

fn main() -> Result<(), eframe::Error> {
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
    //init: bool,
    frame_limit: u8,
    dark_mode: bool,
    next_frame: Duration,
    status: bool,
    clients: u8,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            //init: true,
            frame_limit: 60,
            dark_mode: true,
            next_frame: Duration::from_secs(0),
            status: false,
            clients: 0,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Information");

            ui.horizontal(|ui| {
                ui.label(format!("Number of clients connected: {}", self.clients));
                if ui
                    .button("Refresh")
                    .on_hover_text("Refreshes the number of clients connected to the server")
                    .clicked()
                {
                    self.clients = heartbeat::get_clients();
                }
            });
            ui.separator();

            ui.heading("Options");

            if ui
                .add(egui::Checkbox::new(&mut self.status, "Deny Games")).on_hover_text("Denys games from running; recomended disable durring breaktime and leave on otherwise").changed()
            {
                *STATUS.lock().unwrap() = self.status;
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
