
use std::collections::HashSet;

/// Computes the sum of all elements in the input i32 slice named `slice`
pub fn sum(slice: &[i32]) -> i32 {
    return slice.iter().sum();
}

/// Deduplicates items in the input vector `vs`. Produces a vector containing
/// the first instance of each distinct element of `vs`, preserving the
/// original order.
pub fn dedup(vs: &Vec<i32>) -> Vec<i32> {
    
    let mut vec: Vec<i32> = Vec::new();
    let mut alreadyInserted: HashSet<i32> = HashSet::new();

    for num in vs.into_iter() {
        if !alreadyInserted.contains(num) {
            vec.push(i32::from(*num));
            alreadyInserted.insert(i32::from(*num));
        } 
    }

    return vec;
}

/// Filters a vector `vs` using a predicate `pred` (a function from `i32` to
/// `bool`). Returns a new vector containing only elements that satisfy `pred`.
pub fn filter(vs: &Vec<i32>, pred: &Fn(i32) -> bool) -> Vec<i32> {
    let mut vec: Vec<i32> = Vec::new();

    for num in vs.into_iter(){ 
        if pred(*num) {
            vec.push(i32::from(*num));
        }
    }
    
    return vec;
}
