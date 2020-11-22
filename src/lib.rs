use futures::future::{self, Ready};
use tarpc::context;

// This is the service definition. It looks a lot like a trait definition.
// It defines one RPC, hello, which takes one arg, name, and returns a String.
#[tarpc::service]
pub trait World {
    async fn hello(name: String) -> String;
}

// This is the type that implements the generated World trait. It is the business logic
// and is used to start the server.
#[derive(Clone)]
pub struct HelloServer;

#[tarpc::server]
impl World for HelloServer {
    // Each defined rpc generates two items in the trait, a fn that serves the RPC, and
    // an associated type representing the future output by the fn.

    async fn hello(self, _: context::Context, name: String) -> String {
       println!("Responding to hello");
       format!("Hello, {}!", name)
    }
}


use slog::*;
use slog_async::*;
use slog_scope::GlobalLoggerGuard;
use slog_term::*;
pub async fn create_stdout_logger(level: Level) -> (Logger, GlobalLoggerGuard) {
    let decorator = TermDecorator::new().build();
    let drain = FullFormat::new(decorator).build().fuse();
    let drain = Async::new(drain).chan_size(100_000).build().fuse();
    let drain = Filter::new(drain, move |record: &Record| {
        record.level().is_at_least(level)
    })
    .fuse();
    let log = Logger::root(drain, o!());
    // Set up global logger so that libraries can log
    slog_stdlog::init_with_level(slog_to_log_level(level)).unwrap();
    let _guard = slog_scope::set_global_logger(log.clone());
    (log, _guard)
}

fn slog_to_log_level(x: Level) -> log::Level {
    match x {
        Level::Trace => log::Level::Trace,
        Level::Debug => log::Level::Debug,
        Level::Info => log::Level::Info,
        Level::Warning => log::Level::Warn,
        Level::Error => log::Level::Error,
        Level::Critical => log::Level::Error,
    }
}
