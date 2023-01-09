use std::io;

fn main() {
    let mut service_type = String::new();
    println!("実行したい内容を入力してください。(0: 登録、　1: 集計)");
    io::stdin().read_line(&mut service_type).unwrap();

    // 改行コードや余分なスペースを取り除き、u8型に変換
    let service_type: u8 = service_type.trim().parse().expect("数値で入力してください。");

    // 入力値のバリデーション

    if service_type == 0 {
        print!("登録サービス");
    } else if service_type == 1 {
        print!("集計サービス");
    }
}
