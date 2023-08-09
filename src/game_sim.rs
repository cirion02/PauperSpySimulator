use crate::cards::{Card, Hand, remove_from_hand};
use crate::library::{Deck, draw, remove_card_from_deck, remove_multiple_cards, starting_library, draw_land, draw_non_land};
use crate::game_state::{starting_game_state, mana_total, GameState};
use crate::calculations::{fatty_this_turn, spy_this_turn, mana_this_turn, basic_forest_in_deck_count, haunted_mire_in_deck_count, contains_initial_mana_sources};
use crate::card_characteristics::{becomes_land_free, is_land};

use std::cmp::min;

//Turn to spy, turn to spy/fatty
type SimResult = (i8, i8);

const LOG_PLAYS:bool = true;

fn log(text:&str){
    if !LOG_PLAYS {return}
    println!("{}", text)
}

fn log_hand(hand:&Hand){
    if !LOG_PLAYS {return}
    print!("Hand: ");
    hand.iter().for_each(|card| print!("{}, ", card));
    println!("");
}

fn log_turn(game_state:&GameState){
    if !LOG_PLAYS {return}
    println!("Turn: {}", game_state.turn_count)
}

pub fn sim_game_opening_hand(starting_hand:Hand, going_first:bool) -> SimResult{
    let mut starting_deck = starting_library();
    remove_multiple_cards(&starting_hand, &mut starting_deck);
    let mut mut_starting_hand = &mut starting_hand.to_owned();
    sim_game(&mut mut_starting_hand, &mut starting_deck, going_first)
}

pub fn sim_game_random_hand(going_first:bool) -> SimResult{
    let mut starting_deck:Deck = Deck::new();
    let mut starting_hand:Hand = Hand::new();
    let mut hand_size:i8 = 7;
    let mut done = false;

    while !done {
        starting_deck = starting_library();
        starting_hand = Hand::new();
        for _ in 0..hand_size {
            draw(&mut starting_hand, &mut starting_deck);
        };
        log_hand(&starting_hand);
        done = hand_size <= 4 || contains_initial_mana_sources(&starting_hand);
        hand_size -= 1;
        if !done {log("Took a milligan")};
    }
    sim_game(&mut starting_hand, &mut starting_deck, going_first)
}

