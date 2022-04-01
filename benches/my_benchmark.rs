use criterion::{criterion_group, criterion_main, Criterion};
use tracing::{dispatcher::DefaultGuard, info_span};
use tracing_benchmark_experiments::{DropLayer, DropSubscriber};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::Registry;

fn set_up_subscriber() -> DefaultGuard {
    let subscriber = DropSubscriber::new();
    tracing::subscriber::set_default(subscriber)
}

fn set_up_layer() -> DefaultGuard {
    let subscriber_with_layer = Registry::default().with(DropLayer {});
    tracing::subscriber::set_default(subscriber_with_layer)
}

pub fn create_span(c: &mut Criterion) {
    let mut group = c.benchmark_group("create_span");

    group.bench_function("subscriber", |b| {
        let _guard = set_up_subscriber();
        b.iter_with_large_drop(|| info_span!("foo"));
    });

    group.bench_function("layer", |b| {
        let _guard = set_up_layer();
        b.iter_with_large_drop(|| info_span!("foo"));
    });
}

pub fn enter_span(c: &mut Criterion) {
    let mut group = c.benchmark_group("enter_span");

    group.bench_function("subscriber", |b| {
        let _guard = set_up_subscriber();
        let span = info_span!("foo");
        b.iter_with_large_drop(|| span.enter());
    });

    group.bench_function("layer", |b| {
        let _guard = set_up_layer();
        let span = info_span!("foo");
        b.iter_with_large_drop(|| span.enter());
    });
}

pub fn enter_exit_span(c: &mut Criterion) {
    let mut group = c.benchmark_group("enter_exit_span");

    group.bench_function("subscriber", |b| {
        let _guard = set_up_subscriber();
        let span = info_span!("foo");
        b.iter(|| span.enter());
    });

    group.bench_function("layer", |b| {
        let _guard = set_up_layer();
        let span = info_span!("foo");
        b.iter(|| span.enter());
    });
}

criterion_group!(benches, create_span, enter_span, enter_exit_span);
criterion_main!(benches);
