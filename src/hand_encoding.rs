use crate::cards::{Card,Hand};

const CARD_LIST:[Card;18] = [
    Card::BalustradeSpy,
    Card::TinderWall,
    Card::DimirHouseGuard,
    Card::StreetWraith,
    Card::GenerousEnt,
    Card::TrollOfKhazadDum,
    Card::LotlethGiant,
    Card::AbundantHarvest,
    Card::Exhume,
    Card::LandGrant,
    Card::AcornHarvest,
    Card::DreadReturn,
    Card::DarkRitual,
    Card::SongsOfTheDamned,
    Card::ConjurersBauble,
    Card::JackOLantern,
    Card::HauntedMire,
    Card::Forest,
];

pub fn encoded_to_hand(encoded:u64) -> Hand {
    let mut hand = Hand::new();
    let mut code = encoded.clone();

    let mut index = 0;

    while code != 0 {
        let count = code & 7;

        for _ in 0..count{
            hand.push(CARD_LIST[index].clone())
        }

        code = (code & (!7)) >> 3;

        index += 1;
    }

    hand
}