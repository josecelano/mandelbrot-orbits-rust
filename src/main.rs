#![warn(rust_2018_idioms)]
#![allow(elided_lifetimes_in_paths)]

use num::Complex;
use num::complex::ComplexFloat;

/// Try to determine if `c` is in the Mandelbrot set, using at most `limit`
/// iterations to decide.
///
/// If `c` is not a member, return `Some(i)`, where `i` is the number of
/// iterations it took for `c` to leave the circle of radius two centered on the
/// origin. If `c` seems to be a member (more precisely, if we reached the
/// iteration limit without being able to prove that `c` is not a member),
/// return `None`.
fn escape_time(c: Complex<f64>, limit: usize) -> Option<usize> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..limit {
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
        z = z * z + c;
    }

    None
}

use std::str::FromStr;

/// Parse the string `s` as a coordinate pair, like `"400x600"` or `"1.0,0.5"`.
///
/// Specifically, `s` should have the form <left><sep><right>, where <sep> is
/// the character given by the `separator` argument, and <left> and <right> are both
/// strings that can be parsed by `T::from_str`.
///
/// If `s` has the proper form, return `Some<(x, y)>`. If it doesn't parse
/// correctly, return `None`.
fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.find(separator) {
        None => None,
        Some(index) => {
            match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
                (Ok(l), Ok(r)) => Some((l, r)),
                _ => None
            }
        }
    }
}

#[test]
fn test_parse_pair() {
    assert_eq!(parse_pair::<i32>("",        ','), None);
    assert_eq!(parse_pair::<i32>("10,",     ','), None);
    assert_eq!(parse_pair::<i32>(",10",     ','), None);
    assert_eq!(parse_pair::<i32>("10,20",   ','), Some((10, 20)));
    assert_eq!(parse_pair::<i32>("10,20xy", ','), None);
    assert_eq!(parse_pair::<f64>("0.5x",    'x'), None);
    assert_eq!(parse_pair::<f64>("0.5x1.5", 'x'), Some((0.5, 1.5)));
}

/// Parse a pair of floating-point numbers separated by a comma as a complex
/// number.
fn parse_complex(s: &str) -> Option<Complex<f64>> {
    match parse_pair(s, ',') {
        Some((re, im)) => Some(Complex { re, im }),
        None => None
    }
}

#[test]
fn test_parse_complex() {
    assert_eq!(parse_complex("1.25,-0.0625"),
               Some(Complex { re: 1.25, im: -0.0625 }));
    assert_eq!(parse_complex(",-0.0625"), None);
}

/// Given the row and column of a pixel in the output image, return the
/// corresponding point on the complex plane.
///
/// `bounds` is a pair giving the width and height of the image in pixels.
/// `pixel` is a (column, row) pair indicating a particular pixel in that image.
/// The `upper_left` and `lower_right` parameters are points on the complex
/// plane designating the area our image covers.
fn pixel_to_point(bounds: (usize, usize),
                  pixel: (usize, usize),
                  upper_left: Complex<f64>,
                  lower_right: Complex<f64>)
    -> Complex<f64>
{
    let (width, height) = (lower_right.re - upper_left.re,
                           upper_left.im - lower_right.im);
    Complex {
        re: upper_left.re + pixel.0 as f64 * width  / bounds.0 as f64,
        im: upper_left.im - pixel.1 as f64 * height / bounds.1 as f64
        // Why subtraction here? pixel.1 increases as we go down,
        // but the imaginary component increases as we go up.
    }
}

#[test]
fn test_pixel_to_point() {
    assert_eq!(pixel_to_point((100, 200), (25, 175),
                              Complex { re: -1.0, im:  1.0 },
                              Complex { re:  1.0, im: -1.0 }),
               Complex { re: -0.5, im: -0.75 });
}

