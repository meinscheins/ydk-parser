use reqwest::Error;
use serde_json::Value;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use crate::{CardSimple, DeckSimple, SubDeck};

pub async fn load_deck_list(file: &str) -> Result<DeckSimple, Box<dyn std::error::Error>> {
    
    let mut deck = DeckSimple::new() ;
    //open file and read line by line
    let file = File::open(file)?;
    let reader = BufReader::new(file);
    //set subdeck to main
    let mut current_subdeck = SubDeck::Main;
    for line in reader.lines() {
        let line_str = line.expect("Read line error.");
        //switch to extra/side deck 
        if line_str == "#extra" {
            current_subdeck = SubDeck::Extra;
        }     
        if line_str == "!side" {
            current_subdeck = SubDeck::Side;
        }
        if !(line_str.contains("#") || line_str.contains("!") ) {
            let id = line_str.parse::<i64>()?;
            //check if card is already in deck to avoid unnecessary API calls
            if let Some(card) = is_in_deck(id, &deck, &current_subdeck) {
                deck.add_card(card, &current_subdeck);
            } else {
                let name = get_card_name(id).await?;
                let card = CardSimple::new(id, name);
                deck.add_card(card, &current_subdeck);
            }          
        }
    }
    Ok((deck))
}

async fn get_card_name(id: i64) -> Result<String, Box<dyn std::error::Error>> {
    let query_url = format!("https://db.ygoprodeck.com/api/v7/cardinfo.php?id={}", id);
    //query API
    let req:String = reqwest::get(query_url)
     .await?
     .text()
     .await?;
    //extract name from json
    let json: Value = serde_json::from_str(&req).unwrap();
    if let Some(name) = json["data"][0]["name"].as_str() {
        return Ok(name.to_string());
    }
    Err("Cannot find card name.".into())
} 

pub fn is_in_deck(id: i64, deck: &DeckSimple, &subdeck: &SubDeck) -> Option<CardSimple> {
    if subdeck == SubDeck::Main {
        for card in &deck.main {
            if card.get_id() == id {
                return Some(card.clone());
            }
        }
    }
    if subdeck == SubDeck::Extra {
        for card in &deck.extra {
            if card.get_id() == id {
                return Some(card.clone());
            }
        }
    }
    if subdeck == SubDeck::Side {
        for card in &deck.side {
            if card.get_id() == id {
                return Some(card.clone());
            }
        }
    }

    
    
    return None;
}