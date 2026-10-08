mod app;
mod settings;
mod state;
mod tabs;
mod titlebar;

use eframe::egui::{self, Color32};

use crate::app::QuickNoteApp;
use crate::state::AppState;

const APP_TITLE: &str = "Quicknote";
const APP_ID: &str = "com.jcgit1.quicknote";

fn main() -> eframe::Result<()> {
    let state = AppState::load();

    let mut builder = egui::ViewportBuilder::default()
        .with_title(APP_TITLE)
        .with_app_id(APP_ID)
        .with_decorations(false)
        .with_inner_size([480.0, 640.0])
        .with_min_inner_size([320.0, 44.0]);

    if state.shrunken {
        if let Some([w, _h]) = state.last_inner_size {
            builder = builder.with_inner_size([w, 44.0]);
        }
    }

    let options = eframe::NativeOptions {
        viewport: builder,
        ..Default::default()
    };

    eframe::run_native(
        APP_TITLE,
        options,
        Box::new(move |cc| Ok(Box::new(QuickNoteApp::new(cc, state)))),
    )
}

pub fn color32(c: [u8; 3]) -> Color32 {
    Color32::from_rgb(c[0], c[1], c[2])
}
