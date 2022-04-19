use tracing_benchmark_experiments::spans;
use tracing_benchmark_experiments::tracing_setup;

// The purpose of this file is to be able to generate flamegraphs
// with the cargo flamegraph tool

fn main() {
    tracing_setup::set_up_drop_subscriber();
    spans::span_08_fields();
}

// #[tokio::main]
// async fn main() {
//     tracing_setup::set_up_subscriber();
//     spans::span_32_fields();
//     println!("Hello world!");
// }
