#[cfg(unix)]
pub(crate) mod unix;

#[cfg(not(unix))]
pub(crate) mod stubs;

#[cfg(unix)]
pub(crate) use unix as platform;

pub use platform::signal;
pub use platform::terminal;
