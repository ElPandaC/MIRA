use reqwest::blocking::Client;
use reqwest::header::USER_AGENT;
use crate::{AppConfig, db_controller};
use scraper::{Html, Selector};
use regex::Regex;
use indicatif::{ProgressBar, ProgressStyle};
use flate2::read::GzDecoder;
use std::io::BufReader;
use csv::ReaderBuilder;
use chrono::prelude::*;
use std::time::{Duration, SystemTime};
use std::path::Path;

pub fn parse_historical_data(config: &AppConfig) {
    let base_url = format!("{}{}/", config.bybit.base_url_trading, config.base_settings.base_symbol);
    //println!("Base URL: {}", base_url);
    parse_tar_url(base_url, &config.base_settings.base_symbol);
    parse_tar_to_db(&config.base_settings.base_symbol, config.base_settings.base_days_deph);
}

pub fn parse_to_realtime_data() {
    // TODO: Parse real-time data from CCXT to sqlite
}

fn fetch_and_parse_url(client: &Client, url: &str) -> Result<Html, reqwest::Error> {
    let response = client.get(url)
        .header(USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.3")
        .send()?;
    let body = response.text()?;
    Ok(Html::parse_document(&body))
}

pub fn parse_tar_url(url: String, symbol: &str) {
    let client = Client::new();
    if let Err(e) = db_controller::create_table_hd(symbol) {
        eprintln!("Ошибка при создании таблицы: {}", e);
    } else {
     //   println!("Таблица создана успешно");
    }

    match fetch_and_parse_url(&client, &url) {
        Ok(document) => {
            let selector = Selector::parse("a[href]").unwrap();
            let re = Regex::new(r"(DOGEUSDT)(\d{4}-\d{2}-\d{2})\.csv\.gz").unwrap();
            let elements: Vec<_> = document.select(&selector).collect();
            let pb = ProgressBar::new(elements.len() as u64);
            pb.set_style(ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
                .expect("Ошибка при установке стиля прогресс-бара")
                .progress_chars("##-"));

            let mut new_links_count = 0;
            println!("Начинаем процесс сбора ссылок");
            for element in elements {
                if let Some(link) = element.value().attr("href") {
                    if let Some(captures) = re.captures(link) {
                        let symbol = captures.get(1).map_or("", |m| m.as_str());
                        let date = captures.get(2).map_or("", |m| m.as_str());
                        let urls = format!("{}{}{}.csv.gz", url, symbol, date);
                        match db_controller::insert_data_to_sumbols(&symbol, &urls, &date) {
                            Ok(rows_affected) => {
                                if rows_affected > 0 {
                                    new_links_count += 1;
                                }
                            }
                            Err(e) => eprintln!("Ошибка при добавлении ссылки: {}", e),
                        }
                    }
                }
                pb.inc(1);
            }
            pb.finish_with_message("Все ссылки добавлены");
            println!("Все ссылки добавлены");
            println!("Добавлено новых ссылок: {}", new_links_count);
        },
        Err(e) => eprintln!("Ошибка при выполнении запроса: {}", e),
    }
}

fn download_and_parse_csv(client: &Client, url: &str) -> Result<Vec<(f64, f64, f64)>, reqwest::Error> {
    let response = client.get(url)
        .header(USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.3")
        .send()?;
    let body = response.bytes()?;
    let decoder = GzDecoder::new(BufReader::new(&body[..]));
    let mut rdr = ReaderBuilder::new().from_reader(decoder);
    let mut trades = Vec::new();
    for result in rdr.records() {
        match result {
            Ok(record) => {
                let timestamp: f64 = record[0].parse().unwrap_or_default();
                let price: f64 = record[4].parse().unwrap_or_default();
                let size: f64 = record[3].parse().unwrap_or_default();
                trades.push((timestamp, price, size));
            }
            Err(e) => {
                eprintln!("Ошибка при чтении CSV записи из файла {}: {}", url, e);
                break;
            }
        }
    }
    Ok(trades)
}

pub fn parse_tar_to_db(symbol: &str, max_count: u16) {
    let client = Client::new();
    let tablenames = ["t_1m", "t_5m", "t_15m", "t_30m"];
    for table in &tablenames {
        if let Err(e) = db_controller::create_table_candles(table, symbol.to_string()) {
            eprintln!("Ошибка при создании таблицы: {}", e);
        } else {
           // println!("Таблица создана успешно");
        }
    }

    let urls = db_controller::select_tar_to_parse(symbol, max_count).expect("Ошибка при выборке данных");
    let pb = ProgressBar::new(urls.len() as u64);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
        .expect("Ошибка при установке стиля прогресс-бара")
        .progress_chars("##-"));

    for url_record in urls {
        match download_and_parse_csv(&client, &url_record.url) {
            Ok(trades) => {
                if !trades.is_empty() {
                    let candles_1m = create_candles(&trades, Duration::new(60, 0));
                    let candles_5m = create_candles(&trades, Duration::new(300, 0));
                    let candles_15m = create_candles(&trades, Duration::new(900, 0));
                    let candles_30m = create_candles(&trades, Duration::new(1800, 0));

                    save_candles_to_db(&candles_1m, "t_1m", symbol);
                    save_candles_to_db(&candles_5m, "t_5m", symbol);
                    save_candles_to_db(&candles_15m, "t_15m", symbol);
                    save_candles_to_db(&candles_30m, "t_30m", symbol);

                    db_controller::mark_as_downloaded(symbol, &url_record.url).expect("Ошибка изменения параметров записи");
                } else {
                    eprintln!("Файл {} пуст или поврежден", url_record.url);
                }
            },
            Err(e) => eprintln!("Ошибка при выполнении запроса для URL {}: {}", url_record.url, e),
        }
        pb.inc(1);
    }
    pb.finish_with_message("Все данные обработаны");
    println!("Парсинг и обработка исторических данных завершена");
}

fn create_candles(trades: &Vec<(f64, f64, f64)>, interval: Duration) -> Vec<(i64, f64, f64, f64, f64, f64)> {
    let mut candles = Vec::new();
    let mut current_candle: Option<(i64, f64, f64, f64, f64, f64)> = None;

    for &(timestamp, price, size) in trades {
        let time = SystemTime::UNIX_EPOCH + Duration::from_secs_f64(timestamp);
        let datetime: DateTime<Utc> = time.into();
        let interval_start = datetime.timestamp() - (datetime.timestamp() % interval.as_secs() as i64);

        if let Some((start, open, high, low, close, volume)) = current_candle {
            if interval_start == start {
                current_candle = Some((start, open, high.max(price), low.min(price), price, volume + size));
            } else {
                candles.push((start, open, high, low, close, volume));
                current_candle = Some((interval_start, price, price, price, price, size));
            }
        } else {
            current_candle = Some((interval_start, price, price, price, price, size));
        }
    }

    if let Some(candle) = current_candle {
        candles.push(candle);
    }

    candles
}

fn save_candles_to_db(candles: &Vec<(i64, f64, f64, f64, f64, f64)>, table: &str, symbol: &str) {
    let db_path = format!("storage/db/clear/{}.db", symbol);
    if !Path::new(&db_path).exists() {
        eprintln!("Файл базы данных не существует: {}", db_path);
        return;
    }

    db_controller::insert_data_to_candles(symbol.parse().unwrap(), table, candles).expect("Ошибка при сохранении свечей в базу данных");
}
