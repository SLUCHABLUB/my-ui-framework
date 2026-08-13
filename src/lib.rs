use std::process::ExitCode;

pub struct Ui;

pub enum Effect {
    None,
}

pub fn run<App, Message, Backend>(
    initial_state: App,
    view: fn(&App, &mut Ui),
    update: fn(&mut App, Message) -> Effect,
    backend: Backend,
) -> ExitCode {
    // TODO: Construct a runtime.
    let _ = (initial_state, view, update);

    // TODO: Pass the runtime to the backend.
    let _ = backend;

    ExitCode::SUCCESS
}
