use std::io;

use tracing_subscriber::fmt;
use tracing_subscriber::registry::LookupSpan;

use crate::Neovim;

/// A [`tracing_subscriber::Layer`] implementation that displays logs in the
/// Neovim message area.
pub struct TracingLayer<S> {
    inner: fmt::Layer<
        S,
        fmt::format::DefaultFields,
        fmt::format::Format<fmt::format::Full, ()>,
        MessageAreaWriter,
    >,
}

#[derive(Copy, Clone)]
struct MessageAreaWriter;

impl<S> TracingLayer<S> {
    pub(crate) fn new(nvim: &mut Neovim) -> Self {
        let inner = fmt::Layer::default()
            .with_ansi(false)
            .without_time()
            .with_writer(MessageAreaWriter::new(nvim));

        Self { inner }
    }
}

impl MessageAreaWriter {
    fn new(_nvim: &mut Neovim) -> Self {
        todo!();
    }
}

impl fmt::MakeWriter<'_> for MessageAreaWriter {
    type Writer = Self;

    fn make_writer(&self) -> Self::Writer {
        todo!();
    }
}

impl io::Write for MessageAreaWriter {
    fn write(&mut self, _buf: &[u8]) -> io::Result<usize> {
        todo!();
    }

    fn flush(&mut self) -> io::Result<()> {
        todo!();
    }
}

impl<S: tracing::Subscriber> tracing_subscriber::Layer<S> for TracingLayer<S>
where
    S: tracing::Subscriber + for<'a> LookupSpan<'a>,
{
    fn on_register_dispatch(&self, subscriber: &tracing::Dispatch) {
        self.inner.on_register_dispatch(subscriber);
    }

    fn on_layer(&mut self, subscriber: &mut S) {
        self.inner.on_layer(subscriber);
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
        ctx: tracing_subscriber::layer::Context<'_, S>,
    ) -> bool {
        self.inner.enabled(metadata, ctx)
    }

    fn on_new_span(
        &self,
        attrs: &tracing::span::Attributes<'_>,
        id: &tracing::span::Id,
        ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
        self.inner.on_new_span(attrs, id, ctx);
    }

    fn on_record(
        &self,
        span: &tracing::span::Id,
        values: &tracing::span::Record<'_>,
        ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
        self.inner.on_record(span, values, ctx);
    }

    fn on_follows_from(
        &self,
        span: &tracing::span::Id,
        follows: &tracing::span::Id,
        ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
        self.inner.on_follows_from(span, follows, ctx);
    }

    fn event_enabled(
        &self,
        event: &tracing::Event<'_>,
        ctx: tracing_subscriber::layer::Context<'_, S>,
    ) -> bool {
        self.inner.event_enabled(event, ctx)
    }

    fn on_event(
        &self,
        event: &tracing::Event<'_>,
        ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
        self.inner.on_event(event, ctx);
    }

    fn on_enter(
        &self,
        id: &tracing::span::Id,
        ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
        self.inner.on_enter(id, ctx);
    }

    fn on_exit(
        &self,
        id: &tracing::span::Id,
        ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
        self.inner.on_exit(id, ctx);
    }

    fn on_close(
        &self,
        id: tracing::span::Id,
        ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
        self.inner.on_close(id, ctx);
    }

    fn on_id_change(
        &self,
        old: &tracing::span::Id,
        new: &tracing::span::Id,
        ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
        self.inner.on_id_change(old, new, ctx);
    }

    fn boxed(
        self,
    ) -> Box<dyn tracing_subscriber::Layer<S> + Send + Sync + 'static>
    where
        Self: Sized,
        Self: tracing_subscriber::Layer<S> + Send + Sync + 'static,
        S: tracing::Subscriber,
    {
        self.inner.boxed()
    }
}
