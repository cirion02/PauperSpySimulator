use crate::cards::Card;
use std::cmp::max;
use crate::game_state::{ManaTypes, GameState};

pub fn is_land(card:&Card) -> bool {
    match card {
        Card::Forest => true,
        Card::HauntedMire => true,
        _ => false
    }
}

pub fn becomes_land_free(card:&Card) -> bool {
    match card {
        Card::Forest => true,
        Card::HauntedMire => true,
        Card::LandGrant => true,
        _ => false
    }
}

pub fn becomes_land(card:&Card) -> bool {
    match card {
        Card::Forest => true,
        Card::HauntedMire => true,
        Card::LandGrant => true,
        Card::TrollOfKhazadDum => true,
        Card::GenerousEnt => true,
        _ => false
    }
}

pub fn is_ritual(card:&Card) -> bool {
    match card {
        Card::DarkRitual => true,
        Card::SongsOfTheDamned => true,
        Card::TinderWall => true,
        _ => false
    }
}

pub fn add_mana_always(card:&Card, game_state:&GameState) -> ManaTypes{
    match card {
        Card::TinderWall => (0,0,0,1),
        Card::DarkRitual => (0,2,0,0),
        Card::SongsOfTheDamned => (0,max(0,game_state.creatures_in_yard-1),0,0),
        _ => (0,0,0,0)
    }
}

pub fn add_mana_as_land(card:&Card, _:&GameState, hand_has_land:bool, deck_contains_forest:bool) -> ManaTypes{
    match card {
        Card::Forest => (0,0,1,0),
        Card::LandGrant => (0,0, if hand_has_land && deck_contains_forest {1} else {0}, 0),
        _ => (0,0,0,0)
    }
}