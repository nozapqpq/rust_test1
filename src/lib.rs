use std::fs::File;
use std::io::{self, BufReader};
use std::collections::HashMap;
use encoding_rs::SHIFT_JIS;
use csv::ReaderBuilder;
use iced::{Column, Element, Text, Application, Settings, Command, Align, Container, Length};
use async_std::task;
use std::time::Duration;


#[derive(Debug, Clone)]
pub enum Message {
    UpdateDisplay,
}

pub struct MyApp {
    title: String,
    data: HashMap<String, Vec<String>>,
    current_index: usize,
}

impl MyApp {
    pub fn new(title: String, data: HashMap<String, Vec<String>>, current_index: usize) -> Self {
        MyApp {
            title,
            data,
            current_index,
        }
    }

    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn current_index(&self) -> usize {
        self.current_index
    }
}

pub fn read_csv(filename: &str) -> io::Result<(String, HashMap<String, Vec<String>>)> {
    // CSVファイルをShift-JISエンコーディングで開く
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    // CSVリーダーを作成し、Shift-JISエンコーディングを指定する
    let mut csv_reader = ReaderBuilder::new().from_reader(reader);

    let mut data = HashMap::new();

    // レコードの処理は以前のコードと同じです
    /*
    for result in csv_reader.records() {
        let record = result?;
        let mut iter = record.into_iter();
        if let Some(key) = iter.next() {
            // Shift-JISエンコーディングからUTF-8に変換
            let key_bytes: Vec<u8> = key.bytes().collect();
            let (key_utf8, _, _) = SHIFT_JIS.decode(&key_bytes);
            let values: Vec<String> = iter.map(|s| s.to_string()).collect();
            data.insert(key_utf8.to_string(), values);
        }
    }
    */
    for result in csv_reader.records() {
        let record = result?;
        let mut iter = record.into_iter();
        if let Some(key) = iter.next() {
            let key_string = String::from_utf8_lossy(key.as_bytes()).to_string(); // バイト文字列から直接文字列に変換
            let values: Vec<String> = iter.map(|s| s.to_string()).collect();
            data.insert(key_string, values);
        }
    }

    // 最初のヘッダーをタイトルとして取得
    let title = data.keys().next().unwrap_or(&String::from("No title")).clone();

    Ok((title, data))
}

impl Application for MyApp {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (MyApp, Command<Message>) {
        let (title, data) = read_csv("data.csv").unwrap_or_else(|err| {
            eprintln!("Error reading CSV file: {}", err);
            (String::from(""), HashMap::new())
        });

        (
            MyApp {
                title,
                data,
                current_index: 0,
            },
            Command::perform(update_interval(), |_| Message::UpdateDisplay),
        )
    }

    fn title(&self) -> String {
        self.title.clone()
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::UpdateDisplay => {
                if self.data.is_empty() {
                    return Command::none();
                }
                self.current_index = (self.current_index + 1) % self.data.len();
                self.title = self.data.keys().nth(self.current_index).unwrap().clone();
    
                Command::perform(update_interval(), |_| Message::UpdateDisplay)
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        let title_text = Text::new(&self.title)
            .size(66)
            .color([1.0, 0.0, 0.0]); // タイトルを赤色に設定
    
        let mut columns = Column::new()
            .spacing(30) // 行間を50pxに設定
            .align_items(Align::Start) // テキストを左寄せに設定
            .push(title_text);
    
        if let Some(content) = self.data.get(&self.title) {
            for (index, item) in content.iter().enumerate() {
                let mut item_text = Text::new(item)
                    .size(54); // テキストのサイズを3倍に設定
                if index == 0 {
                    item_text = item_text.color([0.0, 0.0, 1.0]); // 青色に設定
                }
                columns = columns.push(item_text);
            }
        }
    
        // テキスト間の行間を調整するためにContainerを使用する
        Container::new(columns)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Align::Start) // コンテナ全体を中央に配置
            .align_y(Align::Center) // コンテナ全体を中央に配置
            .padding(50) // パディングを50pxに設定
            .into()
    }
}

async fn update_interval() {
    task::sleep(Duration::from_secs(12)).await;
}

