#![feature(test)]

extern crate test;
extern crate im;
extern crate rand;

use std::iter::FromIterator;
use std::collections::BTreeMap;
use test::Bencher;
use rand::{weak_rng, Rng};

use im::map::Map;

fn random_keys(size: usize) -> Vec<i64> {
    let mut gen = weak_rng();
    let mut set = Vec::new();
    while set.len() < size {
        let next = gen.gen::<i64>() % 10000;
        if !set.contains(&next) {
            set.push(next);
        }
    }
    set
}

fn reorder<A: Copy>(vec: &Vec<A>) -> Vec<A> {
    let mut gen = weak_rng();
    let mut set = vec.clone();
    let mut out = Vec::new();
    while set.len() > 0 {
        let i = gen.gen::<usize>() % set.len();
        let v = set.remove(i);
        out.push(v)
    }
    out
}

fn map_lookup_n(size: usize, b: &mut Bencher) {
    let keys = random_keys(size);
    let order = reorder(&keys);
    let m = Map::from_iter(keys.into_iter().map(|i| (i,1)));
    b.iter(|| {
        for i in &order {
            m.get(&i);
        }
    })
}

#[bench]
fn map_lookup_10(b: &mut Bencher) {
    map_lookup_n(10, b)
}

#[bench]
fn map_lookup_100(b: &mut Bencher) {
    map_lookup_n(100, b)
}

#[bench]
fn map_lookup_1000(b: &mut Bencher) {
    map_lookup_n(1000, b)
}

fn map_insert_n(size: usize, b: &mut Bencher) {
    let keys = random_keys(size);
    b.iter(|| {
        let mut m = Map::new();
        for i in keys.clone() {
            m = m.insert(i, i)
        }
    })
}

#[bench]
fn map_insert_10(b: &mut Bencher) {
    map_insert_n(10, b)
}

#[bench]
fn map_insert_100(b: &mut Bencher) {
    map_insert_n(100, b)
}

#[bench]
fn map_insert_1000(b: &mut Bencher) {
    map_insert_n(1000, b)
}

fn btreemap_insert_n(size: usize, b: &mut Bencher) {
    let keys = random_keys(size);
    b.iter(|| {
        let mut m = BTreeMap::new();
        for i in keys.clone() {
            m.insert(i, i);
        }
    })
}

#[bench]
fn btreemap_insert_10(b: &mut Bencher) {
    btreemap_insert_n(10, b)
}

#[bench]
fn btreemap_insert_100(b: &mut Bencher) {
    btreemap_insert_n(100, b)
}

#[bench]
fn btreemap_insert_1000(b: &mut Bencher) {
    btreemap_insert_n(1000, b)
}

fn btreemap_insert_clone_n(size: usize, b: &mut Bencher) {
    let keys = random_keys(size);
    b.iter(|| {
        let mut m = BTreeMap::new();
        for i in keys.clone() {
            m = m.clone();
            m.insert(i, i);
        }
    })
}

#[bench]
fn btreemap_insert_clone_10(b: &mut Bencher) {
    btreemap_insert_clone_n(10, b)
}

#[bench]
fn btreemap_insert_clone_100(b: &mut Bencher) {
    btreemap_insert_clone_n(100, b)
}

#[bench]
fn btreemap_insert_clone_1000(b: &mut Bencher) {
    btreemap_insert_clone_n(1000, b)
}

fn map_remove_n(size: usize, b: &mut Bencher) {
    let keys = random_keys(size);
    let order = reorder(&keys);
    let map = Map::from_iter(keys.into_iter().map(|i| (i,i)));
    b.iter(|| {
        let mut m = map.clone();
        for i in &order {
            m = m.remove(&i)
        }
    })
}

#[bench]
fn map_remove_10(b: &mut Bencher) {
    map_remove_n(10, b)
}

#[bench]
fn map_remove_100(b: &mut Bencher) {
    map_remove_n(100, b)
}

#[bench]
fn map_remove_1000(b: &mut Bencher) {
    map_remove_n(1000, b)
}

#[bench]
fn map_remove_min(b: &mut Bencher) {
    let map = Map::from_iter((0..1000).into_iter().map(|i| (i,i)));
    b.iter(|| {
        let mut m = map.clone();
        assert!(!m.is_empty());
        for _ in 0..1000 {
            m = m.delete_min()
        }
        assert!(m.is_empty())
    })
}

#[bench]
fn map_remove_max(b: &mut Bencher) {
    let map = Map::from_iter((0..1000).into_iter().map(|i| (i,i)));
    b.iter(|| {
        let mut m = map.clone();
        assert!(!m.is_empty());
        for _ in 0..1000 {
            m = m.delete_max()
        }
        assert!(m.is_empty())
    })
}

fn map_insert_once_n(size: usize, b: &mut Bencher) {
    let mut keys = random_keys(size + 1);
    let key = keys.pop().unwrap();
    let map = Map::from_iter(keys.into_iter().map(|i| (i,i)));
    b.iter(|| {
        map.insert(key, key)
    })
}

#[bench]
fn map_insert_once_10(b: &mut Bencher) {
    map_insert_once_n(10, b)
}

#[bench]
fn map_insert_once_100(b: &mut Bencher) {
    map_insert_once_n(100, b)
}

#[bench]
fn map_insert_once_1000(b: &mut Bencher) {
    map_insert_once_n(1000, b)
}

#[bench]
fn map_insert_once_10000(b: &mut Bencher) {
    map_insert_once_n(10000, b)
}

fn map_remove_once_n(size: usize, b: &mut Bencher) {
    let keys = random_keys(size + 1);
    let key = keys[0];
    let map = Map::from_iter(keys.into_iter().map(|i| (i,i)));
    b.iter(|| {
        map.remove(&key)
    })
}

#[bench]
fn map_remove_once_10(b: &mut Bencher) {
    map_remove_once_n(10, b)
}

#[bench]
fn map_remove_once_100(b: &mut Bencher) {
    map_remove_once_n(100, b)
}

#[bench]
fn map_remove_once_1000(b: &mut Bencher) {
    map_remove_once_n(1000, b)
}

#[bench]
fn map_remove_once_10000(b: &mut Bencher) {
    map_remove_once_n(10000, b)
}

fn map_lookup_once_n(size: usize, b: &mut Bencher) {
    let keys = random_keys(size + 1);
    let key = keys[0];
    let map = Map::from_iter(keys.into_iter().map(|i| (i,i)));
    b.iter(|| {
        map.get(&key)
    })
}

#[bench]
fn map_lookup_once_10(b: &mut Bencher) {
    map_lookup_once_n(10, b)
}

#[bench]
fn map_lookup_once_100(b: &mut Bencher) {
    map_lookup_once_n(100, b)
}

#[bench]
fn map_lookup_once_1000(b: &mut Bencher) {
    map_lookup_once_n(1000, b)
}

#[bench]
fn map_lookup_once_10000(b: &mut Bencher) {
    map_lookup_once_n(10000, b)
}
