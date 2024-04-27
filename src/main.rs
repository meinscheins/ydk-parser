use reqwest::Result;
use ydk_parser::{update_json, Database};

#[tokio::main]
async fn main() -> Result<()> {
    update_json().await?;
    let mut db: Database = Database::new();
    db.load_from_json()?;
    db.print();
    Ok(())
}
