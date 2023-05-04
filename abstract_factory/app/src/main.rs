mod render;

use render::render;

use macos_gui::factory::MacFactory;
use windows_gui::factory::WindowsFactory;

fn main() {
    let windows = false;

    if windows {
        render(WindowsFactory);
    } else {
        render(MacFactory);
    }
}
