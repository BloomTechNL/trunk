use std::io::{self, Write};
use std::process::{Command, ExitStatus, Stdio};

pub trait OutputSink {
    fn write_str(&self, s: &str);
    fn run(&self, cmd: &mut Command) -> io::Result<ExitStatus>;
    fn capture(&self, cmd: &mut Command) -> io::Result<(ExitStatus, Vec<u8>)>;
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

    fn capture(&self, cmd: &mut Command) -> io::Result<(ExitStatus, Vec<u8>)> {
        let output = cmd.stdout(Stdio::piped()).stderr(Stdio::null()).output()?;
        Ok((output.status, output.stdout))
    }
}
