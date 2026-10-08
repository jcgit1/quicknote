use std::time::{Duration, Instant};

use eframe::egui::{self, Color32};

use crate::settings;
use crate::state::{AppState, Tab};
use crate::tabs::{self, RenameState};
use crate::titlebar::{self, WinAction, TITLEBAR_H};

const SAVE_DEBOUNCE: Duration = Duration::from_millis(500);
const MIN_W: f32 = 320.0;
const MIN_BODY_H: f32 = 60.0;

pub struct QuickNoteApp {
    ctx: egui::Context,
    state: AppState,
    renaming: Option<RenameState>,
    dirty: bool,
    last_save: Instant,
    last_size: Option<[f32; 2]>,
}

impl QuickNoteApp {
    pub fn new(cc: &eframe::CreationContext<'_>, state: AppState) -> Self {
        let last_size = state.last_inner_size;
        Self {
            ctx: cc.egui_ctx.clone(),
            state,
            renaming: None,
            dirty: false,
            last_save: Instant::now(),
            last_size,
        }
    }

    fn bg(&self) -> Color32 {
        Color32::from_rgb(self.state.bg_color[0], self.state.bg_color[1], self.state.bg_color[2])
    }

    fn text(&self) -> Color32 {
        Color32::from_rgb(
            self.state.text_color[0],
            self.state.text_color[1],
            self.state.text_color[2],
        )
    }

    fn apply_theme(&self) {
        let bg = self.bg();
        let text = self.text();
        let dark = shade(bg, 0.8);
        let lighter = shade(bg, 1.3);

        let mut visuals = egui::Visuals::dark();
        visuals.window_fill = bg;
        visuals.panel_fill = bg;
        visuals.extreme_bg_color = dark;
        visuals.override_text_color = Some(text);
        visuals.widgets.noninteractive.bg_fill = bg;
        visuals.widgets.noninteractive.fg_stroke.color = text;
        visuals.widgets.inactive.weak_bg_fill = dark;
        visuals.widgets.inactive.bg_fill = dark;
        visuals.widgets.inactive.fg_stroke.color = text;
        visuals.widgets.hovered.bg_fill = lighter;
        visuals.widgets.hovered.fg_stroke.color = text;
        visuals.widgets.hovered.weak_bg_fill = dark;
        visuals.widgets.active.bg_fill = dark;
        visuals.widgets.active.fg_stroke.color = text;
        visuals.selection.bg_fill = lighter;
        visuals.selection.stroke.color = text;
        self.ctx.set_visuals(visuals);
    }

    fn viewport(&self) -> (Option<[f32; 2]>, Option<[f32; 2]>, bool) {
        let (size, pos, maxed) = self.ctx.input(|i| {
            let v = i.viewport();
            let size = v.inner_rect.map(|r| [r.size().x, r.size().y]);
            let pos = v.outer_rect.map(|r| [r.min.x, r.min.y]);
            (size, pos, v.maximized.unwrap_or(false))
        });
        (size, pos, maxed)
    }

    fn handle_win_action(&mut self, action: WinAction) {
        let (size, pos, maxed) = self.viewport();
        let [w, h] = size.unwrap_or([480.0, 640.0]);
        let [px, py] = pos.unwrap_or([0.0, 0.0]);

        match action {
            WinAction::ToggleShrink => {
                if self.state.shrunken {
                    let [rw, rh] = self.last_size.unwrap_or([480.0, 640.0]);
                    self.state.shrunken = false;
                    self.ctx
                        .send_viewport_cmd(egui::ViewportCommand::InnerSize(
                            egui::vec2(rw.max(MIN_W), rh.max(TITLEBAR_H + MIN_BODY_H)),
                        ));
                    if let Some([ox, oy]) = self.state.last_pos {
                        self.ctx.send_viewport_cmd(egui::ViewportCommand::OuterPosition(
                            egui::pos2(ox, oy),
                        ));
                    }
                } else {
                    self.last_size = Some([w, h]);
                    self.state.last_inner_size = Some([w, h]);
                    self.state.last_pos = Some([px, py]);
                    self.state.shrunken = true;
                    self.ctx
                        .send_viewport_cmd(egui::ViewportCommand::InnerSize(
                            egui::vec2(w.max(MIN_W), TITLEBAR_H),
                        ));
                }
                self.mark_dirty();
            }
            WinAction::Minimize => {
                self.ctx
                    .send_viewport_cmd(egui::ViewportCommand::Minimized(true));
            }
            WinAction::Maximize => {
                self.ctx.send_viewport_cmd(egui::ViewportCommand::Maximized(!maxed));
            }
            WinAction::Close => {
                self.state.save().ok();
                self.ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            }
            WinAction::ToggleSettings => {
                self.state.settings_open = !self.state.settings_open;
                self.mark_dirty();
            }
        }
    }

