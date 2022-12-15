use std::collections::{VecDeque, BinaryHeap};

pub fn run() {
    let mut q = VecDeque::new();

    q.push_back(1);
    q.push_back(2);
    q.push_back(3);

    println!("{:?}", q);

    q.push_front(4);

    println!("{:?}", q);

    q.pop_front();

    println!("{:?}", q);

    q.pop_back();

    println!("{:?}", q);

    // バイナリーヒープ = 最大の要素が先頭に来る
    let mut bh = BinaryHeap::new();

    bh.push(1);
    bh.push(10);
    bh.push(20);
    bh.push(2);

    println!("{:?}", bh);

    bh.pop();

    println!("{:?}", bh);
}
