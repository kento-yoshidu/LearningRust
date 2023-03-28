pub fn run() {
    let mut v = vec![1, 2, 3];

    println!("{:?}", v);

    v.push(4);

    println!("{:?}", v);

    println!("{:?}", &v[0..2]);
}
