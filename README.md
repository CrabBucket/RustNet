# RustNet
# MLP Handwritten Digit Detection

A multi-layer perceptron (MLP) implemented in Rust to detect handwritten digits using the MNIST dataset.

## Components

- **Neurons**: Each neuron in an MLP represents a linear combination of the inputs, which is passed through an activation function. The neuron class will have fields for the weights, biases, and the output value of the neuron.

- **Layers**: The neurons in an MLP are organized into layers, with each layer having a set of neurons. The layer class will have fields to hold a collection of neurons and the number of neurons in the layer.

- **Activation functions**: Activation functions are used to introduce non-linearity into the model. The library will include classes for the popular choices like sigmoid, tanh, and ReLU

- **Weights and Biases**: weights and biases are the parameters to be learned during the training of the network, so we will implement classes to represent weights and biases that can be updated as training progresses.

- **Network**: The network class will represent the entire network, which will include fields to hold the layers, and the output. This class will also have methods for forward propagation and backpropagation

- **Loss function**: A loss function will be used to measure how far the network's predictions are from the correct output. We will include popular loss function like cross-entropy 

- **Optimization Algorithm**: An optimization algorithm will be implemented to update the weights and biases after each iteration of the training process. Stochastic Gradient Descent(SGD) is the chosen optimization algorithm

- **DataLoader**: A DataLoader will be implemented that can be used to load the MNIST dataset and preprocess the data before feeding it to the network.

## Requirements
- Rust >= 1.40.0
- Any OpenCL-compatible library for GPU acceleration (optional)

## Usage

Coming Soon...

