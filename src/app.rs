use crate::config::HudConfig;
use crate::draw::{draw_key, draw_mouse};
use crate::position::save_window_position;
use crate::state::HudState;
use eframe::egui;
use std::process::Command;
use std::time::{Duration, Instant};

#[derive(Clone, Copy)]
struct KeySpec {
    id: &'static str,
    label: &'static str,
    col: f32,
    row: f32,
    width: KeyWidth,
    extra_gap_units: GapUnits,
}

#[derive(Clone, Copy)]
enum KeyWidth {
    Unit,
    Tab,
    Ctrl,
    Shift,
    Space,
}

#[derive(Clone, Copy)]
enum GapUnits {
    Fixed(f32),
    SpaceExtra,
}

impl KeyWidth {
    fn resolve(self, cfg: &HudConfig) -> f32 {
        match self {
            Self::Unit => 1.0,
            Self::Tab => cfg.tab_w_mul,
            Self::Ctrl => cfg.tab_w_mul,
            Self::Shift => cfg.shift_w_mul,
            Self::Space => cfg.space_w_mul,
        }
    }
}

impl GapUnits {
    fn resolve(self, cfg: &HudConfig) -> f32 {
        match self {
            Self::Fixed(value) => value,
            Self::SpaceExtra => cfg.space_extra_gaps,
        }
    }
}

const KEY_SPECS: [KeySpec; 23] = [
    KeySpec {
        id: "BACKQUOTE",
        label: "`",
        col: 0.0,
        row: 0.0,
        width: KeyWidth::Unit,
        extra_gap_units: GapUnits::Fixed(0.0),
    },
    KeySpec {
        id: "1",
        label: "1",
        col: 1.4,
        row: 0.0,
        width: KeyWidth::Unit,
        extra_gap_units: GapUnits::Fixed(0.0),
    },
    KeySpec {
        id: "2",
        label: "2",
        col: 2.4,
        row: 0.0,
        width: KeyWidth::Unit,
        extra_gap_units: GapUnits::Fixed(0.0),
    },
    KeySpec {
        id: "3",
        label: "3",
        col: 3.4,
        row: 0.0,
        width: KeyWidth::Unit,
        extra_gap_units: GapUnits::Fixed(0.0),
    },
    KeySpec {
        id: "4",
        label: "4",
        col: 4.4,
        row: 0.0,
        width: KeyWidth::Unit,
        extra_gap_units: GapUnits::Fixed(0.0),
    },
    KeySpec {
        id: "TAB",
        label: "Tab",
        col: 0.0,
        row: 1.0,
        width: KeyWidth::Tab,
        extra_gap_units: GapUnits::Fixed(0.0),
    },
    KeySpec {
        id: "Q",
        label: "Q",
        col: 1.4,
        row: 1.0,
        width: KeyWidth::Unit,
        extra_gap_units: GapUnits::Fixed(0.0),
    },
    KeySpec {
        id: "W",
        label: "W",
        col: 2.4,
        row: 1.0,
        width: KeyWidth::Unit,
        extra_gap_units: GapUnits::Fixed(0.0),
    },
    KeySpec {
        id: "E",
        label: "E",
        col: 3.4,
        row: 1.0,
        width: KeyWidth::Unit,
        extra_gap_units: GapUnits::Fixed(0.0),
    },
    KeySpec {
        id: "R",
        label: "R",
        col: 4.4,
        row: 1.0,
        width: KeyWidth::Unit,
        extra_gap_units: GapUnits::Fixed(0.0),
    },
    KeySpec {
        id: "CAPSLOCK",
        label: "Caps",
        col: 0.0,
        row: 2.0,
        width: KeyWidth::Tab,
        extra_gap_units: GapUnits::Fixed(0.0),
    },
    KeySpec {
        id: "A",
        label: "A",
        col: 1.4,
        row: 2.0,
        width: KeyWidth::Unit,
        extra_gap_units: GapUnits::Fixed(0.0),
    },
    KeySpec {
        id: "S",
        label: "S",
        col: 2.4,
        row: 2.0,
        width: KeyWidth::Unit,
        extra_gap_units: GapUnits::Fixed(0.0),
    },
    KeySpec {
        id: "D",
        label: "D",
        col: 3.4,
        row: 2.0,
        width: KeyWidth::Unit,
        extra_gap_units: GapUnits::Fixed(0.0),
    },
    KeySpec {
        id: "F",
        label: "F",
        col: 4.4,
        row: 2.0,
        width: KeyWidth::Unit,
        extra_gap_units: GapUnits::Fixed(0.0),
    },
    KeySpec {
        id: "LSHIFT",
        label: "Shift",
        col: 0.0,
        row: 3.0,
        width: KeyWidth::Shift,
        extra_gap_units: GapUnits::Fixed(0.0),
    },
    KeySpec {
        id: "Z",
        label: "Z",
        col: 1.4,
        row: 3.0,
        width: KeyWidth::Unit,
        extra_gap_units: GapUnits::Fixed(0.0),
    },
    KeySpec {
        id: "X",
        label: "X",
        col: 2.4,
        row: 3.0,
        width: KeyWidth::Unit,
        extra_gap_units: GapUnits::Fixed(0.0),
    },
    KeySpec {
        id: "C",
        label: "C",
        col: 3.4,
        row: 3.0,
        width: KeyWidth::Unit,
        extra_gap_units: GapUnits::Fixed(0.0),
    },
    KeySpec {
        id: "V",
        label: "V",
        col: 4.4,
        row: 3.0,
        width: KeyWidth::Unit,
        extra_gap_units: GapUnits::Fixed(0.0),
    },
    KeySpec {
        id: "LCTRL",
        label: "Ctrl",
        col: 0.0,
        row: 4.0,
        width: KeyWidth::Ctrl,
        extra_gap_units: GapUnits::Fixed(0.0),
    },
    KeySpec {
        id: "LALT",
        label: "Alt",
        col: 1.6,
        row: 4.0,
        width: KeyWidth::Unit,
        extra_gap_units: GapUnits::Fixed(0.0),
    },
    KeySpec {
        id: "SPACE",
        label: "Space",
        col: 2.6,
        row: 4.0,
        width: KeyWidth::Space,
        extra_gap_units: GapUnits::SpaceExtra,
    },
];

