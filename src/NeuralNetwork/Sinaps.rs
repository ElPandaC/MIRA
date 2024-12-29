use crate::NeuralNetwork::Coordinate;

pub struct Sinapses {
    index: u128,
    weight: f64,
    position_neuron_one: Coordinate,
    position_neuron_two: Coordinate,
}
impl Sinapses {

    ///Активация синапса
    pub fn activate_sinaps(&mut self, weight: f64){
        self.weight = self.weight * weight;
    }
    ///Создание нового синапса
    pub fn new(index:u128, weight:f64, position_neuron_one: Coordinate, position_neuron_two: Coordinate) -> Sinapses {
        Sinapses { index, weight, position_neuron_one, position_neuron_two }
    }
    ///Получить данные синапса
    pub fn get_sinapses_data(&self) -> (u128, f64, f64, f64, f64, f64, f64, f64) {
        let (x_one, y_one, z_one) = self.position_neuron_one.get_position();
        let (x_two, y_two, z_two) = self.position_neuron_two.get_position();
        (self.index, self.weight, x_one, y_one, z_one, x_two, y_two, z_two)
    }
}