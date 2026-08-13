use crate::Runtime;
use std::process::ExitCode;

pub trait Backend: Sized {
    fn drive<App, Message>(self, runtime: Runtime<App, Message>) -> ExitCode;
}
