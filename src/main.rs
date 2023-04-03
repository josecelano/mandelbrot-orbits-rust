use image::RgbImage;
use mandelbrot_orbits_rust::{
    parse_args, render_colorized, render_greyscale, write_image, Arguments,
};

fn main() {
    let args = parse_args();

    greyscale(&args);
    colorized(&args);
}

fn greyscale(args: &Arguments) {
    let mut pixels = vec![0; args.bounds.0 * args.bounds.1];

    render_greyscale(&mut pixels, args.bounds, args.upper_left, args.lower_right);

    write_image(&args.filepath, &pixels, args.bounds).expect("error writing PNG file");
}

fn colorized(args: &Arguments) {
    // Create a new ImgBuf with width: img_x and height: img_y
    let mut img = RgbImage::new(args.bounds.0 as u32, args.bounds.1 as u32);

    render_colorized(&mut img, args.bounds, args.upper_left, args.lower_right);

    // Save the image. The format is deduced from the path
    img.save(format!(
        "./output/colorized_mandelbrot_{}x{}.png",
        args.bounds.0, args.bounds.1
    ))
    .unwrap();
}
