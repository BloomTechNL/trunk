use std::io;
use std::process::{Command, ExitStatus, Stdio};
use std::sync::Mutex;

use g_cli::output::OutputSink;

/// Test-only `OutputSink` that accumulates everything written into an
/// internal buffer — both direct `write_str` calls and the stdout of any
/// child process the application spawns through `run`.
///
/// `take()` is stateful: it returns everything written since the last call
/// and then clears the buffer, so successive test assertions see only the
/// output produced by the most recent command.
pub struct CapturingSink {
    buf: Mutex<String>,
}

impl CapturingSink {
    pub fn new() -> Self {
        CapturingSink {
            buf: Mutex::new(String::new()),
        }
    }

    pub fn take(&self) -> String {
        let mut b = self.buf.lock().unwrap();
        std::mem::take(&mut *b)
    }
}

impl OutputSink for CapturingSink {
    fn write_str(&self, s: &str) {
        self.buf.lock().unwrap().push_str(s);
    }

    fn run(&self, cmd: &mut Command) -> io::Result<ExitStatus> {
        let output = cmd.stdout(Stdio::piped()).output()?;
        self.buf
            .lock()
            .unwrap()
            .push_str(&String::from_utf8_lossy(&output.stdout));
        Ok(output.status)
    }
}
