## Basic Neural Network in Rust

This is a very basic neural network built in Rust purely to understand rust better. There is no abstraction/deep learning library/fancy kernel that maps a classifier into a higher dimensional space. This code is probably ugly, badly written and slower than it code be.

The model architecture here is dead simple.

[28,28] Input MNIST vector --> [784,1] Input Layer --> [56,1] Hidden Layer --> [10,1] Output Layer

The cost function used is the cross entropy cost function. Apparently it works well for NN's that use sigmoids activation functions (which this one does).

Work in progress
