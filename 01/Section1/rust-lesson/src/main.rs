fn main() {
    /*
    println!("Hello, world!");
    
    let mut x = 3;
    println!("{}", x);
    //=> 3

    x = 6;
    println!("{}", x);
    //=> 6

    const MAX_COUNT: i32 = 100;

    println!("{:p}", &MAX_COUNT);

    let i2: i64 = 1;
    let i3: i64 = 2;

    println!("i2 is {:p}", &i2);
    println!("i3 is {:p}", &i3);

    let y = 5;
    println!("{:p}", &y);
    //=> 0xdd3c4ff6ac

    let y = y + 5;
    println!("{:p}", &y);
    //=> 0xdd3c4ff704

    let t1 = (500, 6.4, "dummy");
    let (x, y, z) = t1;

    // println!("{}, {}, {}", x, y, z);
    
    println!("{:?}", t1);
    //=> (500, 6.4, "dummy")

    println!("{}, {}, {}", t1.0, t1.1, t1.2);

    let t2 = ((0, 1), (2, 3));

    let ((ref x1_ptr, ref y1_ptr), _) = t2;

    println!("{:p}, {:p}", &x1_ptr, &y1_ptr);
    //=> 0xf1aaddf528, 0xf1aaddf530

    let mut t2 = (0, 1);

    println!("{:p}, {:p}", &t2.0, &t2.1);
    //=> 0xaeb7bff440, 0xaeb7bff444

    t2.0 = 100;

    println!("{:p}, {:p}", &t2.0, &t2.1);
    //=> 0xaeb7bff440, 0xaeb7bff444
    */

    let mut t2 = ((0, 1), (2, 3));

    let ((ref mut x1_ptr, ref mut y1_ptr), _) = t2;

    println!("{:p}, {:p}", &x1_ptr, &y1_ptr);
    //=> 0x2cb3d3f628, 0x2cb3d3f630

    *x1_ptr = 100;
    *y1_ptr = 200;

    println!("{:p}, {:p}", &x1_ptr, &y1_ptr);
    //=> 0x2cb3d3f628, 0x2cb3d3f630
}
