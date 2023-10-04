#[derive(Debug)]
pub struct Settings {
    pub speed: f32,
    pub volume: f32,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            speed: 1.0,
            volume: 1.0,
        }
    }
}

impl Settings {
    pub fn from_env() -> Self {
        let mut settings = Self::default();

        settings.speed = std::env::var("SUNREST_SPEED")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(settings.speed);

        settings.volume = std::env::var("SUNREST_VOLUME")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(settings.volume);

        settings
    }
}
