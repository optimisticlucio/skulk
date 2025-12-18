use std::{collections::HashMap, fs::File, path::PathBuf};
use resvg::{usvg, tiny_skia};

use custom_card_exporter::{adjust_template_to_association, implement_card_in_template, read_card_template, structs::{Card, Character, CharacterAssociation}};

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

    // Load up the basic card SVG.
    let card_template = read_card_template().unwrap_or_else(|err| panic!("Failed reading card SVG: {}", err));

    // Let's get the SVGs for every permutation with the various characters
    let mut association_name_to_template: HashMap<String, xmltree::Element> = HashMap::new();
    let mut character_associations: Vec<CharacterAssociation> = characters.iter().map(|character| character.association.clone()).collect();
    character_associations.push(CharacterAssociation::default()); // <- Town Association

    for association in character_associations {
        association_name_to_template.insert(association.name.clone(), 
            adjust_template_to_association(card_template.clone(), &association));
    }

    for card in cards {
        let cardname = card.name.clone();
        let implemented_card = implement_card_in_template(card_template.clone(), card);
        
        println!("Writing SVG into buffer...");

        // Convert the SVG into STR.
        let mut svg_write_buffer: Vec<u8> = Vec::new();
        implemented_card.write(&mut svg_write_buffer).expect("Writing into buffer failed");

        println!("Writing buffer into string...");

        // Buffer into string
        let svg_str = String::from_utf8(svg_write_buffer).expect("Invalid UTF-8");

        println!("Parsing into usvg tree..");

        // Parse into usvg tree
        let mut opt = usvg::Options::default();
        opt.resources_dir = Some(PathBuf::from("assets"));
        let tree = usvg::Tree::from_str(&svg_str, &opt).unwrap();


        // Render to pixmap
        let pixmap_size = tree.size().to_int_size();

        println!("Rendering into pixmap as {}X{}...", &pixmap_size.width(), &pixmap_size.height());
        
        let mut pixmap = tiny_skia::Pixmap::new(pixmap_size.width(), pixmap_size.height()).unwrap();

        resvg::render(&tree, usvg::Transform::default(), &mut pixmap.as_mut());

        println!("Exporting!");
        // Save or use the pixmap
        pixmap.save_png(format!("output/{}.png", cardname.to_ascii_lowercase())).unwrap();
    }

    println!("Done!");
}
