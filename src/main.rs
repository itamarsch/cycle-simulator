use field::{Field, FieldActions};
use rand::prelude::*;
use robot::Robot;

use crate::robot::RobotConfig;

mod actions;
mod field;
mod robot;

const STEP: f32 = 0.01;
const MATCH_TIME: f32 = 135.0;

struct Alliance {
    a: Robot,
    b: Robot,
    c: Robot,
}

impl Alliance {
    fn tick(&mut self, t: f32, field: &Field, rng: &mut impl Rng) -> FieldActions {
        FieldActions::new(
            t,
            (self.a.name, self.a.tick(rng, field, (&self.b, &self.c))),
            (self.b.name, self.b.tick(rng, field, (&self.a, &self.c))),
            (self.c.name, self.c.tick(rng, field, (&self.a, &self.b))),
        )
    }
}

fn run_match(mut alliance: Alliance, mut rng: impl Rng) -> Field {
    (0..)
        .map(|x| x as f32 * STEP)
        .take_while(|t| *t <= MATCH_TIME)
        .fold(Field::default(), |field, t| {
            let actions = alliance.tick(t, &field, &mut rng);
            field.apply(actions, true)
        })
}

fn main() {
    let default: RobotConfig =
        serde_json::from_str(include_str!("../default_config.json")).expect("File is valid json");
    let mut scores = Vec::with_capacity(10000);
    let mut game_pieces_playeds = Vec::with_capacity(10000);
    for i in 10_000..10_001 {
        let mut rng = rand::rngs::StdRng::seed_from_u64(i);
        let alliance = Alliance {
            a: Robot::new(robot::ScoringStrategy::Amp, default.clone(), "A", &mut rng),
            b: Robot::new(
                robot::ScoringStrategy::SpeakerAndAmp,
                default.clone(),
                "SA2",
                &mut rng,
            ),
            c: Robot::new(
                robot::ScoringStrategy::SpeakerAndAmp,
                default.clone(),
                "SA1",
                &mut rng,
            ),
        };
        let final_field = run_match(alliance, rng);
        let (score, game_pieces_played) = final_field.get_score();
        scores.push(score);
        game_pieces_playeds.push(game_pieces_played);
    }
    println!(
        "AVG score: {}, AVG Game pieces played: {}",
        avg(scores),
        avg(game_pieces_playeds)
    );
}

fn avg(vec: Vec<i32>) -> f32 {
    vec.iter().sum::<i32>() as f32 / vec.len() as f32
}
