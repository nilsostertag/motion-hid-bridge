/// Motion Layer für Walking Pad / Maus
/// ΔY → Geschwindigkeit (m/s), geglättet, Deadzone, Sensitivity

pub struct MotionState {
    /// Aktuelle Geschwindigkeit (m/s)
    pub speed_mps: f32,
    /// Roh-ΔY vom Sensor
    pub raw_delta_y: i32,
    /// Geglättete Geschwindigkeit (Moving Average)
    pub smooth_speed: f32,
}

impl MotionState {
    pub fn new() -> Self {
        Self {
            speed_mps: 0.0,
            raw_delta_y: 0,
            smooth_speed: 0.0,
        }
    }

    /// Update Moving Average
    /// alpha = Glättungsfaktor [0.0 - 1.0], kleiner = stärker geglättet
    pub fn smooth(&mut self, alpha: f32) {
        self.smooth_speed = self.smooth_speed * (1.0 - alpha) + self.speed_mps * alpha;
    }
}

/// Berechnet MotionState aus ΔY, dt und Sensitivity
pub fn update_motion(dy: i32, dt: f32, sensitivity: f32, prev: &mut MotionState) -> MotionState {
    // --- Deadzone ---
    let dy = if dy.abs() < 2 { 0 } else { dy };

    // --- dt Clamp ---
    let dt = dt.clamp(0.005, 0.1); // min 5ms, max 100ms

    // --- Geschwindigkeit berechnen ---
    let speed = (dy as f32 / 1000.0 / dt) * sensitivity; // 1 ΔY = 1mm vorläufig

    // MotionState erstellen
    let mut motion = MotionState {
        speed_mps: speed,
        raw_delta_y: dy,
        smooth_speed: prev.smooth_speed, // vorherige glatte Geschwindigkeit übernehmen
    };

    // Moving Average anwenden
    motion.smooth(0.2); // alpha = 0.2 → stark geglättet

    motion
}
