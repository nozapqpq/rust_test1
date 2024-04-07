#[cfg(test)]
mod csv_parsing_test {
    use rust_test1::read_csv;
    use super::*;
    use std::io::Write;
    use std::fs;

    #[test]
    fn test_csv_parsing() {
        // CSVファイルのデータが正しく解析されることをテスト
        let csv_data = "Title1,Value11,Value12\nTitle2,Value21,Value22";
        let mut file = fs::File::create("test_data.csv").unwrap();
        writeln!(file, "{}", csv_data).unwrap();
        file.sync_all().unwrap();

        let (_, data) = read_csv("test_data.csv").unwrap();

        assert_eq!(data.len(), 1);
        assert_eq!(data.get("Title2").unwrap(), &vec!["Value21", "Value22"]);

        fs::remove_file("test_data.csv").unwrap();
    }
}