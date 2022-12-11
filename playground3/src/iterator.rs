use std::{result, ops::RangeToInclusive};

struct Counter {
    start : u32,
    end: u32
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        if self.start >= self.end {
            None
        } else {
            let result = Some(self.start);
            self.start += 1;
            result
        }
    }
}

pub fn run() {
    let v = vec![1, 2, 3, 4, 5];

    let v1_itr = v.iter();

    for v in v1_itr {
        println!("{:?}", v);
    }

    let mut c = Counter {
        start: 1,
        end: 5
    };

    println!("{:?}", c.next());
    println!("{:?}", c.next());
    println!("{:?}", c.next());
    println!("{:?}", c.next());
    println!("{:?}", c.next());

    let v = vec![1, 2, 3, 4, 5];

    // ベクターをイテレーターに変換
    let m = v.iter().map(|x| x * 2);

    for val in m {
        println!("{}", val);
    }

    // コレクションに変換
    let c: Vec<_> = v.iter().map(|x| x * 2).collect();

    println!("{:?}", c);

    let f: Vec<_> = v.iter().filter(|x| *x % 2 == 0).collect();

    for fil in f {
        println!("{}", fil);
    }
}
