// 配列は要素数が固定だが、Vec<T>は要素数が可変な型。
// Vec<T>は構造体

pub fn run() {
    // マクロでvecを定義
    let mut vec = vec![0, 1, 2, 3];
    //=> 1: vec [0, 1, 2, 3]

    println!("1: vec {:?}", vec);

    // 末尾に追加
    vec.push(4);

    println!("2: pushした後のvec {:?}", vec);
    //=> 2: pushした後のvec [0, 1, 2, 3, 4]

    println!("3: [n]で各要素にアクセスできる, {}", vec[2]);
    //=> 3: [n]で各要素にアクセスできる, 2

    // &vec[start..end]で部分列を取り出すことができる
    println!("4: スライスで値を取り出す {:?}", &vec[0..2]);
    //=> 4: スライスで部分的に値を取り出す [0, 1]

    for num in vec {
        println!("x: for-in文を使ってvecの値を出力する {}", num);
    }

    // println!("[Error] {:?}", vec);

    // with_capacityでキャパシティーを指定
    let mut v1 = Vec::<i32>::with_capacity(5);

    println!("v1のlen = {}, capacity = {}", v1.len(), v1.capacity());
    //=> v1のlen = 0, capacity = 5

    v1.push(10);
    v1.push(20);
    v1.push(30);

    println!("要素を3つ追加した後のv1のlen = {}, capacity = {}", v1.len(), v1.capacity());
    //=> 要素を3つ追加した後のv1のlen = 3, capacity = 5

    v1.push(40);
    v1.push(50);
    v1.push(60);

    // キャパシティーが10に拡張されている
    println!("さらに要素を3つ追加した後のv1のlen = {}, capacity = {}", v1.len(), v1.capacity());
    //=> さらに要素を3つ追加した後のv1のlen = 6, capacity = 10
}

// https://qiita.com/kanna/items/ea5b15f1b4ce0fee2ab3
