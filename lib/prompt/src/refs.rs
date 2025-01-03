use std::{
    borrow::{Borrow, BorrowMut},
    sync::Arc,
};
use tokio::sync::{Mutex, MutexGuard};

pub(crate) type EngineRef = Arc<Mutex<mishell_core::Engine>>;

pub(crate) struct EngineReader<'a> {
    pub engine: MutexGuard<'a, mishell_core::Engine>,
}

impl AsRef<mishell_core::Engine> for EngineReader<'_> {
    fn as_ref(&self) -> &mishell_core::Engine {
        self.engine.borrow()
    }
}

pub(crate) struct EngineWriter<'a> {
    pub engine: MutexGuard<'a, mishell_core::Engine>,
}

impl AsMut<mishell_core::Engine> for EngineWriter<'_> {
    fn as_mut(&mut self) -> &mut mishell_core::Engine {
        self.engine.borrow_mut()
    }
}
