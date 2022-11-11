fn main() {
    let arr = [1, 2];
    let arr2 = arr;

    println!("{:p}", &arr);
    //=> 0x7fff175c5808
    println!("{:p}", &arr2);
    //=> 0x7fff175c5810

    let arr = [1, 2];
    let ref_arr = &arr;

    println!("{:p}", &arr);
    //=> 0x7fffbc496990
    println!("{:p}", ref_arr);
    //=> 0x7fffbc496990
}
