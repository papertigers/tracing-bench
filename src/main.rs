use std::{
    ops::Range,
    sync::{Arc, Barrier},
};
use tracing::{info, instrument, trace};

#[cfg(feature = "registry")]
mod dynamic;
#[cfg(feature = "registry")]
use dynamic::DynamicReloadHandle;

#[cfg(feature = "registry")]
mod util;

#[cfg(feature = "registry")]
lazy_static::lazy_static! {
    static ref RELOAD_HANDLE: DynamicReloadHandle = new_registry();
}

fn log_stuff() {
    trace!("hello");
}

#[instrument]
fn do_work(n: usize) -> usize {
    let _ = std::iter::repeat_with(log_stuff).take(n).count();
    let acc = Range { start: 0, end: n }.fold(0usize, |acc, x| acc.saturating_add(x));
    info!("added up to {acc}!");
    std::iter::repeat_with(log_stuff).take(n).count()
}

#[cfg(not(feature = "registry"))]
fn setup_default_subscriber() {
    use tracing_subscriber::{EnvFilter, FmtSubscriber};

    FmtSubscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        .with_ansi(false)
        .init();
}

#[cfg(feature = "registry")]
fn new_registry() -> DynamicReloadHandle {
    use tracing_subscriber::{fmt::Layer, prelude::*, reload, EnvFilter, Registry};

    use crate::dynamic::DynamicLayer;
    let default_layer = Layer::new().with_ansi(false);
    let (default_layer, _default_handle) =
        reload::Layer::new(default_layer.with_filter(EnvFilter::from_default_env()));
    let (dynamic_layer, dynamic_handle) = DynamicLayer::new();

    Registry::default()
        .with(default_layer.and_then(dynamic_layer))
        .init();

    dynamic_handle
}

#[cfg(feature = "registry")]
fn setup_registry_subscriber() {
    lazy_static::initialize(&RELOAD_HANDLE);
}

fn setup_subscriber() {
    #[cfg(not(feature = "registry"))]
    setup_default_subscriber();

    #[cfg(feature = "registry")]
    setup_registry_subscriber();
}

fn main() -> Result<(), anyhow::Error> {
    setup_subscriber();

    let args: Vec<String> = std::env::args().collect();
    let mut opts = getopts::Options::new();
    opts.reqopt("t", "threads", "threads", "number of threads");
    opts.reqopt("c", "count", "count", "number of iterations");
    opts.optflag("r", "reload", "reload dynamic layer");

    let matches = opts.parse(&args[1..])?;
    let threads = matches.opt_get::<usize>("t")?.unwrap();
    let ntries = matches.opt_get::<usize>("c")?.unwrap();

    #[cfg(feature = "registry")]
    if matches.opt_defined("r") {
        let filter = "trace".parse()?;
        // This is simply to make sure the compiler doesn't optimize for the layer always containing a None
        RELOAD_HANDLE.swap(filter)?;
    }

    let mut handles = Vec::with_capacity(threads);
    let barrier = Arc::new(Barrier::new(threads));

    for i in 0..threads {
        let b = Arc::clone(&barrier);
        handles.push(
            std::thread::Builder::new()
                .name(format!("thread-{i}"))
                .spawn(move || {
                    b.wait();
                    let result = do_work(ntries);
                    info!(?result, "finished");
                })?,
        );
    }

    for handle in handles {
        let _ = handle.join().unwrap();
    }

    Ok(())
}
