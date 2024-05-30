#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::*;

fn main() {
    let native_options = NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1280.0, 1024.0])
            .with_drag_and_drop(true),
        ..Default::default()
    };
    run_native(
        "System Theme Demo App",
        native_options,
        Box::new(|cc| {
            cc.egui_ctx.style_mut(|style| {
                *style = unsafe {
                    // Done because egui_demo_app isn't in crates.io, so i have to get it from github, so these two styles are technically different
                    // You shouldn't have to transmute in your app
                    // Also you probably shouldn't unwrap here, an unwrap_or_default or printing an error message would be better, this is just for testing
                    std::mem::transmute(egui_system_theme::system_theme().unwrap())
                }
            });
            Box::new(egui_demo_app::WrapApp::new(cc))
        }),
    )
    .unwrap();
}
