use criterion::{black_box, criterion_group, criterion_main, Criterion};
use asciimusic::image::{Payload, Image, Pixel};
use rand::Rng;

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let dimensions = (500, 500);
    let data: Vec<u8> = (0..1000).map(|_| rng.gen()).collect();
    let payload = Payload::new(&data);
    let origin_image = Image::new(dimensions, std::iter::from_fn(||
        Some(Pixel {
            r: rng.gen(),
            g: rng.gen(),
            b: rng.gen(),
            a: rng.gen(),
        })).take(dimensions.0 as usize * dimensions.1 as usize).collect::<Vec<Pixel>>());
    c.bench_function("image 500x500 1000 rand", move |b| b.iter(|| {
        let mut image = origin_image.clone();
        image.bake_payload(&payload);
        let read_data = image.read_payload().expect("Could not read payload").data().expect("Could not read data");
        assert_eq!(data, read_data);
    }));

    let data: Vec<u8> = (0..500).map(|_| rng.gen()).collect();
    let payload = Payload::new(&data);
    let origin_image = Image::new(dimensions, std::iter::from_fn(||
        Some(Pixel {
            r: rng.gen(),
            g: rng.gen(),
            b: rng.gen(),
            a: rng.gen(),
        })).take(dimensions.0 as usize * dimensions.1 as usize).collect::<Vec<Pixel>>());

    c.bench_function("image 500x500 500 rand", move |b| b.iter(|| {
        let mut image = origin_image.clone();
        image.bake_payload(&payload);
        let read_data = image.read_payload().expect("Could not read payload").data().expect("Could not read data");
        assert_eq!(data, read_data);
    }));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
