use crate::{actions::ScoringType, STEP};

pub struct FieldSnapshot {
    t: f32,
    actions: [Option<ScoringType>; 3],
}
impl FieldSnapshot {
    pub fn new(t: f32, actions: [Option<ScoringType>; 3]) -> FieldSnapshot {
        FieldSnapshot { t, actions }
    }
}

#[derive(Default)]
enum AmplifyState {
    #[default]
    Off,
    One,
    Two,
}
impl AmplifyState {
    fn next(&self) -> Self {
        match self {
            AmplifyState::Off => AmplifyState::One,
            AmplifyState::One => AmplifyState::Two,
            AmplifyState::Two => AmplifyState::Two,
        }
    }
}

#[derive(Default)]
pub struct Field {
    speaker: i32,
    amplified_speaker: i32,
    amp: i32,
    amplified_amp: i32,
    time_left_for_amplified: Option<f32>,
    current_amplify: AmplifyState,
}

impl Field {
    pub fn apply(mut self, actions: FieldSnapshot) -> Self {
        if let Some(ref mut time_left_for_amplified) = self.time_left_for_amplified {
            *time_left_for_amplified -= STEP;
            if *time_left_for_amplified <= 0.0 {
                self.time_left_for_amplified = None;
            }
        }

        for action in actions.actions.into_iter().flatten() {
            match action {
                ScoringType::Amp => {
                    if self.time_left_for_amplified.is_some() {
                        self.amplified_amp += 1;
                    } else {
                        self.amp += 1;
                        self.current_amplify = self.current_amplify.next();
                    }
                }
                ScoringType::Speaker => {
                    if self.time_left_for_amplified.is_some() {
                        self.amplified_speaker += 1;
                    } else if let AmplifyState::Two = self.current_amplify {
                        self.current_amplify = AmplifyState::Off;
                        const AMPLIFICATION_TIME: f32 = 10f32;
                        self.time_left_for_amplified = Some(AMPLIFICATION_TIME);
                        self.amplified_speaker += 1;
                    } else {
                        self.speaker += 1;
                    }
                }
            }
        }
        self
    }
}
