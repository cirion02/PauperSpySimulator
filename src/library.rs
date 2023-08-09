use crate::cards::{Card,Hand};
use rand::thread_rng;
use rand::seq::SliceRandom;
use crate::card_characteristics::is_land;

pub type Deck = Vec<Card>;

pub fn draw(hand:&mut Hand, deck:&mut Deck){
    match deck.pop() {
        Some(card) => hand.push(card),
        None => panic!("Draw from an empty library")
    }
}

pub fn remove_card_from_deck(find_card:&Card, deck:&mut Deck){
    let index = match deck.iter().position(|card| card==find_card) {
        Some(i) => i,
        None => panic!("Trying to search for a card this is not in the library")
    };

    deck.remove(index);

    shuffle(deck);
}

pub fn remove_multiple_cards(find_cards:&Vec<Card>, deck:&mut Deck){
    find_cards.iter().for_each(|find_card| {
        let index = match deck.iter().position(|card| card==find_card) {
            Some(i) => i,
            None => panic!("Trying to search for a card this is not in the library")
        };

        deck.remove(index);
    });

    shuffle(deck);
}

fn shuffle(deck:&mut Deck){
    deck.shuffle(&mut thread_rng());
}

pub fn draw_land(hand:&mut Hand, deck:&mut Deck){
    if !deck.iter().any(is_land) {panic!("Trying to get a land from a deck that does not contain any")}
    match deck.pop() {
        Some(card) => if is_land(&card) {
            hand.push(card)
        } else {
            deck.insert(0, card);
            draw_land(hand, deck);
        },
        None => panic!("Draw from an empty library")
    }
}

pub fn draw_non_land(hand:&mut Hand, deck:&mut Deck){
    if !deck.iter().any(|x| !is_land(x)) {panic!("Trying to get a nonland from a deck that does not contain any")}
    match deck.pop() {
        Some(card) => if !is_land(&card) {
            hand.push(card)
        } else {
            deck.insert(0, card);
            draw_land(hand, deck);
        },
        None => panic!("Draw from an empty library")
    }
}

const DECK_LIST:[(Card, i8); 18] = [
    (Card::TinderWall, 4),
    (Card::BalustradeSpy, 4),
    (Card::DimirHouseGuard, 4),
    (Card::StreetWraith, 4),
    (Card::GenerousEnt, 4),
    (Card::TrollOfKhazadDum, 4),
    (Card::LotlethGiant, 2),
    (Card::AbundantHarvest, 4),
    (Card::Exhume, 4),
    (Card::LandGrant, 4),
    (Card::AcornHarvest, 2),
    (Card::DreadReturn, 4),
    (Card::DarkRitual, 4),
    (Card::SongsOfTheDamned, 4),
    (Card::ConjurersBauble, 2),
    (Card::JackOLantern, 2),
    (Card::Forest, 1),
    (Card::HauntedMire, 3)
];

pub fn starting_library() -> Deck{
    let mut deck:Deck = Deck::new();

    DECK_LIST.iter().for_each(|(card, count)| for _ in 0..*count {deck.push(card.to_owned())});

    if deck.len() != 60 {panic!("Starting deck is not 60 cards")};

    shuffle(&mut deck);

    deck
}