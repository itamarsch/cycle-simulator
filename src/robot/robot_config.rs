use serde::{Deserialize, Serialize};

use super::ScoringStrategy;

#[derive(Serialize, Deserialize, Clone)]
pub struct PlacingConfig {
    pub scoring_strategy: ScoringStrategy,
    pub drive_time: f32,
    pub drive_time_noise_factor: f32,
    pub speaker_score_time: f32,
    pub amp_score_time: f32,
    pub place_time_noise_factor: f32,
    pub scoring_success_percent: f32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RobotConfig {
    pub name: String,
    pub teleop_config: PlacingConfig,
    pub auto_config: PlacingConfig,

    pub climbing_time: f32,
    pub climbing_time_noise_factor: f32,
    pub climb_success_percent: f32,
    pub trap_success_percent: f32,
}

impl RobotConfig {
    pub fn placing_config(&self, t: f32) -> &PlacingConfig {
        if t <= 15.0 {
            &self.auto_config
        } else {
            &self.teleop_config
        }
    }
}
