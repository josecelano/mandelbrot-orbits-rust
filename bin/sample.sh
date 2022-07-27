#!/bin/bash

# Generate sample image
cargo run ./output/mandelbrot_2000x2000.png 2000x2000 -2.0,2.00 2.0,-2.0

# Show image
eog output/mandelbrot_2000x2000.png