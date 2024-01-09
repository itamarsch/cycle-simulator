use cycle_simulation::{
    robot::{Robot, RobotConfig},
    run_match, Alliance,
};
use rand::prelude::*;

fn main() {
    let default: RobotConfig =
        serde_json::from_str(include_str!("../default_config.json")).expect("File is valid json");

    let matches = (10_000u32..10_001u32)
        .map(|i| {
            let mut rng = rand::rngs::StdRng::seed_from_u64(i as u64);
            let alliance = Alliance {
                a: Robot::new(default.clone(), &mut rng),
                b: Robot::new(default.clone(), &mut rng),
                c: Robot::new(default.clone(), &mut rng),
            };
            let final_field = run_match(alliance, rng, false);
            final_field.get_score()
        })
        .collect::<Vec<_>>();
    println!(
        "AVG score: {}, AVG Game pieces played: {}, AVG climbs points: {}",
        avg(matches
            .iter()
            .map(|score_summarization| score_summarization.game_piece_points)),
        avg(matches
            .iter()
            .map(|score_summarization| score_summarization.game_pieces_played)),
        avg(matches
            .iter()
            .map(|score_summarization| score_summarization.stage_points))
    );
}

fn avg(s: impl ExactSizeIterator<Item = i32>) -> f32 {
    let len = s.len() as f32;
    s.sum::<i32>() as f32 / len
}
