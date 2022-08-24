use std::option::Option::{self, Some, None};

pub(crate) fn binarysearch <T: Ord> (real: &Vec<T>, item: T) -> Option<usize>{
    return binarysearch_proper(real.as_slice(), item);
}

 fn binarysearch_proper<T: Ord>(real: &[T], item: T) -> Option<usize>{
    if real.len() <= 0 {
        return None;
    }
    
    if real.len() == 1 {
        if real[0] == item {
            return Some(1)
        }
        return None;
    }

    let index:usize = real.len() / 2;

    if real[index] == item {
        return Some(index);
    }

    let halves: (&[T], &[T]) = real.split_at(index);

    if real[index] > item {
        return binarysearch_proper(halves.0, item);
    }

    return binarysearch_proper(halves.1, item);
    
}