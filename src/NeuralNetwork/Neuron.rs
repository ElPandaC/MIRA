use crate::NeuralNetwork::{Coordinate, NeuralNetwork};

enum NeuronChoise{
    ActivateSinapse(f64),
    CreateIntermediateNeuron(f64),
}

impl NeuronChoise {
    pub fn new(weight_neurone: WeightNeurone, weight_input: f64) -> NeuronChoise {
        if weight_neurone.weight_neuron *  weight_input > weight_neurone.weight_choice {
            NeuronChoise::ActivateSinapse(weight_neurone.weight_neuron *  weight_input)
        }
        else {
            NeuronChoise::CreateIntermediateNeuron(weight_neurone.weight_neuron *  weight_input)
        }
    }
}
pub struct WeightNeurone{
    pub(crate) weight_neuron: f64,
    pub(crate) weight_choice: f64,
}

impl WeightNeurone {
    ///Создать новый вес нейрона
    pub fn new(weight: f64) -> WeightNeurone{
        WeightNeurone { weight_neuron: weight.clone(), weight_choice: weight.clone() }
    }
    /// Клонировать вес нейрона
    pub fn clone(&self) -> WeightNeurone{
        WeightNeurone { weight_neuron: self.weight_neuron, weight_choice: self.weight_choice }
    }
}

pub struct Neurone {
    position: Coordinate,
    index: u128,
    weights: WeightNeurone
}

impl Neurone {
    pub fn new(x: f64, y: f64, z: f64, index: u128,weight: f64) -> Self {
        let coord = Coordinate::new(x, y, z);
        let new_weight = WeightNeurone::new(weight);
        Neurone {
            position: coord,
            index,
            weights: new_weight }
    }

    pub fn clone(&self) -> Neurone {
        Neurone {
            position: self.position.clone(),
            index: self.index,
            weights: self.weights.clone(),
        }
    }
    ///Получить индекс нейрона
    pub fn get_index(&self) -> u128 {
        self.index
    }
    ///Получить позицию нейрона
    pub fn get_position(&self) -> (f64, f64, f64) {
        self.position.get_position()
    }
    ///Получить веса нейрона
    pub fn get_weight(&self) -> (f64,f64){
        (self.weights.weight_neuron, self.weights.weight_choice)
    }

    pub fn activate(&self, weight: f64, nn:&NeuralNetwork) -> f64{
        let choise = NeuronChoise::new(self.weights.clone(),weight);

        match choise {
            NeuronChoise::ActivateSinapse(weight_output) => {
                println!("Нейрон с индексом: {:?} выбрал: Активация синапса", self.index);
                //TODO: Получить список синапсов и выбрать наиболее подходящий. Как?
                weight_output
            }
            NeuronChoise::CreateIntermediateNeuron(weight_output) => {
                println!("Нейрон с индексом: {:?} выбрал: Cоздать новый промежуточный нейрон", self.index);
                weight_output
            }
        }
    }

}




