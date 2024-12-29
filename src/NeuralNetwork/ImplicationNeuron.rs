use crate::NeuralNetwork::Coordinate;
use crate::NeuralNetwork::Neuron::WeightNeurone;

pub struct INeuron{
    index:u128,
    position: Coordinate,
    weights:WeightNeurone
}

impl INeuron {
    pub fn new(coordinate:Coordinate, index: u128,weight: f64) -> Self {
        let new_weight = WeightNeurone::new(weight);
        INeuron {
            position: coordinate,
            index,
            weights: new_weight }
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
}