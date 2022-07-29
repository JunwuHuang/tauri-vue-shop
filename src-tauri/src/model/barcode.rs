use rusqlite::{Connection, Result};
use dotenv::dotenv;
use std::env;
use std::io::Read;

pub struct Barcode {
    pub id: String,
    pub barcode: String,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
    pub enabled: bool,
}

pub struct BarcodeModel {
    pub conn: Connection,
}

impl BarcodeModel {
    pub fn new() -> Result<BarcodeModel> {
        dotenv().ok();
        let mut sql_file_name = String::from("/BarcodeTable.sql");
        let mut sql_path = format!("{}{}", env::var("MIGRATIONS_DIR").unwrap(), sql_file_name);

        let mut file_ref = std::fs::File::open(sql_path).unwrap();
        println!(file_ref);
        let mut sql = String::new();

        let conn = Connection::open(env::var("DB_PATH").unwrap())?;
        conn.execute(file_ref.read_to_string(&mut sql).unwrap());

        Ok(BarcodeModel { conn })
    }

    pub fn get_barcode_info(&self, barcode: &str) -> Result<Barcode> {
        let mut stmt = self.conn.prepare("SELECT * FROM barcode WHERE barcode = ?", [barcode]).unwrap();
        let todo = stmt.query();
        Ok(todo)
    }
}