/// Render a rectangle of the Mandelbrot set into a buffer of pixels.
///
/// The `bounds` argument gives the width and height of the buffer `pixels`,
/// which holds one grayscale pixel per byte. The `upper_left` and `lower_right`
/// arguments specify points on the complex plane corresponding to the upper-
/// left and lower-right corners of the pixel buffer.
fn render(pixels: &mut [u8],
          bounds: (usize, usize),
          upper_left: Complex<f64>,
          lower_right: Complex<f64>)
{
    assert!(pixels.len() == bounds.0 * bounds.1);

    for row in 0..bounds.1 {
        for column in 0..bounds.0 {
            let point = pixel_to_point(bounds, (column, row),
                                       upper_left, lower_right);

            pixels[row * bounds.0 + column] =
                match escape_time(point, 255) {
                    None => {
                        // Mandelbrot Set point
                        // Calculate period
                        let z0 = Complex { re: 0.0, im: 0.0 };
                        let period = calculate_period(z0, point);

                        let color = match period {
                            0 => 210, // Belong to Mandelbrot Set but we cannot calculate the period
                            1 => 0,   // Black
                            2 => 100,
                            3 => 150,
                            4 => 160,
                            5 => 170,
                            6 => 180,
                            7 => 190,
                            _ => 200,
                        };

                        color
                    },
                    // Not a Mandelbrot Set point
                    //Some(count) => 255 - count as u8 // With grayscale depending on the escape time
                    Some(_count) => 255 // White if it's not in the Mandelbrot Set
                };
        }
    }
}

use image::ImageEncoder;
use image::ColorType;
use image::codecs::png::PngEncoder;
use std::fs::File;
use image::ImageError;

/// Write the buffer `pixels`, whose dimensions are given by `bounds`, to the
/// file named `filename`.
fn write_image(filename: &str, pixels: &[u8], bounds: (usize, usize))
    -> Result<(), ImageError>
{
    let output = File::create(filename)?;

    let encoder = PngEncoder::new(output);

    encoder.write_image(&pixels,
                   bounds.0 as u32, bounds.1 as u32,
                   ColorType::L8)?;

    Ok(())
}

use std::env;
use text_colorizer::*;

fn print_usage(app_name: &String) {
    eprintln!("Usage: {} FILE PIXELS UPPERLEFT LOWERRIGHT", app_name.green());
    eprintln!("Example: {} mandel.png 1000x750 -1.20,0.35 -1,0.20", app_name.green());
}

#[derive(Debug)]
struct Arguments {
    filepath: String,
    bounds: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
}

fn parse_args() -> Arguments {
    let args: Vec<String> = env::args().collect();

    if args.len() - 1 != 4 {
        print_usage(&args[0]);
        eprintln!("{} wrong number of arguments: expected 4, got {}", "Error:".red().bold(), args.len() - 1);
        std::process::exit(1);
    }

    Arguments {
        filepath: args[1].clone(),
        bounds: parse_pair(&args[2], 'x')
        .expect("error parsing image dimensions"),
        upper_left: parse_complex(&args[3])
        .expect("error parsing upper left corner point"),
        lower_right: parse_complex(&args[4])
        .expect("error parsing lower right corner point"),
    }
}

/// The Mandelbrot Set base formula:
/// φ(z) = z² + c = (z * z) + c
fn phi(z: Complex<f64>, c: Complex<f64>) -> Complex<f64> {
    (z * z) + c
}

#[test]
fn test_phi() {
    let z = Complex { re: 1., im: 1. };
    let c = Complex { re: 1., im: 1. };
    let result = phi(z, c);
    let expected_result = (z * z) + c; // 1+3i
    assert_eq!(result, expected_result, "expected φ(z) = z² + c = {:?} where z = {:?}, got {:?}", expected_result, z, result);
}

/*
std::complex<fp> phi_n(int n, std::complex<fp> z, const std::complex<fp> c) {
	while(n--) {
		z = z * z + c;
	}
	return z;
}
*/

/// N recursive iterations of the base Mandelbrot formula:
/// φn(z) = (((z² + c)²) + c)² + c where n = 3
/// 
/// φ(z) = z² + c = z1
/// φ(z) = z1² + c = z2
/// ...
/// φ(zm) = zm² + c = zn
/// 
/// Where zm = n -1
fn phi_n(z: Complex<f64>,c: Complex<f64>, n: usize) -> Complex<f64> {
    let mut result = z.clone();
    for _iter in 1..=n {
        result = phi(result, c);
    }
    result
}

