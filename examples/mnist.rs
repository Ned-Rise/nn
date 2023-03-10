extern crate blas_src;

use ndarray::{s, Array1, Array2};
use nn::conversions::one_hot_encode;
use nn::{Layer, Loss, Model, Sequential};

pub fn load_mnist_dataset(
) -> ((Array2<f64>, Array2<f64>), (Array2<f64>, Array2<f64>)) {
    let image_size = 28 * 28;

    let x_train_raw: Vec<f64> =
        mnist_read::read_data("data/mnist/train-images.idx3-ubyte")
            .into_iter()
            .map(|d| d as f64 / 255.0)
            .collect();

    let y_train_raw: Vec<f64> =
        mnist_read::read_labels("data/mnist/train-labels.idx1-ubyte")
            .into_iter()
            .map(|l| l as f64)
            .collect();

    let x_test_raw: Vec<f64> =
        mnist_read::read_data("data/mnist/t10k-images.idx3-ubyte")
            .into_iter()
            .map(|d| d as f64 / 255.0)
            .collect();

    let y_test_raw: Vec<f64> =
        mnist_read::read_labels("data/mnist/t10k-labels.idx1-ubyte")
            .into_iter()
            .map(|l| l as f64)
            .collect();

    let x_train: Array2<f64> = Array2::from_shape_vec(
        (x_train_raw.len() / image_size, image_size),
        x_train_raw,
    )
    .unwrap();

    let y_train = one_hot_encode(Array1::<f64>::from_vec(y_train_raw));

    println!(
        "\nx_train: {:?}, y_train: {:?}",
        x_train.shape(),
        y_train.shape()
    );

    let x_test: Array2<f64> = Array2::from_shape_vec(
        (x_test_raw.len() / image_size, image_size),
        x_test_raw,
    )
    .unwrap();

    let y_test = one_hot_encode(Array1::<f64>::from_vec(y_test_raw));

    println!(
        "x_test: {:?}, y_test: {:?}\n",
        x_test.shape(),
        y_test.shape()
    );

    ((x_train, y_train), (x_test, y_test))
}

fn main() {
    let epochs = 20;
    let batch_size = 10000;
    let learning_rate = 0.1;

    let ((x_train, y_train), _) = load_mnist_dataset();

    let mut model = Sequential::new();
    model.add_layer(Layer::new_dense(
        x_train.ncols(),
        800,
        Some(learning_rate),
    ));
    model.add_layer(Layer::new_relu());
    model.add_layer(Layer::new_dropout(800, 1, 10));
    model.add_layer(Layer::new_dense(
        800,
        y_train.ncols(),
        Some(learning_rate),
    ));
    model.set_loss_fn(Loss::new_scce());

    println!("Training data has {} datapoints.\n", x_train.shape()[0]);
    println!("Starting training\n");

    let number_of_batches =
        (x_train.shape()[0] as f64 / batch_size as f64).floor() as i32;

    println!(
        "\nWill train {} epochs each with {} batches of size {}",
        epochs, number_of_batches, batch_size
    );

    for e in 0..epochs {
        println!("\nEpoch: {}", e + 1);

        for i in 0..number_of_batches {
            let start = i * batch_size;
            let end = start + batch_size;

            let x_batch = x_train.slice(s![start..end, ..]).to_owned();
            let y_batch = y_train.slice(s![start..end, ..]).to_owned();

            model.fit(&x_batch, &y_batch);

            println!(
                "[{}/{}]: loss: {:.4} acc: {:.4}",
                end,
                x_train.nrows(),
                model.loss(&x_batch, &y_batch),
                model.accuracy(&x_batch, &y_batch),
            );
        }
    }

    model.save("data/models/mnist.json").unwrap();

    let loaded_model = Sequential::load("data/models/mnist.json").unwrap();

    let x_test = x_train.slice(s![0..4usize, ..]).to_owned();
    let y_test = y_train.slice(s![0..4usize, ..]).to_owned();

    println!(
        "loaded model acc: {:.4}",
        loaded_model.accuracy(&x_test, &y_test),
    );
}
