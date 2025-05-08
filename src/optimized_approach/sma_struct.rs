
// See here: https://doc.rust-lang.org/stable/book/ch18-03-oo-design-patterns.html
// for more details
//
// These structs are similar to Python
// class in  syntax
pub struct SMAStruct {
    // holds the passed 
    // series
    pub x: Vec<f64>,
    // holds the resulting
    // series
    // NOTE: This could (potentially) be an array
    // the size can be precalculated - could be
    // faster.
    // (For later)
    // y: Vec<f64>
    // '---> Incorrect. According to: https://doc.rust-lang.org/book/ch05-03-method-syntax.html
    // its supposed to the passed args. Not returned.
    // 
    // define window_size instead
    pub window_size: usize

    // If the element is NOT
    // pub(lic), then its private,
    // and used internally, ie. you cant pass 
    // values.
    // See rustc --explain E0451
    // Also: https://doc.rust-lang.org/stable/book/ch05-03-method-syntax.html?highlight=method#defining-methods
    // "IT WORKS IN THE EXAMPLE...."
}

impl SMAStruct {
    pub fn calculate(&self) -> () {
        // the same implementation
        // but with more steps

        // calculate length
        let vector_size: usize = self.x.len();

        // define slices
        // let int_slice = &self.x[..];
        let iter = self.x.windows(self.window_size);

        println!("Length: {}", vector_size);
        for window in iter {
            let window_mean: f64 = window.iter().sum::<f64>() / self.window_size as f64;
            println!("SMA {}", window_mean);
            // unsafe {
            //     let window_mean: f64 = window.iter().sum::<f64>() / self.window_size as f64;
            //     println!("SMA {}", window_mean);
            // }
            // NOTE: unsafe is for pointer arithmatrics. Not iterators.
        }
    }
}