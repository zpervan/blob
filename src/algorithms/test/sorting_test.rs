use image::{Pixel, Rgba};
use super::sorting;

#[test]
fn GivenUnsortedArray_WhenPartiallySorting_ThenResultingArraySortedCorrectly() {
    let number_of_elements : usize = 5;
    let mut rgba_values: Vec<Rgba<u8>> = Vec::new();
    rgba_values.reserve(number_of_elements);

    // Add random Rgba<u8> values
    rgba_values.push(image::Rgba([2 as u8, 2 as u8, 2 as u8, 0 as u8]));
    rgba_values.push(image::Rgba([0 as u8, 0 as u8, 0 as u8, 0 as u8]));
    rgba_values.push(image::Rgba([1 as u8, 1 as u8, 1 as u8, 0 as u8]));
    rgba_values.push(image::Rgba([4 as u8, 4 as u8, 4 as u8, 0 as u8]));
    rgba_values.push(image::Rgba([3 as u8, 3 as u8, 3 as u8, 0 as u8]));

    sorting::partial_sort(rgba_values.len(), &mut rgba_values);

    // Alpha is currently not taken in consideration for the sorting algorithm
    let (expected_r , expected_g, expected_b, _expected_a) = (2 as u8, 2 as u8, 2 as u8, 0 as u8);

    let half_window_width :usize = (rgba_values.len() - 1) / 2;
    let (actual_r, actual_g, actual_b, _actual_a) = rgba_values[half_window_width].channels4();

    assert_eq!(expected_r, actual_r);
    assert_eq!(expected_g, actual_g);
    assert_eq!(expected_b, actual_b);
}

#[test]
fn GivenUnsortedArrayWithLessThanAllowedElementCount_WhenPartiallySorting_ThenArrayIsUnsorted() {
    let number_of_elements : usize = 2;

    let rgba_values_0 = image::Rgba([2 as u8, 2 as u8, 2 as u8, 0 as u8]);
    let rgba_values_1 = image::Rgba([0 as u8, 0 as u8, 0 as u8, 0 as u8]);

    let mut rgba_values: Vec<Rgba<u8>> = Vec::new();
    rgba_values.reserve(number_of_elements);

    rgba_values.push(rgba_values_0.clone());
    rgba_values.push(rgba_values_1.clone());

    sorting::partial_sort(rgba_values.len(), &mut rgba_values);

    assert_eq!(rgba_values_0, rgba_values[0]);
    assert_eq!(rgba_values_1, rgba_values[1]);
}