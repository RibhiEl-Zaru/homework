#![cfg(test)]

use crate::problem1::{sum, dedup, filter};
use crate::problem2::mat_mult;
use crate::problem3::sieve;
use crate::problem4::{hanoi, Peg};

//
// Problem 1
//

// Part 1

#[test]
fn test_sum_medium() {
    let array = [1,2,3,4,5, 1,2,3,4,5, 1,2,3,4,5, 1,2,3,4,5, 1,2,3,4,5, 1,2,3,4,5];
    assert_eq!(sum(&array), 90);
}



#[test]
fn test_dedup() {
    let array: Vec<i32> = vec![2, 1, 2, 3,  1, 3, 4, 4, 5, 5];
    assert_eq!(dedup(&array), [2, 1, 3, 4, 5]); // verify dedup

    let other_order: Vec<i32> = vec![3,2,2,3,4,4,1];
    assert_eq!(dedup(&other_order), [3, 2, 4, 1]); // verify dedup

    assert_eq!(array, vec![2, 1, 2, 3,  1, 3, 4, 4, 5, 5]); // make sure og array is untouched
}


#[test]
fn test_filter() {
    let array: Vec<i32> = vec![1, 1, 2, 2, 3, 3, 4, 4, 5, 5];
    assert_eq!(filter(&array, &isOdd), vec![1, 1, 3, 3, 5, 5]);
    assert_eq!(array, vec![1, 1, 2, 2, 3, 3, 4, 4, 5, 5])
    
}

fn isOdd(value: i32) -> bool {
    value % 2 != 0
}
