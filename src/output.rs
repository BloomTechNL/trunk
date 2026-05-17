use std::io::{self, Write};
use std::process::{Command, ExitStatus};

pub trait OutputSink {
    fn write_str(&self, s: &str);
    fn run(&self, cmd: &mut Command) -> io::Result<ExitStatus>;
}

pub struct StdoutSink;

impl OutputSink for StdoutSink {
    fn write_str(&self, s: &str) {
        let stdout = io::stdout();
        let mut handle = stdout.lock();
        let _ = handle.write_all(s.as_bytes());
        let _ = handle.flush();
    }

    fn run(&self, cmd: &mut Command) -> io::Result<ExitStatus> {
        cmd.status()
    }
}
