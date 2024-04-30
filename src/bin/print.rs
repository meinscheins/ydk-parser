use reqwest::Result;
use ydk_parser::load_deck_list;

#[tokio::main]
async fn main() -> Result<()> {
    // update_json().await?;
    // let mut db: Database = Database::new();
    // db.load_from_json()?;
    // db.print();
    let deck = load_deck_list("yubel.ydk").await.expect("Cannot load deck.");
    println!("## Main ##");
    for card in deck.main {
        println!("{}", card.get_name());
    }
    println!("## Extra ##");
    for card in deck.extra {
        println!("{}", card.get_name());
    }
    println!("## Side ##");
    for card in deck.side {
        println!("{}", card.get_name());
    }
    Ok(())
}
