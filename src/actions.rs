pub struct TimedAction {
    pub time_left: f32,
    pub action: Action,
}
impl TimedAction {
    pub fn new(action: Action, time_left: f32) -> TimedAction {
        TimedAction { time_left, action }
    }
}

#[derive(Copy, Clone)]
pub enum Action {
    Driving,
    NoteScoring(NoteScoringType),
    Climbing,
}

#[derive(Copy, Clone, Debug)]
pub enum NoteScoringType {
    Amp,
    Speaker,
}
