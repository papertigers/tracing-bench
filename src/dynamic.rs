use std::sync::Arc;

use anyhow::Context;
use tracing::warn;
use tracing_appender::non_blocking::NonBlocking;
use tracing_subscriber::{
    filter::Filtered,
    fmt::{
        self,
        format::{self, DefaultFields},
    },
    reload, EnvFilter, Layer, Registry,
};

type FilteredLayer = Filtered<reload::Layer<FmtLayer, Registry>, EnvFilter, Registry>;
type FmtLayer = Option<fmt::Layer<Registry, DefaultFields, format::Format, NonBlocking>>;

pub(crate) struct DynamicLayer {
    inner: reload::Layer<FilteredLayer, Registry>,
}
super::util::impl_tracing_layer!(DynamicLayer);

impl DynamicLayer {
    pub(crate) fn new() -> (Self, DynamicReloadHandle) {
        let (fmt_layer, fmt_handle) = reload::Layer::new(None);
        let (inner, filter_handle) =
            reload::Layer::new(fmt_layer.with_filter(EnvFilter::default()));

        (
            Self { inner },
            DynamicReloadHandle {
                filter_handle,
                fmt_handle: Arc::new(fmt_handle),
            },
        )
    }
}

#[allow(dead_code)]
pub(crate) struct DynamicReloadHandle {
    filter_handle: reload::Handle<FilteredLayer, Registry>,
    fmt_handle: Arc<reload::Handle<FmtLayer, Registry>>,
}

impl DynamicReloadHandle {
    /// Make sure the compiler doesn't optimize away the possibility of the inner handles ever changing
    pub(crate) fn swap(&self, filter: EnvFilter) -> Result<(), anyhow::Error> {
        self.filter_handle
            .modify(|filtered| *filtered.filter_mut() = filter)
            .context("failed to swap filters")?;

        let (non_blocking, _guard) = tracing_appender::non_blocking(std::io::stdout());
        self.fmt_handle
            .reload(Some(fmt::Layer::default().with_writer(non_blocking)))
            .context("failed to swap fmt handles")?;

        // Reset before we drop the tracing_appender guard
        if let Err(e) = self.fmt_handle.reload(None) {
            warn!("failed to reset DyanmicLayer's fmt handle {e:?}");
        }

        Ok(())
    }
}
