## Basic Neural Network in Rust

<<<<<<< HEAD
TODO :

The model architecture here is dead simple. I might change it in favour of a simple CNN if it doesn't perform but the architecture is this:

## Architecture

[784] Input MNIST vector --> [196] Layer 1 --> [49] Hidden Layer --> [10] Output Layer

**Layer 1**

- Input Dimensions: [784]
- Output Dimensions: [196]
- Weight Dimensions: [784,196]
- Bias Dimensions: [196]
- Activation Function: Softmax

**Hidden Layer**

- Input Dimensions: [196]
- Output Dimensions: [49]
- Weight Dimensions: [196,49]
- Bias Dimensions: [49]
- Activation Function: Softmax

**Output Layer**

- Input Dimensions: [49]
- Output Dimensions: [10]
- Weight Dimensions: [49,10]
- Bias Dimensions: [10]
- Activation Function: Softmax

The cost function used is the cross entropy cost function. Apparently it works well for NN's that use softmax activation functions (which this one does).

WIP
=======
This is a very basic neural network built in Rust purely to understand rust better. There is no abstraction/deep learning library/fancy kernel that maps a classifier into a higher dimensional space. This code is probably ugly, badly written and slower than it code be.

The model architecture here is dead simple.

[28,28] Input MNIST vector --> [784,1] Input Layer --> [56,1] Hidden Layer --> [10,1] Output Layer

The cost function used is the cross entropy cost function. Apparently it works well for NN's that use sigmoids activation functions (which this one does).

Work in progress
>>>>>>> 6fcefd962036c50a376caef616a6e97d5b9657ab
