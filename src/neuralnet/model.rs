use crate::neuralnet::layer::*;

pub struct Model {
    input: InputLayer,
    hidden: Vec<HiddenLayer>,
    output: OutputLayer,

}

impl Model {
    pub fn new(input: InputLayer, hidden_sizes: Vec<usize>, output_size: usize) -> Self {
        
        let output = Dense::new(hidden_sizes.last().unwrap(), output_size, Activation::Sigmoid);

        Model { input, hidden, output }
    }
}