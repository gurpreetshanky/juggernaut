extern crate juggernaut;

use juggernaut::nl::NeuralLayer;
use juggernaut::nn::NeuralNetwork;
use juggernaut::activation::Sigmoid;
use juggernaut::sample::Sample;
use juggernaut::matrix::MatrixTrait;


fn main() {
    println!("Juggernaut...");

    let dataset = vec![
        Sample::new(vec![0f64, 0f64, 1f64], vec![0f64]),
        Sample::new(vec![0f64, 1f64, 1f64], vec![0f64]),
        Sample::new(vec![1f64, 0f64, 1f64], vec![1f64]),
        Sample::new(vec![1f64, 1f64, 1f64], vec![1f64]),
    ];

    println!("Creating the network...");

    let mut test = NeuralNetwork::new();

    let sig_activation = Sigmoid::new();
    // 1st layer = 2 neurons - 3 inputs
    test.add_layer(NeuralLayer::new(2, 3, sig_activation));

    println!("First layer created: 2 neurons 3 inputs");

    // 2nd layer = 1 neuron - 2 inputs
    test.add_layer(NeuralLayer::new(1, 2, sig_activation));

    println!("Second layer created: 1 neuron 2 inputs");

    println!("Training (60,000 epochs)...");

    test.on_error(|err| {
        println!("error({})", err.to_string());
    });

    test.train(dataset, 1000, 0.1f64);

    println!("Done!!");

    let think = test.evaluate(&Sample::predict(vec![1f64, 0f64, 1f64]));

    println!("Evaluate [1, 0, 1] = {:?}", think.get(0, 0));
}
