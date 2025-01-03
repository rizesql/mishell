use std::os::unix::process::ExitStatusExt;

#[derive(Debug, Default)]
pub struct ExitCode {
    value: u8,
    should_exit: bool,
}

impl ExitCode {
    pub fn new(exit_code: u8) -> Self {
        Self {
            value: exit_code,
            ..Self::default()
        }
    }

    pub fn success() -> Self {
        Self::new(0)
    }

    pub fn stopped() -> Self {
        Self::new(128 + nix::libc::SIGTSTP as u8)
    }

    pub fn value(&self) -> u8 {
        self.value
    }

    pub fn should_exit(&self) -> bool {
        self.should_exit
    }
}

impl From<std::process::Output> for ExitCode {
    fn from(output: std::process::Output) -> Self {
        if let Some(code) = output.status.code() {
            return Self::new(to_u8(code));
        }

        #[cfg(unix)]
        if let Some(signal) = output.status.signal() {
            return Self::new(to_u8(signal) + 128);
        }

        tracing::error!("unhandled process exit");
        Self::new(127)
    }
}

fn to_u8(src: i32) -> u8 {
    (src & 0xFF) as u8
}
