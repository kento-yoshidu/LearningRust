struct Point<T> {
    x: T,
    y: T
}

struct PointAnother<T, U> {
    x: T,
    y: U
}

impl<T, U> PointAnother<T, U> {
    fn mixup<V, W>(self, other: PointAnother<V, W>) -> PointAnother<T, W> {
        PointAnother {
            x: self.x,
            y: other.y
      }
  }
}

pub fn run() {
    let number_list = vec![35, 50, 61, 24];
    let char_list = vec!['a', 'b', 'c', 'd'];

    println!("{}", largest(number_list));
    println!("{}", largest(char_list));

    let p1 = Point { x: 1, y: 2};
    let p2 = PointAnother { x: 1.0, y: 2 };
    let p3 = PointAnother { x: 5, y: 2.0 };
    let p4 = PointAnother { x: "Rust", y: "Feris" };
    let p5 = p3.mixup(p4);

    println!("{}, {}", p5.x, p5.y);
}

fn largest<T: PartialOrd + Copy>(list: Vec<T>) -> T {
  let mut largest = list[0];

  for item in list {
    if item > largest {
      largest = item;
    }
  }

  largest
}
