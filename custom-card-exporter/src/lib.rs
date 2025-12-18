use structs::CharacterAssociation;
use xmltree::{Element, XMLNode};
use std::path::PathBuf;
use std::fs::{self, File};
use std::error::Error;
use crate::structs::Card;

pub mod structs;

pub fn read_card_template() -> Result<xmltree::Element, Box<dyn Error>>{
    const RELATIVE_PATH_TO_ASSETS_FOLDER: &str = "assets";

    let card_template_path = PathBuf::from(RELATIVE_PATH_TO_ASSETS_FOLDER).join("card_template.svg");

    let card_template_file = File::open(card_template_path)?;

    Ok(xmltree::Element::parse(card_template_file)?)
}

pub fn adjust_template_to_association(mut card_template: xmltree::Element, character_association: &CharacterAssociation) -> xmltree::Element {
    const TEMPLATE_COLOR_TO_REPLACE: &str = "#75052A";

    println!("Creating template for {} association", &character_association.name);

    fn run_on_self_and_children (current_node: &mut xmltree::Element, character_association: &CharacterAssociation) {
        // Replace fill colors
        if let Some(fill_value) = current_node.attributes.get("fill") {
            if fill_value == TEMPLATE_COLOR_TO_REPLACE {
                current_node.attributes.insert("fill".to_string(), character_association.hex_color.clone());
            }
        }

        // Replace stroke colors
        if let Some(stroke_value) = current_node.attributes.get("stroke") {
            if stroke_value == TEMPLATE_COLOR_TO_REPLACE {
                current_node.attributes.insert("stroke".to_string(), character_association.hex_color.clone());
            }
        }

        
        for child_node in &mut current_node.children {
            if let Some(child_element) = child_node.as_mut_element() {
                run_on_self_and_children(child_element, character_association);
            }
        }

        // TODO: Change the Logo
    }

    run_on_self_and_children(&mut card_template, &character_association);

    card_template
}

pub fn implement_card_in_template(mut card_template: xmltree::Element, card: Card) -> xmltree::Element{
    println!("Implementing {} card", &card.name);
    

    let description_id_element = find_element_by_id_mut(&mut card_template, "Description_2").unwrap();

    println!("Gotten the parent element.");

    let description_element = description_id_element
                .get_mut_child("tspan").unwrap();

    println!("Gotten the child element");

    description_element.children.clear();
    description_element.children.push(XMLNode::Text(card.description));

    println!("Replaced the description");

    // TODO: Replace Name
    // TODO: Replace Image
    // TODO: Replace Cost

    card_template
}

fn find_element_by_id_mut<'a>(element: &'a mut Element, target_id: &str) -> Option<&'a mut Element> {
    if let Some(id) = element.attributes.get("id") {
        if id == target_id {
            return Some(element);
        }
    }

    for child in &mut element.children {
        if let XMLNode::Element(child_element) = child {
            if let Some(found) = find_element_by_id_mut(child_element, target_id) {
                return Some(found);
            }
        }
    }
    
    None
}