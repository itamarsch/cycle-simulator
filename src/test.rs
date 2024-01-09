use cycle_simulation::{
    robot::{Robot, RobotConfig, ScoringStrategy},
    run_match, Alliance,
};
use rand::SeedableRng;

fn main() {
    let default: RobotConfig =
        serde_json::from_str(include_str!("../default_config.json")).expect("File is valid json");
    let mut rng = rand::rngs::StdRng::seed_from_u64(1690);
    let alliance = Alliance {
        a: Robot::new(ScoringStrategy::Amp, default.clone(), "A", &mut rng),
        b: Robot::new(
            ScoringStrategy::SpeakerAndAmp,
            default.clone(),
            "SA2",
            &mut rng,
        ),
        c: Robot::new(
            ScoringStrategy::SpeakerAndAmp,
            default.clone(),
            "SA1",
            &mut rng,
        ),
    };
    let final_field = run_match(alliance, rng, true);
    let score_summarzation = final_field.get_score();
    println!("{:?}", score_summarzation);
}
