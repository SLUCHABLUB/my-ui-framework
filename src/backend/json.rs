use crate::Backend;
use crate::Runtime;
use crate::runtime::TickResult;
use json::JsonValue;
use std::process::ExitCode;

/// A [`Backend`] implementation that constructs a JSON object representing the UI.
/// This only intended for testing.
pub struct JsonBackend {
    pub value: JsonValue,
}

impl Default for JsonBackend {
    fn default() -> JsonBackend {
        JsonBackend {
            value: JsonValue::Null,
        }
    }
}

impl Backend for &mut JsonBackend {
    fn drive<App, Message>(self, mut runtime: Runtime<App, Message>) -> ExitCode {
        #[expect(clippy::never_loop, reason = "TODO")]
        loop {
            let TickResult {} = runtime.tick();

            // TODO: Send a close request.
            break ExitCode::SUCCESS;
        }
    }
}
