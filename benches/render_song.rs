use criterion::{criterion_group, criterion_main, Criterion};
use imagemusic::Song;

pub fn criterion_benchmark(c: &mut Criterion) {
    let song: Song = toml::from_str(include_str!("../minuet.toml")).unwrap();
    c.bench_function("render minuet", move |b| {
        b.iter(|| {
            let samples: Vec<_> = song.samples(48000).collect();
            assert!(samples.len() > 48000);
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
