use std::io;
mod dictionary_of_symbols;// Словарь всех символов, а так-же, их весов
use crate::dictionary_of_symbols::AllChars;
mod NeuralNetwork;
///Запуск системы
pub fn run() {

    let mut input = String::new();
    let mut input_weight = dictionary_of_symbols::InputChars::new();
    let all_chars= AllChars::new();

    println!("Создаём нейронную сеть");
    let mut neural_network = NeuralNetwork::create_base();
    let (x_nn, y_nn,z_nn) = neural_network.get_position();
    println!("\nНейронная сеть с индексом: {} создана. \n\
    Координаты сети: x:{}, y:{}, z:{} ", neural_network.get_index(),x_nn,y_nn,z_nn);
    println!("\nНейрон создан");
    neural_network.list_neurons();
    println!("\nПромежуточный нейрон создан");
    neural_network.list_inrons();
    println!("\nСинапс создан");
    neural_network.list_sinapses();
    println!("\nПроверяю нейронную сеть");
    neural_network= NeuralNetwork::test_neural_network(neural_network,all_chars);


    loop {
        //println!("Введите ваш запрос:");

        //io::stdin().read_line(&mut input).expect("Ошибка чтения");
        //let trimmed_input = "Ввод пользователя: ".to_owned() + input.trim(); //Добавление метаметки


    }
}

