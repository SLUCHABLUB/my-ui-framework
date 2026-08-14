use my_ui_framework::Effect;
use my_ui_framework::JsonBackend;
use my_ui_framework::Ui;
use my_ui_framework::run;
use std::process::ExitCode;

struct App;

enum Message {}

fn view(app: &App, ui: &mut Ui) {
    // TODO: Emit "Hello, World!".
    let _ = (ui, app);
}

fn update(_: &mut App, message: Message) -> Effect {
    match message {}
}

fn main() -> ExitCode {
    let mut backend = JsonBackend::default();

    let exit_code = run(App, view, update, &mut backend);

    println!("{:#}", backend.value);

    exit_code
}
