use crate::Mandelbrot;
use crate::mandelbrot_calculations::generate_mandelbrot;

pub fn render_mandelbrot_set_in_terminal(config: &Mandelbrot) {
    let values = generate_mandelbrot(&config);
    for row in values {
        let mut line = String::with_capacity(row.len());
        for column_val in row {
            let pixel_repr = match column_val {
                0..=2 => ' ',
                3..=5 => '.',
                6..=10 => 'ꔷ',
                11..=30 => '*',
                31..=100 => '+',
                101..=200 => 'x',
                201..=400 => '$',
                401..=700 => '#',
                _ => '%'
            };
            line.push(pixel_repr);
        }
        println!("{}", line);
    }
}
