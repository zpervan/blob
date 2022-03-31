/// Blur the pixels of this image.
/// ```infile``` input image
/// ```outfile``` output image
pub fn blur(infile: String, outfile: String) {
    let input_image = image::open(infile).expect("Failed to open INFILE.");
    let blurred_image = input_image.blur(2.0);
    blurred_image.save(outfile).expect("Failed writing OUTFILE.");
}

/// Brighten the pixels of this image.
/// ```infile``` input image
/// ```outfile``` output image
pub fn brighten(infile: String, outfile: String) {
    let input_image = image::open(infile).expect("Failed to open INFILE.");
    let brighten_image = input_image.brighten(50);
    brighten_image.save(outfile).expect("Failed writing OUTFILE.");
}

/// Grayscale the pixels of this image.
/// ```infile``` input image
/// ```outfile``` output image
pub fn grayscale(infile: String, outfile: String) {
    let input_image = image::open(infile).expect("Failed to open INFILE.");
    input_image.grayscale().save(outfile).expect("Failed to open OUTFILE.");
}

/// Invert the pixels of this image.
/// ```infile``` input image
/// ```outfile``` output image
pub fn invert(infile: String, outfile: String) {
    let mut input_image = image::open(infile).expect("Failed to open INFILE.");
    input_image.invert();
    input_image.save(outfile).expect("Expected outfile");
}
