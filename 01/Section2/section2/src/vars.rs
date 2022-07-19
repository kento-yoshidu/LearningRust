pub fn vars() {
  let array1 = [1, 2, 3, 4, 5];
  let array2 = [0; 10];

  println!("{:?}, \n{:?},\n{},\n{}", array1, array2, array1[0], array2[3]);
  /*
    [1, 2, 3, 4, 5],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    1,
    0
  */
}
