mod data_collections;
mod predict_system;

use std::io;
fn main() {
    loop{
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
fn select_symbols(){
    //TODO Выбор символов для работы
}


