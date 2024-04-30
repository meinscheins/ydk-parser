use reqwest::Result;
use serde_json::Value;
use std::fs;
use crate::CardSimple;

const DATABASE_URL: &str = "https://db.ygoprodeck.com/api/v7/cardinfo.php";

pub struct Database {
    cards: Vec<CardSimple>
}

impl Database {
    pub fn new() -> Database {
        let db: Database = Database{cards: vec![],};
        return db;
    }

    //populize the database card list with card information from json
    pub fn load_from_json(&mut self) -> Result<()> {
        //load content from file
        let content: String = fs::read_to_string("db.json")
                                .expect("File read error"); 
        let json: Value = serde_json::from_str(content.as_str()).unwrap();
        //iterate over json and add name and id of card to database
        if let Some(card_array) = json["data"].as_array(){
            for card in card_array.iter() {
                if let Some(name) = card["name"].as_str() {
                    if let Some(id) = card["id"].as_i64() {
                        let entry: CardSimple = CardSimple::new(id, name.to_string());
                        self.cards.push(entry);
                    }
                }
            }
        }
        Ok(())
        
    }

    //print data base for debug purposes
    pub fn print(&self) {
        for card in self.cards.iter() {
            println!("{0} {1}", card.get_name(), card.get_id());
        }
    }
}

//load the data base with cards from ygoprodeck.com and write it to json
pub async fn update_json() -> Result<()> {
    let req:String = reqwest::get(DATABASE_URL)
    .await?
    .text()
    .await?;
    fs::write("db.json", req).expect("Unable to write file");
    Ok(())
}
