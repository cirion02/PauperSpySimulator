use std::cmp::max;

pub struct GameState {
    pub g_mana:i8,
    pub gb_mana:i8,
    pub tinder_walls:i8,
    pub fatty_in_yard:bool,
    pub creatures_in_yard:i8,
    pub lantern_in_play:i8,
    pub lanterns_in_yard:i8,
    pub turn_count:i8,
}

pub fn starting_game_state() -> GameState {
    GameState { 
        g_mana: 0, 
        gb_mana: 0, 
        tinder_walls: 0, 
        fatty_in_yard: false, 
        creatures_in_yard: 0, 
        lantern_in_play: 0,
        lanterns_in_yard: 0, 
        turn_count: 0
    }
}

// Any, Black, Green, Other
pub type ManaTypes = (i8,i8,i8,i8);

pub fn sum_mana_types(m1:ManaTypes, m2:ManaTypes) -> ManaTypes{
    (m1.0 + m2.0, m1.1 + m2.1, m1.2 + m2.2, m1.3 + m2.3)
}

pub fn max_mana_types(m1:ManaTypes, m2:ManaTypes) -> ManaTypes{
    (max(m1.0, m2.0), max(m1.1, m2.1),max(m1.2, m2.2),max(m1.3, m2.3))
}

pub fn black_mana(mana:ManaTypes) -> i8{
    mana.0 + mana.1
}

pub fn mana_total(mana:ManaTypes) -> i8{
    mana.0 + mana.1 + mana.2 + mana.3
}