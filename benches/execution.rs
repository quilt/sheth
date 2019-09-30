use composer::blob;
use criterion::{criterion_group, criterion_main, Criterion};
use ewasm::{Execute, Runtime};

static SHETH_BINARY: &'static [u8] =
    include_bytes!("../target/wasm32-unknown-unknown/release/sheth.wasm");

fn large_proof(c: &mut Criterion) {
    let (blob, pre_state, _) = blob::generate_with_roots(2, 1, 256);
    let blob = blob.to_bytes();

    c.bench_function("execute(2, 1, 256)", |b| {
        b.iter(|| {
            let mut runtime = Runtime::new(SHETH_BINARY, &blob, pre_state);
            runtime.execute();
        })
    });
}

criterion_group!(benches, large_proof);
criterion_main!(benches);
