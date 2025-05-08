use std::slice::Windows;

// foo() -> return type
pub fn simple_moving_average( 
    x: Vec<f64>, 
    window_size: usize) -> Vec<f64> {

        // extract the length
        // of the vector
        let N = x.len();

        // slice the vector
        let int_slice = &x[..];
        let mut iter = x.windows(window_size);

        // generate an array container
        // of foats
        let output_array_lenght: usize = int_slice.len();
        let mut output_array: Vec<f64> = vec![0.0; output_array_lenght];
        //let mut output_array: [f64; output_array_lenght] = [0.00; output_array_lenght];

        println!("Length: {}", N);
        let mut idx = 0;
        for window in iter {
            let window_mean: f64 = window.iter().sum::<f64>() / window_size as f64;
            output_array[idx] = window_mean;
            idx += 1
            // unsafe {
            //     let window_mean: f64 = window.iter().sum::<f64>() / window_size as f64;
            //     println!("SMA {}", window_mean);
            // }
        }

        return output_array;
}