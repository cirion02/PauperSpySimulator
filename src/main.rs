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

    test();
}

fn test() {
    let sim_count = 1000000;

    let start_time = Instant::now();

    let sim = || game_sim::sim_game_random_hand(true);

    let sum_results = (0..sim_count).map(|_| sim()).fold((0,0), |(a1,a2), (b1,b2)| (a1+b1 as i32,a2+b2 as i32));

    let results = (sum_results.0 as f32 / sim_count as f32, sum_results.1 as f32 / sim_count as f32);

    let elapsed_time = start_time.elapsed();
    println!("Elapsed time: {:?}", elapsed_time);

    println!("Mulling: {:?}", results);
}

fn test_muls() {
    let sim_count = 100000;

    let start_time = Instant::now();

    let sim = || game_sim::sim_game_with_milligan(6, true);

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
        cards::Card::DarkRitual,
        cards::Card::HauntedMire,
    ], true);

    let sum_results = (0..sim_count).map(|_| sim()).fold((0,0), |(a1,a2), (b1,b2)| (a1+b1 as i32,a2+b2 as i32));

    let results = (sum_results.0 as f32 / sim_count as f32, sum_results.1 as f32 / sim_count as f32);

    let elapsed_time = start_time.elapsed();
    println!("Elapsed time: {:?}", elapsed_time);

    println!("Keeping {:?}", results);
}

fn test_hand() {
    let sim_count_main = 50;
    let sim_count_per_hand = 1000;

    let spy_turn_to_beat: f32 = 6.1835017;
    let fatty_turn_to_beat: f32 = 4.890712;

    let start_time = Instant::now();


    let mut sum_results:(f32, f32) = (0.0,0.0);

    for _ in 0..sim_count_main {
        let (starting_hand, _) = game_sim::create_mull_hand_deck(0);

        let sim = || game_sim::sim_game_opening_hand(starting_hand.to_owned(),  true);

        let sum_results_inner = (0..sim_count_per_hand).map(|_| sim()).fold((0,0), |(a1,a2), (b1,b2)| (a1+b1 as i32,a2+b2 as i32));

        let results_inner = (sum_results_inner.0 as f32 / sim_count_per_hand as f32, sum_results_inner.1 as f32 / sim_count_per_hand as f32);

        if results_inner.0 <= spy_turn_to_beat{
            sum_results.0 += results_inner.0;
            sum_results.1 += results_inner.1;
        }
        else {

            sum_results.0 += spy_turn_to_beat;
            sum_results.1 += fatty_turn_to_beat;
            print!("Mulled Hand: ");
            starting_hand.iter().for_each(|card| print!("{}, ", card));
            println!("{:?}", results_inner);
        }

    }

    let results = (sum_results.0 as f32 / sim_count_main as f32, sum_results.1 as f32 / sim_count_main as f32);

    let elapsed_time = start_time.elapsed();
    println!("Elapsed time: {:?}", elapsed_time);

    println!("Mulling: {:?}", results);
}