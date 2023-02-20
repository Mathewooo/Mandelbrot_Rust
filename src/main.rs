use std::env;
use std::fs::File;
use std::io::BufWriter;
use rayon::prelude::*;

use num_complex::Complex;

use mandelbrot::mandelbrot::in_set;
use parse::parse::*;

mod parse;
mod mandelbrot;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 5 {
        eprintln!(
            "Usage: {} FILE, PIXELS, \
            UPPER_LEFT, LOWER_RIGHT", args[0]
        );
        eprintln!(
            "Example: {} mandelbrot.png 400x750 \
             -1.20,0.35 -1,0.20", args[0]
        );
        std::process::exit(1);
    }

    let bounds = parse_pair(&args[2], 'x')
        .expect("Error parsing image dimensions!");
    let upper_left = parse_complex(&args[3])
        .expect("Error parsing upper left corner point!");
    let lower_right = parse_complex(&args[4])
        .expect("Error parsing lower right corner point!");

    let mut pixels = vec![0; bounds.0 * bounds.1];

    parallel_render(&mut pixels, bounds, upper_left, lower_right);
    write_image(&args[1], &pixels, bounds)
}

fn write_image(filename: &str, pixels: &[u8],
               bounds: (usize, usize),
) {
    let file = File::create(filename).unwrap();
    let ref mut w
        = BufWriter::new(file);
    let mut encoder = png::Encoder::new(
        w, bounds.0 as u32, bounds.1 as u32,
    );
    encoder.set_color(png::ColorType::Grayscale);
    encoder.write_header().unwrap()
        .write_image_data(&pixels)
        .expect("Error!");
}

fn pixel_to_point(bounds: (usize, usize), pixel: (usize, usize),
                  upper_left: Complex<f64>,
                  lower_right: Complex<f64>,
) -> Complex<f64> {
    let (width, height) = (
        lower_right.re - upper_left.re,
        upper_left.im - lower_right.im
    );
    Complex {
        re: upper_left.re + pixel.0 as f64
            * width / bounds.0 as f64,
        im: upper_left.im - pixel.1 as f64
            * height / bounds.1 as f64,
    }
}

fn render(pixels: &mut [u8],
          bounds: (usize, usize),
          upper_left: Complex<f64>,
          lower_right: Complex<f64>,
) {
    assert_eq!(pixels.len(), bounds.0 * bounds.1);
    for row in 0..bounds.1 {
        for column in 0..bounds.0 {
            let point = pixel_to_point(
                bounds, (column, row), upper_left, lower_right,
            );
            pixels[row * bounds.0 + column] = match in_set(
                point, 255,
            ) {
                None => 0,
                Some(count) => 255 - count as u8
            }
        }
    }
}

fn parallel_render(pixels: &mut [u8],
                   bounds: (usize, usize),
                   upper_left: Complex<f64>,
                   lower_right: Complex<f64>,
) {
    let bands: Vec<(usize, &mut [u8])> = pixels
        .chunks_mut(bounds.0)
        .enumerate()
        .collect();

    bands.into_par_iter()
        .for_each(|(i, band)| {
        let top = i;
        let band_bounds = (bounds.0, 1);
        let band_upper_left = pixel_to_point(
            bounds, (0, top), upper_left, lower_right
        );
        let band_lower_right = pixel_to_point(
            bounds, (bounds.0, top + 1), upper_left, lower_right
        );
        render(
            band, band_bounds,
            band_upper_left, band_lower_right
        );
    });
}
