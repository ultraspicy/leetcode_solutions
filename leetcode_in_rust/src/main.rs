use std::borrow::Borrow;
use std::cmp;
use std::collections::{BTreeMap, HashMap};
use std::collections::VecDeque;
use std::fmt::Display;
use std::ops::Deref;
use itertools::Itertools;
use leetcode_in_rust::utils::print_vec;

fn main() {

    let mut vectors = vec![
        vec![1, 2, 5],
        vec![1, 2, 6],
        vec![2, 3, 5],
        vec![2, 3, 6],
    ];

    // Retain vectors where the first two elements are not the same
    let new_vectors: Vec<Vec<i32>> = vectors.iter().map(|x| vec![x[0], x[1] + 1, x[2] + 1]).collect();

    println!("Filtered vectors: {:?}", new_vectors);
}