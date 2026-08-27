//! Shared NF config-loading utilities: CLI/env/default path resolution and an
//! ArcSwap-backed hot-reloadable config store, generic over each NF's own
//! config type and YAML-parsing logic.

use arc_swap::ArcSwap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, OnceLock};

/// Debounce window between successive config-file reloads.
const RELOAD_DEBOUNCE: std::time::Duration = std::time::Duration::from_secs(2);

/// Resolve a config file path from `-c`/`--config` CLI args, then `env_var`,
/// then `default_path`.
pub fn resolve_config_path(env_var: &str, default_path: &str) -> String {
    std::env::args()
        .zip(std::env::args().skip(1))
        .find_map(|(a, b)| (a == "-c" || a == "--config").then_some(b))
        .or_else(|| std::env::var(env_var).ok())
        .unwrap_or_else(|| default_path.to_string())
}

/// Resolve the config path, load it via `loader`, publish it into
/// `config_store`, and record the path in `config_path_cell` for the watcher.
/// Returns the resolved path so callers don't need a second resolution just
/// for logging.
pub fn initialize_config<T>(
    config_path_cell: &'static OnceLock<String>,
    config_store: &'static ArcSwap<T>,
    env_var: &str,
    default_path: &str,
    loader: impl FnOnce(&str) -> T,
) -> String {
    let config_path = resolve_config_path(env_var, default_path);
    let _ = config_path_cell.set(config_path.clone());
    config_store.store(Arc::new(loader(&config_path)));
    config_path
}

/// Return the current atomically published configuration snapshot.
pub fn runtime_config<T>(config_store: &'static ArcSwap<T>) -> Arc<T> {
    config_store.load_full()
}

/// Watch the config file recorded in `config_path_cell` and atomically
/// publish reloads (via `loader`) into `config_store` until `shutdown`.
pub fn spawn_config_watcher<T: Send + Sync + 'static>(
    config_path_cell: &'static OnceLock<String>,
    config_store: &'static ArcSwap<T>,
    shutdown: Arc<AtomicBool>,
    loader: impl Fn(&str) -> T + Send + Sync + 'static,
) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        let config_path = match config_path_cell.get() {
            Some(p) => p.clone(),
            None => return,
        };

        // Only watch if file exists
        if tokio::fs::metadata(&config_path).await.is_err() {
            log::warn!("Config file not found, file watching disabled: {config_path}");
            return;
        }

        let (tx, mut rx) = tokio::sync::mpsc::channel::<()>(8);
        let config_path_watch = config_path.clone();
        std::thread::spawn(move || {
            use notify::{RecursiveMode, Watcher};
            let callback_tx = tx;
            let mut watcher = match notify::recommended_watcher(
                move |result: notify::Result<notify::Event>| {
                    if let Ok(event) = result {
                        if matches!(event.kind, notify::EventKind::Modify(_)) {
                            let _ = callback_tx.blocking_send(());
                        }
                    }
                },
            ) {
                Ok(watcher) => watcher,
                Err(e) => {
                    log::warn!("Failed to create config watcher: {e}");
                    return;
                }
            };
            if let Err(e) = watcher.watch(
                std::path::Path::new(&config_path_watch),
                RecursiveMode::NonRecursive,
            ) {
                log::warn!("Failed to watch config file: {e}");
                return;
            }
            loop {
                std::thread::park();
            }
        });

        log::info!("Config file watcher started for: {config_path}");
        let mut last_reload = std::time::Instant::now() - RELOAD_DEBOUNCE;
        while !shutdown.load(Ordering::SeqCst) {
            let Some(()) = rx.recv().await else { break };
            if last_reload.elapsed() < RELOAD_DEBOUNCE {
                continue;
            }
            log::info!("Reloading config file: {config_path}");
            let new_config = loader(&config_path);
            config_store.store(Arc::new(new_config));
            last_reload = std::time::Instant::now();
            log::info!("Config reloaded successfully");
        }

        log::info!("Config file watcher stopped");
    })
}
