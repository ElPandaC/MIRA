use std::io;
use std::collections::HashMap;
use plotly::Plot;
use plotly::Scatter3D;

mod neuron_network {
    pub enum ClassNeyron {
        Neyron,
        IntermediateNeuron,
    }

    pub enum Choise_Intermation_Neuron_Operations {
        Addition,
        Subtraction,
        Division,
        Multiplication,
    }

    impl Choise_Intermation_Neuron_Operations {
        pub fn new(weight_choise: f64, weight: f64) -> Choise_Intermation_Neuron_Operations {
            if weight_choise * weight > 0.5 {
                Choise_Intermation_Neuron_Operations::Addition
            } else if weight_choise * weight > 0.0 {
                Choise_Intermation_Neuron_Operations::Subtraction
            } else if weight_choise * weight > -0.5 {
                Choise_Intermation_Neuron_Operations::Division
            } else {
                Choise_Intermation_Neuron_Operations::Multiplication
            }
        }
    }

    pub enum Choise_Intermation_Neuron {
        AddNeuron(Choise_Intermation_Neuron_Operations),
        AddIntermation_Neuron(Choise_Intermation_Neuron_Operations),
        Activate_Next_Neuron(Choise_Intermation_Neuron_Operations),
    }

    pub enum Choise {
        ReturnOutput,
        AddNeuron,
        ActivateNeuron,
    }

    pub struct NeuronNetwork {
        neurons: Vec<Neuron>,
    }

    impl NeuronNetwork {
        pub fn new() -> Self {
            NeuronNetwork {
                neurons: vec![Neuron::new(0.0, 0.0, 0.0, ClassNeyron::Neyron, 1.0, 1.0, 1.0)],
            }
        }

