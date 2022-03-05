
trait Neuron{
    /// Feed-forward layer.
    /// The feed forward layer takes a 1D input vector, corresponding to a row of 
    /// pixels of the image, and outputs the activation threshold, a float from [0,1]
    fn forward(&self, input_vector : Vec<i32>) -> f32;
}