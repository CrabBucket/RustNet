use rand::Rng;


// This enum represents the three types of layers that a neural network can have: input, hidden, and output.
// Input layers are the first layers in a neural network and are responsible for getting input data.
// Hidden layers are responsible for processing the input data to get some output data.
// Output layers are the last layers in a neural network and are responsible for providing the final output.
enum LayerType { Input, Hidden, Output }

// This enum represents the different types of activation functions that have been implemented.
enum ActivationFunction { Sigmoid, ReLU, LeakyReLU, TanH, Softmax }

trait InputSize {
    fn get_input_size(&self) -> usize;
}

trait Layer<const SIZE: usize, const INPUT: usize> {
    // This function returns the type of this layer.
    fn get_type(&self) -> LayerType;

    // This function returns the size of this layer.
    fn get_size(&self) -> usize;

    // This function creates a new layer with the given size.
    fn new(size: usize) -> Self;

    // This function feeds the given input into this layer and returns the output.
    fn feed_forward(&self, input: [f32; INPUT]) -> [f32; SIZE];
}

// All activations functions should need to apply their their respective functions to the given input.
impl ActivationFunction {
    // This function applies the given activation function to the given input.
    pub fn apply_function(&self, input: f32) -> f32 {
        // This match statement matches the activation function and returns the result of the given function.
        match self {
            ActivationFunction::Sigmoid => 1.0 / (1.0 + (-input).exp()),
            ActivationFunction::ReLU => input.max(0.0),
            ActivationFunction::LeakyReLU => input.max(0.1 * input),
            ActivationFunction::TanH => input.tanh(),
            ActivationFunction::Softmax => 1.0 / (1.0 + (-input).exp()),
        }
    }
}
pub struct InputLayer<const SIZE: usize> {
    size: usize,
    bias: f32,
}
impl<const SIZE: usize, const INPUT: usize> Layer<SIZE, INPUT> for InputLayer<SIZE> {
    fn get_type(&self) -> LayerType {
        LayerType::Input
    }

    fn get_size(&self) -> usize {
        self.size
    }

    fn new(size: usize) -> Self {
        InputLayer { size, bias: 0.0 }
    }

    fn feed_forward(&self, input: Vec<f32>) -> Vec<f32> {
        input
    }
}

//
// This structure represents a hidden layer in a neural network.
//
// Each hidden layer contains a certain number of neurons, and each neuron has
// a bias and an activation function.
//
// The activation function of a neuron is used to transform the output of that
// neuron from the previous layer to the input to the next layer.
//
struct HiddenLayer<const SIZE: usize, const INPUT: usize> {
    bias: [f32; SIZE], // The bias of each neuron in this layer
    activation_function: ActivationFunction, // The activation function used by the neurons in this layer
    weights: [[f32; INPUT]; SIZE],
}

impl<const SIZE: usize, const INPUT: usize> Layer<SIZE, INPUT> for HiddenLayer<SIZE, INPUT> {
    fn get_type(&self) -> LayerType { LayerType::Hidden }
    fn get_size(&self) -> usize { SIZE }
    fn new(size: usize) -> Self {
        HiddenLayer {
            bias: [0.0; SIZE],
            activation_function: ActivationFunction::Sigmoid,
            weights: [[0.0; INPUT]; SIZE],
        }
    }

    fn feed_forward(&self, input: [f32; INPUT]) -> [f32; SIZE] {
        let mut output = [0.0; SIZE];
        for i in 0..SIZE {
            for j in 0..INPUT {
                output[i] += input[j] * self.weights[i][j];
            }
            output[i] += self.bias[i];
            output[i] = self.activation_function.apply_function(output[i]);
        }
        output
    }

    
}


fn random_f32() -> f32 {
    (std::f32::consts::PI * (std::time::SystemTime::now().elapsed().unwrap().as_nanos() as f32))
    .sin()
    .abs()
    / std::f32::consts::PI / 2.0 * 0.3
}
impl<const SIZE: usize, const INPUT: usize> HiddenLayer<SIZE, INPUT> {
    pub fn randomize_bias(&mut self) {
        for i in 0..SIZE {
            self.bias[i] = random_f32();
        }
    }
    pub fn randomize_weights(&mut self) {
        for i in 0..SIZE {
            for j in 0..INPUT {
                self.weights[i][j] = random_f32();
            }
        }
    }
    pub fn back_prop(&self, input: [f32; INPUT], output: [f32; SIZE], expected: [f32; SIZE]){
        let mut error = [0.0; SIZE];
        for i in 0..SIZE {
            error[i] = expected[i] - output[i];
        }
        for i in 0..SIZE {
            for j in 0..INPUT {
                self.weights[i][j] += error[i] * input[j];
            }
            self.bias[i] += error[i];
        }
    }

}

pub struct OutputLayer {
    size: usize,
    bias: f32,
    activation_function: ActivationFunction,
    
}
pub impl Layer for OutputLayer {
    fn get_type(&self) -> LayerType {
        LayerType::Output
    }

    fn get_size(&self) -> usize {
        self.size
    }

    fn new(size: usize) -> Self {
        OutputLayer { size }
    }

    fn feed_forward(&self, input: Vec<f32>) -> f32 {
        input.iter().sum()
    }
}