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