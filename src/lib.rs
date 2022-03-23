use image::{GenericImageView, Pixel, Rgba};

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

fn swap(vec: &mut Vec<Rgba<u8>>, min_index: &usize, cur_index: &usize)
{
    let temp = vec[*min_index];
    vec[*min_index] = vec[*cur_index];
    vec[*cur_index] = temp;
}

fn larger_then(a: &Rgba<u8>, b: &Rgba<u8>) -> bool
{
    let (a_r, a_g, a_b, _a_a) = a.channels4();
    let (b_r, b_g, b_b, _b_a) = b.channels4();

    return (a_r > b_r) && (a_g > b_g) && (a_b > b_b);
}

pub fn median_filter(infile: String, outfile: String)
{
    let window_width: usize = 5;
    let half_window_width : usize = (window_width - 1) / 2;
    let input_image = image::open(infile).expect("Cannot open input file");
    let mut image_buffer = image::ImageBuffer::new(input_image.width(), input_image.height());

    // For debugging
    // println!("Image pixel count: {}", input_image.pixels().count());
    // println!("Image x: {}", input_image.bounds().0);
    // println!("Image y: {}", input_image.bounds().1);
    // println!("Image width: {}", input_image.bounds().2);
    // println!("Image height: {}", input_image.bounds().3);

    // y - height iteration, x - width iteration
    for y in 0..input_image.bounds().3 {
        for x in 0..input_image.bounds().2 {

            // Skip first two or last two pixels based on hal
            if (x < half_window_width as u32) || (x >= (input_image.bounds().2 - half_window_width as u32)) {
                continue;
            }

            let mut pixel_vector = Vec::new();
            pixel_vector.reserve(window_width as usize);

            // Grab 5 neighbouring pixels
            for i in -2i32..=2i32 {
                let x_signed = x as i32;
                let x_position = (i + x_signed) as u32;
                pixel_vector.push(input_image.get_pixel(x_position, y));
            }

            // Some partial selection sorting until half window width
            let mut min_index : usize = 0;

            for i in 0usize..half_window_width as usize {
                min_index = i ;

                for j in (i + 1)..=half_window_width {
                    if larger_then(&pixel_vector[min_index], &pixel_vector[j]) {
                        min_index = j;
                    }
                }

                swap(&mut pixel_vector, &min_index, &i);
            }

            // Add filtered middle element to image buffer
            *image_buffer.get_pixel_mut(x, y) = pixel_vector[half_window_width];
        }
    }
    image_buffer.save(outfile).unwrap();
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

        let c = num_complex::Complex::new(-0.4, 0.4);
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
