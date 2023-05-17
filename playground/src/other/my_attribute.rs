/* Attribute、属性 */

// attributeはモジュールや関数などに付与するメタデータ

/* Outer Attribute */
// 要素(関数や構造体など)に適用
// #[]で宣言

/* Inner Attribute */
// モジュールやクレート全体に適用
// #![]で宣言

/* #[cfg] */
// 条件付きコンパイル

// OSがLinuxの時のみコンパイルされる
#[cfg(target_os = "linux")]
fn do_something() {}

// テスト実行時のみコンパイルされる
#[cfg(test)]
fn test() {}

/* #[derive] */
// 構造体やEnumに任意のトレイトを実装する
#[derive(Debug, Clone)]
struct Persion {}

/* #[allow] */
// warningやerrorを抑制する
// dead_codeは使用していない関数にwarningを出す
#[allow(dead_code)]
fn unused() {}

/* #[test] */
// テスト時に実行される

pub fn run() {
    println!("Attribute");
}

// https://zenn.dev/moxak/articles/a328d78987f133

// https://qiita.com/dalance/items/1911a775ee23f3e35d18
