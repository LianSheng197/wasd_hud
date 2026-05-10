use crate::config::HudConfig;
use eframe::egui;
use std::collections::HashSet;

pub fn draw_key(
    cfg: &HudConfig,
    ui: &egui::Ui,
    origin: egui::Pos2,
    pressed: &HashSet<&'static str>,
    id: &'static str,
    text: &'static str,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
) {
    let is_down = pressed.contains(id);
    let rect = egui::Rect::from_min_size(origin + egui::vec2(x, y), egui::vec2(w, h));

    let active = egui::Color32::from_rgb(cfg.active_r, cfg.active_g, cfg.active_b);

    let fill_color = if is_down {
        active
    } else {
        egui::Color32::from_rgba_unmultiplied(0, 0, 0, cfg.key_inactive_fill_a)
    };

    let stroke_color = if is_down {
        active
    } else {
        egui::Color32::from_rgba_unmultiplied(255, 255, 255, cfg.key_inactive_stroke_a)
    };

    let text_color = if is_down {
        egui::Color32::BLACK
    } else {
        egui::Color32::WHITE
    };

    ui.painter().rect_filled(rect, cfg.key_round, fill_color);
    ui.painter().rect_stroke(
        rect,
        cfg.key_round,
        egui::Stroke::new(cfg.key_stroke_w, stroke_color),
    );

    ui.painter().text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        text,
        egui::FontId::proportional(cfg.key_font),
        text_color,
    );
}

pub fn draw_mouse(
    cfg: &HudConfig,
    ui: &egui::Ui,
    origin: egui::Pos2,
    pressed: &HashSet<&'static str>,
    wheel_up_ticks: u8,
    wheel_down_ticks: u8,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
) {
    let rect = egui::Rect::from_min_size(origin + egui::vec2(x, y), egui::vec2(w, h));

    let active = egui::Color32::from_rgb(cfg.active_r, cfg.active_g, cfg.active_b);

    let outer_rounding =
        (w.min(h) * cfg.mouse_round_mul).clamp(cfg.mouse_round_min, cfg.mouse_round_max);
    let outer_fill = egui::Color32::from_rgba_unmultiplied(0, 0, 0, cfg.mouse_outer_fill_a);
    let outer_stroke = egui::Stroke::new(
        cfg.mouse_outer_stroke_w,
        egui::Color32::from_rgba_unmultiplied(255, 255, 255, cfg.mouse_outer_stroke_a),
    );
    ui.painter().rect_filled(rect, outer_rounding, outer_fill);
    ui.painter().rect_stroke(rect, outer_rounding, outer_stroke);

    let top_h = h * cfg.mouse_top_h_mul;
    let mid_w = w * cfg.mouse_mid_w_mul;

    let top_rect = egui::Rect::from_min_size(rect.min, egui::vec2(w, top_h));
    let mid_rect = egui::Rect::from_center_size(
        egui::pos2(rect.center().x, rect.min.y + top_h * 0.5),
        egui::vec2(mid_w, top_h * 0.80),
    );

    let left_rect =
        egui::Rect::from_min_max(top_rect.min, egui::pos2(mid_rect.min.x, top_rect.max.y));
    let right_rect =
        egui::Rect::from_min_max(egui::pos2(mid_rect.max.x, top_rect.min.y), top_rect.max);

    let l_down = pressed.contains("LMB");
    let r_down = pressed.contains("RMB");
    let m_down = pressed.contains("MMB");
    let wheel_flash = wheel_up_ticks > 0 || wheel_down_ticks > 0;

    let inactive = egui::Color32::from_rgba_unmultiplied(0, 0, 0, 0);
    let pad = cfg.mouse_inner_pad;

    let left_shape = left_rect.shrink(pad);
    ui.painter().rect_filled(
        left_shape,
        egui::Rounding {
            nw: outer_rounding,
            ne: 0.0,
            sw: 0.0,
            se: 0.0,
        },
        if l_down { active } else { inactive },
    );

    let right_shape = right_rect.shrink(pad);
    ui.painter().rect_filled(
        right_shape,
        egui::Rounding {
            nw: 0.0,
            ne: outer_rounding,
            sw: 0.0,
            se: 0.0,
        },
        if r_down { active } else { inactive },
    );

    let mid_shape = mid_rect.shrink(pad);
    ui.painter().rect_filled(
        mid_shape,
        egui::Rounding::same(cfg.mouse_inner_round),
        if m_down || wheel_flash {
            active
        } else {
            inactive
        },
    );

    let sep_col = egui::Color32::from_rgba_unmultiplied(255, 255, 255, cfg.mouse_sep_a);
    ui.painter().line_segment(
        [
            egui::pos2(mid_rect.min.x, top_rect.min.y + 10.0),
            egui::pos2(mid_rect.min.x, top_rect.max.y - 10.0),
        ],
        egui::Stroke::new(cfg.mouse_sep_w, sep_col),
    );
    ui.painter().line_segment(
        [
            egui::pos2(mid_rect.max.x, top_rect.min.y + 10.0),
            egui::pos2(mid_rect.max.x, top_rect.max.y - 10.0),
        ],
        egui::Stroke::new(cfg.mouse_sep_w, sep_col),
    );
    ui.painter().line_segment(
        [
            egui::pos2(rect.min.x + 10.0, top_rect.max.y),
            egui::pos2(rect.max.x - 10.0, top_rect.max.y),
        ],
        egui::Stroke::new(cfg.mouse_sep_w, sep_col),
    );

    let hint = if wheel_up_ticks > 0 {
        "^"
    } else if wheel_down_ticks > 0 {
        "v"
    } else {
        ""
    };
    if !hint.is_empty() {
        ui.painter().text(
            rect.center() + egui::vec2(0.0, -10.0),
            egui::Align2::CENTER_CENTER,
            hint,
            egui::FontId::proportional(cfg.wheel_hint_font),
            egui::Color32::from_rgba_unmultiplied(255, 255, 255, cfg.wheel_hint_a),
        );
    }
}
