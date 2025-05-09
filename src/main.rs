use rust_playground::simple_approach::sma::simple_moving_average;
use rust_playground::optimized_approach::sma_struct::SMAStruct;
use rust_playground::unsafe_approach::sma::simple_moving_average_pointer;

fn main() {
    // construct vector
    // NOTE: the vec!-macro is a must. That is just
    // how it works. Otherwise its an array.
    let vector_numeric: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];

    // call function
    let output_value =simple_moving_average(&vector_numeric, 2);
    println!(
        "First {:?}", output_value
    );

    // call optimized function
    // let container = sma_container {
    //     x: vector_numeric, // might be illegal to reuse. Lets see.
    //     window_size: 2
    // };
    // Error: error[E0382]: use of moved value: `vector_numeric`
    // For more information about this error, try `rustc --explain E0382`.
    // For later studies
    let new_vector: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];

    let container = SMAStruct {
        x: new_vector, // might be illegal to reuse. Lets see.
        window_size: 2
    };

    container.calculate();


    // unsafe approach
    let output_value = simple_moving_average_pointer(&vector_numeric, 2);
    println!("Pointer: {:?}", output_value);

    
}
