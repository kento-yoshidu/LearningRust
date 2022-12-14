pub fn run() {
    let v1 = vec!["Rust", "Python", "Java"];
    let v2 = vec!["PHP", "GO"];

    let v3 = [v1, v2].concat();
    println!("{:?}", v3);

    let (v4, v5) = v3.split_at(2);

    println!("{:?}, {:?}", v4, v5);

    let mut v1 = [5, 4, 3, 7, 1];

    v1.sort();

    println!("{:?}", v1);

    v1.reverse();

    println!("{:?}", v1);

    #[derive(Debug)]
    struct S {
        val1: i32,
        val2: i32
    }

    let mut v1 = vec![
        S { val1: 3, val2: 1},
        S { val1: 2, val2: 2},
        S { val1: 1, val2: 3}
    ];

    v1.sort_by_key(|s| s.val1);

    println!("{:?}", v1);

    let v1 = vec!{1, 2, 3, 4, 5};

    println!("{:?}", v1.contains(&1));
    println!("{:?}", v1.contains(&6));

    let i = v1.iter().position(|x| *x == 5);

    println!("{:?}", i);
    //=> Some(4) ないならNone
}
