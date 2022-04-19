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
            BenchmarkId::new("drop_subscriber", rand_string.length),
            &rand_string.s,
            |b, s| {
                let _guard = tracing_setup::set_up_drop_subscriber();
                b.iter_with_large_drop(|| info_span!("foo", aws.xray.trace_id = ?s));
            },
        );
        group.bench_with_input(
            BenchmarkId::new("drop_layer", rand_string.length),
            &rand_string.s,
            |b, s| {
                let _guard = tracing_setup::set_up_drop_layer();
                b.iter_with_large_drop(|| info_span!("foo", aws.xray.trace_id = ?s));
            },
        );

        group.bench_with_input(
            BenchmarkId::new("xray_layer", rand_string.length),
            &rand_string.s,
            |b, s| {
                let _guard = tracing_setup::set_up_xray_layer();
                b.iter_with_large_drop(|| info_span!("foo", aws.xray.trace_id = ?s));
            },
        );
    }
    group.finish();
}

pub fn enter_span_single_field(c: &mut Criterion) {
    let mut group = c.benchmark_group("enter_span_single_field");

    for rand_string in gen_rand_strings().iter() {
        group.bench_with_input(
            BenchmarkId::new("drop_subscriber", rand_string.length),
            &rand_string.s,
            |b, s| {
                let _guard = tracing_setup::set_up_drop_subscriber();
                let span = info_span!("foo", aws.xray.trace_id = ?s);
                b.iter_with_large_drop(|| span.enter());
            },
        );
        group.bench_with_input(
            BenchmarkId::new("drop_layer", rand_string.length),
            &rand_string.s,
            |b, s| {
                let _guard = tracing_setup::set_up_drop_layer();
                let span = info_span!("foo", aws.xray.trace_id = ?s);
                b.iter_with_large_drop(|| span.enter());
            },
        );

        group.bench_with_input(
            BenchmarkId::new("xray_layer", rand_string.length),
            &rand_string.s,
            |b, s| {
                let _guard = tracing_setup::set_up_xray_layer();
                let span = info_span!("foo", aws.xray.trace_id = ?s);
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
            BenchmarkId::new("drop_subscriber", rand_string.length),
            &rand_string.s,
            |b, s| {
                let _guard = tracing_setup::set_up_drop_subscriber();
                let span = info_span!("foo", aws.xray.trace_id = ?s);
                b.iter(|| span.enter());
            },
        );
        group.bench_with_input(
            BenchmarkId::new("drop_layer", rand_string.length),
            &rand_string.s,
            |b, s| {
                let _guard = tracing_setup::set_up_drop_layer();
                let span = info_span!("foo", aws.xray.trace_id = ?s);
                b.iter(|| span.enter());
            },
        );

        group.bench_with_input(
            BenchmarkId::new("xray_layer", rand_string.length),
            &rand_string.s,
            |b, s| {
                let _guard = tracing_setup::set_up_xray_layer();
                let span = info_span!("foo", aws.xray.trace_id = ?s);
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
            $group.bench_function(format!("drop_subscriber_{}", stringify!($func)), |b| {
                let _guard = tracing_setup::set_up_drop_subscriber();
                b.iter_with_large_drop(|| tbe::$func());
            });

            $group.bench_function(format!("drop_layer_{}", stringify!($func)), |b| {
                let _guard = tracing_setup::set_up_drop_layer();
                b.iter_with_large_drop(|| tbe::$func());
            });

            $group.bench_function(format!("xray_layer_{}", stringify!($func)), |b| {
                let _guard = tracing_setup::set_up_xray_layer();
                b.iter_with_large_drop(|| tbe::$func());
            });
        )*
    }
}

macro_rules! enter_span_x_fields {
    ( $( $func: ident ),*; $group:ident ) => {
        $(
            $group.bench_function(format!("drop_subscriber_{}", stringify!($func)), |b| {
                let _guard = tracing_setup::set_up_drop_subscriber();
                let span = tbe::$func();
                b.iter_with_large_drop(|| span.enter());
            });

            $group.bench_function(format!("drop_layer_{}", stringify!($func)), |b| {
                let _guard = tracing_setup::set_up_drop_layer();
                let span = tbe::$func();
                b.iter_with_large_drop(|| span.enter());
            });

            $group.bench_function(format!("xray_layer_{}", stringify!($func)), |b| {
                let _guard = tracing_setup::set_up_xray_layer();
                let span = tbe::$func();
                b.iter_with_large_drop(|| span.enter());
            });
        )*
    }
}

macro_rules! enter_exit_span_x_fields {
    ( $( $func: ident ),*; $group:ident ) => {
        $(
            $group.bench_function(format!("drop_subscriber_{}", stringify!($func)), |b| {
                let _guard = tracing_setup::set_up_drop_subscriber();
                let span = tbe::$func();
                b.iter(|| span.enter());
            });

            $group.bench_function(format!("drop_layer_{}", stringify!($func)), |b| {
                let _guard = tracing_setup::set_up_drop_layer();
                let span = tbe::$func();
                b.iter(|| span.enter());
            });

            $group.bench_function(format!("xray_layer_{}", stringify!($func)), |b| {
                let _guard = tracing_setup::set_up_xray_layer();
                let span = tbe::$func();
                b.iter(|| span.enter());
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

    enter_span_x_fields!(
        span_04_fields,
        span_08_fields,
        span_16_fields,
        span_32_fields;
        group
    );
}

pub fn enter_exit_span_multi_field(c: &mut Criterion) {
    let mut group = c.benchmark_group("enter_exit_span_multi_field");

    enter_exit_span_x_fields!(
        span_04_fields,
        span_08_fields,
        span_16_fields,
        span_32_fields;
        group
    );
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
