#![allow(clippy::missing_panics_doc)]
use criterion::{criterion_group, criterion_main, Criterion};

pub fn fast_benchmarks(c: &mut Criterion) {
    let rustdoc_json = rustdoc_json::Builder::default()
        .toolchain("nightly")
        .manifest_path("/home/martin/src/cargo-public-api/test-apis/comprehensive_api/Cargo.toml")
        .build()
        .unwrap();

    c.bench_function("process comprehensive_api JSON", |b| {
        b.iter(|| {
            let _ = public_api::Builder::from_rustdoc_json(&rustdoc_json)
                .sorted(false) // We don't care about sorting time
                .build()
                .unwrap();
        });
    });
}

pub fn slow_benchmarks(c: &mut Criterion) {
    // https://github.com/awslabs/aws-sdk-rust.git ;  cd aws-sdk-rust
    // cargo +nightly rustdoc -p aws-sdk-ec2 -- -Zunstable-options -wjson
    let rustdoc_json = "/home/martin/src/aws-sdk-rust/target/doc/aws_sdk_ec2.json";

    c.bench_function("process aws_sdk_ec2 JSON", |b| {
        b.iter(|| {
            let _ = public_api::Builder::from_rustdoc_json(rustdoc_json)
                .sorted(false) // We don't care about sorting time
                .build()
                .unwrap();
        });
    });
}

criterion_group!(
   name = benchmarks;
   config = Criterion::default().sample_size(10);
   targets = fast_benchmarks, slow_benchmarks
);
criterion_main!(benchmarks);
