mod file;

/// A Trait For Watcher Object
pub trait WatcherTrait {
    /// start watch
    fn watch(&mut self);
    /// watched content
    fn content(&self) -> String;
    /// watched content version
    fn version(&self) -> usize;
}

pub use self::file::FileWatcher;
