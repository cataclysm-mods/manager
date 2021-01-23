use tracing::subscriber::SetGlobalDefaultError;
use tracing_subscriber::{fmt, registry::Registry, EnvFilter};
use tracing_subscriber::prelude::*;

pub fn init() -> Result<(), SetGlobalDefaultError> {
    let fmt_layer = fmt::Layer::new().with_target(true);

    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("info"))
        .unwrap();

    let subscriber = Registry::default()
        .with(filter_layer)
        .with(fmt_layer);

    tracing::subscriber::set_global_default(subscriber)
}
