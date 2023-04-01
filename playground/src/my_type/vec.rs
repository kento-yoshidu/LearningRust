// 配列は要素数が固定だが、Vec<T>は要素数が可変な型。
// Vec<T>は構造体

pub fn run() {
    // インスタンス生成
    // Vec<i32>と推論される
    let mut vec = vec![0, 1, 2, 3];

    println!("1: vec {:?}", vec);

    vec.push(4);

    println!("2: pushした後のvec {:?}", vec);

    println!("3: [n]で各要素にアクセスできる, {}", vec[2]);

    // &vec[start..end]で部分列を取り出すことができる
    println!("4: スライスで値を取り出す {:?}", &vec[0..2]);
}
