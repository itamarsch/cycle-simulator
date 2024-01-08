use color_print::cprintln;

use crate::{actions::ScoringType, STEP};

pub struct FieldActions {
    t: f32,
    actions: [(&'static str, Option<ScoringType>); 3],
}
impl FieldActions {
    pub fn new(
        t: f32,
        a: (&'static str, Option<ScoringType>),
        b: (&'static str, Option<ScoringType>),
        c: (&'static str, Option<ScoringType>),
    ) -> FieldActions {
        let actions = [a, b, c];
        FieldActions { t, actions }
    }
}

#[derive(Default, Debug)]
pub struct Field {
    pub t: f32,
    pub speaker: i32,
    pub amplified_speaker: i32,
    pub amp: i32,
    pub time_left_for_amplified: Option<f32>,
    pub current_amplify: AmplifyState,
}

impl Field {
    pub fn get_score(&self) -> (i32, i32) {
        let speaker_score = self.speaker * 2;
        let amplified_speaker_score = self.amplified_speaker * 5;
        let amp_score = self.amp;

        let game_pieces_played = self.amp + self.amplified_speaker + self.speaker;
        let score = amp_score + amplified_speaker_score + speaker_score;

        (score, game_pieces_played)
    }
    pub fn apply(mut self, actions: FieldActions) -> Self {
        self.t = actions.t;
        if let Some(ref mut time_left_for_amplified) = self.time_left_for_amplified {
            *time_left_for_amplified -= STEP;
            if *time_left_for_amplified <= 0.0 {
                cprintln!("<yellow>{} *** Stop amplification ***</>", self.t,);
                self.time_left_for_amplified = None;
                self.current_amplify = AmplifyState::Off;
            }
        }

        for (action, name) in actions
            .actions
            .into_iter()
            .filter_map(|(name, maybe_action)| maybe_action.map(|action| (action, name)))
        {
            match action {
                ScoringType::Amp => {
                    cprintln!( "<green>{} Robot {} added to amplifier, Amplify state before placement: {:?}</>", self.t, name, self.current_amplify);
                    self.amp += 1;
                    if self.time_left_for_amplified.is_none() {
                        self.current_amplify = self.current_amplify.next();
                    }
                }
                ScoringType::Speaker => {
                    // Basic startegy every team will probably use:
                    // If you can amplify the speaker,
                    // amplify it once a robot shoots to the speaker
                    if let AmplifyState::Two = self.current_amplify {
                        if self.time_left_for_amplified.is_none() {
                            cprintln!("<yellow>{} *** Start amplification ***</>", self.t);
                            const AMPLIFICATION_TIME: f32 = 10f32;
                            self.time_left_for_amplified = Some(AMPLIFICATION_TIME);
                        }
                    }

                    cprintln!("<blue>{} Robot {} shot to speaker, Amplify state before placement: {:?}</>", self.t,name, self.current_amplify);
                    if self.time_left_for_amplified.is_some() {
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
#[derive(Default, Debug)]
pub enum AmplifyState {
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
    pub fn as_int(&self) -> u32 {
        match self {
            AmplifyState::Off => 0,
            AmplifyState::One => 1,
            AmplifyState::Two => 2,
        }
    }
}
