use gui::Checkbox;

pub struct MacCheckbox;

impl Checkbox for MacCheckbox {
    fn switch(&self) {
        println!("Mac button switched");
    }
}
