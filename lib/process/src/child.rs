use os_pipe::PipeReader;

#[derive(Debug)]
pub struct ChildPipe(PipeReader);

impl From<PipeReader> for ChildPipe {
    fn from(value: PipeReader) -> Self {
        Self(value)
    }
}

impl std::io::Read for ChildPipe {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.0.read(buf)
    }
}

pub struct ChildProcess {
    pub stdout: Option<ChildPipe>,
    pub stderr: Option<ChildPipe>,
    exit_status: i32,
}

impl ChildProcess {}
