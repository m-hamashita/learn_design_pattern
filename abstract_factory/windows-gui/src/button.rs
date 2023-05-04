use gui::Button;

pub struct WindowsButton;

impl Button for WindowsButton {
    fn press(&self) {
        println!("Windows Button pressed");
    }
}
