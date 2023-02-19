pub mod mandelbrot {
    use num_complex::Complex;

    pub fn in_set(c: Complex<f64>, limit: usize) -> Option<usize> {
        let mut z = Complex { re: 0.0, im: 0.0 };
        for i in 0..limit {
            if z.norm_sqr() > 4.0 {
                return Some(i);
            }
            z = z * z + c;
        }
        None
    }
}