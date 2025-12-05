// Example: ML Model Training and Inference

use gul_ml::{Activation, Inference, Layer, Model, Trainer};

fn main() -> Result<(), String> {
    println!("GUL ML Example - Image Classification\n");

    // Create model
    let mut model = Model::new("mnist_classifier");

    // Add layers
    model.add_layer(Layer::Dense {
        input_size: 784, // 28x28 images
        output_size: 128,
        activation: Activation::ReLU,
    });

    model.add_layer(Layer::Dropout { rate: 0.2 });

    model.add_layer(Layer::Dense {
        input_size: 128,
        output_size: 64,
        activation: Activation::ReLU,
    });

    model.add_layer(Layer::Dense {
        input_size: 64,
        output_size: 10, // 10 classes
        activation: Activation::Softmax,
    });

    println!("{}\n", model.summary());

    // Train model
    println!("Training model...");
    let mut trainer = Trainer::new(model)
        .learning_rate(0.001)
        .batch_size(32)
        .epochs(10);

    // Mock training data
    let x_train = vec![vec![0.0; 784]; 1000];
    let y_train = vec![vec![0.0; 10]; 1000];

    let history = trainer.train(&x_train, &y_train)?;
    println!("Final loss: {:.4}", history.final_loss());
    println!("Best accuracy: {:.4}\n", history.best_accuracy());

    // Inference
    println!("Running inference...");
    let inference = Inference::new(Model::new("mnist_classifier"));

    let test_input = vec![0.0; 784];
    let prediction = inference.predict(&test_input)?;

    println!("Prediction: {:?}", prediction);
    println!(
        "Predicted class: {}",
        prediction
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .map(|(idx, _)| idx)
            .unwrap()
    );

    Ok(())
}
