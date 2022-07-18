pub fn vars() {
  let mut t2 = ((0, 1), (2, 3));

  let (mut x1_ptr, _) = t2;

  println!("{}, {}", x1_ptr);
  //=> 0xa6be5af6a0
}