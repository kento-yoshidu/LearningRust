use std::fs;
use std::error::Error;

use md_merge::add;

#[test]
fn merge_file() -> Result<(), Box<dyn Error>> {
   // テスト用のファイルパス
    let base_file = "test_base.md";
    let append_file = "test_append.md";
    let output_file = "test_output.md";

    // テスト用のコンテンツ
    let base_content = "# This is the base file\nContent of the base file.";
    let append_content = "# This is the appended file\nContent of the appended file.";
    let expected_content = "# This is the base file\nContent of the base file.\n\n# This is the appended file\nContent of the appended file.";

    // テスト用のファイルを生成
    fs::write(base_file, base_content)?;
    fs::write(append_file, append_content)?;

    // ファイルをマージ
    add(base_file, append_file, output_file)?;

    // 生成されたファイルの内容を確認
    let result_content = fs::read_to_string(output_file)?;

    // テスト後のクリーンアップ
    fs::remove_file(base_file)?;
    fs::remove_file(append_file)?;
    fs::remove_file(output_file)?;

    assert_eq!(result_content, expected_content);

    Ok(())
}
