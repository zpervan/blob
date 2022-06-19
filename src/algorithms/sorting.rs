use image::{Rgba, Pixel};

pub fn partial_sort(window_width: usize, pixel_vector: &mut Vec<Rgba<u8>>) {
    let half_window_width: usize = (window_width - 1) / 2;

    if half_window_width <= 1 {
        return;
    }

    for i in 0usize..=half_window_width {
        let mut min_index = i;

        for j in (i + 1)..window_width {
            if max(&pixel_vector[min_index], &pixel_vector[j]) {
                min_index = j;
            }
        }

        swap(pixel_vector, &min_index, &i);
    }
}

// todo: Consider to use `channels()` instead of deprecated `channels4()`
fn max(a: &Rgba<u8>, b: &Rgba<u8>) -> bool
{
    let (a_r, a_g, a_b, _a_a) = a.channels4();
    let (b_r, b_g, b_b, _b_a) = b.channels4();

    return (a_r > b_r) && (a_g > b_g) && (a_b > b_b);
}

fn swap(vec: &mut Vec<Rgba<u8>>, min_index: &usize, current_index: &usize)
{
    let temp = vec[*min_index];
    vec[*min_index] = vec[*current_index];
    vec[*current_index] = temp;
}