#[test]
fn test_phi_n() {
    let z = Complex { re: 1., im: 1. };
    let c = Complex { re: 1., im: 1. };

    // For n = 1, φ1(z) = z² + c = 1+3i
    let n1: usize = 1;
    let result1 = phi_n(z, c, n1);
    let expected_result1 = (z * z) + c; // 1+3i
    assert_eq!(result1, expected_result1, "expected φn(z) = {:?} where (z, c, n) = ({:?}, {:?}, {:?}), got {:?}", expected_result1, z, c, n1, result1);

    // For n = 2, φ2(z) = φ1(z)² + c = ???
    let n2: usize = 2;
    let result2 = phi_n(z, c, n2);
    let expected_result2 = (result1 * result1) + c; // -7+7i
    assert_eq!(result2, expected_result2, "expected φn(z) = {:?} where (z, c, n) = ({:?}, {:?}, {:?}), got {:?}", expected_result2, z, c, n2, result2);    
}

/*
std::complex<fp> phi_prime(const std::complex<fp> z, [[maybe_unused]] const std::complex<fp> c) {
	return 2. * z;
}
*/

/// The derivative of the Mandelbrot Set base formula: 
/// φ'(z) = 2 * z
fn phi_prime(z: Complex<f64>) -> Complex<f64> {
    2. * z
}

#[test]
fn test_phi_prime() {
    let z = Complex { re: 1., im: 1. };
    let result = phi_prime(z);
    let expected_result = Complex { re: 2., im: 2. };
    assert_eq!(result, expected_result, "expected φ'(z) = 2 * z = {:?} where z = {:?}, got {:?}", expected_result, z, result);
}

/*
std::complex<fp> lambda(const int n, const std::complex<fp> z, const std::complex<fp> c) {
	std::complex<fp> lambda = phi_prime(z, c);
	for(int i = 1; i < n; i++) {
		lambda *= phi_prime(phi_n(i, z, c), c);
	}
	return lambda;
}
*/

/// λ multiplier function of a point "c"
fn lambda(z: Complex<f64>,c: Complex<f64>, n: usize) -> Complex<f64> {

    let mut result = phi_prime(c);

    // DEBUG
    // print!("{ } phi_prime for z ({:?},{:?}): {:?}\n", n, z.re(), z.im(), result);

    for iter in 1..n {
        result = result * phi_prime(phi_n(z, c, iter));
        
        // DEBUG
        //print!("{ } lambda for c ({:?},{:?}): {:?}\n", iter, c.re(), c.im(), result);
    }
    result
}

#[test]
fn test_lambda() {
    /* 
    
    From: https://www.ibiblio.org/e-notes/MSet/cperiod.htm
    
    Here are some preperiodic points with period 1. 
    All these points lie outside the main cardioid and the relevant fixed points are repelling.
    A point is preperiodic with period n if its critical orbit becomes periodic with period n after k (a finite number) steps.
    
    -----------------------------------------------------
    | num | k |	c	               | |λ|     | Arg(λ)o  |
    -----------------------------------------------------
    | 1	  | 2 |	-2	               | 4	     |  0       |
    | 2	  | 3 |	-1.54369	       | 1.67857 |  180     |
    | 3	  | 3 |	-0.22816+1.11514i  | 3.08738 | -23.126  |
    | 4	  | 4 |	-1.89291	       | 1.92774 |  180     |
    | 5	  | 4 |	-1.29636+0.44185i  | 3.52939 | -5.7209  |
    | 6	  | 4 |	-0.10110+0.95629i  | 1.32833 |  119.553 |
    | 7   | 4 |	 0.34391+0.70062i  | 2.45805 | -30.988  |
    -----------------------------------------------------

    */

    /*
    let z0 = Complex { re: 0., im: 0. };
    assert_eq!(lambda(z0, Complex { re: -2., im: 0. }, 1).abs(), 4.);
    assert_eq!(lambda(z0, Complex { re: -1.54369, im: 0. }, 0).abs(), 1.67857);
    assert_eq!(lambda(z0, Complex { re: -0.22816, im: 1.11514 }, 1).abs(), 3.08738);
    assert_eq!(lambda(z0, Complex { re: -1.89291, im: 0. }, 1).abs(), 1.9277203709764796);
    assert_eq!(lambda(z0, Complex { re: -1.29636, im: 0.44185 }, 1).abs(), 3.5293950841929753);
    assert_eq!(lambda(z0, Complex { re: -0.10110, im: 0.95629 }, 1).abs(), 1.3283565532026762);
    assert_eq!(lambda(z0, Complex { re: 0.34391, im: 0.70062 }, 1).abs(), 2.4580724661444995);
    */

    let z = Complex { re: 0., im: 0. };
    let c = Complex { re: 1., im: 1. };

    // For n = 1
    let n1: usize = 1;
    let result1 = lambda(z, c, n1);
    let expected_result1 = Complex { re: 2.0, im: 2.0 };
    assert_eq!(result1, expected_result1, "expected λ(z,c,n) = {:?} where (z, c, n) = ({:?}, {:?}, {:?}), got {:?}", expected_result1, z, c, n1, result1);

}

