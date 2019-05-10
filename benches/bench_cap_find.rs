use criterion;
use criterion::{criterion_group, criterion_main};
use image;

use rqrr;

fn find_caps<S>(mut img: rqrr::SearchableImage<S>) -> Vec<rqrr::CapStone> where S: rqrr::SearchableImageBuffer {
    rqrr::capstones_from_image(&mut img)
}

fn bench_find_caps(c: &mut criterion::Criterion) {
    let img = include_bytes!("../tests/data/cap/art_small.png");
    let img = image::load_from_memory(img).unwrap().to_luma();
    let w = img.width() as usize;
    let h = img.height() as usize;

    let img = rqrr::SearchableImage::from_greyscale(w, h, |x, y| {
        img.get_pixel(x as u32, y as u32).data[0]
    });

    c.bench_function("find_caps art", move |b| {
        b.iter(|| find_caps(img.clone()))
    });
}



criterion_group!(benches, bench_find_caps);
criterion_main!(benches);
