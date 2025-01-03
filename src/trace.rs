use std::sync::{Arc, LazyLock};

use crate::events;

static TRACE_EVENT_CONFIG: LazyLock<Arc<tokio::sync::Mutex<Option<events::TraceEventConfig>>>> =
    LazyLock::new(|| Arc::new(tokio::sync::Mutex::new(None)));

pub fn init() {
    let mut evt_config = TRACE_EVENT_CONFIG.try_lock().unwrap();
    *evt_config = Some(events::TraceEventConfig::all());
}