pub struct HudApp {
    state: HudState,
    cfg: HudConfig,
    last_revision: u64,
    last_saved_pos: Option<egui::Pos2>,
    last_save_at: Instant,
    startup_guard_done: bool,
    system_info_line1: String,
    rust_info_line: String,
}

impl HudApp {
    pub fn new(state: HudState, cfg: HudConfig) -> Self {
        Self {
            state,
            cfg,
            last_revision: 0,
            last_saved_pos: None,
            last_save_at: Instant::now(),
            startup_guard_done: false,
            system_info_line1: build_system_info_line1(),
            rust_info_line: build_rust_info_line(),
        }
    }
}

fn command_stdout(cmd: &str, args: &[&str]) -> Option<String> {
    let out = Command::new(cmd).args(args).output().ok()?;
    if !out.status.success() {
        return None;
    }
    let text = String::from_utf8(out.stdout).ok()?;
    let trimmed = text.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_owned())
    }
}

fn build_system_info_line1() -> String {
    let app_ver = env!("CARGO_PKG_VERSION");
    let kernel = command_stdout("uname", &["-r"]).unwrap_or_else(|| "unknown-kernel".to_owned());
    format!("v{app_ver} | {kernel}")
}

fn build_rust_info_line() -> String {
    command_stdout("rustc", &["--version"]).unwrap_or_else(|| "rustc unknown".to_owned())
}

