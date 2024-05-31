#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::{egui::*, *};
use egui_demo_lib::DemoWindows;

fn main() {
    let native_options = NativeOptions {
        ..Default::default()
    };
    run_native(
        "egui_system_theme Demo App",
        native_options,
        Box::new(|cc| Box::new(SystemThemeDemoApp::new(cc))),
    )
    .unwrap();
}

#[derive(Default)]
struct SystemThemeDemoApp {
    demo_windows: DemoWindows,
}

impl SystemThemeDemoApp {
    fn new(cc: &CreationContext<'_>) -> Self {
        // Force GTK for testing purposes
        // std::env::remove_var("XDG_CURRENT_DESKTOP");
        // std::env::remove_var("DESKTOP_SESSION");

        // Here i'm unwrapping system_theme() for testing purposes,
        // but you should probably print out or handle the error gracefully in your app.
        cc.egui_ctx.set_style(egui_system_theme::system_theme().unwrap());

        Self::default()
    }
}

impl App for SystemThemeDemoApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        egui_system_theme::titlebar_extension(ctx, "menu_bar_real", true, |ui| {
            ui.menu_button("File", |ui| {
                if ui.button("New").clicked()
                    || ui.button("Open").clicked()
                    || ui.button(":)").clicked()
                {
                    ui.close_menu();
                }
            });
        });

        self.demo_windows.ui(ctx);

        CentralPanel::default().show(ctx, |ui| {
            ui.heading("^^ The titlebar (above the egui_demo_lib one) should blend in with the operating system's titlebar");
        });
        // Window::new("Widget gallery").show(ctx, |ui| {

        // });
    }
}
