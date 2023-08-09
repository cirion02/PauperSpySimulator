use std::fmt;

#[derive(PartialEq, Debug, Clone)]
pub enum Card {
    BalustradeSpy,
    TinderWall,
    DimirHouseGuard,
    StreetWraith,
    GenerousEnt,
    TrollOfKhazadDum,
    LotlethGiant,
    AbundantHarvest,
    Exhume,
    LandGrant,
    AcornHarvest,
    DreadReturn,
    DarkRitual,
    SongsOfTheDamned,
    ConjurersBauble,
    JackOLantern,
    Forest,
    HauntedMire
}


pub type Hand = Vec<Card>;

pub fn remove_from_hand(find_card:&Card, hand:&mut Hand){
    let index = match hand.iter().position(|card| card==find_card) {
        Some(i) => i,
        None => panic!("Trying to search for a card this is not in the library")
    };

    hand.remove(index);
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            Card::BalustradeSpy => "Balustrade Spy",
            Card::TinderWall => "Tinder Wall",
            Card::DimirHouseGuard => "Dimir House Guard",
            Card::StreetWraith => "Street Wraith",
            Card::GenerousEnt => "Generous Ent",
            Card::TrollOfKhazadDum => "Troll Of Khazad Dum",
            Card::LotlethGiant => "Lotleth Giant",
            Card::AbundantHarvest => "Abundant Harvest",
            Card::Exhume => "Exhume",
            Card::LandGrant => "Land Grant",
            Card::AcornHarvest => "Acorn Harvest",
            Card::DreadReturn => "Dread Return",
            Card::DarkRitual => "Dark Ritual",
            Card::SongsOfTheDamned => "Songs Of TheDamned",
            Card::ConjurersBauble => "Conjurers Bauble",
            Card::JackOLantern => "Jack-O'-Lantern",
            Card::Forest => "Forest",
            Card::HauntedMire => "Haunted Mire"
        })
    }
}