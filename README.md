# Mandelbrot
A CLI binary file that generates a mandelbrot set within the terminal or stores it as a png file if `-image` flag is set, otherwise the resulting mandelbrot is displayed on the terminal.

`Rayon` is used to parallelize the computation of the mandelbrot set to improve performance when dealing with high resolution images and/or when the iterations are set to a high value.