#![feature(test)]
extern crate test;
extern crate pcg;

use test::{Bencher, black_box};
use pcg::PcgGenerator;

const BENCH_ROUNDS: usize = 1_000;

#[bench]
fn bench_pcg32(b: &mut Bencher) {
    let mut rng = pcg::Pcg32::with_stream(42, 54);
    b.iter(|| for _ in 0..BENCH_ROUNDS { black_box(rng.next()); });
}

#[bench]
fn bench_pcg32_oneseq(b: &mut Bencher) {
    let mut rng = pcg::Pcg32OneSeq::with_state(42);
    b.iter(|| for _ in 0..BENCH_ROUNDS { black_box(rng.next()); });
}

#[bench]
fn bench_pcg32_unique(b: &mut Bencher) {
    let mut rng = pcg::Pcg32Unique::with_state(42);
    b.iter(|| for _ in 0..BENCH_ROUNDS { black_box(rng.next()); });
}

#[bench]
fn bench_pcg32_fast(b: &mut Bencher) {
    let mut rng = pcg::Pcg32Fast::with_state(42);
    b.iter(|| for _ in 0..BENCH_ROUNDS { black_box(rng.next()); });
}

#[bench]
fn bench_pcg32_once_insecure(b: &mut Bencher) {
    let mut rng = pcg::Pcg32OnceInsecure::with_stream(42, 54);
    b.iter(|| for _ in 0..BENCH_ROUNDS { black_box(rng.next()); });
}
