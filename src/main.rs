use std::io;
use plotly::Plot;
use plotly::Scatter3D;

mod neuron_network {
    pub enum ClassNeyron {
        Neyron,
    }

    pub enum Choise {
        ReturnOutput,
        AddNeuron,
        ActivateNeuron
    }

    pub struct NeuronNetwork {
        neurons: Vec<Neuron>,
    }

    impl NeuronNetwork {
        pub fn new() -> Self {
            NeuronNetwork {
                neurons: vec![Neuron::new(0.0, 0.0, 0.0)],
            }
        }

        pub fn add_neuron(&mut self, x: f64, y: f64, z: f64) {
            self.neurons.push(Neuron::new(x, y, z));
        }

        pub fn activate_neurons(&mut self) {
            let mut new_neurons = Vec::new(); // временное хранилище для новых нейронов

            for neuron in &self.neurons {
                match neuron.choise_action() {
                    Choise::ReturnOutput => {
                        println!("Нейрон активировал функцию ответа пользователю");
                    }
                    Choise::AddNeuron => {
                        println!("Добавлен новый нейрон");
                        new_neurons.push(Neuron::new(neuron.x + 1.0, neuron.y + 1.0, neuron.z + 1.0));
                    }
                    Choise::ActivateNeuron => {
                        println!("Нейрон активировал функцию активации нейрона");
                    }
                }
            }

            // Добавляем новые нейроны после завершения итерации
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
        weights: f64,
        weights_2: f64,
    }

    impl Neuron {
        pub fn new(x: f64, y: f64, z: f64) -> Self {
            let class = ClassNeyron::Neyron;
            Neuron {
                class,
                x,
                y,
                z,
                weights: 0.5,   // Значение по умолчанию
                weights_2: -0.2, // Значение по умолчанию
            }
        }

        pub fn choise_action(&self) -> Choise {
            if self.weights * self.weights_2 > 0.5 {
                Choise::ReturnOutput
            } else if self.weights * self.weights_2 < 0.0 {
                Choise::AddNeuron
            } else {

            }

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
            // Добавляем нейрон с координатами, считанными из строки (например, "1.0 2.0 3.0")
            let coords: Vec<f64> = trimmed_input
                .split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect();

            if coords.len() == 3 {
                neuron_network.add_neuron(coords[0], coords[1], coords[2]);
            } else {
                println!("Введите три числа, разделенные пробелами, для координат x, y, z.");
            }

            neuron_network.activate_neurons();
            display_neurons_as_points(neuron_network.get_neurons());
            println!("Отображение 3D-графика нейронов...");
            plot_neurons_3d(neuron_network.get_neurons());
        }
    }

    println!("Отображение 3D-графика нейронов...");
    plot_neurons_3d(neuron_network.get_neurons());
}
