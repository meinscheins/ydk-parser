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