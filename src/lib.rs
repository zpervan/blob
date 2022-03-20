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

fn change_value(value: &mut i32, direction: &mut bool)
{
    if *value >= 255 {
        *direction = true;
    }

    if *value <= 0 {
        *direction = false;
    }

    if *direction {
        *value += 1;
    } else {
        *value -= 1;
    }
}

pub fn generate(outfile: String)
{
    let mut image_buffer = image::ImageBuffer::new(DEFAULT_WIDTH, DEFAULT_HEIGHT);

    let mut red_direction: bool = true;
    let mut blue_direction: bool = true;
    let mut green_direction: bool = true;

    let mut red_value = 0;
    let mut blue_value = 100;
    let mut green_value = 200;

    for (_x, _y, pixel) in image_buffer.enumerate_pixels_mut() {
        change_value(&mut red_value, &mut red_direction);
        change_value(&mut blue_value, &mut blue_direction);
        change_value(&mut green_value, &mut green_direction);

        *pixel = image::Rgb([red_value as u8, green_value as u8, blue_value as u8]);
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
