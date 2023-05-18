// イテレーターはIteratorトレイトを実装している

// イテラブル = IntoIteratorを実装している型

// std::iter::Iteratorは次の様に実装されている
// nextメソッドはOption型を返す。次の値があればSome<T>を、無ければNoneを返す。
// Itemは関連型。インスタンスはItemの型を定義しなければならない
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
    // 続く
}

// IntoIteratorはイテレーターを返す
pub trait IntoIterator {
    type Item;
    type IntoIter: Iterator<Item=Self::Item>;
    // 値をイテレーターに変換している
    fn into_iter(self) -> Self::IntoIter;
}

// 🦀❓ Iteratorトレイトが実装可能かどうかはどうやって決まる?何を確認すれば分かる?

// RangeはIteratorを実装しているが、Vecは実装していない

// for i in x {} なら、xに対してinto_iterが呼ばれる
//=> https://mmi.hatenablog.com/entry/2019/02/18/011231

pub fn run() {
    let mut range = 1..3;

    println!("{:?}", range.next());
    //=> Some(1)
    println!("{:?}", range.next());
    //=> Some(2)
    println!("{:?}", range.next());
    //=> None

    let mut vec = vec![1, 2, 3];

    // 🦀❌ VecはIteratorを実装していないためエラー
    // println!("{:?}", vec.next());
    // error[E0599]: no method named `next` found for struct `Vec<{integer}>` in the current scope

    // じゃあ何でfor文で回せるの?
    for i in &vec {
        println!("vecをfor文で回す {}", i);
        //=> vecをfor文で回す 1
    }

    // for文はvecにinto_iter()を適用する
    // into_iter()が値をイテレーターに変換する

    // for文は以下と同じようなことをやっている
    let mut vec_to_iter = (&vec).into_iter();

    // イテレーターに変換されていることが分かる
    println!("{:?}", vec_to_iter);
    // Iter([1, 2, 3])

    // これならnext()を呼べる
    println!("{:?}", vec_to_iter.next());
    //=> Some(1)
    println!("{:?}", vec_to_iter.next());
    //=> Some(2)
    println!("{:?}", vec_to_iter.next());
    //=> Some(3)
    println!("{:?}", vec_to_iter);
    //=> Iter([])

    // 以降、iter()やinto_iter()と所有権の関係について考える

    let vec2 = vec![1, 2, 3];

    for i in vec2.iter() {
        println!("vec2をfor文で回す {}", i);
        //=> vec2をfor文で回す 1
        // ...
    }

    // iter()は参照しているだけなのでfor文で回した後もvec2にアクセスできる
    println!("{:?}", vec2);
    //=> [1, 2, 3]

    let vec3 = vec![1, 2, 3];

    for i in vec3.into_iter() {
        println!("vec3をfor文で回す {}", i);
        //=> vec3をfor文で回す 1
        // ...
    }

    // 🦀❌ iter_intoはmoveが起きるので所有権エラーになる
    // println!("{:?}", vec3);
    //                  ^^^^ value borrowed here after move
}

// https://speakerdeck.com/optim/domination-of-the-rust-iterators?slide=38
