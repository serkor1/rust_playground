use std::slice::Windows;

pub fn simple_moving_average(
    x: Vec<f64>, 
    window_size: usize) {

        // extract the length
        // of the vector
        let N = x.len();

        // slice the vector
        let int_slice = &x[..];
        let mut iter = x.windows(window_size);

        println!("Length: {}", N);
        for window in iter {
            unsafe {
                let window_mean: f64 = window.iter().sum::<f64>() / window_size as f64;
                println!("SMA {}", window_mean);
            }
        }
}