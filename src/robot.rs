use rand::Rng;

use crate::{
    actions::{Action, CycleAction, ScoringType},
    field::Field,
};

enum ScoringStrategy {
    Amp,
    Speaker,
    SpeakerAndAmp,
    AmpAndSpeaker,
}
pub struct Robot {
    speaker_drive_time: f32,
    speaker_score_time: f32,
    amp_drive_time: f32,
    amp_score_time: f32,
    drive_time_noise_factor: f32,
    place_time_noise_factor: f32,
    success_percent: f32,

    pub current_action: CycleAction,
    scoring_strategy: ScoringStrategy,
}

impl Robot {
    pub fn a() -> Self {
        const AMP_DRIVE_TIME: f32 = 0.0;
        const AMP_SCORE_TIME: f32 = 0.0;
        const SPEAKER_DRIVE_TIME: f32 = 0.0;
        const SPEAKER_SCORE_TIME: f32 = 0.0;
        const DRIVE_TIME_NOISE_FACTOR: f32 = 0.0;
        const PLACE_TIME_NOISE_FACTOR: f32 = 0.0;
        const SUCCESS_PERCENT: f32 = 0.8;
        Robot {
            speaker_drive_time: SPEAKER_DRIVE_TIME,
            speaker_score_time: SPEAKER_SCORE_TIME,
            amp_drive_time: AMP_DRIVE_TIME,
            amp_score_time: AMP_SCORE_TIME,
            drive_time_noise_factor: DRIVE_TIME_NOISE_FACTOR,
            place_time_noise_factor: PLACE_TIME_NOISE_FACTOR,
            success_percent: SUCCESS_PERCENT,

            current_action: CycleAction::new(Action::Driving, AMP_DRIVE_TIME, ScoringType::Amp),
            scoring_strategy: ScoringStrategy::Amp,
        }
    }

    pub fn tick(
        &mut self,
        t: f32,
        rng: &mut impl Rng,
        field: &Field,
        other_robots: (&Robot, &Robot),
    ) -> Option<ScoringType> {
        self.current_action.time_left -= t;
        let mut action = None;
        if self.current_action.time_left < 0.0 {
            match self.current_action.action {
                Action::Driving => {
                    let time_addition = match self.current_action.score_type {
                        ScoringType::Amp => self.amp_score_time,
                        ScoringType::Speaker => self.speaker_score_time,
                    };
                    let max_noise = time_addition * self.place_time_noise_factor;
                    self.current_action.time_left +=
                        time_addition + rng.gen_range(-max_noise..max_noise);
                    self.current_action.action = Action::Scoring;
                }
                Action::Scoring => {
                    if rng.gen_range(0.0..1.0) < self.success_percent {
                        action = Some(self.current_action.score_type);
                    }

                    let next_scoring_type = self.get_next_scoring_type(field, other_robots);
                    let time_addition = match next_scoring_type {
                        ScoringType::Amp => self.amp_drive_time,
                        ScoringType::Speaker => self.speaker_drive_time,
                    };
                    let max_noise = time_addition * self.drive_time_noise_factor;
                    self.current_action.time_left +=
                        time_addition + rng.gen_range(-max_noise..max_noise);
                    self.current_action.action = Action::Driving;
                }
            }
        }
        action
    }

    fn get_next_scoring_type(&self, field: &Field, robots: (&Robot, &Robot)) -> ScoringType {
        todo!("Implement strategy")
    }
}