/*
bool is_period(const int n, std::complex<fp> z, const std::complex<fp> c) {
	for(int i = 0; i < max_period; i++) {
		if(std::abs(lambda(n, z, c)) >= 1) {
			return false;
		}
		z = z * z + c;
	}
	return true;
}
*/

/// It checks is point "c" has a period of "p".
/// If |λ| < 1, the point is attracting.
/// If |λ| > 1, the point is repelling.
/// If |λ| = 1, the point is indifferent.
fn is_period_p(z: Complex<f64>,c: Complex<f64>, n: usize) -> bool {
    let max_period = 40;

    let mut result = z.clone();

    for _iter in 0..max_period {

        // DEBUG
        // print!("{ } calling lambda with (result, c, n) = ({:?}, {:?}, {:?})\n", iter, result, c , n);

        let lambda = lambda(result, c, n);
        let lambda_abs = lambda.abs();

        // DEBUG
        // print!("{ } lambda modulus for c ({:?},{:?}): {:?} where λ = {:?} \n", iter, c.re(), c.im(), lambda, lambda_abs);

        if lambda_abs >= 1. {
            return false;
        }

        result = phi(result, c);
    }

    true
}

#[test]
fn test_is_period() {

    let z = Complex { re: 0., im: 0. };

    // Point with period of 1
    let c1 = Complex { re: 0., im: 0. };
    assert_eq!(is_period_p(z, c1, 1), true, "expected period of point {:?} to be 1", c1);

    // Another point with period of 1
    let c2 = Complex { re: -0.1, im: 0.1 };
    assert_eq!(is_period_p(z, c2, 1), true, "expected period of point {:?} to be 1", c2);

    // Point with period of 2
    //let c3 = Complex { re: -1.0, im: 0. };
    //assert_eq!(is_period_p(z, c3, 2), true, "expected period of point {:?} to be 2", c3);
}

/*
	 * Assume we've converged on an attractive fixed point here
	 * Plug it into multiplier equation and check ...?
     for(int p = 1; p <= max_period; p++) {
		if(is_period(p, z, c)) {
			return {false, 0, p};
		}
	}
	return {false, 0, 0};
*/

/// Period 0 means the point does not belong to the Mandelbrot Set.
fn calculate_period(z: Complex<f64>, c: Complex<f64>) -> usize {

    let max_period = 40;
    let mut period = 0;

    for p in 1..max_period {
        if is_period_p(z, c, p) {
            period = p;
            break;
        }
    }

    // DEBUG
    // Force one pixel to be white to easily locate it on the image.
    if c.re == -1.0 && c.im == 0.0 {
        // DEBUG
        print!("Period for point ({:?}, {:?}) is {}", c.re, c.im, period);
    }

    period
}

#[test]
fn test_calculate_period() {
    let z0 = Complex { re: 0., im: 0. };

    // Outside
    assert_eq!(calculate_period(z0, Complex { re: 0., im: 0. }), 1);

    // Mandelbrot Set
    assert_eq!(calculate_period(z0, Complex { re: 0., im: 0. }), 1);      // Period 1
    assert_eq!(calculate_period(z0, Complex { re: -0.1, im: 0.1 }), 1);   // Period 1
    //assert_eq!(calculate_period(z0, Complex { re: -1.0, im: 0. }), 2);    // Period 2
    //assert_eq!(calculate_period(z0, Complex { re: -0.1, im: 0.7 }), 3);   // Period 3
}

fn main() {
    let args = parse_args();

    let mut pixels = vec![0; args.bounds.0 * args.bounds.1];

    render(&mut pixels, args.bounds, args.upper_left, args.lower_right);

    write_image(&args.filepath, &pixels, args.bounds)
        .expect("error writing PNG file");
}
