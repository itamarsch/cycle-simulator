use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::{
    actions::{Action, NoteScoringType, TimedAction},
    field::{EndgameState, Field, MessageToField, RobotMessage},
    MATCH_TIME, STEP,
};

mod robot_config;
pub use robot_config::RobotConfig;

#[derive(Serialize, Deserialize, Clone)]
pub enum ScoringStrategy {
    Amp,
    Speaker,
    SpeakerAndAmp,
}
pub struct Robot {
    config: RobotConfig,
    pub current_action: TimedAction,
}

impl Robot {
    pub fn new(mut config: RobotConfig, rng: &mut impl Rng) -> Self {
        config.climbing_time =
            gen_time_with_noise(config.climbing_time, config.climbing_time_noise_factor, rng);

        let initial_action = TimedAction::new(
            Action::Driving,
            gen_time_with_noise(
                config.placing_config(0.0).drive_time,
                config.placing_config(0.0).drive_time_noise_factor,
                rng,
            ),
        );
        Robot {
            config,
            current_action: initial_action,
        }
    }

    pub fn tick(
        &mut self,
        rng: &mut impl Rng,
        field: &Field,
        other_robots: (&Robot, &Robot),
    ) -> Option<RobotMessage> {
        self.current_action.time_left -= STEP;
        let mut message_to_field = None;

        // Its time to climb, start climbing!
        if !matches!(self.current_action.action, Action::Climbing)
            && MATCH_TIME - field.t - STEP * 2.0 <= self.config.climbing_time
        {
            self.current_action = TimedAction::new(Action::Climbing, self.config.climbing_time);
            message_to_field = Some(MessageToField::ActionStarted(Action::Climbing));
        }

        if self.current_action.time_left < 0.0 {
            // Once action is finished send next action to robot and send action result to field
            (self.current_action, message_to_field) = self.next_action(field, other_robots, rng)
        }
        message_to_field.map(|message| self.build_robot_message(message))
    }

    fn build_robot_message(&self, message: MessageToField) -> RobotMessage {
        RobotMessage::new(self.config.name.to_owned(), message)
    }

    pub fn next_action(
        &self,
        field: &Field,
        other_robots: (&Robot, &Robot),
        rng: &mut impl Rng,
    ) -> (TimedAction, Option<MessageToField>) {
        match self.current_action.action {
            // Finished driving pick how to score based on startegy
            Action::Driving => self.on_driving_finished(field, other_robots, rng),
            // Finished scoring start driving
            Action::NoteScoring(scoring_type) => {
                self.on_scoring_finished(scoring_type, field, other_robots, rng)
            }
            Action::Climbing => self.on_climbing_finished(field, other_robots, rng),
        }
    }

    pub fn on_driving_finished(
        &self,
        field: &Field,
        other_robots: (&Robot, &Robot),
        rng: &mut impl Rng,
    ) -> (TimedAction, Option<MessageToField>) {
        let message_to_field = None;

        let next_scoring_type = self.get_next_scoring_type(field, other_robots);
        let time_addition = match next_scoring_type {
            NoteScoringType::Amp => self.config.placing_config(field.t).amp_score_time,
            NoteScoringType::Speaker => self.config.placing_config(field.t).speaker_score_time,
        };

        let next_action = TimedAction {
            time_left: self.current_action.time_left
                + gen_time_with_noise(
                    time_addition,
                    self.config.placing_config(field.t).place_time_noise_factor,
                    rng,
                ),
            action: Action::NoteScoring(next_scoring_type),
        };
        (next_action, message_to_field)
    }

    pub fn on_scoring_finished(
        &self,
        scoring_type: NoteScoringType,
        field: &Field,
        _other_robots: (&Robot, &Robot),
        rng: &mut impl Rng,
    ) -> (TimedAction, Option<MessageToField>) {
        let message_to_field = match scoring_type {
            NoteScoringType::Amp => Some(MessageToField::NoteScored(scoring_type)),
            NoteScoringType::Speaker
                if rng.gen_range(0.0..1.0)
                    < self.config.placing_config(field.t).scoring_success_percent =>
            {
                Some(MessageToField::NoteScored(scoring_type))
            }

            NoteScoringType::Speaker => Some(MessageToField::NoteFailed(scoring_type)),
        };

        let next_action = TimedAction {
            time_left: self.current_action.time_left
                + gen_time_with_noise(
                    self.config.placing_config(field.t).drive_time,
                    self.config.placing_config(field.t).drive_time_noise_factor,
                    rng,
                ),
            action: Action::Driving,
        };
        (next_action, message_to_field)
    }

    pub fn on_climbing_finished(
        &self,
        _field: &Field,
        _other_robots: (&Robot, &Robot),
        rng: &mut impl Rng,
    ) -> (TimedAction, Option<MessageToField>) {
        let message_to_field = Some(MessageToField::FinishedClimbing(
            if rng.gen_range(0.0..1.0) < self.config.climb_success_percent {
                if rng.gen_range(0.0..1.0) < self.config.trap_success_percent {
                    EndgameState::Trapped
                } else {
                    EndgameState::FailedTrapping
                }
            } else {
                EndgameState::FailedClimbing
            },
        ));

        let next_action = TimedAction {
            time_left: f32::INFINITY,
            action: self.current_action.action,
        };
        (next_action, message_to_field)
    }

    // Runs after robot finished intake and is at the wing, selects how to place based on field state
    fn get_next_scoring_type(&self, field: &Field, robots: (&Robot, &Robot)) -> NoteScoringType {
        match self.config.placing_config(field.t).scoring_strategy {
            ScoringStrategy::Amp => NoteScoringType::Amp,
            ScoringStrategy::Speaker => NoteScoringType::Speaker,
            ScoringStrategy::SpeakerAndAmp => {
                let amount_of_robots_playing_amp =
                    amount_of_amps_for_action(&robots.0.current_action.action)
                        + amount_of_amps_for_action(&robots.1.current_action.action);
                let current_amount_for_amplify = field.current_amplify.as_int();

                if current_amount_for_amplify + amount_of_robots_playing_amp < 2 {
                    NoteScoringType::Amp
                } else {
                    NoteScoringType::Speaker
                }
            }
        }
    }
}

fn amount_of_amps_for_action(action: &Action) -> u32 {
    match action {
        Action::NoteScoring(NoteScoringType::Amp) => 1,
        _ => 0,
    }
}

fn gen_time_with_noise(time_addition: f32, noise_factor: f32, rng: &mut impl Rng) -> f32 {
    let max_noise = time_addition * noise_factor;
    time_addition + rng.gen_range(0.0..max_noise)
}
