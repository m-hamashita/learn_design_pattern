use crate::{button::MacButton, checkbox::MacCheckbox};
use gui::{Button, Checkbox, GuiFactory, GuiFactoryDynamic};

pub struct MacFactory;

impl GuiFactory for MacFactory {
    type B = MacButton;
    type C = MacCheckbox;

    fn create_button(&self) -> Self::B {
        MacButton
    }

    fn create_checkbox(&self) -> Self::C {
        MacCheckbox
    }
}

impl GuiFactoryDynamic for MacFactory {
    fn create_button(&self) -> Box<dyn Button> {
        Box::new(MacButton)
    }

    fn create_checkbox(&self) -> Box<dyn Checkbox> {
        Box::new(MacCheckbox)
    }
}
