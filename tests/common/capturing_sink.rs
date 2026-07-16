#![allow(dead_code)]
use std::cell::RefCell;
use std::io;
use std::process::{Command, ExitStatus, Stdio};
use std::rc::Rc;

use g_cli::output::OutputSink;

#[derive(Clone)]
pub struct CapturingSink {
    buf: Rc<RefCell<String>>,
}

impl CapturingSink {
    pub fn new() -> Self {
        Self {
            buf: Rc::new(RefCell::new(String::new())),
        }
    }

    pub fn take(&self) -> String {
        std::mem::take(&mut *self.buf.borrow_mut())
    }
}

impl OutputSink for CapturingSink {
    fn write_str(&self, s: &str) {
        self.buf.borrow_mut().push_str(s);
    }

    fn run(&self, cmd: &mut Command) -> io::Result<ExitStatus> {
        let output = cmd.stdout(Stdio::piped()).stderr(Stdio::null()).output()?;
        self.buf
            .borrow_mut()
            .push_str(&String::from_utf8_lossy(&output.stdout));
        Ok(output.status)
    }

    fn capture(&self, cmd: &mut Command) -> io::Result<(ExitStatus, Vec<u8>)> {
        let output = cmd.stdout(Stdio::piped()).stderr(Stdio::null()).output()?;
        self.buf
            .borrow_mut()
            .push_str(&String::from_utf8_lossy(&output.stdout));
        Ok((output.status, output.stdout))
    }
}
