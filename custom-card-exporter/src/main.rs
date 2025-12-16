use custom_card_exporter::structs::{Card, Character};

fn main() {
    const RELATIVE_PATH_TO_CARD_CSV: &str = "../csv/cards.csv";
    const RELATIVE_PATH_TO_CHARACTER_CSV: &str = "../csv/characters.csv";

    let cards: Vec<Card> = csv::Reader::from_path(RELATIVE_PATH_TO_CARD_CSV)
                    .unwrap_or_else(|err| panic!("Failed reading card CSV: {}", err))
                    .deserialize().collect::<Result<Vec<_>,csv::Error>>()
                    .unwrap_or_else(|err| panic!("Failed deserializing card CSV: {}", err));

    let characters: Vec<Character> = csv::Reader::from_path(RELATIVE_PATH_TO_CHARACTER_CSV)
                    .unwrap_or_else(|err| panic!("Failed reading character CSV: {}", err))
                    .deserialize().collect::<Result<Vec<_>,csv::Error>>()
                    .unwrap_or_else(|err| panic!("Failed deserializing character CSV: {}", err));

    for card in cards {
        println!("The card \"{}\" costs {} and appears {} times in the deck.", 
            card.name, 
            card.cost.map(|card| card.to_string()).unwrap_or("nothing".to_owned()),
            card.amount_in_deck);
    }
    
    println!("Done!");
}
