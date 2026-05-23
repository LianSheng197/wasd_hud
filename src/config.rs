#[derive(Clone, Copy)]
pub struct HudConfig {
    pub win_w: f32,
    pub win_h: f32,
    pub grip_w: f32,
    pub grip_h: f32,
    pub grip_round: f32,
    pub grip_font: f32,
    pub key_u: f32,
    pub key_gap: f32,
    pub top_pad: f32,
    pub left_pad: f32,
    pub tab_w_mul: f32,
    pub shift_w_mul: f32,
    pub space_w_mul: f32,
    pub space_extra_gaps: f32,
    pub key_round: f32,
    pub key_stroke_w: f32,
    pub key_font: f32,
    pub mouse_gap: f32,
    pub mouse_w: f32,
    pub mouse_height_trim_mul: f32,
    pub mouse_outer_fill_a: u8,
    pub mouse_outer_stroke_a: u8,
    pub mouse_outer_stroke_w: f32,
    pub mouse_sep_a: u8,
    pub mouse_sep_w: f32,
    pub mouse_inner_pad: f32,
    pub mouse_mid_w_mul: f32,
    pub mouse_top_h_mul: f32,
    pub mouse_round_mul: f32,
    pub mouse_round_min: f32,
    pub mouse_round_max: f32,
    pub mouse_inner_round: f32,
    pub active_r: u8,
    pub active_g: u8,
    pub active_b: u8,
    pub key_inactive_fill_a: u8,
    pub key_inactive_stroke_a: u8,
    pub wheel_flash_ticks: u8,
    pub wheel_hint_font: f32,
    pub wheel_hint_a: u8,
}

impl Default for HudConfig {
    fn default() -> Self {
        Self {
            win_w: 500.0,
            win_h: 370.0,
            grip_w: 90.0,
            grip_h: 24.0,
            grip_round: 6.0,
            grip_font: 14.0,
            key_u: 60.0,
            key_gap: 8.0,
            top_pad: 32.0,
            left_pad: 5.0,
            tab_w_mul: 1.4,
            shift_w_mul: 1.4,
            space_w_mul: 2.8,
            space_extra_gaps: 2.0,
            key_round: 8.0,
            key_stroke_w: 2.0,
            key_font: 20.0,
            mouse_gap: 20.0,
            mouse_w: 100.0,
            mouse_height_trim_mul: 0.4,
            mouse_outer_fill_a: 60,
            mouse_outer_stroke_a: 140,
            mouse_outer_stroke_w: 2.0,
            mouse_sep_a: 90,
            mouse_sep_w: 2.0,
            mouse_inner_pad: 6.0,
            mouse_mid_w_mul: 0.22,
            mouse_top_h_mul: 0.33,
            mouse_round_mul: 0.22,
            mouse_round_min: 14.0,
            mouse_round_max: 24.0,
            mouse_inner_round: 14.0,
            active_r: 0,
            active_g: 255,
            active_b: 100,
            key_inactive_fill_a: 120,
            key_inactive_stroke_a: 140,
            wheel_flash_ticks: 10,
            wheel_hint_font: 18.0,
            wheel_hint_a: 180,
        }
    }
}

impl HudConfig {
    pub fn scaled(scale: f32) -> Self {
        let mut c = Self::default();

        c.win_w *= scale;
        c.win_h *= scale;
        c.grip_w *= scale;
        c.grip_h *= scale;
        c.grip_round *= scale;
        c.grip_font *= scale;
        c.key_u *= scale;
        c.key_gap *= scale;
        c.top_pad *= scale;
        c.left_pad *= scale;
        c.key_round *= scale;
        c.key_stroke_w *= scale;
        c.key_font *= scale;
        c.mouse_gap *= scale;
        c.mouse_w *= scale;
        c.mouse_outer_stroke_w *= scale;
        c.mouse_sep_w *= scale;
        c.mouse_inner_pad *= scale;
        c.mouse_inner_round *= scale;
        c.wheel_hint_font *= scale;

        c
    }
}
