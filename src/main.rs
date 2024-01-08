use field::{Field, FieldActions};
use rand::prelude::*;
use robot::Robot;

mod actions;
mod field;
mod robot;

const STEP: f32 = 0.01;

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
    const MATCH_TIME: f32 = 135.0;
    (0..)
        .map(|x| x as f32 * STEP)
        .take_while(|t| *t <= MATCH_TIME)
        .fold(Field::default(), |field, t| {
            let actions = alliance.tick(t, &field, &mut rng);
            field.apply(actions)
        })
}

fn main() {
    let mut rng = rand::rngs::StdRng::seed_from_u64(1690);
    let alliance: Alliance = Alliance {
        a: Robot::new(
            robot::ScoringStrategy::Amp,
            Default::default(),
            "A",
            &mut rng,
        ),
        b: Robot::new(
            robot::ScoringStrategy::Speaker,
            Default::default(),
            "S",
            &mut rng,
        ),
        c: Robot::new(
            robot::ScoringStrategy::SpeakerAndAmp,
            Default::default(),
            "SA",
            &mut rng,
        ),
    };
    let final_field = run_match(alliance, rng);
    final_field.get_score();
}
