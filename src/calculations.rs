use crate::cards::{Card, Hand};
use crate::game_state::{GameState, ManaTypes, sum_mana_types, max_mana_types, black_mana, mana_total};
use crate::card_characteristics::{add_mana_always, add_mana_as_land, is_land, becomes_land_free};


fn can_make_black(_:&Hand, game_state:&GameState) -> bool{
    game_state.gb_mana > 0 || game_state.lanterns_in_yard > 0
}

fn can_make_black_with_lantern(hand:&Hand, _:&GameState) -> bool{
    hand.iter().any(|card| card == &Card::JackOLantern)
}

pub fn basic_forest_in_deck_count(hand:&Hand, game_state:&GameState) -> i8{
    let forest_count = 1;

    forest_count - game_state.g_mana - hand.iter().filter(|card| card == &&Card::Forest).count() as i8
}

pub fn haunted_mire_in_deck_count(hand:&Hand, game_state:&GameState) -> i8{
    let haunted_mire_count = 3;

    haunted_mire_count - game_state.gb_mana - hand.iter().filter(|card| card == &&Card::HauntedMire).count() as i8
}

pub fn mana_this_turn(hand:&Hand, game_state:&GameState) -> ManaTypes {
    let lands_in_hand:bool = hand.iter().any(is_land);
    let deck_contains_forest:bool = basic_forest_in_deck_count(hand, game_state) > 0;

    let always_mana = hand.iter().fold((0,0,0,0), |mt, card| sum_mana_types(mt, add_mana_always(card, game_state)));

    let land_mana = hand.iter().fold((0,0,0,0), |mt, card| max_mana_types(mt, add_mana_as_land(card, game_state, lands_in_hand, deck_contains_forest)));

    let in_play_mana = (game_state.gb_mana, 0, game_state.g_mana, game_state.tinder_walls*2);

    sum_mana_types(sum_mana_types(always_mana, land_mana),in_play_mana)

}

pub fn spy_this_turn(hand:&Hand, game_state:&GameState) -> bool{
    let extra_mana: i8 = if can_make_black(hand, game_state) {0} else {
        if game_state.lantern_in_play > 0 {1} else {
            if can_make_black_with_lantern(hand, game_state) {2} else {return false}
        }
    };

    if hand.iter().any(|card| card == &Card::BalustradeSpy) {
        let mana = mana_this_turn(hand, game_state);
        if black_mana(mana) >= 1 && mana_total(mana) >= 6 + extra_mana {return true}
    }
    else if hand.iter().any(|card| card == &Card::DimirHouseGuard) {
        let mana = mana_this_turn(hand, game_state);
        if black_mana(mana) >= 3 && mana_total(mana) >= 9 + extra_mana {return true}
    }

    false
}

pub fn fatty_this_turn(hand:&Hand, game_state:&GameState) -> bool {
    if !game_state.fatty_in_yard {return false};

    let extra_mana: i8 = if can_make_black(hand, game_state) {0} else {
        if can_make_black_with_lantern(hand, game_state) {2} else {return false}
    };

    if hand.iter().any(|card| card == &Card::Exhume) {
        let mana = mana_this_turn(hand, game_state);
        if black_mana(mana) >= 1 && mana_total(mana) >= 2 + extra_mana {return true}
    }
    else if hand.iter().any(|card| card == &Card::DreadReturn) {
        let mana = mana_this_turn(hand, game_state);
        if black_mana(mana) >= 2 && mana_total(mana) >= 4 + extra_mana {return true}
    }

    false
}

pub fn spy_this_turn_mid_turn(hand:&Hand, game_state:&GameState, mana_used:i8) -> bool{
    let extra_mana: i8 = if can_make_black(hand, game_state) {0} else {
        if game_state.lantern_in_play > 0 {1} else {
            if can_make_black_with_lantern(hand, game_state) {2} else {return false}
        }
    } + mana_used;

    if hand.iter().any(|card| card == &Card::BalustradeSpy) {
        let mana = mana_this_turn(hand, game_state);
        if black_mana(mana) >= 1 && mana_total(mana) >= 6 + extra_mana {return true}
    }
    else if hand.iter().any(|card| card == &Card::DimirHouseGuard) {
        let mana = mana_this_turn(hand, game_state);
        if black_mana(mana) >= 3 && mana_total(mana) >= 9 + extra_mana {return true}
    }

    false
}

pub fn fatty_this_turn_mid_turn(hand:&Hand, game_state:&GameState, mana_used:i8) -> bool {
    if !game_state.fatty_in_yard {return false};

    let extra_mana: i8 = if can_make_black(hand, game_state) {0} else {
        if can_make_black_with_lantern(hand, game_state) {2} else {return false}
    } + mana_used;

    if hand.iter().any(|card| card == &Card::Exhume) {
        let mana = mana_this_turn(hand, game_state);
        if black_mana(mana) >= 1 && mana_total(mana) >= 2 + extra_mana {return true}
    }
    else if hand.iter().any(|card| card == &Card::DreadReturn) {
        let mana = mana_this_turn(hand, game_state);
        if black_mana(mana) >= 2 && mana_total(mana) >= 4 + extra_mana {return true}
    }

    false
}

pub fn contains_initial_mana_sources(hand:&Hand) -> bool{
    hand.iter().any(becomes_land_free)
}