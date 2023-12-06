use image::{ImageBuffer};
use crate::Mandelbrot;
use crate::mandelbrot_calculations::generate_mandelbrot;


pub fn generate_mandelbrot_image(config: &Mandelbrot) {
    let mut img = ImageBuffer::new(
        config.width,
        config.height
    );

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let r = (0.3 * x as f32) as u8;
        let b = (0.3 * y as f32) as u8;
        *pixel = image::Rgb([r, 0, b]);
    }

    let data = generate_mandelbrot(config);
    for (y, row) in data.iter().enumerate() {
        for (x, col_val) in row.iter().enumerate() {
            let pixel = img.get_pixel_mut(x as u32, y as u32);
            let image::Rgb(data) = *pixel;
            *pixel = image::Rgb([data[0], *col_val as u8, data[2]]);
        }
    }
    img.save("fractal.png").unwrap()
}