use std::io;
use plotly::Plot;
use plotly::Scatter3D;

mod neuron_network {
    pub enum ClassNeyron {
        Neyron(String),
        Intermediate_Neuron(String),
        Sinaps(Neuron, Neuron)
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

        pub fn get_neurons(&self) -> &Vec<Neuron> {
            &self.neurons
        }
    }

    pub struct Neuron {
        pub(crate) x: f64,
        pub(crate) y: f64,
        pub(crate) z: f64,
    }

    impl Neuron {
        pub fn new(x: f64, y: f64, z: f64) -> Self {
            Neuron { x, y, z }
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
        println!("Введите запрос:");
        input.clear(); // Очищаем строку перед новым вводом
        io::stdin().read_line(&mut input).expect("Ошибка чтения линии");

        let trimmed_input = input.trim();
        if trimmed_input.is_empty() {
            println!("Пожалуйста, введите хоть что-то");
        } else if trimmed_input == "flugegenhaime" {
            break;
        } else {
            neuron_network.add_neuron(1.0, 2.0, 3.0);
            display_neurons_as_points(neuron_network.get_neurons());
            println!("Отображение 3D-графика нейронов...");
            plot_neurons_3d(neuron_network.get_neurons());
        }
    }
}
