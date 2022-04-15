# tracing-benchmark-experiments

Benchmarking the `Subscriber` trait vs the `Layer` trait.

## Single Field
For the single field case, we test increasing string lengths for the field. The length of the string is incremented by powers of 2.

![Create Span Single Field Violin graph](./img/create_span_single_field_violin.svg)
![Enter Span Single Violin graph](./img/enter_span_single_field_violin.svg)
![Enter Exit Span Single Field Violin graph](./img/enter_exit_span_single_field_violin.svg)

## Multifield
In the multifield case, the fields themselves are really small.

![Create Span Multifield Violin graph](./img/create_span_multi_field_violin.svg)
![Enter Span Multifield Violin graph](./img/enter_span_multi_field_violin.svg)
![Enter Exit Span Multifield Violin graph](./img/enter_exit_span_multi_field_violin.svg)
