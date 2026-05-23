use eframe::egui;
use std::fs;
use std::path::Path;

const POSITION_FILE: &str = ".wasd_hud_position";

pub fn load_window_position() -> Option<egui::Pos2> {
    let text = fs::read_to_string(POSITION_FILE).ok()?;
    let mut parts = text.split_whitespace();
    let x: f32 = parts.next()?.parse().ok()?;
    let y: f32 = parts.next()?.parse().ok()?;
    Some(egui::pos2(x, y))
}

pub fn save_window_position(pos: egui::Pos2) {
    let content = format!("{:.3} {:.3}\n", pos.x, pos.y);
    let _ = fs::write(Path::new(POSITION_FILE), content);
}
