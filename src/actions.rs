pub struct CycleAction {
    pub time_left: f32,
    pub action: Action,
}
impl CycleAction {
    pub fn new(action: Action, time_left: f32) -> CycleAction {
        CycleAction { time_left, action }
    }
}

pub enum Action {
    Driving,
    Scoring(ScoringType),
    Climbing,
}

#[derive(Copy, Clone, Debug)]
pub enum ScoringType {
    Amp,
    Speaker,
}
