use std::collections::HashSet;
use std::sync::{Arc, Mutex};

#[derive(Default)]
pub struct InputState {
    pub pressed: HashSet<&'static str>,
    pub wheel_up_ticks: u8,
    pub wheel_down_ticks: u8,
    pub listener_ok: bool,
    pub listener_error: Option<String>,
    pub event_count: u64,
}

#[derive(Clone)]
pub struct HudState {
    pub inner: Arc<Mutex<InputState>>,
}

impl HudState {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(InputState::default())),
        }
    }
}
