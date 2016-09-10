#![cfg(test)]

use problem1::{sum, dedup, filter};
use problem2::sieve;
use problem3::hanoi;
use problem4::{bloom, djb2, fnv, jenkins};

//
// Problem 1
//

// Part 1

#[test]
fn test_sum_small() {
    let array = [1, 2, 3, 4, 5];
    assert_eq!(sum(&array), 15);
}

#[test]
fn test_sum_medium() {
    let mut array = [0i32; 100];
    for i in 1..101 {
        array[i-1] = i as i32;
    }
    assert_eq!(sum(&array), 5050);
}

#[test]
fn test_sum_cancel() {
    let array = [1, -1];
    assert_eq!(sum(&array), 0);
}

// Part 2

#[test]
fn test_dedup_small() {
    let vs = vec![1, 2, 2, 3, 4, 1];
    assert_eq!(dedup(&vs),  vec![1, 2, 3, 4]);
}

#[test]
fn test_dedup_all_same() {
    let vs = vec![-1, -1, -1, -1, -1, -1];
    assert_eq!(dedup(&vs), vec![-1]);
}

// Part 3

fn even_predicate(x: i32) -> bool {
    (x % 2) == 0
}

#[test]
fn test_filter_small() {
    let vs = vec![1, 2, 3, 4, 5];
    assert_eq!(filter(&vs, &even_predicate), vec![2, 4]);
}

//
// Problem 2
//

#[test]
fn test_sieve_basic() {
    assert_eq!(vec![2, 3, 5, 7, 11], sieve(12));
}

#[test]
fn test_sieve_larger() {
    assert_eq!(vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97], sieve(100));
}

#[test]
fn test_sieve_none() {
    assert_eq!(vec![0u32; 0], sieve(1));
}

//
// Problem 3
//

#[test]
fn test_hanoi_1_disks() {
    let result = hanoi(1);
    assert_eq!(vec![(1, 3)], result);
    assert_eq!(1, result.len());
}

#[test]
fn test_hanoi_2_disks() {
    let result = hanoi(2);
    assert_eq!(vec![(1, 2), (1, 3), (2, 3)], result);
}

#[test]
fn test_hanoi_3_disks() {
    let result = hanoi(3);
    assert_eq!(vec![(1, 3), (1, 2), (3, 2), (1, 3), (2, 1), (2, 3), (1, 3)], result);
}

//
// Problem 4
//

#[test]
fn test_bloom_foods() {
    let data = vec!["apple", "blueberry", "carrot", "date", "eggplant",
        "fig", "grapefruit"];
    let hashes = [djb2, fnv, jenkins];
    assert_eq!(true, bloom(&data, hashes, "carrot"));
    assert_eq!(true, bloom(&data, hashes, "milk"));
    assert_eq!(false, bloom(&data, hashes, "bread"));
}
