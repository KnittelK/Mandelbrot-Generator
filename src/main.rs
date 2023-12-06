mod mandelbrot_calculations;
mod renders;

use clap::Parser;
use crate::renders::terminal::render_mandelbrot_set_in_terminal;
use crate::renders::image::generate_mandelbrot_image;


#[derive(Parser)]
struct Mandelbrot {

    /// The width of the image to be generated
    #[arg(long, default_value_t = 2560)]
    width: u32,

    /// The height of the image generated
    #[arg(long, default_value_t = 1440)]
    height: u32,

    #[arg(long, default_value_t = -2.)]
    real_start: f64,

    #[arg(long, default_value_t = 1.)]
    real_end: f64,

    #[arg(long, default_value_t = -1.)]
    img_start: f64,

    #[arg(long, default_value_t = 1.)]
    img_end: f64,

    #[arg(short, long, default_value_t = 1000)]
    iterations: usize,

    #[arg(long)]
    image: bool,
}


fn main() {
    let mandelbrot_config = Mandelbrot::parse();
    if mandelbrot_config.image {
        generate_mandelbrot_image(&mandelbrot_config)
    } else {
        render_mandelbrot_set_in_terminal(&mandelbrot_config);
    }
}
