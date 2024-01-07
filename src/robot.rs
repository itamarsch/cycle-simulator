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

    pub current_action: CycleAction,
    scoring_strategy: ScoringStrategy,
}

impl Robot {
    pub fn a() -> Self {
        const AMP_DRIVE_TIME: f32 = 0.0;
        const AMP_SCORE_TIME: f32 = 0.0;
        const SPEAKER_DRIVE_TIME: f32 = 0.0;
        const SPEAKER_SCORE_TIME: f32 = 0.0;
        Robot {
            speaker_drive_time: SPEAKER_DRIVE_TIME,
            speaker_score_time: SPEAKER_SCORE_TIME,
            amp_drive_time: AMP_DRIVE_TIME,
            amp_score_time: AMP_SCORE_TIME,

            current_action: CycleAction::new(Action::Driving, AMP_DRIVE_TIME, ScoringType::Amp),
            scoring_strategy: ScoringStrategy::Amp,
        }
    }

    pub fn tick(
        &mut self,
        t: f32,
        field: &Field,
        other_robots: (&Robot, &Robot),
    ) -> Option<ScoringType> {
        self.current_action.time_left -= t;
        let action = None;
        if self.current_action.time_left < 0.0 {
            match self.current_action.action {
                Action::Driving => {
                    let time_addition = match self.current_action.score_type {
                        ScoringType::Amp => self.amp_score_time,
                        ScoringType::Speaker => self.speaker_score_time,
                    };
                    self.current_action.time_left += time_addition + todo!("random time") as f32;
                    self.current_action.action = Action::Scoring;
                }
                Action::Scoring => {
                    if todo!("Add propability for success") {
                        action = Some(self.current_action.score_type);
                    }

                    let next_scoring_type = self.get_next_scoring_type(field);
                    let time_addition = match next_scoring_type {
                        ScoringType::Amp => self.amp_drive_time,
                        ScoringType::Speaker => self.speaker_drive_time,
                    };
                    self.current_action.time_left += time_addition + todo!("random time") as f32;
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
