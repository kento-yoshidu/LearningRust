use std::collections::{HashMap, HashSet};

pub fn run() {
    let mut map = HashMap::new();

    map.insert("Java", 11);
    map.insert("Rust", 1);

    println!("{:?}", map);

    map.insert("Rust", 1);

    println!("{:?}", map);

    map.insert("Rust", 2);

    println!("{:?}", map);

    let mut set = HashSet::new();

    set.insert(1);
    set.insert(2);
    set.insert(3);

    println!("{:?}", set);

    set.insert(3);

    println!("{:?}", set);
}
