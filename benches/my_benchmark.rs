use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use tracing::{dispatcher::DefaultGuard, info_span};
use tracing_benchmark_experiments::drop_span_events::{DropLayer, DropSubscriber};
use tracing_benchmark_experiments::spans as tbe;
use tracing_benchmark_experiments::tracing_setup;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::Registry;
mod util;
use util::gen_rand_strings;

pub fn create_span_single_field(c: &mut Criterion) {
    let mut group = c.benchmark_group("create_span_single_field");

    for rand_string in gen_rand_strings().iter() {
        group.bench_with_input(
            BenchmarkId::new("subscriber", rand_string.length),
            &rand_string.s,
            |b, s| {
                let _guard = tracing_setup::set_up_subscriber();
                b.iter_with_large_drop(|| info_span!("foo", my_field = ?s));
            },
        );
        group.bench_with_input(
            BenchmarkId::new("layer", rand_string.length),
            &rand_string.s,
            |b, s| {
                let _guard = tracing_setup::set_up_layer();
                b.iter_with_large_drop(|| info_span!("foo", my_field = ?s));
            },
        );
    }
    group.finish();
}

pub fn enter_span_single_field(c: &mut Criterion) {
    let mut group = c.benchmark_group("enter_span_single_field");

    for rand_string in gen_rand_strings().iter() {
        group.bench_with_input(
            BenchmarkId::new("subscriber", rand_string.length),
            &rand_string.s,
            |b, s| {
                let _guard = tracing_setup::set_up_subscriber();
                let span = info_span!("foo", my_field = ?s);
                b.iter_with_large_drop(|| span.enter());
            },
        );
        group.bench_with_input(
            BenchmarkId::new("layer", rand_string.length),
            &rand_string.s,
            |b, s| {
                let _guard = tracing_setup::set_up_layer();
                let span = info_span!("foo", my_field = ?s);
                b.iter_with_large_drop(|| span.enter());
            },
        );
    }
    group.finish();
}

pub fn enter_exit_span_single_field(c: &mut Criterion) {
    let mut group = c.benchmark_group("enter_exit_span_single_field");

    for rand_string in gen_rand_strings().iter() {
        group.bench_with_input(
            BenchmarkId::new("subscriber", rand_string.length),
            &rand_string.s,
            |b, s| {
                let _guard = tracing_setup::set_up_subscriber();
                let span = info_span!("foo", my_field = ?s);
                b.iter(|| span.enter());
            },
        );
        group.bench_with_input(
            BenchmarkId::new("layer", rand_string.length),
            &rand_string.s,
            |b, s| {
                let _guard = tracing_setup::set_up_layer();
                let span = info_span!("foo", my_field = ?s);
                b.iter(|| span.enter());
            },
        );
    }
    group.finish();
}

// multi field

macro_rules! create_span_x_fields {
    ( $( $func: ident ),*; $group:ident ) => {
        $(
            $group.bench_function(format!("subscriber_{}", stringify!($func)), |b| {
                let _guard = tracing_setup::set_up_subscriber();
                b.iter_with_large_drop(|| tbe::$func());
            });

            $group.bench_function(format!("layer_{}", stringify!($func)), |b| {
                let _guard = tracing_setup::set_up_layer();
                b.iter_with_large_drop(|| tbe::$func());
            });
        )*
    }
}

pub fn create_span_multi_field(c: &mut Criterion) {
    let mut group = c.benchmark_group("create_span_multi_field");

    create_span_x_fields!(
        span_00_fields,
        span_04_fields,
        span_08_fields,
        span_12_fields,
        span_16_fields,
        span_20_fields,
        span_24_fields,
        span_28_fields,
        span_32_fields;
        group
    );
}

pub fn enter_span_multi_field(c: &mut Criterion) {
    let mut group = c.benchmark_group("enter_span_multi_field");

    group.bench_function("subscriber_4_fields", |b| {
        let _guard = tracing_setup::set_up_subscriber();
        let span = tbe::span_04_fields();
        b.iter_with_large_drop(|| span.enter());
    });

    group.bench_function("subscriber_8_fields", |b| {
        let _guard = tracing_setup::set_up_subscriber();
        let span = tbe::span_08_fields();
        b.iter_with_large_drop(|| span.enter());
    });

    group.bench_function("subscriber_16_fields", |b| {
        let _guard = tracing_setup::set_up_subscriber();
        let span = tbe::span_16_fields();
        b.iter_with_large_drop(|| span.enter());
    });

    group.bench_function("subscriber_32_fields", |b| {
        let _guard = tracing_setup::set_up_subscriber();
        let span = tbe::span_32_fields();
        b.iter_with_large_drop(|| span.enter());
    });

    group.bench_function("layer_4_fields", |b| {
        let _guard = tracing_setup::set_up_layer();
        let span = tbe::span_04_fields();
        b.iter_with_large_drop(|| span.enter());
    });

    group.bench_function("layer_8_fields", |b| {
        let _guard = tracing_setup::set_up_layer();
        let span = tbe::span_08_fields();
        b.iter_with_large_drop(|| span.enter());
    });

    group.bench_function("layer_16_fields", |b| {
        let _guard = tracing_setup::set_up_layer();
        let span = tbe::span_16_fields();
        b.iter_with_large_drop(|| span.enter());
    });

    group.bench_function("layer_32_fields", |b| {
        let _guard = tracing_setup::set_up_layer();
        let span = tbe::span_32_fields();
        b.iter_with_large_drop(|| span.enter());
    });
}

pub fn enter_exit_span_multi_field(c: &mut Criterion) {
    let mut group = c.benchmark_group("enter_exit_span_multi_field");

    group.bench_function("subscriber_4_fields", |b| {
        let _guard = tracing_setup::set_up_subscriber();
        let span = tbe::span_04_fields();
        b.iter(|| span.enter());
    });

    group.bench_function("subscriber_8_fields", |b| {
        let _guard = tracing_setup::set_up_subscriber();
        let span = tbe::span_08_fields();
        b.iter(|| span.enter());
    });

    group.bench_function("subscriber_16_fields", |b| {
        let _guard = tracing_setup::set_up_subscriber();
        let span = tbe::span_16_fields();
        b.iter(|| span.enter());
    });

    group.bench_function("subscriber_32_fields", |b| {
        let _guard = tracing_setup::set_up_subscriber();
        let span = tbe::span_32_fields();
        b.iter(|| span.enter());
    });

    group.bench_function("layer_4_fields", |b| {
        let _guard = tracing_setup::set_up_layer();
        let span = tbe::span_04_fields();
        b.iter(|| span.enter());
    });

    group.bench_function("layer_8_fields", |b| {
        let _guard = tracing_setup::set_up_layer();
        let span = tbe::span_08_fields();
        b.iter(|| span.enter());
    });

    group.bench_function("layer_16_fields", |b| {
        let _guard = tracing_setup::set_up_layer();
        let span = tbe::span_16_fields();
        b.iter(|| span.enter());
    });

    group.bench_function("layer_32_fields", |b| {
        let _guard = tracing_setup::set_up_layer();
        let span = tbe::span_32_fields();
        b.iter(|| span.enter());
    });
}

criterion_group!(
    benches,
    create_span_single_field,
    enter_span_single_field,
    enter_exit_span_single_field,
    create_span_multi_field,
    enter_span_multi_field,
    enter_exit_span_multi_field
);
criterion_main!(benches);
