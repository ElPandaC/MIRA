mod Neuron;

use crate::dictionary_of_symbols::AllChars;
use crate::NeuralNetwork::Neuron::{Neurone, WeightNeurone};
mod Sinaps;
use crate::NeuralNetwork::Sinaps::Sinapses;
mod ImplicationNeuron;
use crate::NeuralNetwork::ImplicationNeuron::INeuron;

/// Структура координат
pub struct Coordinate {
    x: f64,
    y: f64,
    z: f64,
}

enum ChoiseNN{
    activate_neuron(f64, Coordinate),
    return_null_value
}

impl ChoiseNN {
    pub fn new(neural_network: &NeuralNetwork, input:f64, coordinate: Coordinate) -> ChoiseNN{
        if neural_network.weight.weight_neuron * input > neural_network.weight.weight_choice {
            ChoiseNN::activate_neuron(neural_network.weight.weight_neuron * input, coordinate)
        }
        else { ChoiseNN::return_null_value }
    }
}

impl Coordinate {
    /// Создание новой позиции
    pub fn new(x: f64, y: f64, z: f64) -> Coordinate {
        Coordinate { x, y, z }
    }
    ///Получить позицию
    pub fn get_position(&self) -> (f64, f64, f64) {
        (self.x, self.y, self.z)
    }

    pub fn clone(&self) -> Coordinate{
        Coordinate{x: self.x, y: self.y, z: self.z}
    }
}

pub struct NeuralNetwork {
    index:u128,
    weight: WeightNeurone,
    position: Coordinate,
    neurons: Vec<Neurone>,
    synapses: Vec<Sinapses>,
    intermediate_neuron: Vec<INeuron>
}

