use my_ui_framework::Backend;
use my_ui_framework::Effect;
use my_ui_framework::Ui;
use my_ui_framework::run;
use std::process::ExitCode;

struct App;

enum Message {}

// TODO: Use a real backend.
struct MockBackend;

impl Backend for MockBackend {
    fn drive<App, Message>(self, runtime: my_ui_framework::Runtime<App, Message>) -> ExitCode {
        drop(runtime);
        ExitCode::SUCCESS
    }
}

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
