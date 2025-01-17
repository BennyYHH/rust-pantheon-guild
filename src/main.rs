#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod app;
mod scenes;
mod engine;

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    use log::info;

    std::env::set_var("RUST_LOG", "info");

    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`)
    info!("Initializing app...");
    let native_options = eframe::NativeOptions {
        min_window_size: Some(egui::Vec2 { x: 1024.0, y: 768.0 }),
        initial_window_size: Some(egui::Vec2 { x: 1024.0, y: 768.0 }),
        ..Default::default()
    };
    // native_options. = Some(egui::Vec2::new(1024.0, 768.0));
    eframe::run_native(
        "Rust Pillar Guild (RPG)",
        native_options,
        Box::new(|cc| Box::new(app::App::new(cc))),
    )
}

// When compiling to web using trunk:
#[cfg(target_arch = "wasm32")]
fn main() {
    // Redirect `log` message to `console.log` and friends:
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new()
            .start(
                "the_canvas_id", // hardcode it
                web_options,
                Box::new(|cc| Box::new(ui::TemplateApp::new(cc))),
            )
            .await
            .expect("failed to start eframe");
    });
}
