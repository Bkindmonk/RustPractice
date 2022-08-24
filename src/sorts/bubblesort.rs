pub(crate) fn bubblesort <T: Ord + Copy>(real: &mut Vec<T>) {
    bubblesort_proper(real.as_mut_slice())
}

fn bubblesort_proper <T: Ord + Copy>(real: &mut [T]) {
    let mut length: usize = real.len();
    if length < 2 {
        return;
    }
    let mut swap: T = real[0];
    while length > 1 {
        let mut new_length: usize = 0;
        (1..length - 1).for_each(|index: usize| {
            if real[index - 1] > real[index] {
                swap = real[index];
                real[index] = real[index - 1];
                real[index - 1] = swap
            }
            new_length = index;
        });
        length = new_length;
    }
}