mod data_collections;
mod predicti_system;

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
            "2" => data_collections::parse_historical_data(),
            "3" => predicti_system::normalize_data(),
            "4" => predicti_system::train_model_predictions(),
            "5" => predicti_system::predict_future_prices(),
            "6" => break,
            _ => println!("Некорректный ввод, попробуйте снова."),
        }
    }


}
fn select_symbols(){
    //TODO Выбор символов для работы
}


