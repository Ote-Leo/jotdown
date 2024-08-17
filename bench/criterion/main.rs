use criterion::criterion_group;
use criterion::criterion_main;

fn gen_block(c: &mut criterion::Criterion) {
    let mut group = c.benchmark_group("block");
    for (name, input) in bench_input::INPUTS {
        group.throughput(criterion::Throughput::Bytes(input.len() as u64));
        group.bench_with_input(
            criterion::BenchmarkId::from_parameter(name),
            input,
            |b, &input| {
                b.iter(|| jotdown::Parser::new(input));
            },
        );
    }
}
criterion_group!(block, gen_block);

fn gen_inline(c: &mut criterion::Criterion) {
    let mut group = c.benchmark_group("inline");
    for (name, input) in bench_input::INPUTS {
        group.throughput(criterion::Throughput::Bytes(input.len() as u64));
        group.bench_with_input(
            criterion::BenchmarkId::from_parameter(name),
            input,
            |b, &input| {
                b.iter_batched(
                    || jotdown::Parser::new(input),
                    |p| p.last().unwrap(),
                    criterion::BatchSize::SmallInput,
                );
            },
        );
    }
}
criterion_group!(inline, gen_inline);

fn gen_html(c: &mut criterion::Criterion) {
    let mut group = c.benchmark_group("html");
    for (name, input) in bench_input::INPUTS {
        group.throughput(criterion::Throughput::Elements(
            jotdown::Parser::new(input).count() as u64,
        ));
        group.bench_with_input(
            criterion::BenchmarkId::from_parameter(name),
            input,
            |b, &input| {
                b.iter_batched(
                    || jotdown::Parser::new(input).collect::<Vec<_>>(),
                    |p| jotdown::html::render_to_string(p.into_iter()),
                    criterion::BatchSize::SmallInput,
                );
            },
        );
    }
}
criterion_group!(html, gen_html);

fn gen_html_ref(c: &mut criterion::Criterion) {
    let mut group = c.benchmark_group("html_ref");
    for (name, input) in bench_input::INPUTS {
        group.throughput(criterion::Throughput::Elements(
            jotdown::Parser::new(input).count() as u64,
        ));
        group.bench_with_input(
            criterion::BenchmarkId::from_parameter(name),
            input,
            |b, &input| {
                b.iter_batched(
                    || jotdown::Parser::new(input).collect::<Vec<_>>(),
                    |p| {
                        use jotdown::RenderRef;
                        let mut s = String::new();
                        jotdown::html::Renderer::default()
                            .push_ref(p.as_slice().iter(), &mut s)
                            .unwrap();
                        s
                    },
                    criterion::BatchSize::SmallInput,
                );
            },
        );
    }
}
criterion_group!(html_ref, gen_html_ref);

fn gen_html_clone(c: &mut criterion::Criterion) {
    let mut group = c.benchmark_group("html_clone");
    for (name, input) in bench_input::INPUTS {
        group.throughput(criterion::Throughput::Elements(
            jotdown::Parser::new(input).count() as u64,
        ));
        group.bench_with_input(
            criterion::BenchmarkId::from_parameter(name),
            input,
            |b, &input| {
                b.iter_batched(
                    || jotdown::Parser::new(input).collect::<Vec<_>>(),
                    |p| jotdown::html::render_to_string(p.iter().cloned()),
                    criterion::BatchSize::SmallInput,
                );
            },
        );
    }
}
criterion_group!(html_clone, gen_html_clone);

fn gen_full(c: &mut criterion::Criterion) {
    let mut group = c.benchmark_group("full");
    for (name, input) in bench_input::INPUTS {
        group.throughput(criterion::Throughput::Bytes(input.len() as u64));
        group.bench_with_input(
            criterion::BenchmarkId::from_parameter(name),
            input,
            |b, &input| {
                b.iter_with_large_drop(|| {
                    jotdown::html::render_to_string(jotdown::Parser::new(input))
                });
            },
        );
    }
}
criterion_group!(full, gen_full);

criterion_main!(block, inline, html, html_ref, html_clone, full);
