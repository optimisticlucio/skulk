use serde::Deserialize;
use serde_with::with_prefix;

#[derive(Deserialize)]
pub struct Card {
    /// Which character this card is associated with.
    #[serde(rename = "alliance")]
    pub association: String,
    /// How much a given card costs. None if it's a card that can't be bought, such as a starting deck card.
    pub cost: Option<u8>, // The cost should't go above 20, nor go into the negatives.
    pub name: String,
    pub description: String,
    #[serde(rename = "img")]
    pub art_filename: Option<String>,
    #[serde(rename = "count")]
    pub amount_in_deck: u8, // If a card is showing up more than 255 times in the deck, get help.
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
    #[serde(flatten, with = "prefix_association")]
    pub association: CharacterAssociation,
}

with_prefix!(pub prefix_association "association_");

#[derive(Deserialize, Clone)]
pub struct CharacterAssociation {
    /// The name to reference in the decklist.
    pub name: String,
    /// Hex code of the color for cards of this association.
    pub hex_color: String,
    /// Filename of the icon for this association.
    pub icon: String,
}

impl Default for CharacterAssociation {
    fn default() -> Self {
        Self {
            name: "town".to_string(),
            hex_color: "#C8B898".to_string(),
            icon: "UNDEFINED".to_string()
        }
    }
}