use criterion::{criterion_group, criterion_main, Criterion};
use imagemusic::Song;

pub fn criterion_benchmark(c: &mut Criterion) {
    let song: Song = toml::from_str(include_str!("minuet.toml")).unwrap();
    let song_twopoint_envelope: Song =
        toml::from_str(include_str!("minuet-twopoint-envelope.toml")).unwrap();
    let song_simple_envelope: Song =
        toml::from_str(include_str!("minuet-simple-envelope.toml")).unwrap();

    let mut group = c.benchmark_group("render minuet");

    group.bench_function("default envelope", move |b| {
        b.iter(|| {
            let samples: Vec<_> = song.samples(48000).collect();
            assert!(samples.len() > 48000);
        })
    });

    group.bench_function("twopoint envelope", move |b| {
        b.iter(|| {
            let samples: Vec<_> = song_twopoint_envelope.samples(48000).collect();
            assert!(samples.len() > 48000);
        })
    });

    group.bench_function("simple envelope", move |b| {
        b.iter(|| {
            let samples: Vec<_> = song_simple_envelope.samples(48000).collect();
            assert!(samples.len() > 48000);
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