fn sim_game(hand:&mut Hand, deck:&mut Deck, going_first:bool) -> SimResult{
    let mut game_state = starting_game_state();

    let mut result:SimResult = (127,127);

    if !going_first {draw(hand, deck);}

    loop {
        game_state.turn_count += 1;
        log_turn(&game_state);
        log_hand(hand);

        while hand.iter().any(|card| card == &Card::StreetWraith){
            remove_from_hand(&Card::StreetWraith, hand);
            draw(hand, deck);
            log("Cycle street wraith");
            log_hand(hand);
        }

        if spy_this_turn(&hand, &game_state) {
            log("Win with spy");
            result.0 = game_state.turn_count;
            result.1 = min(game_state.turn_count, result.1);
            break;
        }

        if fatty_this_turn(&hand, &game_state) {
            log("Reanimate a fatty");
            result.1 = min(game_state.turn_count, result.1);
        }

        let mut made_land_drop = false;

        let mana = mana_this_turn(hand, &game_state);
        let mut wants_mana = 6 - mana_total(mana) - hand.iter().filter(|card| becomes_land_free(&card.to_owned())).count() as i8;

        let mut static_mana = game_state.gb_mana + game_state.g_mana;

        let would_play_forest = !hand.contains(&Card::HauntedMire) && 
                                        hand.contains(&Card::Forest) && 
                                            (wants_mana <= 1 ||
                                                (static_mana == 0 || 
                                                    !(hand.contains(&Card::GenerousEnt) 
                                                        || hand.contains(&Card::TrollOfKhazadDum) 
                                                        || hand.contains(&Card::AbundantHarvest))));

        if would_play_forest {
            log("Play a forest");
            static_mana += 1;
            game_state.g_mana += 1;
            made_land_drop = true;
            wants_mana -= 1;
            remove_from_hand(&Card::Forest, hand);
        }

        let mut forests_in_deck = basic_forest_in_deck_count(hand, &game_state);
        let mut mires_in_deck = haunted_mire_in_deck_count(hand, &game_state);

        if !made_land_drop && mires_in_deck == 0 && forests_in_deck > 0 && hand.contains(&Card::LandGrant) && !hand.iter().any(is_land){
            log("Land Grant for forest and play it");
            remove_from_hand(&Card::LandGrant, hand);
            hand.push(Card::Forest);
            remove_card_from_deck(&Card::Forest, deck);
            forests_in_deck -= 1;
            static_mana += 1;
            game_state.g_mana += 1;
            made_land_drop = true;
            remove_from_hand(&Card::Forest, hand);
        }

        while static_mana > 0 {
            if wants_mana > 0 {
                if mires_in_deck > 0 && hand.contains(&Card::TrollOfKhazadDum){
                    log("Cycle Troll for Mire");
                    remove_from_hand(&Card::TrollOfKhazadDum, hand);
                    static_mana -= 1;
                    hand.push(Card::HauntedMire);
                    remove_card_from_deck(&Card::HauntedMire, deck);
                    mires_in_deck -= 1;
                    wants_mana -= 1;
                    game_state.fatty_in_yard = true;
                    game_state.creatures_in_yard += 1;
                    continue;
                }
                if mires_in_deck > 0 && hand.contains(&Card::GenerousEnt){
                    log("Cycle Ent for Mire");
                    remove_from_hand(&Card::GenerousEnt, hand);
                    static_mana -= 1;
                    hand.push(Card::HauntedMire);
                    remove_card_from_deck(&Card::HauntedMire, deck);
                    mires_in_deck -= 1;
                    wants_mana -= 1;
                    game_state.fatty_in_yard = true;
                    game_state.creatures_in_yard += 1;
                    continue;
                }
                if forests_in_deck > 0 && hand.contains(&Card::GenerousEnt){
                    log("Cycle Ent for Forest");
                    remove_from_hand(&Card::GenerousEnt, hand);
                    static_mana -= 1;
                    hand.push(Card::Forest);
                    remove_card_from_deck(&Card::Forest, deck);
                    forests_in_deck -= 1;
                    wants_mana -= 1;
                    game_state.fatty_in_yard = true;
                    game_state.creatures_in_yard += 1;
                    if !made_land_drop && !hand.contains(&Card::HauntedMire) {
                        static_mana += 1;
                        game_state.g_mana += 1;
                        made_land_drop = true;
                        remove_from_hand(&Card::Forest, hand);
                    }
                    continue;
                }
                if forests_in_deck + mires_in_deck > 0 && hand.contains(&Card::AbundantHarvest){
                    log("Harvest for a land");
                    remove_from_hand(&Card::AbundantHarvest, hand);
                    static_mana -= 1;
                    draw_land(hand, deck);
                    match hand.last() {
                        None => panic!("Drew a land and somehow has an empty hand."),
                        Some(Card::Forest) => forests_in_deck -= 1,
                        Some(Card::HauntedMire) => mires_in_deck -= 1,
                        _ => panic!("Drew a land which is somehow not mire or forest")
                    }
                    log_hand(hand);
                    wants_mana -= 1;
                    if !made_land_drop && !hand.contains(&Card::HauntedMire) {
                        log("Play forest");
                        static_mana += 1;
                        game_state.g_mana += 1;
                        made_land_drop = true;
                        remove_from_hand(&Card::Forest, hand);
                    }
                    continue;
                }
                if hand.contains(&Card::TinderWall){
                    log("Play Tinder Wall");
                    remove_from_hand(&Card::TinderWall, hand);
                    static_mana -= 1;
                    wants_mana -= 1;
                    game_state.creatures_in_yard += 1;
                    game_state.tinder_walls += 1;
                    continue;
                }
            }
            if hand.contains(&Card::DimirHouseGuard) && !hand.contains(&Card::BalustradeSpy) && static_mana > 2{
                log("Transmute dimir house guard");
                static_mana -= 3;
                remove_from_hand(&Card::DimirHouseGuard, hand);
                game_state.creatures_in_yard += 1;
                hand.push(Card::BalustradeSpy);
                remove_card_from_deck(&Card::BalustradeSpy, deck);
                continue;
            }
            if game_state.lantern_in_play > 0 {
                log("Crack Jack-o-lantern");
                static_mana -= 1;
                game_state.lantern_in_play -= 1;
                game_state.lanterns_in_yard += 1;
                draw(hand, deck);
                forests_in_deck = basic_forest_in_deck_count(hand, &game_state);
                mires_in_deck = haunted_mire_in_deck_count(hand, &game_state);
                log_hand(hand);
                continue;
            }
            if hand.contains(&Card::ConjurersBauble) {
                log("Play + Crack bauble");
                static_mana -= 1;
                remove_from_hand(&Card::ConjurersBauble, hand);
                draw(hand, deck);
                forests_in_deck = basic_forest_in_deck_count(hand, &game_state);
                mires_in_deck = haunted_mire_in_deck_count(hand, &game_state);
                log_hand(hand);
                continue;
            }
            if static_mana > 1 && hand.contains(&Card::JackOLantern) {
                log("Play + Crack Jack-o-lantern");
                static_mana -= 2;
                remove_from_hand(&Card::JackOLantern, hand);
                game_state.lanterns_in_yard += 1;
                draw(hand, deck);
                forests_in_deck = basic_forest_in_deck_count(hand, &game_state);
                mires_in_deck = haunted_mire_in_deck_count(hand, &game_state);
                log_hand(hand);
                continue;
            }
            if hand.contains(&Card::AbundantHarvest) {
                log("Play Harvest for a nonland");
                static_mana -= 1;
                remove_from_hand(&Card::AbundantHarvest, hand);
                draw_non_land(hand, deck);
                log_hand(hand);
                continue;
            }
            if hand.contains(&Card::JackOLantern) {
                log("Play jack-o-lantern");
                static_mana -= 1;
                remove_from_hand(&Card::JackOLantern, hand);
                game_state.lantern_in_play += 1;
                continue;
            }
            break;
        }

        if !made_land_drop && hand.contains(&Card::HauntedMire) {
            log("Play Haunted Mire");
            remove_from_hand(&Card::HauntedMire, hand);
            game_state.gb_mana += 1;
            made_land_drop = true;
        }

        if !made_land_drop && mires_in_deck > 0 && hand.contains(&Card::LandGrant) {
            log("Land Grant for Haunted Mire and play it");
            remove_from_hand(&Card::LandGrant, hand);
            remove_card_from_deck(&Card::HauntedMire, deck);
            game_state.gb_mana += 1;
            made_land_drop = true;
        }

        draw(hand, deck);
    }

    result
}