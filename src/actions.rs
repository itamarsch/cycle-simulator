pub struct CycleAction {
    pub time_left: f32,
    pub action: Action,
    pub score_type: ScoringType,
}
impl CycleAction {
    pub fn new(action: Action, time_left: f32, score_type: ScoringType) -> CycleAction {
        CycleAction {
            time_left,
            action,
            score_type,
        }
    }
}

pub enum Action {
    Driving,
    Scoring,
}

#[derive(Copy, Clone)]
pub enum ScoringType {
    Amp,
    Speaker,
}
