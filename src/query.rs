use std::path::Path;

use anyhow::Result;

use crate::git::git_passthrough;
use crate::handler::Handler;
use crate::output::OutputSink;

// ---------------------------------------------------------------------------
// g l / g s / g d  — read-only pass-throughs
// ---------------------------------------------------------------------------

pub struct LogHandler<'a, O: OutputSink> {
    sink: &'a O,
}

impl<'a, O: OutputSink> LogHandler<'a, O> {
    pub const fn new(sink: &'a O) -> Self {
        Self { sink }
    }
}

impl<O: OutputSink> Handler<&Path> for LogHandler<'_, O> {
    fn handle(&self, dir: &Path) -> Result<()> {
        git_passthrough(dir, &["log"], self.sink)
    }
}

pub struct StatusHandler<'a, O: OutputSink> {
    sink: &'a O,
}

impl<'a, O: OutputSink> StatusHandler<'a, O> {
    pub const fn new(sink: &'a O) -> Self {
        Self { sink }
    }
}

impl<O: OutputSink> Handler<&Path> for StatusHandler<'_, O> {
    fn handle(&self, dir: &Path) -> Result<()> {
        git_passthrough(dir, &["status"], self.sink)
    }
}

pub struct DiffHandler<'a, O: OutputSink> {
    sink: &'a O,
}

impl<'a, O: OutputSink> DiffHandler<'a, O> {
    pub const fn new(sink: &'a O) -> Self {
        Self { sink }
    }
}

impl<O: OutputSink> Handler<&Path> for DiffHandler<'_, O> {
    fn handle(&self, dir: &Path) -> Result<()> {
        git_passthrough(dir, &["diff"], self.sink)
    }
}
