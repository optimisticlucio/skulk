use structs::CharacterAssociation;
use xmltree::{Element, XMLNode};
use std::path::PathBuf;
use std::fs::{self, File};
use std::error::Error;
use crate::structs::Card;
use resvg::{usvg, tiny_skia};

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

        let mut attributes_to_print = current_node.attributes.clone();
        let _ = attributes_to_print.remove("d");

        //println!("Element: {}, Attributes: {:?}", current_node.name, attributes_to_print);
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

        // Replace stop-colors
        if let Some(stop_color_value) = current_node.attributes.get("stop-color") {
            if stop_color_value == TEMPLATE_COLOR_TO_REPLACE {
                current_node.attributes.insert("stop-color".to_string(), character_association.hex_color.clone());
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
    const DISTANCE_BETWEEN_ONE_LINE_TO_THE_NEXT: f64 = 16.0;

    println!("Implementing {} card", &card.name);

    // Replace description.
    let description_id_element = find_element_by_id_mut(&mut card_template, "Description_2").unwrap();

    let description_element = description_id_element
                .get_mut_child("tspan").unwrap();

    description_element.children.clear();
    description_element.children.push(XMLNode::Text(card.description));

    // Replace name
    let name_id_element = find_element_by_id_mut(&mut card_template, "Title").unwrap();

    let name_element = name_id_element
                .get_mut_child("tspan").unwrap();

    name_element.children.clear();
    name_element.children.push(XMLNode::Text(card.name));

    // Replace Cost

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

fn replace_str_of_id_in_element(&mut card_template: xmltree::Element, target_id: &str, replacement: &str) {
    let id_element = find_element_by_id_mut(&mut card_template, "Description_2").unwrap();

    let element = id_element
                .get_mut_child("tspan").unwrap();

    element.children.clear();
    element.children.push(XMLNode::Text(card.description));
}

/// Given a valid svg str, exports it to the output/ subfolder.
pub fn export_svg_from_str(file_name: &str, svg_str: &str) {
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
    pixmap.save_png(format!("output/{}.png", file_name)).unwrap();
}