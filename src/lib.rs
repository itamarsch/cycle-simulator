use field::{AllianceActions, Field};
use rand::prelude::*;
use robot::Robot;

pub mod actions;
pub mod field;
pub mod robot;

const STEP: f32 = 0.01;
const MATCH_TIME: f32 = 150.0;
pub const PRINT: bool = false;

#[macro_export]
macro_rules! cprintln_if_print {
    ($x:expr, $($y:expr),*) => {
        if $crate::PRINT {
            color_print::cprintln!($x, $($y),*);
        }
    };
}

pub struct Alliance {
    pub a: Robot,
    pub b: Robot,
    pub c: Robot,
}

impl Alliance {
    pub fn tick(&mut self, t: f32, field: &Field, rng: &mut impl Rng) -> AllianceActions {
        let a_message = self.a.tick(rng, field, (&self.b, &self.c));
        let b_message = self.b.tick(rng, field, (&self.a, &self.c));
        let c_message = self.c.tick(rng, field, (&self.a, &self.b));
        AllianceActions::new(t, a_message, b_message, c_message)
    }
}

pub fn run_match(mut alliance: Alliance, mut rng: impl Rng) -> Field {
    (0..)
        .map(|x| x as f32 * STEP)
        .take_while(|t| *t <= MATCH_TIME)
        .fold(Field::default(), |field, t| {
            let actions = alliance.tick(t, &field, &mut rng);
            field.apply(actions)
        })
}
