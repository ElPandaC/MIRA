use config::{Config, File, Environment};
use serde::Deserialize;
use std::io;

mod data_collections;
mod predict_system;

// Загрузка конфига
#[derive(Debug, Deserialize)]
struct AppConfig {
    base_url: BaseUrl,
    base_settings: BaseSettings,
}

#[derive(Debug, Deserialize)]
struct BaseUrl {
    bybit: String,
}

#[derive(Debug, Deserialize)]
struct BaseSettings {
    base_symbol: String,
    base_days_deph: u16,
}

impl AppConfig {
    fn from_env() -> Result<Self, config::ConfigError> {
        let builder = Config::builder()
            .add_source(File::with_name("config/config"))
            .add_source(Environment::with_prefix("APP"));
        let config = builder.build()?;
        config.try_deserialize()
    }
}

fn main() {
    match AppConfig::from_env() {
        Ok(config) => {
            loop {
                let mut choices = String::new();
                println!("Выберите действие:
            1: Парсить исторические данные
            2: Нормализовать данные
            3: Обучить модель для предсказаний
            4: Предсказать следующие свечи
            Q: Выход");

                io::stdin().read_line(&mut choices).expect("Ошибка чтения строки");

                match choices.trim() {
                    "1" => data_collections::parse_historical_data(),
                    "2" => predict_system::normalize_data(),
                    "3" => predict_system::train_model_predictions(),
                    "4" => predict_system::predict_future_prices(),
                    "q" => break,
                    _ => println!("Некорректный ввод, попробуйте снова."),
                }
            }
        }
        Err(e) => {
            eprintln!("Ошибка загрузки конфига: {}", e);
        }
    }
}

fn select_symbols() {
    // TODO Выбор символов для работы
}
