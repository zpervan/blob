const DEFAULT_WIDTH: u32 = 800;
const DEFAULT_HEIGHT: u32 = 800;

pub fn blur(infile: String, outfile: String) {
    let input_image = image::open(infile).expect("Failed to open INFILE.");
    let blurred_image = input_image.blur(2.0);
    blurred_image.save(outfile).expect("Failed writing OUTFILE.");
}

pub fn brighten(infile: String, outfile: String) {
    let input_image = image::open(infile).expect("Failed to open INFILE.");
    let brighten_image = input_image.brighten(50);
    brighten_image.save(outfile).expect("Failed writing OUTFILE.");
}

pub fn crop(infile: String, outfile: String) {
    let mut input_image = image::open(infile).expect("Failed to open INFILE.");
    let cropped_image = input_image.crop(50, 50, 250, 250);
    cropped_image.save(outfile).expect("Failed writing OUTFILE.");
}

pub fn rotate(infile: String, outfile: String) {
    let input_image = image::open(infile).expect("Failed to open INFILE.");
    let rotated_image = input_image.rotate180();
    rotated_image.save(outfile).expect("Failed writing OUTFILE.");
}

pub fn invert(infile: String, outfile: String) {
    let mut input_image = image::open(infile).expect("Failed to open INFILE.");
    input_image.invert();
    input_image.save(outfile).expect("Expected outfile");
}

pub fn grayscale(infile: String, outfile: String) {
    let input_image = image::open(infile).expect("Failed to open INFILE.");
    input_image.grayscale().save(outfile).expect("Failed to open OUTFILE.");
}

pub fn generate(outfile: String)
{
    let mut image_buffer = image::ImageBuffer::new(DEFAULT_WIDTH, DEFAULT_HEIGHT);

    for (x, y, pixel) in image_buffer.enumerate_pixels_mut() {
        *pixel = image::Rgb([1, 1, 255]);
    }

    image_buffer.save(outfile).expect("Expected outfile");
}

pub fn fractal(outfile: String) {
    let mut imgbuf = image::ImageBuffer::new(DEFAULT_WIDTH, DEFAULT_HEIGHT);

    let scale_x = 3.0 / DEFAULT_WIDTH as f32;
    let scale_y = 3.0 / DEFAULT_HEIGHT as f32;

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        // Use red and blue to be a pretty gradient background
        let red = (0.3 * x as f32) as u8;
        let blue = (0.3 * y as f32) as u8;

        // Use green as the fractal foreground (here is the fractal math part)
        let cx = y as f32 * scale_x - 1.5;
        let cy = x as f32 * scale_y - 1.5;

        let c = num_complex::Complex::new(-0.4, 0.6);
        let mut z = num_complex::Complex::new(cx, cy);

        let mut green = 0;
        while green < 255 && z.norm() <= 2.0 {
            z = z * z + c;
            green += 1;
        }

        // Actually set the pixel. red, green, and blue are u8 values!
        *pixel = image::Rgb([red, green, blue]);
    }

    imgbuf.save(outfile).unwrap();
}
