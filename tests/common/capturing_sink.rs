#![allow(dead_code)]
use std::cell::RefCell;
use std::io;
use std::process::{Command, ExitStatus, Stdio};
use std::rc::Rc;

use g_cli::output::OutputSink;

use super::timeline::Timeline;

#[derive(Clone)]
pub struct CapturingSink {
    buf: Rc<RefCell<String>>,
    timeline: Timeline,
}

impl CapturingSink {
    pub fn new(timeline: Timeline) -> Self {
        Self {
            buf: Rc::new(RefCell::new(String::new())),
            timeline,
        }
    }

    pub fn take(&self) -> String {
        std::mem::take(&mut *self.buf.borrow_mut())
    }

    fn describe(cmd: &Command) -> String {
        let program = cmd.get_program().to_string_lossy().into_owned();
        cmd.get_args().next().map_or(program.clone(), |sub| {
            format!("{program} {}", sub.to_string_lossy())
        })
    }
}

impl OutputSink for CapturingSink {
    fn write_str(&self, s: &str) {
        self.buf.borrow_mut().push_str(s);
    }

    fn run(&self, cmd: &mut Command) -> io::Result<ExitStatus> {
        self.timeline.record(Self::describe(cmd));
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
