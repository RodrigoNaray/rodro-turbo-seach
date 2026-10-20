//! On-demand filename search. One sleeping worker, a latest-request mailbox, no index.
use std::{
    path::{Path, PathBuf},
    sync::{
        Arc, Condvar, Mutex,
        atomic::{AtomicU64, Ordering},
    },
    time::{Duration, Instant},
};

#[derive(Debug, Default)]
pub struct Matches {
    pub paths: Vec<PathBuf>,
    pub limited: bool,
}
struct Request {
    generation: u64,
    root: PathBuf,
    query: String,
}
#[derive(Default)]
struct Mailbox {
    request: Option<Request>,
    result: Option<(u64, Matches)>,
    stopped: bool,
}
#[derive(Default)]
struct Shared {
    mailbox: Mutex<Mailbox>,
    wake: Condvar,
    generation: AtomicU64,
}
pub struct FileSearch(Arc<Shared>);
impl FileSearch {
    pub fn new() -> Self {
        let shared = Arc::new(Shared::default());
        let worker = shared.clone();
        std::thread::spawn(move || {
            loop {
                let request = {
                    let mut mailbox = worker.mailbox.lock().unwrap();
                    while mailbox.request.is_none() && !mailbox.stopped {
                        mailbox = worker.wake.wait(mailbox).unwrap();
                    }
                    if mailbox.stopped {
                        return;
                    }
                    mailbox.request.take().unwrap()
                };
                let matches = scan(&request.root, &request.query, || {
                    worker.generation.load(Ordering::Relaxed) != request.generation
                });
                let mut mailbox = worker.mailbox.lock().unwrap();
                if worker.generation.load(Ordering::Relaxed) == request.generation {
                    mailbox.result = Some((request.generation, matches));
                }
            }
        });
        Self(shared)
    }
    pub fn cancel(&self) {
        self.0.generation.fetch_add(1, Ordering::Relaxed);
        let mut mailbox = self.0.mailbox.lock().unwrap();
        mailbox.request = None;
        mailbox.result = None;
    }
    pub fn request(&self, root: PathBuf, query: String) -> u64 {
        self.cancel();
        let generation = self.0.generation.load(Ordering::Relaxed);
        self.0.mailbox.lock().unwrap().request = Some(Request {
            generation,
            root,
            query,
        });
        self.0.wake.notify_one();
        generation
    }
    pub fn take(&self, generation: u64) -> Option<Matches> {
        self.0
            .mailbox
            .lock()
            .unwrap()
            .result
            .take()
            .and_then(|(id, result)| (id == generation).then_some(result))
    }
}
impl Drop for FileSearch {
    fn drop(&mut self) {
        self.cancel();
        self.0.mailbox.lock().unwrap().stopped = true;
        self.0.wake.notify_one();
    }
}

fn scan(root: &Path, query: &str, cancelled: impl Fn() -> bool) -> Matches {
    let query = query.trim().to_lowercase();
    if query.chars().count() < 2 {
        return Matches::default();
    }
    let start = Instant::now();
    let mut stack = vec![(root.to_owned(), 0)];
    let mut best = Vec::new();
    let mut visited = 0;
    let mut limited = false;
    'walk: while let Some((directory, depth)) = stack.pop() {
        if cancelled() {
            return Matches::default();
        }
        let Ok(entries) = std::fs::read_dir(directory) else {
            continue;
        };
        for entry in entries {
            if cancelled() {
                return Matches::default();
            }
            if visited >= 30_000 || start.elapsed() >= Duration::from_millis(700) {
                limited = true;
                break 'walk;
            }
            visited += 1;
            let Ok(entry) = entry else {
                continue;
            };
            let name = entry.file_name();
            let name = name.to_string_lossy();
            if name.starts_with('.') {
                continue;
            }
            let Ok(kind) = entry.file_type() else {
                continue;
            };
            // Never traverse symlinks (cycles, mounts or paths outside the selected root).
            if kind.is_dir() {
                if matches!(name.as_ref(), "node_modules" | "target" | "vendor") {
                    continue;
                }
                if depth < 16 && stack.len() < 2048 {
                    stack.push((entry.path(), depth + 1));
                } else {
                    limited = true;
                }
            } else if !kind.is_file() {
                continue;
            }
            let normalized = name.to_lowercase();
            if normalized.contains(&query) {
                let rank = if normalized == query {
                    0
                } else if normalized.starts_with(&query) {
                    1
                } else {
                    2
                };
                best.push((rank, normalized, entry.path()));
                best.sort_unstable();
                best.truncate(5);
            }
        }
    }
    Matches {
        paths: best.into_iter().map(|(_, _, path)| path).collect(),
        limited,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn worker_delivers_latest_request_and_cancels_pending_work() {
        let worker = FileSearch::new();
        let root = std::env::temp_dir().join(format!("turbo-worker-test-{}", std::process::id()));
        std::fs::create_dir_all(&root).unwrap();
        std::fs::write(root.join("latest-document"), "").unwrap();
        worker.request(root.clone(), "obsolete".into());
        let generation = worker.request(root.clone(), "latest".into());
        let deadline = Instant::now() + Duration::from_secs(3);
        let result = loop {
            if let Some(result) = worker.take(generation) {
                break result;
            }
            assert!(Instant::now() < deadline, "worker did not finish");
            std::thread::sleep(Duration::from_millis(5));
        };
        assert_eq!(result.paths, [root.join("latest-document")]);
        let generation = worker.request(root.clone(), "latest".into());
        worker.cancel();
        assert!(worker.take(generation).is_none());
        std::fs::remove_dir_all(root).unwrap();
    }
    #[test]
    fn finds_names_ranks_and_skips_hidden_builds_and_symlinks() {
        let root = std::env::temp_dir().join(format!("turbo-files-test-{}", std::process::id()));
        std::fs::create_dir_all(root.join("documents")).unwrap();
        for directory in [".private", "node_modules", "target"] {
            std::fs::create_dir_all(root.join(directory)).unwrap();
            std::fs::write(root.join(directory).join("report-secret"), "").unwrap();
        }
        for name in ["report", "REPORT-final", "my-report", "unrelated"] {
            std::fs::write(root.join("documents").join(name), "report in content only").unwrap();
        }
        std::os::unix::fs::symlink(&root, root.join("loop")).unwrap();
        let result = scan(&root, " REPORT ", || false);
        assert!(!result.limited);
        assert_eq!(
            result
                .paths
                .iter()
                .map(|p| p.file_name().unwrap().to_str().unwrap())
                .collect::<Vec<_>>(),
            ["report", "REPORT-final", "my-report"]
        );
        assert!(scan(&root, "r", || false).paths.is_empty());
        assert!(scan(&root, "report", || true).paths.is_empty());
        std::fs::remove_dir_all(root).unwrap();
    }
}
