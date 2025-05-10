use criterion::{black_box, criterion_group, criterion_main, Criterion};

#[path = "../src/scrap.rs"]
mod ferron_lib;
use ferron_lib::{sanitize_url, sanitize_url_old};

fn bench_san(c: &mut Criterion) {
  let mut group = c.benchmark_group("Sanitizer");
  //for i in [20u64, 21u64].iter() {
  group.bench_function("Original", |b| {
    b.iter(|| sanitize_url_old(black_box("%2Fhome%2F../%2E%2E%2Fadmin//config"), false))
  });
  group.bench_function("New", |b| {
    b.iter(|| sanitize_url(black_box("%2Fhome%2F../%2E%2E%2Fadmin//config"), false))
  });
  //}
  group.finish();
}

criterion_group!(benches, bench_san);
criterion_main!(benches);
