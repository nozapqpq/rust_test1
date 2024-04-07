#[cfg(test)]
mod test_application_behavior {
    use rust_test1::MyApp;
    use rust_test1::Message;
    use std::collections::HashMap;
    use iced::Application;

    #[test]
    fn test_application_behavior() {
        // アプリケーションの振る舞いが期待通りであることをテスト
        let mut app = MyApp::new(String::new(), HashMap::new(), 0);

        // title()メソッドのテスト
        assert_eq!(app.title(), "");

        // update()メソッドのテスト
        app.update(Message::UpdateDisplay);
        assert_eq!(app.current_index(), 0); // current_index の値をテスト

        // view()メソッドのテスト
        let _element = app.view();
        // ここでElementの検証を行う
    }
}
