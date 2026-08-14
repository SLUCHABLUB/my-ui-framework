use crate::Effect;
use crate::Ui;
use std::process::ExitCode;

#[must_use = "the exit code should pe propagated"]
pub fn run<App, Message, Backend>(
    initial_state: App,
    view: fn(&App, &mut Ui),
    update: fn(&mut App, Message) -> Effect,
    backend: Backend,
) -> ExitCode
where
    Backend: crate::Backend,
{
    backend.drive(Runtime {
        app: initial_state,
        view,
        update,
    })
}

pub struct Runtime<App, Message> {
    #[expect(unused)]
    app: App,
    #[expect(unused)]
    view: fn(&App, &mut Ui),
    #[expect(unused)]
    update: fn(&mut App, Message) -> Effect,
}

impl<App, Message> Runtime<App, Message> {
    pub fn tick(&mut self) -> TickResult {
        // TODO: Go though pending messages.
        // TODO: Rebuild the view if we have to.
        TickResult {}
    }
}

#[must_use = "it may contain edits to the UI"]
pub struct TickResult {
    // TODO: Add the `Edit`s.
    // TODO: Add a `TickRequest`.
}
