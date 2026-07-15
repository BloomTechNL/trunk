#![allow(dead_code)]
use std::io;
use std::process::{Command, ExitStatus, Stdio};
use std::sync::Mutex;

use g_cli::output::OutputSink;

pub struct CapturingSink {
    buf: Mutex<String>,
}

impl CapturingSink {
    pub const fn new() -> Self {
        Self {
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
        let output = cmd.stdout(Stdio::piped()).stderr(Stdio::null()).output()?;
        self.buf
            .lock()
            .unwrap()
            .push_str(&String::from_utf8_lossy(&output.stdout));
        Ok(output.status)
    }

    fn capture(&self, cmd: &mut Command) -> io::Result<(ExitStatus, Vec<u8>)> {
        let output = cmd.stdout(Stdio::piped()).stderr(Stdio::null()).output()?;
        self.buf
            .lock()
            .unwrap()
            .push_str(&String::from_utf8_lossy(&output.stdout));
        Ok((output.status, output.stdout))
    }
}
