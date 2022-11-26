use core::fmt::Debug;
use std::fmt::Display;

fn max<T: PartialOrd>(a: T, b: T) -> T {
    if a >= b {
        a
    } else {
        b
    }
}

fn max2<T>(a: T, b: T) -> T
where T: PartialOrd + Debug
{
    if a >= b {
        a
    } else {
        b
    }
}

struct Point<T> {
    x: T,
    y: T
}

impl<T: PartialOrd + Debug> Point<T> {
    fn max3(&self) -> &T {
        if self.x >= self.y {
            &self.x
        } else {
            &self.y
        }
    }

    fn print_arg<U: Display>(&self, value: U) {
        println!("self.x is {:?}", self.x);
        println!("value is{}", value);
    }
}

pub fn run() {
    println!("{}", max(1, 2));
    println!("{}", max2(3, 2));

    let p1 = Point { x: 5, y: 2 };
    let p2 = Point { x: 5.3, y: 20.1 };
    let p3 = Point { x: "a", y: "b" };

    println!("{}", p1.max3());
    println!("{}", p2.max3());
    println!("{}", p3.max3());

    p1.print_arg("test");
}