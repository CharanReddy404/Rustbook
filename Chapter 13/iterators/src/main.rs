#![allow(unused)]

pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];
    let mut v1_iter = v1.iter();
    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);

    let mut v1 = vec![1, 2, 3];
    let mut v1_iter_mut = v1.iter_mut();
    assert_eq!(v1_iter_mut.next(), Some(&mut 1));
    assert_eq!(v1_iter_mut.next(), Some(&mut 2));
    assert_eq!(v1_iter_mut.next(), Some(&mut 3));
    assert_eq!(v1_iter_mut.next(), None);

    let v1 = vec![1, 2, 3];
    let mut v1_iter_into = v1.into_iter();
    assert_eq!(v1_iter_into.next(), Some(1));
    assert_eq!(v1_iter_into.next(), Some(2));
    assert_eq!(v1_iter_into.next(), Some(3));
    assert_eq!(v1_iter_into.next(), None);
}

#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];
    let v1_inter = v1.iter();
    let total: i32 = v1_inter.sum();
    assert_eq!(total, 6);
}

fn main() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    for value in v1_iter {
        println!("Got: {}", value);
    }

    println!("-----------------------------");

    let v1: Vec<i32> = vec![1, 2, 3];
    let v2: Vec<i32> = v1.iter().map(|x| x + 1).collect();
    assert_eq!(v2, vec![2, 3, 4]);
}
