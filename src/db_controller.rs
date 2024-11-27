use rusqlite::{params, Connection, Result};
use std::fs;
use std::time::{Duration, SystemTime};
use chrono::{DateTime, Timelike, Utc};

pub struct Candles {
    pub id: i128,          // ID
    pub timeframe: i64,    // Timeframe
    pub days: String,      // День недели
    pub hours: u8,         // Час
    pub minutes: u8,       // Минуты
    pub open: f64,         // Open
    pub close: f64,        // Close
    pub high: f64,         // High
    pub low: f64,          // Low
    pub volume: f64,       // Volume
}

impl Candles {
    pub fn new(id: i128, timeframe: i64, days: String, hours: u8, minutes: u8, open: f64, close: f64, high: f64, low: f64, volume: f64) -> Self {
        Candles {
            id,
            timeframe,
            days: String::from(days),
            hours,
            minutes,
            open,
            close,
            high,
            low,
            volume,
        }
    }

    pub fn create_table(conn: &Connection, table_name: &str) -> Result<()> {
        conn.execute(
            &format!(
                "CREATE TABLE IF NOT EXISTS {} (
                id INTEGER PRIMARY KEY,
                timeframe INTEGER NOT NULL UNIQUE,
                days TEXT NOT NULL,
                hours INTEGER NOT NULL,
                minutes INTEGER NOT NULL,
                open REAL NOT NULL,
                close REAL NOT NULL,
                high REAL NOT NULL,
                low REAL NOT NULL,
                volume REAL NOT NULL
            )",
                table_name
            ),
            [],
        )?;
        conn.execute(
            &format!("CREATE INDEX IF NOT EXISTS idx_{}_timeframe ON {}(timeframe)", table_name, table_name),
            [],
        )?;
        Ok(())
    }

    pub fn insert_data(conn: &Connection, table_name: &str, timeframe: i64, days: String, hours: u8, minutes: u8, open: f64, close: f64, high: f64, low: f64, volume: f64) -> Result<usize> {
        conn.execute(
            &format!(
                "INSERT INTO {} (timeframe, days, hours, minutes, open, close, high, low, volume) VALUES (?,?,?,?,?,?,?,?,?)",
                table_name
            ),
            params![timeframe, days, hours, minutes, open, close, high, low, volume],
        )
    }
}

pub fn create_table_candles(table_name: &str, symbols: String) -> Result<()> {
    let db_path = format!("storage/db/clear/{}.db", symbols);
    if let Some(parent_dir) = std::path::Path::new(&db_path).parent() {
        fs::create_dir_all(parent_dir).expect("Ошибка создания директории");
    }
    let conn = Connection::open(&db_path)?;
    Candles::create_table(&conn, table_name)?;
   // println!("База данных создана успешно");
    Ok(())
}

pub fn insert_data_to_candles(symbols: String, table_name: &str, candles: &Vec<(i64, f64, f64, f64, f64, f64)>) -> Result<()> {
    let db_path = format!("storage/db/clear/{}.db", symbols);
    let mut conn = Connection::open(db_path)?;
    let transaction = conn.transaction()?;

    for &(timestamp, open, high, low, close, volume) in candles {
        let datetime: DateTime<Utc> = (SystemTime::UNIX_EPOCH + Duration::from_secs(timestamp as u64)).into();
        let days = datetime.format("%A").to_string();
        let hours = datetime.hour() as u8;
        let minutes = datetime.minute() as u8;

        transaction.execute(
            &format!(
                "INSERT INTO {} (timeframe, days, hours, minutes, open, close, high, low, volume) VALUES (?,?,?,?,?,?,?,?,?)",
                table_name
            ),
            params![timestamp, days, hours, minutes, open, close, high, low, volume],
        )?;
    }

    transaction.commit()?;
    Ok(())
}

pub struct SumbolsTable {
    id: i32,
    table_name: String,
    pub(crate) url: String,
    date: String,
    download: bool,
}

impl SumbolsTable {
    pub fn new(table_name: &str, id: i32, url: String, date: String, download: bool) -> Self {
        SumbolsTable {
            table_name: String::from(table_name),
            id,
            url: String::from(url),
            date: String::from(date),
            download,
        }
    }

    pub fn create_table(conn: &Connection, table_name: &str) -> Result<()> {
        conn.execute(
            &format!(
                "CREATE TABLE IF NOT EXISTS {} (
                    id    INTEGER PRIMARY KEY,
                    url  TEXT NOT NULL,
                    date TEXT NOT NULL UNIQUE,
                    download BOOLEAN NOT NULL
                )",
                table_name
            ),
            [],
        )?;
        conn.execute(
            &format!("CREATE INDEX IF NOT EXISTS idx_{}_date ON {}(date)", table_name, table_name),
            [],
        )?;
        Ok(())
    }

    pub fn insert_data(conn: &Connection, table_name: &str, url: &str, date: &str, download: bool) -> Result<usize> {
        let rows_affected = conn.execute(
            &format!(
                "INSERT OR IGNORE INTO {} (url, date, download) VALUES (?, ?, ?)",
                table_name
            ),
            params![url, date, download],
        )?;
        Ok(rows_affected)
    }

    pub fn select_tar_to_parse(conn: &Connection, table_name: &str, max_url: u16) -> Result<Vec<SumbolsTable>> {
        let mut stmt = conn.prepare(&format!(
            "SELECT id, url, date, download FROM {} WHERE download = ? ORDER BY date DESC LIMIT ?",
            table_name
        ))?;
        let mut rows = stmt.query(params![false, max_url as i64])?;

        let mut results = Vec::new();
        while let Some(row) = rows.next()? {
            let id: i32 = row.get(0)?;
            let url: String = row.get(1)?;
            let date: String = row.get(2)?;
            let download: bool = row.get(3)?;
            results.push(SumbolsTable::new(table_name, id, url, date, download));
        }
        Ok(results)
    }

    pub fn mark_as_downloaded(conn: &Connection, table_name: &str, url: &str, download: bool) -> Result<()> {
        conn.execute(
            &format!("UPDATE {} SET download = ? WHERE url = ?", table_name),
            params![download, url],
        )?;
        Ok(())
    }
}

pub fn create_table_hd(table_name: &str) -> Result<()> {
    let db_path = "storage/db/raw/symbols.db";
    if let Some(parent_dir) = std::path::Path::new(db_path).parent() {
        fs::create_dir_all(parent_dir).expect("Ошибка создания директории");
    }
    let conn = Connection::open(db_path)?;
    SumbolsTable::create_table(&conn, table_name)?;
    //println!("База данных создана успешно");
    Ok(())
}

pub fn insert_data_to_sumbols(table_name: &str, url: &str, date: &str) -> Result<usize> {
    let db_path = "storage/db/raw/symbols.db";
    let conn = Connection::open(db_path)?;
    let rows_affected = SumbolsTable::insert_data(&conn, table_name, url, date, false)?;
    Ok(rows_affected)
}

pub fn select_tar_to_parse(table_name: &str, max_url: u16) -> Result<Vec<SumbolsTable>> {
    let db_path = "storage/db/raw/symbols.db";
    let conn = Connection::open(db_path)?;
    SumbolsTable::select_tar_to_parse(&conn, table_name, max_url)
}

pub fn mark_as_downloaded(table_name: &str, url: &str) -> Result<()> {
    let db_path = "storage/db/raw/symbols.db";
    let conn = Connection::open(db_path)?;
    let download = true;
    SumbolsTable::mark_as_downloaded(&conn, table_name, url, download)
}
