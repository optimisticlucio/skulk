use serde::Deserialize;

#[derive(Deserialize)]
pub struct Card {
    /// Which character this card is associated with.
    #[serde(rename = "alliance")]
    pub association: CardCharacterAssociation,
    /// How much a given card costs. None if it's a card that can't be bought, such as a starting deck card.
    pub cost: Option<u8>, // The cost should't go above 20, nor go into the negatives.
    pub name: String,
    pub description: String,
    #[serde(rename = "img")]
    pub art_filename: Option<String>,
    #[serde(rename = "count")]
    pub amount_in_deck: u8, // If a card is showing up more than 255 times in the deck, get help.
}


/// Part of the Card struct. Represents which character this card is associated with.
#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CardCharacterAssociation {
    /// Part of the Town deck.
    Neutral, 
    /// The Mothman
    Mothman, 
    /// The Flatwoods Monster
    Flatwoods, 
    /// The Loveland Frogman
    Loveland, 
    /// The Wendigo
    Wendigo, 
}

#[derive(Deserialize)]
pub struct Character {
    pub name: String,
    #[serde(rename = "img")]
    pub art_filename: Option<String>,
    /// Something the given character does during setup.
    pub modifier: Option<String>,
    #[serde(rename = "ability-cost")]
    pub ability_cost: u8,
    #[serde(rename = "ability")]
    pub ability_description: String,
}