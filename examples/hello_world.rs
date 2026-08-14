use my_ui_framework::Effect;
use my_ui_framework::JsonBackend;
use my_ui_framework::UiRoot;
use my_ui_framework::run;
use std::process::ExitCode;

struct App;

enum Message {}

fn view(_: &App, ui: UiRoot) {
    ui.plain_text("Hello, World!");
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
