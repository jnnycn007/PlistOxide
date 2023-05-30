#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![deny(
    warnings,
    clippy::cargo,
    clippy::nursery,
    unused_extern_crates,
    rust_2021_compatibility
)]

use std::path::PathBuf;

use app::PlistOxideApp;
use eframe::{IconData, NativeOptions};

mod app;
mod style;
mod widgets;

static ICON: &[u8; 95126] = include_bytes!("app_icon/icon512x512@2x.png");

fn main() {
    let path = std::env::args().nth(1).map(PathBuf::from);

    eframe::run_native(
        "com.ChefKissInc.PlistOxide",
        NativeOptions {
            #[cfg(target_os = "macos")]
            fullsize_content: true,
            icon_data: Some(IconData {
                rgba: ICON.to_vec(),
                width: 1024,
                height: 1024,
            }),
            drag_and_drop_support: true,
            ..Default::default()
        },
        Box::new(|cc| Box::new(PlistOxideApp::new(cc, path))),
    )
    .unwrap();
}
