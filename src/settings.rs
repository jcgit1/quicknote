use eframe::egui::{self, Color32, Ui};

use crate::state::AppState;

const PRESETS: [([u8; 3], [u8; 3], &str); 6] = [
    ([40, 44, 52], [224, 228, 234], "Charcoal"),
    ([64, 48, 40], [240, 224, 205], "Warm"),
    ([244, 238, 220], [60, 55, 45], "Parchment"),
    ([30, 52, 48], [205, 240, 230], "Mint"),
    ([52, 40, 56], [235, 215, 240], "Plum"),
    ([20, 22, 28], [200, 208, 220], "Midnight"),
];

/// Draw the appearance settings window.
pub fn ui(ui: &mut Ui, state: &mut AppState, _bg: Color32, _text: Color32) {
    egui::Window::new("Quicknote — Appearance")
        .resizable(true)
        .default_size([300.0, 240.0])
        .collapsible(false)
        .show(ui, |ui| {
            ui.label("Window background:");
            if ui.color_edit_button_srgb(&mut state.bg_color).changed() {
                // state already mutated in place
            }

            ui.add_space(8.0);
            ui.label("Text color:");
            if ui.color_edit_button_srgb(&mut state.text_color).changed() {
                // state already mutated in place
            }

            ui.add_space(12.0);
            ui.separator();
            ui.label("Presets:");

            ui.horizontal_wrapped(|ui| {
                ui.spacing_mut().item_spacing.x = 4.0;
                for (bg_c, text_c, name) in PRESETS {
                    let swatch = Color32::from_rgb(bg_c[0], bg_c[1], bg_c[2]);
                    let inner = egui::Frame::default()
                        .fill(swatch)
                        .corner_radius(6.0)
                        .stroke((1.0, shade(swatch, 0.5)))
                        .inner_margin(2.0)
                        .show(ui, |_ui| {});
                    let resp = inner.response.on_hover_text(name);
                    if resp.clicked() {
                        state.bg_color = bg_c;
                        state.text_color = text_c;
                    }
                    ui.add_space(6.0);
                }
            });
        });
}

fn shade(c: Color32, factor: f32) -> Color32 {
    Color32::from_rgb(
        (c.r() as f32 * factor).clamp(0.0, 255.0) as u8,
        (c.g() as f32 * factor).clamp(0.0, 255.0) as u8,
        (c.b() as f32 * factor).clamp(0.0, 255.0) as u8,
    )
}
