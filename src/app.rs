use crate::config::HudConfig;
use crate::draw::{draw_key, draw_mouse};
use crate::state::HudState;
use eframe::egui;

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

const KEY_SPECS: [KeySpec; 17] = [
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
        id: "X",
        label: "X",
        col: 2.4,
        row: 3.0,
        width: KeyWidth::Unit,
        extra_gap_units: GapUnits::Fixed(0.0),
    },
    KeySpec {
        id: "LALT",
        label: "Alt",
        col: 1.4,
        row: 4.0,
        width: KeyWidth::Unit,
        extra_gap_units: GapUnits::Fixed(0.0),
    },
    KeySpec {
        id: "SPACE",
        label: "Space",
        col: 2.4,
        row: 4.0,
        width: KeyWidth::Space,
        extra_gap_units: GapUnits::SpaceExtra,
    },
];

pub struct HudApp {
    state: HudState,
    cfg: HudConfig,
}

impl HudApp {
    pub fn new(state: HudState, cfg: HudConfig) -> Self {
        Self { state, cfg }
    }
}

impl eframe::App for HudApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let cfg = self.cfg;

        egui::Area::new(egui::Id::new("hud"))
            .fixed_pos(egui::pos2(0.0, 0.0))
            .movable(false)
            .show(ctx, |ui| {
                let grip_size = egui::vec2(cfg.grip_w, cfg.grip_h);
                let (grip_rect, grip_resp) =
                    ui.allocate_exact_size(grip_size, egui::Sense::click_and_drag());

                if grip_resp.drag_started() {
                    ctx.send_viewport_cmd(egui::ViewportCommand::StartDrag);
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

                let (pressed, wheel_up, wheel_down, event_count, listener_error) = {
                    let mut state = self.state.inner.lock().unwrap();
                    let pressed = state.pressed.clone();
                    let wheel_up = state.wheel_up_ticks;
                    let wheel_down = state.wheel_down_ticks;
                    let event_count = state.event_count;
                    let listener_error = state.listener_error.clone();

                    state.wheel_up_ticks = state.wheel_up_ticks.saturating_sub(1);
                    state.wheel_down_ticks = state.wheel_down_ticks.saturating_sub(1);

                    (pressed, wheel_up, wheel_down, event_count, listener_error)
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

                let status = if let Some(err) = listener_error {
                    format!("listener error: {err}")
                } else {
                    format!("events: {event_count}")
                };
                ui.painter().text(
                    origin + egui::vec2(8.0, 8.0),
                    egui::Align2::LEFT_TOP,
                    status,
                    egui::FontId::proportional(12.0),
                    egui::Color32::from_rgba_unmultiplied(255, 180, 180, 220),
                );
            });

        ctx.request_repaint();
    }
}
