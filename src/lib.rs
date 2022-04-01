use tracing::Subscriber;
use tracing::{event, span, Level};
use tracing_subscriber::Layer;

pub struct DropSubscriber {
    span_id: span::Id,
}

impl DropSubscriber {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        DropSubscriber {
            span_id: span::Id::from_u64(1),
        }
    }
}

impl Subscriber for DropSubscriber {
    fn enabled(&self, _metadata: &tracing::Metadata<'_>) -> bool {
        true
    }

    fn new_span(&self, _span: &span::Attributes<'_>) -> span::Id {
        self.span_id.clone()
    }

    fn record(&self, _span: &span::Id, _values: &span::Record<'_>) {}

    fn record_follows_from(&self, _span: &span::Id, _follows: &span::Id) {}

    fn event(&self, _event: &tracing::Event<'_>) {}

    fn enter(&self, _span: &span::Id) {}

    fn exit(&self, _span: &span::Id) {}
}

pub struct DropLayer {}

impl<S> Layer<S> for DropLayer
where
    S: Subscriber,
{
    fn new_span(
        &self,
        _attrs: &span::Attributes<'_>,
        _id: &span::Id,
        _ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
    }

    fn on_record(
        &self,
        _span: &span::Id,
        _values: &span::Record<'_>,
        _ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
    }

    fn on_follows_from(
        &self,
        _span: &span::Id,
        _follows: &span::Id,
        _ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
    }

    fn on_event(
        &self,
        _event: &tracing::Event<'_>,
        _ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
    }

    fn on_enter(&self, _id: &span::Id, _ctx: tracing_subscriber::layer::Context<'_, S>) {}

    fn on_exit(&self, _id: &span::Id, _ctx: tracing_subscriber::layer::Context<'_, S>) {}

    fn on_close(&self, _id: span::Id, _ctx: tracing_subscriber::layer::Context<'_, S>) {}

    fn on_id_change(
        &self,
        _old: &span::Id,
        _new: &span::Id,
        _ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
    }
}
