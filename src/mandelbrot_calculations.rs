use num_complex::Complex;
use rayon::prelude::*;
use crate::Mandelbrot;


fn calculate_mandelbrot_at_point(c: Complex<f64>, max_iter: usize) -> usize {
    let mut z = Complex::new(0., 0.);
    for i in 0..=max_iter {
        if z.norm_sqr() > 2. {
            return i;
        }
        z = z * z + c;
    }
    max_iter
}

pub fn generate_mandelbrot(mandelbrot_config: &Mandelbrot) -> Vec<Vec<usize>>{
    let x_min = mandelbrot_config.real_start;
    let x_max = mandelbrot_config.real_end;
    let y_min = mandelbrot_config.img_start;
    let y_max = mandelbrot_config.img_end;

    let img_width_in_px = mandelbrot_config.width as usize;
    let img_height_in_px = mandelbrot_config.height as usize;

    let max_iter = mandelbrot_config.iterations;

    let mut rows: Vec<Vec<usize>> = Vec::new();

    for _ in 0..img_height_in_px {
        rows.push(vec![0; img_width_in_px]);
    }

    rows.par_iter_mut().enumerate().for_each(|(img_y, row)| {
        for img_x in 0..img_width_in_px {
            let real_component = x_min + (img_x as f64 / img_width_in_px as f64) * (x_max - x_min);
            let img_component = y_min + (img_y as f64 / img_height_in_px as f64) * (y_max - y_min);
            let c = Complex::new(real_component, img_component);
            let value = calculate_mandelbrot_at_point(c, max_iter);
            row[img_x] = value;
        }
    });

    rows
}