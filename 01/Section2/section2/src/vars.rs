const MAX_POINTS: u32 = 100_000;

pub fn vars() {
  println!("{:p}", &MAX_POINTS);
  //=> 0x7ff6ab1dd438

  let i2: i64 = 1;
  let i3: i64 = 2;


  println!("i2 is {:p}", &i2);
  //=> i2 is 0xd7b99df950

  println!("i3 is {:p}", &i3);
  //=> i3 is 0xd7b99df958
}
