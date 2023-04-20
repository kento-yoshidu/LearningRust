// 配列は要素数が固定だが、Vec<T>は要素数が可変な型。
// Vec<T>は構造体

pub fn run() {
    let mut vec = vec![0, 1, 2, 3];

    println!("1: vec {:?}", vec);

    vec.push(4);

    println!("2: pushした後のvec {:?}", vec);

    println!("3: [n]で各要素にアクセスできる, {}", vec[2]);

    // &vec[start..end]で部分列を取り出すことができる
    println!("4: スライスで値を取り出す {:?}", &vec[0..2]);

    for num in vec {
        println!("x: for-in文を使ってvecの値を出力する {}", num);
    }

    // println!("[Error] {:?}", vec);
}

// https://qiita.com/kanna/items/ea5b15f1b4ce0fee2ab3
