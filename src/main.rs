mod app;
mod config;
mod draw;
mod input;
mod state;

use crate::app::HudApp;
use crate::config::HudConfig;
use crate::input::{map_button, map_key};
use crate::state::HudState;
use eframe::egui;
use rdev::{listen, Event, EventType};
use std::thread;

fn main() -> eframe::Result<()> {
    let cfg = HudConfig::scaled(0.75);

    let state = HudState::new();
    let state_for_thread = state.clone();
    let wheel_flash_ticks = cfg.wheel_flash_ticks;

    thread::spawn(move || {
        let state_for_events = state_for_thread.clone();

        {
            let mut st = state_for_thread.inner.lock().unwrap();
            st.listener_ok = true;
            st.listener_error = None;
        }

        let listen_result = listen(move |event: Event| {
            let mut st = state_for_events.inner.lock().unwrap();
            st.event_count = st.event_count.saturating_add(1);

            match event.event_type {
                EventType::KeyPress(k) => {
                    if let Some(name) = map_key(k) {
                        st.pressed.insert(name);
                    }
                }
                EventType::KeyRelease(k) => {
                    if let Some(name) = map_key(k) {
                        st.pressed.remove(name);
                    }
                }
                EventType::ButtonPress(b) => {
                    if let Some(name) = map_button(b) {
                        st.pressed.insert(name);
                    }
                }
                EventType::ButtonRelease(b) => {
                    if let Some(name) = map_button(b) {
                        st.pressed.remove(name);
                    }
                }
                EventType::Wheel {
                    delta_x: _,
                    delta_y,
                } => {
                    if delta_y > 0 {
                        st.wheel_up_ticks = wheel_flash_ticks;
                    } else if delta_y < 0 {
                        st.wheel_down_ticks = wheel_flash_ticks;
                    }
                }
                _ => {}
            }
        });

        if let Err(err) = listen_result {
            let mut st = state_for_thread.inner.lock().unwrap();
            st.listener_ok = false;
            st.listener_error = Some(format!("{err:?}"));
        }
    });

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_decorations(false)
            .with_transparent(true)
            .with_always_on_top()
            .with_inner_size([cfg.win_w, cfg.win_h]),
        ..Default::default()
    };

    eframe::run_native(
        "wasd_hud",
        options,
        Box::new(move |cc| {
            let mut style = (*cc.egui_ctx.style()).clone();
            style.visuals.window_fill = egui::Color32::TRANSPARENT;
            style.visuals.panel_fill = egui::Color32::TRANSPARENT;
            style.visuals.extreme_bg_color = egui::Color32::TRANSPARENT;
            style.visuals.faint_bg_color = egui::Color32::TRANSPARENT;
            cc.egui_ctx.set_style(style);

            Box::new(HudApp::new(state, cfg))
        }),
    )
}
