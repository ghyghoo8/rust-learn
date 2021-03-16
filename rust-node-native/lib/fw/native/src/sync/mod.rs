use neon::prelude::*;

use std::path::Path;

mod lib;
use lib::*;


declare_types! {
    pub class JsFileWatcher for FileWatcher {
        init(mut ctx) {
            let file = ctx.argument::<JsString>(0)?.value();
            info!("crate a file watcher with file {}", file);
            let mut watcher = FileWatcher::New(Path::new(&file));
            watcher.watch();
            Ok(watcher)
        }

        method content(mut ctx) {
            let this = ctx.this();
            let content = {
                let guard = ctx.lock();
                let watcher = this.borrow(&guard);
                watcher.content()
            };
            Ok(ctx.string(&content).upcast())
        }

        method version(mut ctx) {
            let this = ctx.this();
            let version = {
                let guard = ctx.lock();
                let watcher = this.borrow(&guard);
                watcher.version()
            };
            Ok(ctx.number(version as f64).upcast())
        }
    }
}
