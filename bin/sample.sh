#!/bin/bash

# Generate sample image
time cargo run ./output/mandelbrot_2048x2048.png 2048x2048 -2.0,2.00 2.0,-2.0

# Show image
eog output/mandelbrot_2048x2048.png