impl NeuralNetwork {
    pub fn new(x: f64, y: f64, z: f64, index_first_neurone: u128, weight:f64,sinapses: Sinapses, ineuron: INeuron, index:u128, weight_neurone: WeightNeurone) -> Self {
        NeuralNetwork {
            index: index,
            weight: weight_neurone,
            position: Coordinate::new(x, y, z),
            neurons: vec![Neurone::new(x, y, z, index_first_neurone, weight)],
            synapses: vec![sinapses],
            intermediate_neuron: vec![ineuron]
        }
    }
    ///Получить список нейронов и вывести в консоль
    pub fn list_neurons(&self) {
        println!("Список нейронов и их позиций:");
        for neuron in &self.neurons {
            let (x, y, z) = neuron.get_position();
            let (weight_neuron, weight_choise) = neuron.get_weight();
            println!("\nНейрон {:?} \n\
            Позиция: x: {:?},y: {:?},z: {:?}\n\
            Веса: вес нейрона: {:?}, вес выбора: {:?}", neuron.get_index(),x,y,z,weight_neuron,weight_choise);
        }
    }
///Получить список промежутоных нейронов и вывести в консоль
    pub fn list_inrons(&self){
        println!("Список промежуточных нейронов:");
        for neuron in &self.intermediate_neuron {
            let (x, y, z) = neuron.get_position();
            let (weight_neuron, weight_choise) = neuron.get_weight();
            println!("\nПромежуточный нейрон {:?} \n\
            Позиция: x: {:?},y: {:?},z: {:?}\n\
            Веса: вес нейрона: {:?}, вес выбора: {:?}", neuron.get_index(),x,y,z,weight_neuron,weight_choise);
        }
    }
    ///Получить список синапсисов и вывести в консоль
    pub fn list_sinapses(&self){
        println!("Список синапсов:");
        for sinaps in &self.synapses {
            let (index, weight, x_one, y_one, z_one, x_two, y_two, z_two) = sinaps.get_sinapses_data();
            println!("\nСинапс {:?} \n\
            Вес: {:?}\n\
            Позиция первого нейрона: x:{:?}, y:{:?}, z:{:?}\n\
            Позиция второго нейрона: x:{:?}, y:{:?}, z:{:?}", index,weight,x_one, y_one, z_one, x_two, y_two, z_two );
        }
    }
    ///Получить позицию нейронной сети
    pub fn get_position(&self) -> (f64, f64, f64) {
        self.position.get_position()
    }
    ///Получить индекс нейронной сети
    pub fn get_index(&self) -> u128 {
        self.index
    }
    ///Активировать нейронную сеть, передав входные веса
    pub(crate) fn work(&self, input:Vec<f64>) -> Vec<f64>{
        let base_coordinates = Coordinate::new(0.0,0.0,0.0);
        let mut output: Vec<f64> = Vec::new();
        for i in input  {
            let choise = self.activate(i, base_coordinates.clone());
            match choise {
                Ok(cho) => output.push(cho),
                Err(e) => println!("Ошибка: {}", e)
            }
        }
        output
    }
    ///Активировать нейронную сеть
    pub fn activate(&self, input: f64, coordinate: Coordinate) -> Result<f64,String>{
        let choise = ChoiseNN::new(self, input, coordinate.clone());
        let position = coordinate.clone();
        match choise {
            ChoiseNN::activate_neuron(input, coordinate) => {
                println!("Нейронная сеть активировала нейрон с входным весом {} в позиции: {:?}", input, coordinate.get_position());
                let neurone = self.get_neuron(position);
                match neurone{
                    Ok(neuron) => {
                        let output = neuron.activate(input, self);
                        Ok(output) }
                    Err(e) => Err(e)
                }

            }
            ChoiseNN::return_null_value => {
                let e = format!("Нейронная сеть решила вернуть нулевое значение веса");
                Err(e)
            }
        }
    }
    ///Получить нейрон по координатам
    pub fn get_neuron(&self, coordinate: Coordinate) -> Result<Neurone, String> {
        let position = coordinate.clone();
        for neuron in self.neurons.iter() {
            if neuron.get_position() == coordinate.get_position() {
                return Ok(neuron.clone());
            }
        }
        let e = format!("Нейрон не найден по координатам: x:{}, y:{}, z:{}", position.x, position.y, position.z);
        Err(e)
    }
    //TODO: Добавить в NN метод push INeuron и Neuron
    //TODO: Добавить push Sinapse


}
///Создать базовые нейроны и синапсис
pub fn create_base() -> NeuralNetwork {
    let (x, y, z) = (0.0, 0.0, 0.0);
    let coordinates_sinapses_one = Coordinate::new(0.0, 0.0, 0.0);
    let coordinates_sinapses_two = Coordinate::new(0.0, 0.0, 1.0);
    let coordinates_inerone = Coordinate::new(0.0,0.0,0.1);
    let weight = WeightNeurone::new(0.1);
    //TODO:Убрать синапсис заглушку
    let sinapses = Sinapses::new(0,0.0,coordinates_sinapses_one,coordinates_sinapses_two);
    //TODO: Высчитать стартовые веса
    let ineron = INeuron::new(coordinates_inerone,0,0.0);
    NeuralNetwork::new(x, y, z,0,0.0,sinapses,ineron,0,weight)
}
///Тестирование и первичное обучение сети
pub fn test_neural_network(nn: NeuralNetwork,all_chars: AllChars) -> NeuralNetwork{
    let input = "Тестовое сообщение";
    let trimmed_input = "Сид-ключ: ".to_owned() + input.trim();
    let input_weights = encode(trimmed_input.as_str(), all_chars.clone());
    let find_output = "Сид-ответ:".to_owned() + input.trim();
    let find_weights = encode(find_output.as_str(), all_chars.clone());
    let output_weights:Vec<f64> = nn.work(input_weights);
    let output = decode(output_weights,all_chars.clone());

    if output == input {
        println!("\nТест пройден, ответ соответствует шаблону");
    }
    else {
        println!("\nТест провален, начинаю процесс обучения");
    }
    nn
}
///Энкодинг входящей фразы в веса
fn encode (input: &str, mut all_chars: AllChars) -> Vec<f64>{
    let mut weights: Vec<f64> = Vec::new();
    println!("Строка {input} энкодирована в: ");
    for c in input.chars() {
        weights.push(*all_chars.get_char_weight(c));
    }
    for c in weights.iter(){
        println!("{c}");
    }
    weights
}
///Декодинг исходящей фразы из веса в строку
fn decode(input: Vec<f64>, mut all_chars: AllChars) -> String{
    let mut message = String::new();
    for c in input.iter() {
        message.push_str(&all_chars.get_weight_from_char(*c));
    }
    println!("Ответ системы:\n\
    {message}");
    message.to_string()
}
