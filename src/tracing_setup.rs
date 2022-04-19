use crate::drop_span_events::{DropLayer, DropSubscriber};
use tracing::dispatcher::DefaultGuard;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::Registry;

pub fn set_up_drop_subscriber() -> DefaultGuard {
    let subscriber = DropSubscriber::new();
    tracing::subscriber::set_default(subscriber)
}

pub fn set_up_drop_layer() -> DefaultGuard {
    let subscriber_with_layer = Registry::default().with(DropLayer {});
    tracing::subscriber::set_default(subscriber_with_layer)
}

pub async fn set_up_xray_layer() -> DefaultGuard {
    let xray = tracing_xray::Layer::new("tracing-benchmark-experiments".to_owned())
        .await
        .expect("Unable to instantiate tracing-xray layer");
    let subscriber_with_layer = Registry::default().with(xray);
    tracing::subscriber::set_default(subscriber_with_layer)
}
