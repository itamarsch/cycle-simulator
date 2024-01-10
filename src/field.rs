use color_print::cprintln;

use crate::{
    actions::{Action, NoteScoringType},
    STEP,
};

#[derive(Debug)]
pub struct ScoreSummarization {
    pub game_piece_points: i32,
    pub game_pieces_played: i32,
    pub stage_points: i32,
}

pub enum MessageToField {
    NoteScored(NoteScoringType),
    NoteFailed(NoteScoringType),
    ActionStarted(Action),
    FinishedClimbing(EndgameState),
}

pub struct RobotMessage {
    robot_name: String,
    field_action_message: MessageToField,
}

impl RobotMessage {
    pub fn new(name: String, field_action_message: MessageToField) -> RobotMessage {
        RobotMessage {
            robot_name: name,
            field_action_message,
        }
    }
}

pub struct AllianceActions {
    t: f32,
    actions: [Option<RobotMessage>; 3],
}
impl AllianceActions {
    pub fn new(
        t: f32,
        a: Option<RobotMessage>,
        b: Option<RobotMessage>,
        c: Option<RobotMessage>,
    ) -> AllianceActions {
        let actions = [a, b, c];
        AllianceActions { t, actions }
    }
}

#[derive(Default, Debug)]
pub struct Field {
    pub t: f32,
    pub started_teleop: bool,
    pub speaker: i32,
    pub climbs: i32,
    pub amplified_speaker: i32,
    pub auto_speaker: i32,
    pub auto_amp: i32,
    pub amp: i32,
    pub time_left_for_amplified: Option<f32>,
    pub current_amplify: AmplifyState,
    pub traps: i32,
}

impl Field {
    pub fn get_score(&self) -> ScoreSummarization {
        let speaker_score = self.speaker * 2 + self.auto_speaker * 5 + self.amplified_speaker * 5;
        let amp_score = self.amp + self.auto_amp * 2;

        let game_pieces_played =
            self.amp + self.auto_amp + self.amplified_speaker + self.speaker + self.auto_speaker;

        let game_piece_points = amp_score + speaker_score;
        let stage_points = self.climbs * 3 + self.traps * 5;
        ScoreSummarization {
            game_piece_points,
            game_pieces_played,
            stage_points,
        }
    }

    pub fn apply_action(&mut self, message: RobotMessage, print: bool) {
        let robot_name = message.robot_name;
        match message.field_action_message {
            MessageToField::NoteScored(NoteScoringType::Amp) => {
                if self.t < 15.0 {
                    self.auto_amp += 1;
                } else {
                    self.amp += 1;
                }
                if self.time_left_for_amplified.is_none() {
                    self.current_amplify = self.current_amplify.next();
                }
                if print {
                    cprintln!(
                        "<green>{} Robot {} added to amplifier, Amplify state: {:?}</>",
                        self.t,
                        robot_name,
                        self.current_amplify
                    );
                }
            }
            MessageToField::NoteScored(NoteScoringType::Speaker) => {
                // Basic startegy every team will probably use:
                // If you can amplify the speaker,
                // amplify it once a robot shoots to the speaker
                if let AmplifyState::Two = self.current_amplify {
                    if self.time_left_for_amplified.is_none() && self.t > 15.0 {
                        if print {
                            cprintln!("<yellow>{} *** Start amplification ***</>", self.t);
                        }
                        const AMPLIFICATION_TIME: f32 = 10f32;
                        self.time_left_for_amplified = Some(AMPLIFICATION_TIME);
                    }
                }
                if print {
                    cprintln!(
                        "<blue>{} Robot {} shot to speaker, Amplified: {}</>",
                        self.t,
                        robot_name,
                        self.time_left_for_amplified.is_some()
                    );
                }
                if self.t < 15.0 {
                    self.auto_speaker += 1;
                } else if self.time_left_for_amplified.is_some() {
                    self.amplified_speaker += 1;
                } else {
                    self.speaker += 1;
                }
            }
            MessageToField::ActionStarted(Action::NoteScoring(_) | Action::Driving) => {}
            MessageToField::ActionStarted(Action::Climbing) => {
                if print {
                    cprintln!(
                        "<magenta>{} Robot {} Started climbing</>",
                        self.t,
                        robot_name
                    );
                }
            }
            MessageToField::NoteFailed(scoring_type) => {
                if print {
                    cprintln!(
                        "<red>{} Robot {} *failed* placing: {:?}</>",
                        self.t,
                        robot_name,
                        scoring_type,
                    );
                }
            }
            MessageToField::FinishedClimbing(EndgameState::FailedTrapping) => {
                self.climbs += 1;
                if print {
                    cprintln!(
                        "<bright-cyan>{} Robot {} Succeeded Climbing</>",
                        self.t,
                        robot_name
                    );
                }
            }
            MessageToField::FinishedClimbing(EndgameState::FailedClimbing) => {
                if print {
                    cprintln!(
                        "<bright-red>{} Robot {} *failed* climbing</>",
                        self.t,
                        robot_name
                    );
                }
            }
            MessageToField::FinishedClimbing(EndgameState::Trapped) => {
                self.traps += 1;
                self.climbs += 1;
                if print {
                    cprintln!(
                        "<bright-green>{} Robot {} Added to trap</>",
                        self.t,
                        robot_name
                    );
                }
            }
        }
    }

    pub fn apply(mut self, actions: AllianceActions, print: bool) -> Self {
        self.t = actions.t;
        if let Some(ref mut time_left_for_amplified) = self.time_left_for_amplified {
            *time_left_for_amplified -= STEP;
            if *time_left_for_amplified <= 0.0 {
                if print {
                    cprintln!("<yellow>{} *** Stop amplification ***</>", self.t,);
                }
                self.time_left_for_amplified = None;
                self.current_amplify = AmplifyState::Off;
            }
        }

        if !self.started_teleop && self.t >= 15.0 {
            self.started_teleop = true;
            println!("Started Teleop")
        }
        for action in actions.actions.into_iter().flatten() {
            self.apply_action(action, print);
        }
        self
    }
}

#[derive(Default, Debug)]
pub enum AmplifyState {
    #[default]
    Off,
    One,
    Two,
}

pub enum EndgameState {
    FailedClimbing,
    FailedTrapping,
    Trapped,
}

impl AmplifyState {
    fn next(&self) -> Self {
        match self {
            AmplifyState::Off => AmplifyState::One,
            AmplifyState::One => AmplifyState::Two,
            AmplifyState::Two => AmplifyState::Two,
        }
    }
    pub fn as_int(&self) -> u32 {
        match self {
            AmplifyState::Off => 0,
            AmplifyState::One => 1,
            AmplifyState::Two => 2,
        }
    }
}
