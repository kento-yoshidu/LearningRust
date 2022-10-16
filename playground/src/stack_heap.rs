pub fn run() {
    // let a1: [u8; 7000000] = [1; 7000000];

    let mut v1 = vec![1, 2, 3, 4];
    let v2 = vec![5, 6, 7, 8];
    let mut v3 = vec![9, 10];

    /*
    println!("v1 address {:p}", &v1);
    println!("v2 address {:p}", &v2);

    println!("v1 {:?}", v1.as_ptr());
    println!("v1 {}", v1.len());
    println!("v1 {}", v1.capacity());

    v1.insert(1, 100);
    v1.remove(0);

    println!("{:?}", v1);
    //=> [100, 2, 3, 4]
    */

    v1.append(&mut v3);

    println!("{:?}, {:?}", v1, v3);
    //=> [1, 2, 3, 4, 9, 10], []
}