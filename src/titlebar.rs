use eframe::egui::{self, Color32, Rect, Sense, Ui, Vec2};

use crate::state::AppState;

pub const TITLEBAR_H: f32 = 38.0;
const BTN: f32 = 28.0;

#[derive(Clone, Copy, PartialEq)]
pub enum WinAction {
    ToggleShrink,
    Minimize,
    Maximize,
    Close,
    ToggleSettings,
}

fn shade(c: Color32, factor: f32) -> Color32 {
    Color32::from_rgb(
        (c.r() as f32 * factor).clamp(0.0, 255.0) as u8,
        (c.g() as f32 * factor).clamp(0.0, 255.0) as u8,
        (c.b() as f32 * factor).clamp(0.0, 255.0) as u8,
    )
}

/// Draw the custom titlebar. Returns the action the user clicked, if any.
/// Dragging the left title area moves the window.
pub fn ui(ui: &mut Ui, state: &mut AppState, bg: Color32, text: Color32) -> Option<WinAction> {
    egui::Panel::top("quicknote_titlebar")
        .exact_size(TITLEBAR_H)
        .show(ui, |ui| {
            let rect = ui.max_rect();
            ui.painter()
                .rect_filled(rect, 0.0, shade(bg, 0.78));

            let active_name = state
                .tabs
                .get(state.active)
                .map(|t| t.name.as_str())
                .unwrap_or("");
            let title = format!("Quicknote — {active_name}");
            let title_max_x = (rect.max.x - (BTN * 5.0 + 14.0)).max(rect.min.x + 40.0);
            let title_rect = Rect::from_min_max(
                egui::pos2(rect.min.x + 10.0, rect.min.y),
                egui::pos2(title_max_x, rect.max.y),
            );
            let resp = ui
                .put(
                    title_rect,
                    egui::Label::new(egui::RichText::new(title).color(text).strong()),
                )
                .interact(Sense::drag());
            if resp.dragged() {
                ui.ctx()
                    .send_viewport_cmd(egui::ViewportCommand::StartDrag);
            }

            right_buttons(ui, state, bg, text)
        })
        .inner
}

fn right_buttons(
    ui: &mut Ui,
    state: &AppState,
    bg: Color32,
    text: Color32,
) -> Option<WinAction> {
    let hover = shade(bg, 0.6);
    let maximized = ui
        .ctx()
        .input(|i| i.viewport().maximized.unwrap_or(false));

    let mut action = None;

    if button(ui, "⚙", text, hover, "Appearance").clicked() {
        action = Some(WinAction::ToggleSettings);
    }

    let (shrink_label, shrink_tip) = if state.shrunken {
        ("▢", "Restore window")
    } else {
        ("❐", "Collapse to titlebar")
    };
    if button(ui, shrink_label, text, hover, shrink_tip).clicked() {
        action = Some(WinAction::ToggleShrink);
    }

    if button(ui, "—", text, hover, "Minimize").clicked() {
        action = Some(WinAction::Minimize);
    }

    let max_label = if maximized { "🗗" } else { "□" };
    if button(ui, max_label, text, hover, "Maximize").clicked() {
        action = Some(WinAction::Maximize);
    }

    if button(ui, "✕", text, hover, "Close").clicked() {
        action = Some(WinAction::Close);
    }

    action
}

fn button(
    ui: &mut Ui,
    label: &str,
    text: Color32,
    hover: Color32,
    tip: &str,
) -> egui::Response {
    let resp = ui
        .add(
            egui::Button::new(egui::RichText::new(label).color(text))
                .min_size(Vec2::new(BTN, BTN))
                .frame(false)
                .fill(Color32::TRANSPARENT),
        )
        .on_hover_text(tip);
    if resp.hovered() {
        ui.painter()
            .rect_filled(resp.rect, 0.0, hover);
    }
    resp
}
