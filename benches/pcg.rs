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

#[bench]
fn bench_pcg32_flat(b: &mut Bencher) {
    let mut rng = flat::Pcg32Flat::with_stream(42, 54);
    b.iter(|| for _ in 0..BENCH_ROUNDS { black_box(rng.next()); });
}

mod flat {
    pub struct Pcg32Flat {
        state: u64,
        stream: u64,
    }

    impl Pcg32Flat {
        pub fn with_stream(state: u64, specific_seq: u64) -> Self {
            let stream = (specific_seq << 1) | 1;
            Pcg32Flat {
                state: bump(stream, state.wrapping_add(stream)),
                stream: stream,
            }
        }

        #[inline]
        pub fn next(&mut self) -> u32 {
            self.state = bump(self.stream, self.state);
            output(self.state)
        }
    }

    #[inline]
    fn bump(stream: u64, state: u64) -> u64 {
        state.wrapping_mul(6364136223846793005).wrapping_add(stream)
    }

    #[inline]
    fn output(internal: u64) -> u32 {
        #![allow(non_upper_case_globals)]
        const bits: usize = 64;
        const result_bits: usize = 32;
        const spare_bits: usize = bits - result_bits;
        const wanted_op_bits: usize = 5;
        const op_bits: usize = wanted_op_bits;
        const amplifier: usize = wanted_op_bits - op_bits;
        const mask: u64 = (1 << op_bits) - 1;
        const top_spare: usize = op_bits;
        const bottom_spare: usize = spare_bits - top_spare;
        const x_shift: usize = (top_spare + result_bits) / 2;
        const r_shift: usize = bits - op_bits;

        let rot = (internal >> r_shift) & mask;
        let amp_rot = (rot << amplifier) & mask;
        let internal = internal ^ (internal >> x_shift);

        let result = (internal >> bottom_spare) as u32;
        let result = result.rotate_right(amp_rot as u32);

        result
    }
}