impl eframe::App for HudApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let cfg = self.cfg;

        egui::Area::new(egui::Id::new("hud"))
            .fixed_pos(egui::pos2(0.0, 0.0))
            .movable(false)
            .show(ctx, |ui| {
                let close_size = egui::vec2(cfg.grip_h, cfg.grip_h);
                let close_pos = egui::pos2(cfg.win_w - close_size.x - 8.0, 4.0);
                let close_rect = egui::Rect::from_min_size(close_pos, close_size);
                let close_resp = ui.interact(
                    close_rect,
                    egui::Id::new("close_button"),
                    egui::Sense::click(),
                );
                if close_resp.hovered() {
                    ctx.set_cursor_icon(egui::CursorIcon::PointingHand);
                }
                if close_resp.clicked() {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }

                let close_bg = if close_resp.hovered() {
                    egui::Color32::from_rgba_unmultiplied(255, 80, 80, 90)
                } else {
                    egui::Color32::from_rgba_unmultiplied(255, 255, 255, 12)
                };
                ui.painter().rect_filled(close_rect, cfg.grip_round, close_bg);
                ui.painter().text(
                    close_rect.center(),
                    egui::Align2::CENTER_CENTER,
                    "x",
                    egui::FontId::proportional(cfg.grip_font),
                    egui::Color32::from_rgba_unmultiplied(255, 255, 255, 220),
                );

                let grip_size = egui::vec2(cfg.grip_w, cfg.grip_h);
                let (grip_rect, grip_resp) =
                    ui.allocate_exact_size(grip_size, egui::Sense::click_and_drag());

                if grip_resp.drag_started() {
                    ctx.send_viewport_cmd(egui::ViewportCommand::StartDrag);
                }
                if grip_resp.hovered() {
                    ctx.set_cursor_icon(egui::CursorIcon::Grab);
                }

                let grip_bg = if grip_resp.hovered() {
                    egui::Color32::from_rgba_unmultiplied(255, 255, 255, 18)
                } else {
                    egui::Color32::from_rgba_unmultiplied(255, 255, 255, 8)
                };
                ui.painter().rect_filled(grip_rect, cfg.grip_round, grip_bg);
                ui.painter().text(
                    grip_rect.center(),
                    egui::Align2::CENTER_CENTER,
                    "drag",
                    egui::FontId::proportional(cfg.grip_font),
                    egui::Color32::from_rgba_unmultiplied(255, 255, 255, 120),
                );

                let (pressed, wheel_up, wheel_down, event_count, listener_error, revision) = {
                    let mut state = self.state.inner.lock().unwrap();
                    let pressed = state.pressed.clone();
                    let wheel_up = state.wheel_up_ticks;
                    let wheel_down = state.wheel_down_ticks;
                    let event_count = state.event_count;
                    let listener_error = state.listener_error.clone();
                    let revision = state.revision;

                    state.wheel_up_ticks = state.wheel_up_ticks.saturating_sub(1);
                    state.wheel_down_ticks = state.wheel_down_ticks.saturating_sub(1);

                    (pressed, wheel_up, wheel_down, event_count, listener_error, revision)
                };

                let origin = ui.min_rect().min;
                let key_unit = cfg.key_u;
                let key_gap = cfg.key_gap;

                for key in KEY_SPECS {
                    let x = (key_unit + key_gap) * key.col + cfg.left_pad;
                    let y = (key_unit + key_gap) * key.row + cfg.top_pad;
                    let width = key_unit * key.width.resolve(&cfg)
                        + key_gap * key.extra_gap_units.resolve(&cfg);
                    draw_key(
                        &cfg, ui, origin, &pressed, key.id, key.label, x, y, width, key_unit,
                    );
                }

                let row2_y = key_unit + key_gap + cfg.top_pad;
                let row5_y = (key_unit + key_gap) * 4.0 + cfg.top_pad;
                let keyboard_right = (key_unit + key_gap) * 4.4 + cfg.left_pad + key_unit;

                let mouse_x = keyboard_right + cfg.mouse_gap;
                let mouse_top = row2_y;
                let mouse_h = (row5_y + key_unit) - row2_y - key_unit * cfg.mouse_height_trim_mul;

                draw_mouse(
                    &cfg,
                    ui,
                    origin,
                    &pressed,
                    wheel_up,
                    wheel_down,
                    mouse_x,
                    mouse_top,
                    cfg.mouse_w,
                    mouse_h,
                );

                // 繪製狀態文字：events 用紅色，其餘版本資訊用淺灰色。
                let (events_text, status_line1) = if let Some(ref err) = listener_error {
                    (format!("listener error: {err}"), String::new())
                } else {
                    (
                        format!("events: {event_count}"),
                        self.rust_info_line.clone(),
                    )
                };
                let debug_right_margin = cfg.grip_h + 18.0;
                let top_right = origin + egui::vec2(cfg.win_w - debug_right_margin, 0.0);
                let main_font = egui::FontId::proportional(cfg.grip_font);
                let main_gray = egui::Color32::from_rgba_unmultiplied(220, 220, 220, 210);
                let events_red = egui::Color32::from_rgba_unmultiplied(255, 180, 180, 220);

                if listener_error.is_some() {
                    ui.painter().text(
                        top_right,
                        egui::Align2::RIGHT_TOP,
                        events_text,
                        main_font.clone(),
                        events_red,
                    );
                } else {
                    let suffix = format!(" | {}", self.system_info_line1);
                    let suffix_galley = ui
                        .painter()
                        .layout_no_wrap(suffix.clone(), main_font.clone(), main_gray);
                    let suffix_pos =
                        egui::pos2(top_right.x - suffix_galley.size().x, top_right.y);
                    ui.painter().galley(suffix_pos, suffix_galley, main_gray);

                    let events_galley = ui
                        .painter()
                        .layout_no_wrap(events_text.clone(), main_font.clone(), events_red);
                    let events_pos = egui::pos2(
                        suffix_pos.x - 6.0 - events_galley.size().x,
                        top_right.y,
                    );
                    ui.painter().galley(events_pos, events_galley, events_red);
                }

                if !status_line1.is_empty() {
                    ui.painter().text(
                        egui::pos2(top_right.x, origin.y + 12.0),
                        egui::Align2::RIGHT_TOP,
                        status_line1,
                        egui::FontId::proportional(cfg.grip_font),
                        main_gray,
                    );
                }
                if revision != self.last_revision {
                    self.last_revision = revision;
                    ctx.request_repaint();
                }

                if wheel_up > 0 || wheel_down > 0 {
                    ctx.request_repaint_after(Duration::from_millis(16));
                }
            });

        // 維持低頻重繪，避免 HUD 失焦或被視窗管理器降頻時看不到全域輸入更新。
        ctx.request_repaint_after(Duration::from_millis(16));

        if let Some(pos) = ctx.input(|i| i.viewport().outer_rect.map(|r| r.min)) {
            let moved = self
                .last_saved_pos
                .map(|last| (last.x - pos.x).abs() > 0.5 || (last.y - pos.y).abs() > 0.5)
                .unwrap_or(true);
            let due = self.last_save_at.elapsed() >= Duration::from_millis(500);
            if moved && due {
                save_window_position(pos);
                self.last_saved_pos = Some(pos);
                self.last_save_at = Instant::now();
            }
        }

        if !self.startup_guard_done {
            let viewport = ctx.input(|i| i.viewport().clone());
            if let (Some(outer), Some(monitor_size)) = (viewport.outer_rect, viewport.monitor_size) {
                let window_size = outer.size();
                let max_x = (monitor_size.x - window_size.x).max(0.0);
                let max_y = (monitor_size.y - window_size.y).max(0.0);
                let clamped = egui::pos2(outer.min.x.clamp(0.0, max_x), outer.min.y.clamp(0.0, max_y));
                if (clamped.x - outer.min.x).abs() > 0.5 || (clamped.y - outer.min.y).abs() > 0.5 {
                    ctx.send_viewport_cmd(egui::ViewportCommand::OuterPosition(clamped));
                    save_window_position(clamped);
                    self.last_saved_pos = Some(clamped);
                    self.last_save_at = Instant::now();
                }
                self.startup_guard_done = true;
            }
        }
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        if let Some(pos) = self.last_saved_pos {
            save_window_position(pos);
        }
    }
}
