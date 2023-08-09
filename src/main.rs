mod cards;
mod game_state;
mod calculations;
mod card_characteristics;
mod game_sim;
mod library;


use std::time::Instant;

use std::env;

fn main() {

    env::set_var("RUST_BACKTRACE", "1");

    let sim_count = 1;

    let start_time = Instant::now();

    let sim = || game_sim::sim_game_random_hand(true);

    let sum_results = (0..sim_count).map(|_| sim()).fold((0,0), |(a1,a2), (b1,b2)| (a1+b1 as i32,a2+b2 as i32));

    let results = (sum_results.0 as f32 / sim_count as f32, sum_results.1 as f32 / sim_count as f32);

    let elapsed_time = start_time.elapsed();
    println!("Elapsed time: {:?}", elapsed_time);

    println!("{:?}", results);
}