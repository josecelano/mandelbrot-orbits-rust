# Mandelbrot Orbits

[![Test](https://github.com/josecelano/mandelbrot-orbits-rust/actions/workflows/test.yml/badge.svg)](https://github.com/josecelano/mandelbrot-orbits-rust/actions/workflows/test.yml) [![MegaLinter](https://github.com/josecelano/mandelbrot-orbits-rust/actions/workflows/mega-linter.yml/badge.svg)](https://github.com/josecelano/mandelbrot-orbits-rust/actions/workflows/mega-linter.yml)

This is a small project to learn Rust.

It generates an image of the Mandelbrot Set where each pixel color represents the period of the complex point if the point belongs to the Set.

More info about [Mandelbrot Set Periods](https://github.com/josecelano/mandelbrot-explorer).

> WIP: This is a work in progress. For now, it only generates a gray scale image like this:

![./docs/images/mandelbrot_2000x2000.png](./docs/images/mandelbrot_2000x2000.png)

## Requirements

- `cargo`: ^1.62.1.
- `rustc`: ^1.62.1.

## Use

```s
git clone git@github.com:josecelano/mandelbrot-orbits-rust.git
cargo run ./output/mandelbrot_2000x2000.png 2000x2000 -2.0,2.00 2.0,-2.0
```

## Development

Execute MegaLinter locally:

```s
./bin/ml.sh
```

Run only  a single test:

```s
cargo test test_lambda
```

## Credits

- [Jeremy Rifkin](https://github.com/jeremy-rifkin) pointed me at the solution. I have copied/pasted part of his code to detect the period.
- [Jim Blandy, Programming Rust Book's Author](https://github.com/jimblandy).
- [Jason Orendorff, Programming Rust Book's Author](https://github.com/jorendorff).

## Links

- [Same thing in C++ by Jeremy Rifkin](https://github.com/jeremy-rifkin/mandelbrot-orbits).
- [An Introduction to Complex Dynamics and the Mandelbrot Set by professor Robert L. Benedetto](https://rlbenedetto.people.amherst.edu/talks/mhc_ug14.pdf).
- [Initial code to plot the Mandelbrot Set was forked from the O'Reilly Programming Rust book](https://github.com/ProgrammingRust/mandelbrot/tree/single-threaded).
- [Periodic points in the Mandelbrot set](https://www.ibiblio.org/e-notes/MSet/cperiod.htm).
- [Mandelbrot Trajectory Infima](https://github.com/jeremy-rifkin/mandelbrot-trajectory-infima).
- [Displaying the Internal Structure of the Mandelbrot Set by Adam Cunningham](https://www.acsu.buffalo.edu/~adamcunn/downloads/MandelbrotSet.pdf).
- [Mandelbrot orbits explorer](https://mandelbrot-set-periods.online/).
- [Mandelbrot trajectories explorer](https://rifkin.dev/projects/mandelbrot-trajectories/).
- [Mandelbrot in C language with arbitrary precision](https://github.com/josecelano/c-mandelbrot-arbitrary-precision).

## Books

- [The Beauty of Fractals](https://en.wikipedia.org/wiki/The_Beauty_of_Fractals).
- [Programming Rust Book, O'Reilly](https://www.oreilly.com/library/view/programming-rust-2nd/9781492052586/).
