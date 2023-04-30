pub trait Button {
    fn render(&self);
    fn on_click(&self);
}

pub trait Dialog {
    /// これは、Factory method でButton を作成する
    fn create_button(&self) -> Box<dyn Button>;

    fn render(&self) {
        let button = self.create_button();
        button.render();
    }

    fn refresh(&self) {
        println!("Dialog - Refresh");
    }
}
