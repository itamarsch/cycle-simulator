use cycle_simulation::{
    robot::{Robot, RobotConfig},
    run_match, Alliance,
};
use rand::prelude::*;
use rayon::prelude::*;

fn main() {
    let a: RobotConfig =
        serde_json::from_str(include_str!("../a.json")).expect("File is valid json");

    let b: RobotConfig =
        serde_json::from_str(include_str!("../b.json")).expect("File is valid json");
    let c: RobotConfig =
        serde_json::from_str(include_str!("../c.json")).expect("File is valid json");

    let matches = (10_000u32..20_001u32)
        .into_par_iter()
        .map(|i| {
            let mut rng = rand::rngs::StdRng::seed_from_u64(i as u64);
            let alliance = Alliance {
                a: Robot::new(a.clone(), &mut rng),
                b: Robot::new(b.clone(), &mut rng),
                c: Robot::new(c.clone(), &mut rng),
            };
            let final_field = run_match(alliance, rng);
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
