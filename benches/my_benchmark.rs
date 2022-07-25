use bench_pyo3::set_python_path;
use bench_pyo3::Caller;
use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;
use pyo3::Python;

pub fn criterion_benchmark(c: &mut Criterion) {
    set_python_path(".").unwrap();
    let caller = Caller::init().unwrap();

    c.bench_function("0.001", |b| b.iter(|| caller.do_something(0.001)));

    Python::with_gil(|py| {
        c.bench_function("with_gil 0.001", |b| {
            b.iter(|| caller.do_something_with_gil(py, 0.001))
        });
    })
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
