use my_ui_framework::Effect;
use my_ui_framework::Ui;
use my_ui_framework::run;
use std::process::ExitCode;

struct App;

enum Message {}

// TODO: Use a real backend.
struct MockBackend;

fn view(app: &App, ui: &mut Ui) {
    // TODO: Emit "Hello, World!".
    let _ = (ui, app);
}

fn update(_: &mut App, message: Message) -> Effect {
    match message {}
}

fn main() -> ExitCode {
    run(App, view, update, MockBackend)
}
