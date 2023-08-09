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

    test_muls();

    test_keep();
}

fn test_muls() {
    let sim_count = 100000;

    let start_time = Instant::now();

    let sim = || game_sim::sim_game_with_milligan(2, true);

    let sum_results = (0..sim_count).map(|_| sim()).fold((0,0), |(a1,a2), (b1,b2)| (a1+b1 as i32,a2+b2 as i32));

    let results = (sum_results.0 as f32 / sim_count as f32, sum_results.1 as f32 / sim_count as f32);

    let elapsed_time = start_time.elapsed();
    println!("Elapsed time: {:?}", elapsed_time);

    println!("Mulling: {:?}", results);
}

fn test_keep(){
    let sim_count = 100000;

    let start_time = Instant::now();

    let sim = || game_sim::sim_game_opening_hand(vec![
        cards::Card::StreetWraith,
        cards::Card::StreetWraith,
        cards::Card::StreetWraith,
        cards::Card::StreetWraith,
        cards::Card::Forest,
        cards::Card::LandGrant,
        cards::Card::DarkRitual,
    ], true);

    let sum_results = (0..sim_count).map(|_| sim()).fold((0,0), |(a1,a2), (b1,b2)| (a1+b1 as i32,a2+b2 as i32));

    let results = (sum_results.0 as f32 / sim_count as f32, sum_results.1 as f32 / sim_count as f32);

    let elapsed_time = start_time.elapsed();
    println!("Elapsed time: {:?}", elapsed_time);

    println!("Keeping {:?}", results);
}