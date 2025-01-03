#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// An error occurred within the core shell.
    #[error("{0}")]
    Core(#[from] mishell_core::Error),

    #[error("i/o error: {0}")]
    Io(#[from] std::io::Error),

    #[error("input error occurred")]
    InputError,
}
