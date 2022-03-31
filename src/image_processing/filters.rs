use image::GenericImageView;

pub fn median_filter(infile: String, outfile: String)
{
    let window_width: usize = 5;
    let half_window_width: usize = (window_width - 1) / 2;
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
            let bot = -(half_window_width as i32);
            let top = half_window_width as i32;

            for i in bot..=top {
                let x_signed = x as i32;
                let x_position = (i + x_signed) as u32;
                pixel_vector.push(input_image.get_pixel(x_position, y));
            }

            // Some partial selection sorting until half window width
            crate::algorithms::sorting::partial_sort(window_width, &mut pixel_vector);

            // Add filtered middle element to image buffer
            *image_buffer.get_pixel_mut(x, y) = pixel_vector[half_window_width];
        }
    }
    image_buffer.save(outfile).unwrap();
}
