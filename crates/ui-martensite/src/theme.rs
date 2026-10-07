//! Services monitor theme.

pub struct Color(pub u8, pub u8, pub u8);

pub struct Theme {
    pub bg: Color,
    pub status_ok: Color,
}

impl Theme {
    pub fn monitor_dark() -> Self {
        Self {
            bg: Color(14, 16, 20),
            status_ok: Color(16, 185, 129),
        }
    }
}
