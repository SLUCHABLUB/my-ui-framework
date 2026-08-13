use crate::Effect;
use crate::Ui;
use std::process::ExitCode;

pub struct Runtime<App, Message> {
    #[expect(unused)]
    app: App,
    #[expect(unused)]
    view: fn(&App, &mut Ui),
    #[expect(unused)]
    update: fn(&mut App, Message) -> Effect,
}

pub fn run<App, Message, Backend>(
    initial_state: App,
    view: fn(&App, &mut Ui),
    update: fn(&mut App, Message) -> Effect,
    backend: Backend,
) -> ExitCode
where
    Backend: crate::Backend,
{
    let runtime = Runtime {
        app: initial_state,
        view,
        update,
    };

    // TODO: Pass the runtime to the backend.
    let _ = (backend, runtime);

    ExitCode::SUCCESS
}
