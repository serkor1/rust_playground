/**
 * Unsafe approach:
 * 
 * Use pointers
 */

pub fn simple_moving_average_pointer(
    x: &Vec<f64>,
    window_size: usize) -> Vec<f64> {

        // extract pointer
        // not mutable - we are geneating
        // a new container
        let x_ptr: *const f64 = x.as_ptr();

        // calculate output vector
        // size.
        // In essence we are mising
        // the same number indices as the
        // window length 
        let array_length: usize = x.len() - (window_size - 1);
        let mut output_array: Vec<f64> = vec![0.0; array_length];

        for idx in 0..array_length {
            let mut sum = 0.0;
            for j in 0..window_size {
                unsafe {
                    sum += *x_ptr.add(idx + j);
                }
            }
            output_array[idx] = sum / (window_size as f64);
        }

        return output_array;
}