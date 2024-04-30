#[derive(PartialEq, Clone, Debug, Copy)]
pub enum SubDeck {Main, Extra , Side}

#[derive(Clone, Debug)]
pub struct Card{
    id: i64,
    name: String,
    card_type: String,
    desc: String,
    atk: i64,
    def: i64,
    level: i64,
    race: String,
    attribute: String
}

impl Card{
    pub fn new(id: i64, name: String, card_type: String, desc: String,
            atk: i64, def: i64, level: i64, race: String, attribute: String) -> Card {
        return Card{id: id, 
                    name: name,
                    card_type: card_type,
                    desc: desc,
                    atk: atk,
                    def: def,
                    level: level,
                    race: race,
                    attribute: attribute
                }
    }

    pub fn get_name(&self) -> String {
        return self.name.clone();
    }

    pub fn get_id(&self) -> i64 {
        return self.id;
    }
}

#[derive(Clone, Debug)]
pub struct Deck {
    pub main: Vec<Card>,
    pub extra: Vec<Card>,
    pub side: Vec<Card>
}

impl Deck {
    pub fn new() -> Deck {
        return Deck{main: vec![],
                    extra: vec![],
                    side: vec![]
                    };
    }

    pub fn add_card(&mut self, card: Card, &subdeck: &SubDeck) {
        if subdeck == SubDeck::Main {
            self.main.push(card);
        } 
        else if subdeck == SubDeck::Extra {
            self.extra.push(card);
        }
        else if subdeck == SubDeck::Side {
            self.side.push(card);
        }
    }
}

#[derive(Clone, Debug)]
pub struct CardSimple {
    id: i64,
    name: String,
}

impl CardSimple{
    pub fn new(id: i64, name: String) -> CardSimple {
        return CardSimple{id: id, 
                        name: name};
    }

    pub fn get_name(&self) -> String {
        return self.name.clone();
    }

    pub fn get_id(&self) -> i64 {
        return self.id;
    }
}

#[derive(Clone, Debug)]
pub struct DeckSimple {
    pub main: Vec<CardSimple>,
    pub extra: Vec<CardSimple>,
    pub side: Vec<CardSimple>
}

impl DeckSimple {
    pub fn new() -> DeckSimple {
        return DeckSimple{main: vec![],
                        extra: vec![],
                        side: vec![]
                        };
    }

    pub fn add_card(&mut self, card: CardSimple, &subdeck: &SubDeck) {
        if subdeck == SubDeck::Main {
            self.main.push(card);
        } 
        else if subdeck == SubDeck::Extra {
            self.extra.push(card);
        }
        else if subdeck == SubDeck::Side {
            self.side.push(card);
        }
    }
    
}

