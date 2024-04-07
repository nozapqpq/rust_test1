#[cfg(test)]
mod read_csv_nonexistent_file_test {
    use rust_test1::read_csv; // `read_csv`関数が定義されているモジュールにアクセスするため

    #[test]
    fn test_read_csv_nonexistent_file() {
        // 存在しないCSVファイルを指定した場合のテスト
        let filename = "nonexistent_file.csv";
        let result = read_csv(filename);

        // エラーが返されることをテスト
        assert!(result.is_err());
    }
}
