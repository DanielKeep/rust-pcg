#![feature(test)]
extern crate test;
extern crate rand;

use test::{Bencher, black_box};
use rand::Rng;

const BENCH_ROUNDS: usize = 1_000;

#[bench]
fn bench_chacha(b: &mut Bencher) {
    let mut rng = rand::ChaChaRng::new_unseeded();
    b.iter(|| for _ in 0..BENCH_ROUNDS { black_box(rng.next_u32()); });
}

#[bench]
fn bench_isaac(b: &mut Bencher) {
    let mut rng = rand::IsaacRng::new_unseeded();
    b.iter(|| for _ in 0..BENCH_ROUNDS { black_box(rng.next_u32()); });
}

#[bench]
fn bench_isaac64(b: &mut Bencher) {
    let mut rng = rand::Isaac64Rng::new_unseeded();
    b.iter(|| for _ in 0..BENCH_ROUNDS { black_box(rng.next_u32()); });
}

#[bench]
fn bench_os(b: &mut Bencher) {
    let mut rng = rand::OsRng::new().unwrap();
    b.iter(|| for _ in 0..BENCH_ROUNDS { black_box(rng.next_u32()); });
}

#[bench]
fn bench_std(b: &mut Bencher) {
    let mut rng = rand::StdRng::new().unwrap();
    b.iter(|| for _ in 0..BENCH_ROUNDS { black_box(rng.next_u32()); });
}

#[bench]
fn bench_xor_shift(b: &mut Bencher) {
    let mut rng = rand::XorShiftRng::new_unseeded();
    b.iter(|| for _ in 0..BENCH_ROUNDS { black_box(rng.next_u32()); });
}
