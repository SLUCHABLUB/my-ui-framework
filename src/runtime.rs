use crate::Effect;
use crate::UiRoot;
use crate::View;
use std::process::ExitCode;

#[must_use = "the exit code should pe propagated"]
pub fn run<App, Message, Backend>(
    initial_state: App,
    view: fn(&App, UiRoot),
    update: fn(&mut App, Message) -> Effect,
    backend: Backend,
) -> ExitCode
where
    Backend: crate::Backend,
{
    backend.drive(Runtime {
        app: initial_state,
        view_generator: view,
        update,
        view: View::new(),
    })
}

pub struct Runtime<App, Message> {
    app: App,
    view_generator: fn(&App, UiRoot<'_>),
    #[expect(unused, reason = "TODO")]
    update: fn(&mut App, Message) -> Effect,

    view: View,
}

impl<App, Message> Runtime<App, Message> {
    pub fn tick(&mut self) -> TickResult {
        // TODO: Go though pending messages.

        // TODO: Only do this if we have to.
        {
            let ui = self.view.clear();
            (self.view_generator)(&self.app, ui);

            // TODO: Diff the new view against the element tree.
        }

        TickResult {}
    }
}

#[must_use = "it may contain edits to the UI"]
pub struct TickResult {
    // TODO: Add the `Edit`s.
    // TODO: Add a `TickRequest`.
}
