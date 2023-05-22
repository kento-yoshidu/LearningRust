use itertools::{Itertools, MinMaxResult};

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
    let vec6 = String::from("Hello World");

    // chars() = 文字列を分割したIteratorが返る
    // Iterator<char>
    let mut iter6 = vec6.chars().step_by(2);

    println!("iter6 = {:?}", iter6.next());
    //=> ite4 = Some('H')
    println!("iter6 = {:?}", iter6.next());
    //=> ite4 = Some('l')
    println!("iter6 = {:?}", iter6.next());
    //=> ite4 = Some('o')
    println!("iter6 = {:?}", iter6.next());
    //=> ite4 = Some('W')

    /* sum */
    // iteratorの合計値を求める
    let vec7 = vec![1, 2, 3, 4, 5];

    let mut iter7 = vec7.iter();

    // sumの戻り値には型注釈が必要
    println!("iter7にsumを適用 {}\n", iter7.sum::<i32>());
    //=> iter7にsumを適用 15

    // 合計値を吐く、という挙動だけ見ればIteratorのメソッドっぽくないが、next()を呼び出して順々に足し算している
    // 試しにVec<i32>でsum()を呼び出そうとすると、Iteratorではない、というエラーメッセージが出力される
    // println!("🦀❌ vec7にsumを適用? {}", vec7.sum::<i32>());
    //                                      ^^^ `Vec<i32>` is not an iterator; try calling `.into_iter()` or `.iter()`

    /* take */
    // n個分の要素を取り出し、Iterator<T>で返す
    let vec = vec![1, 2, 3, 4, 5];

    let mut iter = vec.iter().take(3);

    println!("takeで先頭3つを取り出す = {:?}", iter.next());
    //=> takeで先頭3つを取り出す = Some(1)
    println!("takeで先頭3つを取り出す = {:?}", iter.next());
    //=> takeで先頭3つを取り出す = Some(2)
    println!("takeで先頭3つを取り出す = {:?}\n", iter.next());
    //=> takeで先頭3つを取り出す = Some(3)

    /* skip */
    // n個分の要素をスキップし、それ以降をIterator<T>で返す
    let vec8 = vec![1, 2, 3, 4, 5];

    // 1, 2, 3をスキップする
    // Iterator<i32>
    let mut iter8 = vec8.iter().skip(3);

    println!("iter8 = {:?}", iter8.next());
    //=> iter8 = Some(4)
    println!("iter8 = {:?}\n", iter8.next());
    //=> iter8 = Some(5)

    /* count */
    // Iteratorの要素数を返す
    // Iterator.next()が最初にNoneを返すまでカウントしている

    let vec9 = vec![1, 2, 3, 4, 5];

    // usize
    println!("vec9の要素数 = {}\n", vec9.iter().count());
    //=> vec9の要素数 = 5

    /* max_by */
    // よく分からない
    /*
    let vec10 = [1, 3, 5, 4, 2];

    let mut iter10 = vec10.iter().max_by(|a, b| {
        println!("a = {}", a);
        println!("b = {}", b);
        b.cmp(a)
    });

    println!("{:?}", iter10);
    */

    /* position_max */
    // 最大のものの位置(index)を返す
    let vec11 = [1, 3, 5, 4, 2];

    // Iterator<i32>
    println!("vec11の最大値のindex = {:?}\n", vec11.iter().position_max());
    //=> vec11の最大値のindex = Some(2)

    /* position_min */
    let vec12 = [1, 3, 5, 4, 2];

    println!("vec12の最小値のindex = {:?}\n", vec12.iter().position_min());
    //=> vec12の最小値のindex = Some(0)

    /* position_minmax */
    // 最小値のindexと最大値のindexを取得する
    // MinMaxResult型のタプルが返る
    let vec13 = vec![1, 3, 5, 4, 2];

    //  MinMaxResult<usize>
    let iter13 = vec13.iter().position_minmax();

    // Result型の一種?なので、unwrapで取り出す
    println!("vec13の最小値のindex = {}, 最大値のindex = {}\n", iter13.into_option().unwrap().0, iter13.into_option().unwrap().1);
    //=> vec13の最小値のindex = 0, 最大値のindex = 2


    /* 実践 */
    // 偶数を取り出し、それぞれを2倍して、その合計値を求める
    let vec = vec![1, 2, 3, 4, 5, 6, 7, 8];

    let mut total: i32 = vec.iter()
                    .filter(|&x| x % 2 == 0)
                    .map(|&x| x * 2)
                    .sum();

    println!("{}", total);
    //=> 40
}

// https://qiita.com/lo48576/items/34887794c146042aebf1

