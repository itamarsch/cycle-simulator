use field::{Field, FieldActions};
use rand::prelude::*;
use robot::Robot;

pub mod actions;
pub mod field;
pub mod robot;

const STEP: f32 = 0.01;
const MATCH_TIME: f32 = 135.0;

pub struct Alliance {
    pub a: Robot,
    pub b: Robot,
    pub c: Robot,
}

impl Alliance {
    pub fn tick(&mut self, t: f32, field: &Field, rng: &mut impl Rng) -> FieldActions {
        FieldActions::new(
            t,
            (self.a.name, self.a.tick(rng, field, (&self.b, &self.c))),
            (self.b.name, self.b.tick(rng, field, (&self.a, &self.c))),
            (self.c.name, self.c.tick(rng, field, (&self.a, &self.b))),
        )
    }
}

pub fn run_match(mut alliance: Alliance, mut rng: impl Rng, print: bool) -> Field {
    (0..)
        .map(|x| x as f32 * STEP)
        .take_while(|t| *t <= MATCH_TIME)
        .fold(Field::default(), |field, t| {
            let actions = alliance.tick(t, &field, &mut rng);
            field.apply(actions, print)
        })
}
