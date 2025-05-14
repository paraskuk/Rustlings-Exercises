pub fn transform_even_odd(slice: &mut [i32]) {
    for element in slice.iter_mut() {
        *element = match *element % 2 {
            0 => *element * 2,    // Double even numbers
            _ => *element - 1,    // Subtract 1 from odd numbers
        };
    }
}