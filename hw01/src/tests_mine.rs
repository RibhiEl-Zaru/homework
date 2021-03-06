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


#[test]
#[should_panic]d
fn test_mat_mult_panic() {
    let mut mat1 = vec![vec![0.;2]; 2];
    for i in 0..mat1.len() {
        mat1[i][i] = 1.;
    }

    let mat2 = vec![vec![5.;3]; 3];
    mat_mult(&mat1, &mat2);
}

#[test]
fn test_mat_mult(){
    let mat1 = vec![vec![1., 2., 3.], vec![4., 5., 6.]];
    let mat2 = vec![vec![7., 8.], vec![9., 10.], vec![11., 12.]];
    let result = mat_mult(&mat1, &mat2);
    let shouldBe = vec![vec![58., 64.], vec![139., 154.]];
    assert_eq!(result, shouldBe);
}


#[test]
#[should_panic]
fn test_empty_matrix_panic(){
    let mat1 = vec![];
    let mat2 = vec![vec![7., 8.], vec![9., 10.], vec![11., 12.]];
    mat_mult(&mat1, &mat2);
}

#[test]
#[should_panic]
fn test_empty_matrix_panic2(){
    let mat1 = vec![vec![7., 8.], vec![9., 10.], vec![11., 12.]];
    let mat2 = vec![];
    mat_mult(&mat1, &mat2);
}


#[test]
fn test_sieve_basic() {
    assert_eq!(vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97, 101, 103, 107, 109, 113], sieve(120));
}



#[test]
fn test_hanoi_2_disks() {
    let result = hanoi(2, Peg::A, Peg::B, Peg::C);
    assert_eq!(vec![(Peg::A, Peg::B), (Peg::A, Peg::C), (Peg::B, Peg::C)], result);
    assert_eq!(3, result.len());
}


#[test]
fn test_hanoi_3_disks() {
    let result = hanoi(3, Peg::A, Peg::B, Peg::C);
    assert_eq!(vec![(Peg::A, Peg::C), (Peg::A, Peg::B), (Peg::C, Peg::B), (Peg::A, Peg::C), (Peg::B, Peg::A), (Peg::B, Peg::C), (Peg::A, Peg::C)], result);
    assert_eq!(7, result.len());
}

#[test]
fn test_hanoi_4_disks() {
    let result = hanoi(4, Peg::A, Peg::B, Peg::C);
    assert_eq!(15, result.len());
}

#[test]
fn test_hanoi_5_disks() {
    let result = hanoi(5, Peg::A, Peg::B, Peg::C);
    assert_eq!(31, result.len());
}



#[test]
fn test_hanoi_6_disks() {
    let result = hanoi(6, Peg::A, Peg::B, Peg::C);
    assert_eq!(63, result.len());
}

fn isOdd(value: i32) -> bool {
    value % 2 != 0
}
