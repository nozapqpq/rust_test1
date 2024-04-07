# Rust使ってみた

## 反省点
- ChatGPTに全面的にお願いした
- テストより先にコードを書いてしまった。とてもよろしくなさそう。

## Rustのインストール方法(Windows)
- 参考：https://qiita.com/RyugaDome/items/76fd9251f4885de69edd
- Rustインストーラを公式からダウンロード
    https://doc.rust-jp.rs/book-ja/ch01-01-installation.html
    - rustupを使う
        https://www.rust-lang.org/ja/tools/install/
- インストーラを実行
    - 不足がある場合の対応
        - Visual StudioのC++ビルドツールのインストール
- Rustのバージョン確認
    `rustc --version`
- Cargoのバージョン確認（Rustのビルドシステム兼パッケージマネージャ）
    `cargo --version`


## 基本情報
- Cargo home directory `C:\Users\noza\.cargo`
- コンパイル `rustc hello.rs`
    - プロジェクトのビルドは、プロジェクトのルートディレクトリで
        - クリーンアップ：`cargo clean`
        - デバッグ：`cargo build`
        - リリース：`cargo build --release`
        - デバッグビルドして即実行する場合：`cargo run`
        - テスト実行：`cargo test`
- 新しいプロジェクトの作成 `cargo new my_project`

## フレームワーク、ライブラリ
- デスクトップアプリケーションを作成するには、UIフレームワークを選択する必要がある
    - とりあえず`iced`を使う。`druid`もある。`gtk-rs`はWindowsと相性が悪いらしい
    - `Cargo.toml`を編集、`dependencies`に`iced`の記述を追加する
