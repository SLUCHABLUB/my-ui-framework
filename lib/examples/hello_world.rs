use my_ui_framework::Effect;
use my_ui_framework::Ui;
use my_ui_framework::run;
use std::process::ExitCode;

struct App;

fn view(app: &App, ui: &mut Ui) {
    // TODO: Emit "Hello, World!".
    let _ = (ui, app);
}

enum Message {}

fn update(app: &mut App, message: Message) -> Effect {
    let _ = app;
    match message {}
}

struct MockBackend;

fn main() -> ExitCode {
    run(App, view, update, MockBackend)
}
