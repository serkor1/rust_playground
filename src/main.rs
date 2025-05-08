use rust_playground::simple_approach::sma::simple_moving_average;
use rust_playground::optimized_approach::sma_struct::sma_container;

fn main() {
    // construct vector
    // NOTE: the vec!-macro is a must. That is just
    // how it works. Otherwise its an array.
    let vector_numeric: Vec<f64> = vec![1.2, 1.3, 1.9, 1.2, 99.1];

    // call function
    simple_moving_average(vector_numeric, 2);


    // call optimized function
    // let container = sma_container {
    //     x: vector_numeric, // might be illegal to reuse. Lets see.
    //     window_size: 2
    // };
    // Error: error[E0382]: use of moved value: `vector_numeric`
    // For more information about this error, try `rustc --explain E0382`.
    // For later studies
    let new_vector: Vec<f64> = vec![1.2, 1.3, 1.9, 1.2, 99.1];

    let container = sma_container {
        x: new_vector, // might be illegal to reuse. Lets see.
        window_size: 2
    };

    container.calculate();
    
}
