use rust_playground::simple_approach::sma::{simple_moving_average};

#[test]
fn it_return_correct_values() {
    // target value
    let target_value: Vec<f64> = vec![1.5, 2.5, 3.5, 4.5, 5.5, 0.0];

    // input value
    let input_value: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
    let result: Vec<f64> = simple_moving_average(&input_value, 2);

    assert_eq!(result, target_value);
}