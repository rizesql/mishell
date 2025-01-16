use lazy_static::lazy_static;
use std::collections::HashSet;
use std::env;
use std::fs::{self};
use std::os::unix::fs::PermissionsExt;
use std::sync::Arc;
use tokio::sync::{Mutex, Notify};

lazy_static! {
    pub static ref EXECUTABLES_CACHE: Arc<ExecutablesCache> = Arc::new(ExecutablesCache::new());
}

pub struct ExecutablesCache {
    cache: Mutex<HashSet<String>>, // Mutex pentru a proteja cache-ul
    notify: Notify,                // Notificare pentru a semnala completarea populării
    populated: Mutex<bool>,        // Flag pentru a verifica dacă cache-ul a fost populat
}

impl ExecutablesCache {
    pub fn new() -> Self {
        ExecutablesCache {
            cache: Mutex::new(HashSet::new()),
            notify: Notify::new(),
            populated: Mutex::new(false),
        }
    }

    fn is_executable(&self, file_path: &str) -> bool {
        if let Ok(metadata) = fs::metadata(file_path) {
            metadata.is_file() && metadata.permissions().mode() & 0o111 != 0
        } else {
            false
        }
    }

    /// Populează cache-ul asincron
    pub async fn populate_cache(&self) {
        let mut is_populated = self.populated.lock().await;
        if *is_populated {
            return; // Cache-ul este deja populat
        }

        tracing::info!("Populating executable cache...");
        let mut cache = self.cache.lock().await;
        cache.insert("exit".to_string());
        cache.insert("cd".to_string());
        cache.insert("exec".to_string());

        if let Ok(path) = env::var("PATH") {
            for dir in path.split(':') {
                match fs::read_dir(dir) {
                    Ok(entries) => {
                        for entry in entries.filter_map(Result::ok) {
                            if let Some(file_name) = entry.file_name().to_str() {
                                let full_path = format!("{}/{}", dir, file_name);

                                if self.is_executable(&full_path) {
                                    cache.insert(file_name.to_string());
                                }
                            }
                        }
                    }
                    Err(_) => {
                        // Ignoră erorile
                    }
                }
            }
        }

        *is_populated = true; // Marchează cache-ul ca populat
        self.notify.notify_waiters(); // Notifică toți cei care așteaptă
    }

    /// Verifică dacă un executabil există în cache
    pub async fn lookup(&self, executable: &str) -> bool {
        let populated = *self.populated.lock().await;

        if !populated {
            tracing::info!("Cache not populated yet, waiting...");
            self.populate_cache().await;
            self.notify.notified().await; // Așteaptă până când cache-ul este populat
        }

        let cache = self.cache.lock().await;
        cache.contains(executable)
    }
}
