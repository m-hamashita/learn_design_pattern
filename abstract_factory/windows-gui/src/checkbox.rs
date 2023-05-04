use gui::Checkbox;

pub struct WindowsCheckbox;

impl Checkbox for WindowsCheckbox {
    fn switch(&self) {
        println!("Windows Checkbox switched");
    }
}
