use rust_playground::simple_approach::sma::simple_moving_average;

fn main() {
    // construct vector
    // NOTE: the vec!-macro is a must. That is just
    // how it works. Otherwise its an array.
    let vector_numeric: Vec<f64> = vec![1.2, 1.3, 1.9, 1.2, 99.1];

    // call function
    simple_moving_average(vector_numeric, 2)
    
}
