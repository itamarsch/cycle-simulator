use rand::Rng;

use crate::{
    actions::{Action, CycleAction, ScoringType},
    field::{Field, FieldActionMessage},
    MATCH_TIME, STEP,
};

pub enum ScoringStrategy {
    Amp,
    Speaker,
    SpeakerAndAmp,
}
pub struct Robot {
    config: RobotConfig,
    pub name: &'static str,
    pub current_action: CycleAction,
    scoring_strategy: ScoringStrategy,
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct RobotConfig {
    drive_time: f32,
    drive_time_noise_factor: f32,
    speaker_score_time: f32,
    amp_score_time: f32,
    place_time_noise_factor: f32,
    success_percent: f32,
    climbing_time: f32,
    climbing_time_noise_factor: f32,
}

impl Robot {
    pub fn new(
        scoring_strategy: ScoringStrategy,
        mut config: RobotConfig,
        name: &'static str,
        rng: &mut impl Rng,
    ) -> Self {
        config.climbing_time =
            gen_time_with_noise(config.climbing_time, config.climbing_time_noise_factor, rng);

        let initial_action = CycleAction::new(
            Action::Driving,
            gen_time_with_noise(config.drive_time, config.drive_time_noise_factor, rng),
        );
        Robot {
            name,
            config,

            current_action: initial_action,
            scoring_strategy,
        }
    }

    pub fn tick(
        &mut self,
        rng: &mut impl Rng,
        field: &Field,
        other_robots: (&Robot, &Robot),
    ) -> Option<FieldActionMessage> {
        self.current_action.time_left -= STEP;
        let mut action = None;

        if !matches!(self.current_action.action, Action::Climbing)
            && MATCH_TIME - field.t <= self.config.climbing_time
        {
            self.current_action = CycleAction::new(Action::Climbing, self.config.climbing_time);
            return Some(FieldActionMessage::ActionStarted(Action::Climbing));
        }

        if self.current_action.time_left < 0.0 {
            match self.current_action.action {
                // Finished driving pick how to score based on startegy
                Action::Driving => {
                    let next_scoring_type = self.get_next_scoring_type(field, other_robots);
                    let time_addition = match next_scoring_type {
                        ScoringType::Amp => self.config.amp_score_time,
                        ScoringType::Speaker => self.config.speaker_score_time,
                    };
                    self.current_action.time_left += gen_time_with_noise(
                        time_addition,
                        self.config.place_time_noise_factor,
                        rng,
                    );
                    self.current_action.action = Action::Scoring(next_scoring_type);
                }
                // Finished scoring start driving
                Action::Scoring(scoring_type) => {
                    match scoring_type {
                        ScoringType::Amp => {
                            action = Some(FieldActionMessage::Scored(scoring_type));
                        }
                        ScoringType::Speaker
                            if rng.gen_range(0.0..1.0) < self.config.success_percent =>
                        {
                            action = Some(FieldActionMessage::Scored(scoring_type));
                        }

                        _ => {
                            action = Some(FieldActionMessage::Failed(scoring_type));
                        }
                    }

                    self.current_action.time_left += gen_time_with_noise(
                        self.config.drive_time,
                        self.config.drive_time_noise_factor,
                        rng,
                    );
                    self.current_action.action = Action::Driving;
                }
                Action::Climbing => {}
            }
        }
        action
    }

    // Runs after robot finished intake and is at the wing, selects how to place based on field state
    fn get_next_scoring_type(&self, field: &Field, robots: (&Robot, &Robot)) -> ScoringType {
        match self.scoring_strategy {
            ScoringStrategy::Amp => ScoringType::Amp,
            ScoringStrategy::Speaker => ScoringType::Speaker,
            ScoringStrategy::SpeakerAndAmp => {
                let amount_of_robots_playing_amp =
                    amount_of_amps_for_action(&robots.0.current_action.action)
                        + amount_of_amps_for_action(&robots.1.current_action.action);
                let current_amount_for_amplify = field.current_amplify.as_int();

                if current_amount_for_amplify + amount_of_robots_playing_amp < 2 {
                    ScoringType::Amp
                } else {
                    ScoringType::Speaker
                }
            }
        }
    }
}

fn amount_of_amps_for_action(action: &Action) -> u32 {
    match action {
        Action::Scoring(ScoringType::Amp) => 1,
        _ => 0,
    }
}

fn gen_time_with_noise(time_addition: f32, noise_factor: f32, rng: &mut impl Rng) -> f32 {
    let max_noise = time_addition * noise_factor;
    time_addition + rng.gen_range(0.0..max_noise)
}
