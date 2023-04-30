mod command;

// TUI crate
use cursive::{
    traits::Nameable,
    views::{Dialog, EditView},
    Cursive,
};

use command::{Command, CopyCommand, CutCommand, PasteCommand};

// clipboard と history を持つ
// clipboard は、コピーするためのもの
// history は、コマンドを実行した履歴を持つ(undo のため)
#[derive(Default)]
struct AppContext {
    clipboard: String,
    history: Vec<Box<dyn Command>>,
}

fn main() {
    let mut app = cursive::default();

    app.set_user_data(AppContext::default());
    app.add_layer(
        Dialog::around(EditView::default().with_name("Editor"))
            .title("Type and use buttons")
            .button("Copy", |s| execute(s, CopyCommand::default()))
            .button("Cut", |s| execute(s, CutCommand::default()))
            .button("Paste", |s| execute(s, PasteCommand::default()))
            .button("Undo", undo)
            .button("Quit", |s| s.quit()),
    );

    app.run();
}

/// Executes a command and then pushes it to a history array.
/// If the command returns false, it is not pushed to the history.
fn execute(app: &mut Cursive, mut command: impl Command + 'static) {
    if command.execute(app) {
        app.with_user_data(|context: &mut AppContext| {
            // Box で囲むことで、dyn Command として扱える
            context.history.push(Box::new(command));
        });
    }
}

/// Pops the last command and executes an undo action.
/// If there is no command to undo, nothing happens.
fn undo(app: &mut Cursive) {
    let mut context = app.take_user_data::<AppContext>().unwrap();
    // Some であれば、undo できる
    if let Some(mut command) = context.history.pop() {
        command.undo(app)
    }
    app.set_user_data(context);
}
