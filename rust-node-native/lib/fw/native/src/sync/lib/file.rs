#![allow(non_snake_case)]
use super::WatcherTrait;
use notify::{DebouncedEvent, RecommendedWatcher, RecursiveMode, Watcher};
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

/// 文件同步器
pub struct FileWatcher {
    /// 文件路径
    path: PathBuf,
    /// 文件内容
    pub content: Arc<Mutex<String>>,
    /// 内容版本，初始化为1，文件一次修改累计1
    pub version: Arc<Mutex<usize>>,
}

impl FileWatcher {
    /// 创建新的同步器
    pub fn New(path: &Path) -> Self {
        if path.exists() {
            let file_content = fs::read_to_string(path).unwrap();
            let content = Arc::new(Mutex::new(file_content));
            let version = Arc::new(Mutex::new(1));
            Self {
                path: PathBuf::from(path),
                content,
                version,
            }
        } else {
            error!("watch an nonexist file!");
            panic!("watch an nonexist file!");
        }
    }
}

impl WatcherTrait for FileWatcher {
    /// 新线程启动观察器
    fn watch(&mut self) {
        let path = fs::canonicalize(&self.path).unwrap();
        let content_guard = self.content.clone();
        let version_guard = self.version.clone();
        thread::spawn(move || {
            let (tx1, rx1) = mpsc::channel();
            let mut watcher: RecommendedWatcher =
                Watcher::new(tx1, Duration::from_secs(1)).unwrap();
            watcher
                .watch(path.parent().unwrap(), RecursiveMode::NonRecursive)
                .unwrap();

            loop {
                match rx1.recv() {
                    // 文件写入，创建
                    Ok(event) => {
                        info!("watch event: {:?}", event);
                        match event {
                            DebouncedEvent::Remove(p)
                            | DebouncedEvent::Write(p)
                            | DebouncedEvent::Create(p) => {
                                if path == p {
                                    let mut content = content_guard.lock().unwrap();
                                    let mut version = version_guard.lock().unwrap();
                                    (*version) += 1;
                                    match fs::read_to_string(&p) {
                                        Ok(c) => {
                                            *content = c;
                                            info!("wrote file: {:?} #{}", p, *version);
                                        }
                                        Err(_) => warn!("file not exist: {:?}", &p),
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                    Err(e) => {
                        error!("watch error: {:?}", e);
                    }
                }
            }
        });
    }

    /// 文件内容
    fn content(&self) -> String {
        let content_guard = self.content.clone();
        let content = content_guard.lock().unwrap();
        (*content).clone()
    }

    /// 文件更新次数
    fn version(&self) -> usize {
        let version_guard = self.version.clone();
        let version = version_guard.lock().unwrap();
        *version
    }
}