        pub fn activate_neurons(&mut self, x: f64, y: f64, z: f64, weight: f64) {
            let mut new_neurons = Vec::new();
            let mut weight_signal = 0.0;

            if let Some(neuron) = self.neurons.iter().find(|a| a.x == x && a.y == y && a.z == z) {
                weight_signal = weight * (neuron.weights_neuron + neuron.weights_choise);

                println!("Найден элемент: x={}, y={}, z={}", neuron.x, neuron.y, neuron.z);
                match neuron.class {
                    ClassNeyron::Neyron => {
                        match neuron.choise_action_neyron(weight) {
                            Choise::ReturnOutput => {
                                println!("Нейрон активировал функцию ответа пользователю");
                                //TODO: Добавить функцию ответ пользователю
                            }
                            Choise::AddNeuron => {
                                println!("Нейрон вырал слздать новый нейрон");
                                println!("Добавлен новый нейрон");
                                new_neurons.push(Neuron::new(
                                    neuron.x + weight_signal,
                                    neuron.y + weight_signal,
                                    neuron.z + weight_signal,
                                    ClassNeyron::Neyron,
                                    neuron.weights_neuron,
                                    neuron.weights_choise,
                                    weight_signal,
                                ));
                            }
                            Choise::ActivateNeuron => {
                                println!("Нейрон активировал функцию активации нейрона");
                                if let Some(neuron) = self.neurons.iter().find(|a| a.x == x && a.y == y && a.z == z) {
                                    println!("Найден элемент: x={}, y={}, z={}", neuron.x, neuron.y, neuron.z);
                                    match neuron.class {
                                        ClassNeyron::Neyron => {
                                            match neuron.choise_action_neyron(weight_signal) {
                                                Choise::ReturnOutput => {
                                                    println!("Нейрон активировал функцию ответа пользователю");
                                                    //TODO: Добавить функцию ответ пользователю
                                                }
                                                Choise::AddNeuron => {
                                                    println!("Добавлен новый промежуточный нейрон");
                                                    new_neurons.push(Neuron::new(
                                                        neuron.x + weight_signal,
                                                        neuron.y + weight_signal,
                                                        neuron.z + weight_signal,
                                                        ClassNeyron::IntermediateNeuron,
                                                        neuron.weights_neuron,
                                                        neuron.weights_choise,
                                                        weight_signal,
                                                    ));
                                                }
                                                Choise::ActivateNeuron => {
                                                    //TODO:Переделать функцию активации нейронов под поиск ближайшего нейрона в 3д пространстве     =>
                                                    println!("Нейрон активировал функцию активации нейрона");
                                                    self.activate_neurons(
                                                        neuron.x + weight_signal,
                                                        neuron.y + weight_signal,
                                                        neuron.z + weight_signal,
                                                        weight_signal,
                                                    );
                                                }
                                            }
                                        }
                                        ClassNeyron::IntermediateNeuron => {
                                            match neuron.choise_action_intermation_neuron(weight_signal) {
                                                Choise_Intermation_Neuron::Activate_Next_Neuron(choise) => {
                                                    match choise {
                                                        Choise_Intermation_Neuron_Operations::Addition => {
                                                            //TODO: Добавить функцию поиска и активации ближайшего промежуточного нейрона с операцией сложения
                                                            let weight_signal = weight + (neuron.weights_neuron + neuron.weights_choise);
                                                        }
                                                        Choise_Intermation_Neuron_Operations::Subtraction => {
                                                            //TODO: Добавить функцию поиска и активации ближайшего промежуточного нейрона с операцией вычитания
                                                            let weight_signal = weight - (neuron.weights_neuron + neuron.weights_choise);
                                                        }
                                                        Choise_Intermation_Neuron_Operations::Division => {
                                                            //TODO: Добавить функцию поиска и активации ближайшего промежуточного нейрона с операцией деления
                                                            let weight_signal = weight / (neuron.weights_neuron + neuron.weights_choise);
                                                        }
                                                        Choise_Intermation_Neuron_Operations::Multiplication => {
                                                            //TODO: Добавить функцию поиска и активации ближайшего промежуточного нейрона с операцией умножения
                                                            let weight_signal = weight * (neuron.weights_neuron + neuron.weights_choise);
                                                        }
                                                    }
                                                }
                                                Choise_Intermation_Neuron::AddNeuron(choise) => {
                                                    match choise {
                                                        Choise_Intermation_Neuron_Operations::Addition => {
                                                            //TODO: Добавить функцию создания нового нейрона с операцией сложения
                                                            let weight_signal = weight + (neuron.weights_neuron + neuron.weights_choise);
                                                            println!("Добавлен новый нейрон");
                                                            new_neurons.push(Neuron::new(
                                                                neuron.x + weight_signal,
                                                                neuron.y + weight_signal,
                                                                neuron.z + weight_signal,
                                                                ClassNeyron::Neyron,
                                                                neuron.weights_neuron,
                                                                neuron.weights_choise,
                                                                weight_signal,
                                                            ));
                                                        }
                                                        Choise_Intermation_Neuron_Operations::Subtraction => {
                                                            //TODO: Добавить функцию создания нового нейрона с операцией вычитания
                                                            let weight_signal = weight - (neuron.weights_neuron + neuron.weights_choise);
                                                            println!("Добавлен новый нейрон");
                                                            new_neurons.push(Neuron::new(
                                                                neuron.x + weight_signal,
                                                                neuron.y + weight_signal,
                                                                neuron.z + weight_signal,
                                                                ClassNeyron::Neyron,
                                                                neuron.weights_neuron,
                                                                neuron.weights_choise,
                                                                weight_signal,
                                                            ));
                                                        }
                                                        Choise_Intermation_Neuron_Operations::Division => {
                                                            //TODO: Добавить функцию создания нового нейрона с операцией деления
                                                            let weight_signal = weight / (neuron.weights_neuron + neuron.weights_choise);
                                                            println!("Добавлен новый нейрон");
                                                            new_neurons.push(Neuron::new(
                                                                neuron.x + weight_signal,
                                                                neuron.y + weight_signal,
                                                                neuron.z + weight_signal,
                                                                ClassNeyron::Neyron,
                                                                neuron.weights_neuron,
                                                                neuron.weights_choise,
                                                                weight_signal,
                                                            ));
                                                        }
                                                        Choise_Intermation_Neuron_Operations::Multiplication => {
                                                            //TODO: Добавить функцию создания нового нейрона с операцией умножения
                                                            let weight_signal = weight * (neuron.weights_neuron + neuron.weights_choise);
                                                            println!("Добавлен новый нейрон");
                                                            new_neurons.push(Neuron::new(
                                                                neuron.x + weight_signal,
                                                                neuron.y + weight_signal,
                                                                neuron.z + weight_signal,
                                                                ClassNeyron::Neyron,
                                                                neuron.weights_neuron,
                                                                neuron.weights_choise,
                                                                weight_signal,
                                                            ));
                                                        }
                                                    }
                                                }
                                                Choise_Intermation_Neuron::AddIntermation_Neuron(choise) => {
                                                    match choise {
                                                        Choise_Intermation_Neuron_Operations::Addition => {
                                                            //TODO: Добавить функцию создания нового промежуточного нейрона с операцией сложения
                                                            let weight_signal = weight + (neuron.weights_neuron + neuron.weights_choise);
                                                            println!("Добавлен новый промежуточный нейрон");
                                                            new_neurons.push(Neuron::new(
                                                                neuron.x + weight_signal,
                                                                neuron.y + weight_signal,
                                                                neuron.z + weight_signal,
                                                                ClassNeyron::IntermediateNeuron,
                                                                neuron.weights_neuron,
                                                                neuron.weights_choise,
                                                                weight_signal,
                                                            ));
                                                        }
                                                        Choise_Intermation_Neuron_Operations::Subtraction => {
                                                            //TODO: Добавить функцию создания нового промежуточного нейрона с операцией вычитания
                                                            let weight_signal = weight - (neuron.weights_neuron + neuron.weights_choise);
                                                            println!("Добавлен новый промежуточный нейрон");
                                                            new_neurons.push(Neuron::new(
                                                                neuron.x + weight_signal,
                                                                neuron.y + weight_signal,
                                                                neuron.z + weight_signal,
                                                                ClassNeyron::IntermediateNeuron,
                                                                neuron.weights_neuron,
                                                                neuron.weights_choise,
                                                                weight_signal,
                                                            ));
                                                        }
                                                        Choise_Intermation_Neuron_Operations::Division => {
                                                            //TODO: Добавить функцию создания нового промежуточного нейрона с операцией деления
                                                            let weight_signal = weight / (neuron.weights_neuron + neuron.weights_choise);
                                                            println!("Добавлен новый промежуточный нейрон");
                                                            new_neurons.push(Neuron::new(
                                                                neuron.x + weight_signal,
                                                                neuron.y + weight_signal,
                                                                neuron.z + weight_signal,
                                                                ClassNeyron::IntermediateNeuron,
                                                                neuron.weights_neuron,
                                                                neuron.weights_choise,
                                                                weight_signal,
                                                            ));
                                                        }
                                                        Choise_Intermation_Neuron_Operations::Multiplication => {
                                                            //TODO: Добавить функцию создания нового промежуточного нейрона с операцией умножения
                                                            let weight_signal = weight * (neuron.weights_neuron + neuron.weights_choise);
                                                            println!("Добавлен новый промежуточный нейрон");
                                                            new_neurons.push(Neuron::new(
                                                                neuron.x + weight_signal,
                                                                neuron.y + weight_signal,
                                                                neuron.z + weight_signal,
                                                                ClassNeyron::IntermediateNeuron,
                                                                neuron.weights_neuron,
                                                                neuron.weights_choise,
                                                                weight_signal,
                                                            ));
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    println!("Элемент не найден");
                                    println!(
                                        "Создаю новый промежуточный нейрон с координатами x: {}, y = {}, z = {}",
                                        neuron.x + weight_signal,
                                        neuron.y + weight_signal,
                                        neuron.z + weight_signal
                                    );

                                    new_neurons.push(Neuron::new(
                                        neuron.x + weight_signal,
                                        neuron.y + weight_signal,
                                        neuron.z + weight_signal,
                                        ClassNeyron::IntermediateNeuron,
                                        neuron.weights_neuron,
                                        neuron.weights_choise,
                                        weight_signal,
                                    ));
                                    println!("Новый промежуточный нейрон создан!");
                                    self.activate_neurons(
                                        neuron.x + weight_signal,
                                        neuron.y + weight_signal,
                                        neuron.z + weight_signal,
                                        weight_signal,
                                    );
                                }
                            }
                        }
                    }
                    ClassNeyron::IntermediateNeuron => {
                        neuron.choise_action_intermation_neuron(weight_signal);
                    }
                }
            } else {
                println!("Элемент не найден");
            }

            // Добавляем новые нейроны после завершения всех операций
            self.neurons.extend(new_neurons);
        }

        pub fn get_neurons(&self) -> &Vec<Neuron> {
            &self.neurons
        }
    }

    pub struct Neuron {
        class: ClassNeyron,
        pub(crate) x: f64,
        pub(crate) y: f64,
        pub(crate) z: f64,
        weights_neuron: f64,
        weights_choise: f64,
        radius_1: f64,
        radius_2: f64,
    }

    impl Neuron {
        pub fn new(
            x: f64,
            y: f64,
            z: f64,
            class: ClassNeyron,
            weight_coise: f64,
            weight_neuron: f64,
            weight: f64,
        ) -> Self {
            Neuron {
                class,
                x,
                y,
                z,
                weights_neuron: weight_neuron + weight,
                weights_choise: weight_coise + weight,
                radius_1: 0.5, //TODO: заменить на динамические значения
                radius_2: 0.8,
            }
        }

        pub fn choise_action_neyron(&self, weight: f64) -> Choise {
            //TODO: Добавить выбор весов активации
            if self.weights_choise * weight > 0.5 {
                Choise::ReturnOutput
            } else if self.weights_choise * weight > 0.0 {
                Choise::AddNeuron
            } else {
                Choise::ActivateNeuron
            }
        }

        pub fn choise_action_intermation_neuron(&self, weight: f64) -> Choise_Intermation_Neuron {
            //TODO: Добавить выбор весов активации
            let operation_choise = Choise_Intermation_Neuron_Operations::new(self.weights_choise, weight);

            if self.weights_choise * weight == 0.5 {
                Choise_Intermation_Neuron::AddNeuron(operation_choise)
            } else if self.weights_choise * weight < 0.0 {
                Choise_Intermation_Neuron::AddIntermation_Neuron(operation_choise)
            } else {
                Choise_Intermation_Neuron::Activate_Next_Neuron(operation_choise)
            }
        }

        pub fn sinaps_operations(&self) {
            //TODO Добавить логику синапсической связи
        }
    }
}

fn display_neurons_as_points(neurons: &[neuron_network::Neuron]) {
    for (index, neuron) in neurons.iter().enumerate() {
        println!("Neuron {}: Point({}, {}, {})", index + 1, neuron.x, neuron.y, neuron.z);
    }
}

fn plot_neurons_3d(neurons: &[neuron_network::Neuron]) {
    let x: Vec<f64> = neurons.iter().map(|n| n.x).collect();
    let y: Vec<f64> = neurons.iter().map(|n| n.y).collect();
    let z: Vec<f64> = neurons.iter().map(|n| n.z).collect();

    let trace = Scatter3D::new(x, y, z)
        .mode(plotly::common::Mode::Markers)
        .marker(plotly::common::Marker::new().size(5).color("red"));

    let mut plot = Plot::new();
    plot.add_trace(trace);
    plot.show();
}

fn main() {
    let chrars_list = AllChars::new();
    let mut input = String::new();
    let mut neuron_network = neuron_network::NeuronNetwork::new();

    loop {
        println!("Введите запрос (или 'flugegenhaime' для выхода):");
        input.clear();
        io::stdin().read_line(&mut input).expect("Ошибка чтения строки");
        let trimmed_input = input.trim();
        if trimmed_input.is_empty() {
            println!("Пожалуйста, введите хоть что-то.");
        } else if trimmed_input == "flugegenhaime" {
            break;
        } else {
            for input_chars in trimmed_input.chars() {
                match chrars_list.char_map.get(&input_chars) {
                    Some(value) => {
                        println!("Вес символа {}: {:?}", input_chars, value);
                        let weight = chrars_list.char_map.get(&input_chars).unwrap();
                        neuron_network.activate_neurons(0.0, 0.0, 0.0, *weight);
                        display_neurons_as_points(neuron_network.get_neurons());
                        println!("Отображение 3D-графика нейронов...");
                        plot_neurons_3d(neuron_network.get_neurons());
                    },
                    None => println!("Нужный символ не найден"),
                }
            }
        }
    }

    println!("Отображение 3D-графика нейронов...");
    plot_neurons_3d(neuron_network.get_neurons());
}