    fn handle_tab_action(&mut self, action: tabs::TabAction) {
        match action {
            tabs::TabAction::Add => {
                let name = tabs::next_name(&self.state);
                let id = tabs::next_id(&self.state);
                self.state.tabs.push(Tab::new(id, name));
                self.state.active = self.state.tabs.len() - 1;
                self.renaming = None;
            }
            tabs::TabAction::Activate(i) => {
                self.state.active = i.min(self.state.tabs.len().saturating_sub(1));
                self.renaming = None;
            }
            tabs::TabAction::Close(id) => {
                if self.state.tabs.len() <= 1 {
                    return;
                }
                if let Some(idx) = self.state.tabs.iter().position(|t| t.id == id) {
                    self.state.tabs.remove(idx);
                    if self.state.active >= self.state.tabs.len() {
                        self.state.active = self.state.tabs.len() - 1;
                    }
                    if self
                        .renaming
                        .as_ref()
                        .map(|r| r.tab_id == id)
                        .unwrap_or(false)
                    {
                        self.renaming = None;
                    }
                }
            }
        }
        self.mark_dirty();
    }

    fn mark_dirty(&mut self) {
        self.dirty = true;
    }

    fn maybe_save(&mut self) {
        if self.dirty && self.last_save.elapsed() >= SAVE_DEBOUNCE {
            self.last_save = Instant::now();
            if self.state.save().is_ok() {
                self.dirty = false;
            }
        }
    }
}

impl eframe::App for QuickNoteApp {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        self.bg().to_normalized_gamma_f32()
    }

    fn save(&mut self, _storage: &mut dyn eframe::Storage) {
        self.state.save().ok();
        self.dirty = false;
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.apply_theme();

        if ui.ctx().input(|i| i.viewport().close_requested()) {
            ui.ctx()
                .send_viewport_cmd(egui::ViewportCommand::CancelClose);
            self.state.save().ok();
            ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
            return;
        }

        let bg = self.bg();
        let text = self.text();

        if let Some(a) = titlebar::ui(ui, &mut self.state, bg, text) {
            self.handle_win_action(a);
        }

        if !self.state.shrunken {
            if let Some(a) =
                tabs::ui(ui, &mut self.state, bg, text, &mut self.renaming)
            {
                self.handle_tab_action(a);
            }

            if let Some(tab) = self.state.tabs.get_mut(self.state.active) {
                egui::CentralPanel::default().show(ui, |ui| {
                    egui::ScrollArea::vertical()
                        .auto_shrink([false, false])
                        .show(ui, |ui| {
                            ui.add(
                                egui::TextEdit::multiline(&mut tab.text)
                                    .font(egui::TextStyle::Monospace)
                                    .frame(egui::Frame::NONE)
                                    .desired_width(f32::INFINITY),
                            )
                            .request_focus();
                        });
                });
            }
        }

        if self.state.settings_open {
            settings::ui(ui, &mut self.state, bg, text);
        }

        self.maybe_save();
    }
}

fn shade(c: Color32, factor: f32) -> Color32 {
    Color32::from_rgb(
        (c.r() as f32 * factor).clamp(0.0, 255.0) as u8,
        (c.g() as f32 * factor).clamp(0.0, 255.0) as u8,
        (c.b() as f32 * factor).clamp(0.0, 255.0) as u8,
    )
}
