use std::ops::{Add, Mul};
use std::cmp;
fn dot<N>(v1: &[N], v2: &[N]) -> N
where N: Add<Output=N> + Mul<Output=N> + Default + Copy + std::ops::AddAssign
{
    let mut total = N::default();
    for i in 0 .. cmp::min(v1.len(), v2.len()) {
    total += v1[i] * v2[i];
    }
    total
}

fn test_dot() {
assert_eq!(dot(&[1, 2, 3, 4], &[1, 1, 1, 1]), 10);
assert_eq!(dot(&[53.0, 7.0, 36.0], &[1.0, 5.0]), 88.0);
}
fn main() {
    test_dot();
    println!("Done.")
}