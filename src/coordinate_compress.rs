use proconio::{input, fastout};
use proconio::marker::{Bytes, Chars, Usize1};
use std::collections::HashSet;

#[fastout]
fn main() {
    input!{
        n: usize,
        ab: [[usize;2];n],
    }
    let mut a = Vec::new(); let mut b = Vec::new();
    for i in 0..n {
        a.push(ab[i][0]);
        b.push(ab[i][1]);
    }
    let a_uniq: HashSet<usize> = a.clone().into_iter().collect();
    let b_uniq: HashSet<usize> = b.clone().into_iter().collect();
    let mut a_uniq: Vec<usize> = a_uniq.into_iter().collect();
    let mut b_uniq: Vec<usize> = b_uniq.into_iter().collect();
    a_uniq.sort(); b_uniq.sort();
    for i in 0..n {
        println!("{} {}",binary_search(&a_uniq, a[i])+1, binary_search(&b_uniq, b[i])+1);
    }
}

fn binary_search(coordinate: &Vec<usize>, key: usize) -> usize {
    let mut left = 0;
    let mut right = coordinate.len();
    while right - left > 1 {
        let mut mid = (right + left) / 2;
        if coordinate[mid] > key { right = mid; }
        else { left = mid; }
    }
    left
}