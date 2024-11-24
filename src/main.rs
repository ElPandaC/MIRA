use std::io;
fn main() {
    loop{
        let mut choices = String::new();
        println!("Выберите действие
        1. Выбрать символы для работы
        2. Парсить исторические данные
        3. Нормализовать данные
        4. Обучить модель для предсказаний
        5. Предсказать следующие свечи
        6. Выход");

        io::stdin().read_line(&mut choices).expect("Ошибка чтения строки");

        match choices.trim() {
            "1" => select_symbols(),
            "2" => parse_historical_data(),
            "3" => normalize_data(),
            "4" => train_model_predictions(),
            "5" => predict_future_prices(),
            "6" => break,
            _ => println!("Некорректный ввод, попробуйте снова."),
        }
    }


}
fn select_symbols(){
    //TODO Выбор символов для работы
}

fn parse_historical_data(){
    //TODO Parse historical data from https://public.bybit.com/trading/ to sqlite
}

fn parse_to_realtime_data(){
    //TODO Parse real-time data from CCXT to sqlite
}

fn normalize_data(){
    //TODO Normalize data to standardize column names and types
}

fn train_model_predictions(){
    //TODO Train model using historical data and make predictions
}

fn predict_future_prices(){
    //TODO Use trained model to predict future prices
}

