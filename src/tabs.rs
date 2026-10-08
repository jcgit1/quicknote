use eframe::egui::{self, Color32, RichText, Ui, Vec2};

use crate::state::AppState;

#[derive(Clone, Copy, PartialEq)]
pub enum TabAction {
    Add,
    Close(u64),
    Activate(usize),
}

pub struct RenameState {
    pub tab_id: u64,
    pub buffer: String,
}

/// Draw the tab strip. `renaming` is `Some` while a tab name is being edited.
/// Returns an action if the user clicked something, and updates `renaming`.
pub fn ui(
    ui: &mut Ui,
    state: &mut AppState,
    bg: Color32,
    text: Color32,
    renaming: &mut Option<RenameState>,
) -> Option<TabAction> {
    egui::Panel::top("quicknote_tabstrip")
        .exact_size(30.0)
        .show(ui, |ui| {
            ui.painter()
                .rect_filled(ui.max_rect(), 0.0, shade(bg, 0.9));

            let mut action = None;
            let count = state.tabs.len();
            let names: Vec<String> = state.tabs.iter().map(|t| t.name.clone()).collect();
            let ids: Vec<u64> = state.tabs.iter().map(|t| t.id).collect();
            let active = state.active;

            ui.horizontal(|ui| {
                ui.add_space(4.0);

                for (i, (tab_id, name)) in ids.iter().zip(names.iter()).enumerate() {
                    let tab_id = *tab_id;
                    let name = name.clone();
                    let is_active = i == active;
                    let name_bg = if is_active { bg } else { shade(bg, 0.8) };
                    let name_fg = if is_active { text } else { shade(text, 0.65) };

                    let editing = renaming
                        .as_ref()
                        .map(|r| r.tab_id == tab_id)
                        .unwrap_or(false);

                    if editing {
                        if let Some(r) = renaming.as_mut() {
                            let resp = ui.add_sized(
                                Vec2::new(130.0, 20.0),
                                egui::TextEdit::singleline(&mut r.buffer)
                                    .font(egui::TextStyle::Body)
                                    .frame(egui::Frame::NONE)
                                    .desired_width(130.0)
                                    .cursor_at_end(true),
                            );
                            let enter =
                                ui.ctx().input(|in_| in_.key_pressed(egui::Key::Enter));
                            if resp.lost_focus() || enter {
                                let id = r.tab_id;
                                let new_name = r.buffer.trim().to_string();
                                apply_rename(state, id, &new_name);
                                *renaming = None;
                            } else {
                                resp.request_focus();
                            }
                        }
                        continue;
                    }

                    let name_btn = ui.add(
                        egui::Button::new(RichText::new(&name).color(name_fg).small())
                            .fill(name_bg)
                            .min_size(Vec2::new(74.0, 22.0))
                            .corner_radius(6.0),
                    );
                    if name_btn.double_clicked() {
                        *renaming = Some(RenameState {
                            tab_id,
                            buffer: name.clone(),
                        });
                    } else if name_btn.clicked() {
                        action = Some(TabAction::Activate(i));
                    }

                    if count > 1 {
                        let close = ui
                            .add(
                                egui::Button::new(RichText::new("×").color(shade(name_fg, 0.8)))
                                    .frame(false)
                                    .min_size(Vec2::new(15.0, 15.0)),
                            )
                            .on_hover_text("Close tab");
                        if close.clicked() {
                            action = Some(TabAction::Close(tab_id));
                        }
                    }
                    ui.add_space(2.0);
                }

                let add = ui
                    .add(
                        egui::Button::new(RichText::new("＋").color(text))
                            .frame(false)
                            .min_size(Vec2::new(24.0, 22.0)),
                    )
                    .on_hover_text("New note");
                if add.clicked() {
                    action = Some(TabAction::Add);
                }
            });

            action
        })
        .inner
}

fn apply_rename(state: &mut AppState, id: u64, name: &str) {
    if let Some(tab) = state.tabs.iter_mut().find(|t| t.id == id) {
        if !name.is_empty() {
            tab.name = name.to_string();
        }
    }
}

pub fn next_name(state: &AppState) -> String {
    let mut n = state.tabs.len() + 1;
    loop {
        let candidate = format!("Note {n}");
        if !state.tabs.iter().any(|t| t.name == candidate) {
            return candidate;
        }
        n += 1;
    }
}

pub fn next_id(state: &AppState) -> u64 {
    state.tabs.iter().map(|t| t.id).max().unwrap_or(0) + 1
}

fn shade(c: Color32, factor: f32) -> Color32 {
    Color32::from_rgb(
        (c.r() as f32 * factor).clamp(0.0, 255.0) as u8,
        (c.g() as f32 * factor).clamp(0.0, 255.0) as u8,
        (c.b() as f32 * factor).clamp(0.0, 255.0) as u8,
    )
}
