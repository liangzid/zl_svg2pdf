#![forbid(unsafe_code)]
#![cfg_attr(not(debug_assertions), deny(warnings))] // Forbid warnings in release builds
#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] //Hide console window in release builds on Windows, this blocks stdout.

// use nalgebra::Vec2;
use eframe::egui::Vec2;

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let app = zl_svg2pdf::TemplateApp::default();
    let mut native_options = eframe::NativeOptions::default();
    native_options.always_on_top = true;
    native_options.decorated = true;
    native_options.drag_and_drop_support = true;
    native_options.initial_window_size = Some(Vec2::new(250.0, 200.0));
    native_options.transparent = true;
    eframe::run_native(Box::new(app), native_options);
}
