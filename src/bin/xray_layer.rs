use tracing_benchmark_experiments::spans;
use tracing_benchmark_experiments::tracing_setup;

// The purpose of this file is to be able to generate flamegraphs
// with the cargo flamegraph tool

fn main() {
    let _guard = tracing_setup::set_up_xray_layer();
    let span = spans::span_08_fields();
    let _ = span.enter();
}