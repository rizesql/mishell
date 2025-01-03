#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("i/o error: {0}")]
    IoError(#[from] std::io::Error),

    /// The requested functionality has not yet been implemented in this shell.
    #[error("UNIMPLEMENTED: {0}")]
    Unimplemented(&'static str),

    /// A system error occurred.
    #[cfg(unix)]
    #[error("System error: {0}")]
    Sys(#[from] nix::errno::Errno),
}
