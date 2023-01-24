


// This enum represents the three types of layers that a neural network can have: input, hidden, and output.
// Input layers are the first layers in a neural network and are responsible for getting input data.
// Hidden layers are responsible for processing the input data to get some output data.
// Output layers are the last layers in a neural network and are responsible for providing the final output.
enum LayerType { Input, Hidden, Output }

// This enum represents the different types of activation functions that have been implemented.
enum ActivationFunction { Sigmoid, ReLU, LeakyReLU, TanH, Softmax }


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

enum LossFunction { MeanSquaredError, CrossEntropy }

impl LossFunction {

}

trait InputSize {
    fn get_input_size(&self) -> usize;
}

pub trait Layer {
    // This function returns the type of this layer.
    fn get_type(&self) -> LayerType;

    // This function returns the size of this layer.
    fn get_size(&self) -> usize;

    // This function creates a new layer with the given size.


}

pub struct InputLayer {
    size: usize,
    bias: f32,
}
impl Layer for InputLayer {
    fn get_type(&self) -> LayerType {
        LayerType::Input
    }

    fn get_size(&self) -> usize {
        self.size
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
pub struct HiddenLayer {
    neurons: usize,
    activation_function: ActivationFunction,
}

impl Layer for HiddenLayer {
    fn get_type(&self) -> LayerType { LayerType::Hidden }
    fn get_size(&self) -> usize { self.neurons }
    fn new(size: usize, activation_function: ActivationFunction) -> Self {
        HiddenLayer {
            neurons: size,
            activation_function: ActivationFunction::Sigmoid,
        }
    }

    
}



pub struct OutputLayer {
    size: usize,
    activation_function: ActivationFunction,
    
}
impl Layer for OutputLayer {
    fn get_type(&self) -> LayerType {
        LayerType::Output
    }

    fn get_size(&self) -> usize {
        self.size
    }

    fn new(size: usize, activation_function: ActivationFunction) -> Self {
        OutputLayer { size, activation_function }
    }
}