use clap::Parser;
mod dataloader;
mod model;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args{
    /// Either `train` to train the model or `inference` to provide an image file to inference
    #[clap(short, long)]
    command: String
    
}

/// Main Function
/// Exposes the argument parser and invokes either training or inferencing of the model
fn main() {
    let args = Args::parse();

    // TRAINING
    if args.command == "train" {
        let training_file = String::from("./data/train-images-idx3-ubyte");
        let label_file = String::from("./data/train-images-idx3-ubyte");
        
        // load the MNIST data as a Dataset struct
        println!("Loading MNIST dataset (might take a while...)...");
        let mnist_dataset = dataloader::read_mnist_training_set(&label_file, &training_file);
        println!("Loaded!")



    }
    else{
        println!("Command not found!")
    }
}
