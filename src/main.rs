use field::{AllianceActions, Field};
use rand::prelude::*;
use robot::Robot;

mod actions;
mod field;
mod robot;

const STEP: f32 = 1.0;

struct Alliance {
    a: Robot,
    b: Robot,
    c: Robot,
}

impl Alliance {
    fn tick(&mut self, t: f32, field: &Field, rng: &mut impl Rng) -> AllianceActions {
        AllianceActions::new(
            t,
            [
                self.a.tick(t, rng, field, (&self.b, &self.c)),
                self.b.tick(t, rng, field, (&self.a, &self.c)),
                self.c.tick(t, rng, field, (&self.a, &self.b)),
            ],
        )
    }
}

fn run_match(mut alliance: Alliance, mut rng: impl Rng) -> Field {
    const MATCH_TIME: f32 = 135.0;
    (0..)
        .map(|x| x as f32 * STEP)
        .take_while(|t| *t < MATCH_TIME)
        .fold(Field::default(), |field, t| {
            let actions = alliance.tick(t, &field, &mut rng);
            field.apply(actions)
        })
}

fn main() {
    let alliance: Alliance = Alliance {
        a: Robot::a(),
        b: Robot::a(),
        c: Robot::a(),
    };
    let rng = rand::rngs::StdRng::seed_from_u64(100);
    run_match(alliance, rng);
}
