use cycle_simulation::{
    robot::{Robot, RobotConfig},
    run_match, Alliance,
};
use rand::SeedableRng;

fn main() {
    let a: RobotConfig =
        serde_json::from_str(include_str!("../a.json")).expect("File is valid json");
    let b: RobotConfig =
        serde_json::from_str(include_str!("../b.json")).expect("File is valid json");
    let c: RobotConfig =
        serde_json::from_str(include_str!("../c.json")).expect("File is valid json");
    let mut rng = rand::rngs::StdRng::seed_from_u64(1699);
    let alliance = Alliance {
        a: Robot::new(a.clone(), &mut rng),
        b: Robot::new(b.clone(), &mut rng),
        c: Robot::new(c.clone(), &mut rng),
    };
    let final_field = run_match(alliance, rng);
    let score_summarzation = final_field.get_score();
    println!("{:?}", score_summarzation);
}
