use std::io;
mod dictionary_of_symbols;// Словарь всех символов, а так-же, их весов

fn main(){
    let mut input = String::new();

    loop {
        println!("Введите ваш запрос:");

        io::stdin().read_line(&mut input).expect("Ошибка чтения");
        let trimmed_input = "Ввод пользователя: ".to_owned() + input.trim(); //Добавление метаметки


    }
}