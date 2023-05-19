pub fn run() {
    /* next */
    // 次の要素をSome<T>で返す
    // なければNoneを返す
    let vec1 = [1, 2, 3];

    let mut iter1 = vec1.iter();

    // Some<i32>
    println!("iter1.next() = {:?}", iter1.next());
    //=> iter1.next() = Some(1)
    println!("iter1.next() = {:?}", iter1.next());
    //=> iter1.next() = Some(2)
    println!("iter1.next() = {:?}", iter1.next());
    //=> iter1.next() = Some(3)
    println!("iter1.next() = {:?}\n", iter1.next());
    //=> iter1.next() = None

    /* map */
    // 要素ごとに関数を適用する
    let vec2 = vec![1, 2, 3];

    // 値を2倍にする関数を順番に適用し返す
    // Iterator<i32>
    let mut iter2 = vec2.iter().map(|x| x * 2);

    println!("iter2 = {:?}", iter2.next());
    //=> iter2 = 2
    println!("iter2 = {:?}", iter2.next());
    //=> iter2 = 4
    println!("iter2 = {:?}", iter2.next());
    //=> iter2 = 6
    println!("iter2 = {:?}\n", iter2.next());
    //=> iter2 = None

    /* filter */
    let vec3 = vec![1, 2, 3, 4, 5];

    // 2で割り切れる数だけを返す
    // Iterator<i32>
    let mut iter3 = vec3.iter().filter(|&i| i % 2 == 0);

    println!("iter3 = {:?}", iter3.next());
    //=> iter3 = 2
    println!("iter3 = {:?}", iter3.next());
    //=> iter3 = 4
    println!("iter3 = {:?}\n", iter3.next());
    //=> iter3 = None

    /* filter_map */
    let vec4 = vec!["1", "2", "a", "b", "c", "3"];

    // i32に変換できたものだけを返す
    // .ok()でOptions型に変換する(初見殺し過ぎる)
    // Iterator<i32>
    let mut iter4 = vec4.iter().filter_map(|arg| arg.parse::<i32>().ok());

    println!("iter4 = {:?}", iter4.next());
    //=> iter4 = Some(1)
    println!("iter4 = {:?}", iter4.next());
    //=> iter4 = Some(2)
    println!("iter4 = {:?}", iter4.next());
    //=> iter4 = Some(3)
    println!("iter4 = {:?}\n", iter4.next());
    //=> iter4 = None

    /* find */
    // 再処理trueだったもののうち最初のものをSome(T)で返す
    let vec5 = vec![9, 11, 12, 14];

    // Some<i32>
    println!("vec5のうち2で割り切れる最初の要素 = {:?}\n", vec5.iter().find(|&i| i % 2 == 0));
    //=> vec4のうち2で割り切れる最初の要素 = Some(12)

    /* maxとmin */
    // Iterator<T>のうち最大 or 最小のものをOption<T>で返す
    // TはOrdトレイトを実装している必要がある
    let vec5 = vec![1, 2, 3, 4, 5];

    // Some<i32>
    println!("vec5のうち最大の要素 = {:?}", vec5.iter().max());
    //=> vec5のうち最大の要素 = Some(5)
    println!("vec5のうち最小の要素 = {:?}\n", vec5.iter().min());
    //=> vec5のうち最小の要素 = Some(1)

    // println!("{:?}")

    /* step_by */
    // n個ごとに返す
    // 競プロの文字列操作で使えそう
    let vec4 = String::from("Hello World");

    // chars() = 文字列を分割したIteratorが返る
    // Iterator<char>
    let mut ite4 = vec4.chars().step_by(2);

    println!("ite4 = {:?}", ite4.next());
    //=> ite4 = Some('H')
    println!("ite4 = {:?}", ite4.next());
    //=> ite4 = Some('l')
    println!("ite4 = {:?}", ite4.next());
    //=> ite4 = Some('o')
    println!("ite4 = {:?}", ite4.next());
    //=> ite4 = Some('W')
}

// https://qiita.com/lo48576/items/34887794c146042aebf1#%E4%B8%AD%E8%BA%AB%E3%82%92%E5%BC%84%E3%82%89%E3%81%9A%E8%A6%81%E7%B4%A0%E3%82%92%E9%81%B8%E6%8A%9E%E3%81%99%E3%82%8B%E7%B3%BB
