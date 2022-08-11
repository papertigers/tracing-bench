/// Implements `tracing_subscriber::Layer<tracing_subscriber::Registry>` for a struct that wraps a
/// tracing reload Layer as an inner type.
macro_rules! impl_tracing_layer {
    ($l:ty) => {
        impl tracing_subscriber::Layer<tracing_subscriber::Registry> for $l {
            fn on_layer(&mut self, subscriber: &mut Registry) {
                self.inner.on_layer(subscriber)
            }

            fn register_callsite(
                &self,
                metadata: &'static tracing::Metadata<'static>,
            ) -> tracing::subscriber::Interest {
                self.inner.register_callsite(metadata)
            }

            fn enabled(
                &self,
                metadata: &tracing::Metadata<'_>,
                ctx: tracing_subscriber::layer::Context<'_, tracing_subscriber::Registry>,
            ) -> bool {
                self.inner.enabled(metadata, ctx)
            }

            fn on_new_span(
                &self,
                attrs: &tracing::span::Attributes<'_>,
                id: &tracing::span::Id,
                ctx: tracing_subscriber::layer::Context<'_, tracing_subscriber::Registry>,
            ) {
                self.inner.on_new_span(attrs, id, ctx)
            }

            fn max_level_hint(&self) -> Option<tracing::metadata::LevelFilter> {
                self.inner.max_level_hint()
            }

            fn on_record(
                &self,
                span: &tracing::span::Id,
                values: &tracing::span::Record<'_>,
                ctx: tracing_subscriber::layer::Context<'_, tracing_subscriber::Registry>,
            ) {
                self.inner.on_record(span, values, ctx)
            }

            fn on_follows_from(
                &self,
                span: &tracing::span::Id,
                follows: &tracing::span::Id,
                ctx: tracing_subscriber::layer::Context<'_, tracing_subscriber::Registry>,
            ) {
                self.inner.on_follows_from(span, follows, ctx)
            }

            fn on_event(
                &self,
                event: &tracing::Event<'_>,
                ctx: tracing_subscriber::layer::Context<'_, tracing_subscriber::Registry>,
            ) {
                self.inner.on_event(event, ctx)
            }

            fn on_enter(
                &self,
                id: &tracing::span::Id,
                ctx: tracing_subscriber::layer::Context<'_, tracing_subscriber::Registry>,
            ) {
                self.inner.on_enter(id, ctx)
            }

            fn on_exit(
                &self,
                id: &tracing::span::Id,
                ctx: tracing_subscriber::layer::Context<'_, tracing_subscriber::Registry>,
            ) {
                self.inner.on_exit(id, ctx)
            }

            fn on_close(
                &self,
                id: tracing::span::Id,
                ctx: tracing_subscriber::layer::Context<'_, tracing_subscriber::Registry>,
            ) {
                self.inner.on_close(id, ctx)
            }

            fn on_id_change(
                &self,
                old: &tracing::span::Id,
                new: &tracing::span::Id,
                ctx: tracing_subscriber::layer::Context<'_, tracing_subscriber::Registry>,
            ) {
                self.inner.on_id_change(old, new, ctx)
            }
        }
    };
}

pub(crate) use impl_tracing_layer;
