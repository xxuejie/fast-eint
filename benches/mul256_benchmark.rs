use criterion::{criterion_group, criterion_main, Criterion};
use eint::{Eint, E256, E512};
use fast_eint::{
    narrowing_right_shift_512, narrowing_right_shift_512_c, widening_mul_256, wrapping_mul_256,
};
use rand_chacha::{
    rand_core::{RngCore, SeedableRng},
    ChaCha20Rng,
};

const BATCH_RUNS: usize = 128;

pub fn fast_batch_mul256_benchmark(c: &mut Criterion) {
    c.bench_function("fast batch mul256", |b| {
        let mut rng = ChaCha20Rng::seed_from_u64(10000);
        let mut buf = vec![0u8; (32 + 32 + 64) * BATCH_RUNS];
        rng.fill_bytes(&mut buf);

        b.iter(|| {
            widening_mul_256(
                &mut buf,
                (32 + 32) * BATCH_RUNS,
                0,
                32 * BATCH_RUNS,
                BATCH_RUNS,
            );
        })
    });
}

pub fn normal_batch_mul256_benchmark(c: &mut Criterion) {
    c.bench_function("normal batch mul256", |b| {
        let mut rng = ChaCha20Rng::seed_from_u64(10000);
        let mut buf = vec![0u8; (32 + 32 + 64) * BATCH_RUNS];
        rng.fill_bytes(&mut buf);

        b.iter(|| {
            for i in 0..BATCH_RUNS {
                let a = E256::get(&buf[i * 32..i * 32 + 32]);
                let b = E256::get(&buf[32 * BATCH_RUNS + i * 32..32 * BATCH_RUNS + i * 32 + 32]);

                let (lo, hi) = a.widening_mul_u(b);

                lo.put(
                    &mut buf[(32 + 32) * BATCH_RUNS + i * 32..(32 + 32) * BATCH_RUNS + i * 32 + 32],
                );
                hi.put(
                    &mut buf[(32 + 32) * BATCH_RUNS + i * 32 + 32
                        ..(32 + 32) * BATCH_RUNS + i * 32 + 64],
                );
            }
        })
    });
}

pub fn fast_single_mul256_benchmark(c: &mut Criterion) {
    c.bench_function("fast single mul256", |b| {
        let mut rng = ChaCha20Rng::seed_from_u64(20000);
        let mut buf = vec![0u8; 32 + 32 + 64];
        rng.fill_bytes(&mut buf);

        b.iter(|| {
            widening_mul_256(&mut buf, 32 + 32, 0, 32, 1);
        })
    });
}

pub fn normal_single_mul256_benchmark(c: &mut Criterion) {
    c.bench_function("normal single mul256", |b| {
        let mut rng = ChaCha20Rng::seed_from_u64(20000);
        let mut buf = vec![0u8; 32 + 32 + 64];
        rng.fill_bytes(&mut buf);

        b.iter(|| {
            let a = E256::get(&buf[0..32]);
            let b = E256::get(&buf[32..64]);

            let (lo, hi) = a.widening_mul_u(b);

            lo.put(&mut buf[64..96]);
            hi.put(&mut buf[96..128]);
        })
    });
}

pub fn fast_batch_narrowing_right_shift_512_benchmark(c: &mut Criterion) {
    c.bench_function("fast batch narrowing_right_shift_512", |b| {
        let mut rng = ChaCha20Rng::seed_from_u64(10000);
        let mut buf = vec![0u8; 64 * BATCH_RUNS];
        rng.fill_bytes(&mut buf);
        let mut result = vec![0u8; 32 * BATCH_RUNS];

        b.iter(|| {
            narrowing_right_shift_512(buf.as_ptr(), result.as_mut_ptr(), 111, BATCH_RUNS);
        })
    });
}

pub fn c_batch_narrowing_right_shift_512_benchmark(c: &mut Criterion) {
    c.bench_function("c batch narrowing_right_shift_512", |b| {
        let mut rng = ChaCha20Rng::seed_from_u64(10000);
        let mut buf = vec![0u8; 64 * BATCH_RUNS];
        rng.fill_bytes(&mut buf);
        let mut result = vec![0u8; 32 * BATCH_RUNS];

        b.iter(|| {
            narrowing_right_shift_512_c(buf.as_ptr(), result.as_mut_ptr(), 111, BATCH_RUNS);
        })
    });
}

pub fn normal_batch_narrowing_right_shift_512_benchmark(c: &mut Criterion) {
    c.bench_function("normal batch narrowing_right_shift_512", |b| {
        let mut rng = ChaCha20Rng::seed_from_u64(10000);
        let mut buf = vec![0u8; 64 * BATCH_RUNS];
        rng.fill_bytes(&mut buf);
        let mut result = vec![0u8; 32 * BATCH_RUNS];

        b.iter(|| {
            for i in 0..BATCH_RUNS {
                let b = E512::get(&buf[i * 64..i * 64 + 64]);
                let a = E512::from(E256::from(111));

                let r = b.wrapping_shr(a.u32());
                r.put_lo(&mut result[i * 32..i * 32 + 32]);
            }
        })
    });
}

pub fn fast_batch_wrapping_mul256_benchmark(c: &mut Criterion) {
    c.bench_function("fast batch wrapping mul256", |b| {
        let mut rng = ChaCha20Rng::seed_from_u64(10000);
        let mut buf_a = vec![0u8; 32 * BATCH_RUNS];
        let mut buf_b = vec![0u8; 32 * BATCH_RUNS];
        let mut buf_c = vec![0u8; 32 * BATCH_RUNS];
        rng.fill_bytes(&mut buf_a);
        rng.fill_bytes(&mut buf_b);

        b.iter(|| {
            wrapping_mul_256(
                buf_a.as_ptr(),
                buf_b.as_ptr(),
                buf_c.as_mut_ptr(),
                BATCH_RUNS,
            );
        })
    });
}

pub fn normal_batch_wrapping_mul256_benchmark(c: &mut Criterion) {
    c.bench_function("normal batch wrapping mul256", |b| {
        let mut rng = ChaCha20Rng::seed_from_u64(10000);
        let mut buf_a = vec![0u8; 32 * BATCH_RUNS];
        let mut buf_b = vec![0u8; 32 * BATCH_RUNS];
        let mut buf_c = vec![0u8; 32 * BATCH_RUNS];
        rng.fill_bytes(&mut buf_a);
        rng.fill_bytes(&mut buf_b);

        b.iter(|| {
            for i in 0..BATCH_RUNS {
                let a = E256::get(&buf_a[i * 32..i * 32 + 32]);
                let b = E256::get(&buf_b[i * 32..i * 32 + 32]);

                let c = a.wrapping_mul(b);

                c.put(&mut buf_c[i * 32..i * 32 + 32]);
            }
        })
    });
}

criterion_group!(
    benches,
    normal_single_mul256_benchmark,
    fast_single_mul256_benchmark,
    normal_batch_mul256_benchmark,
    fast_batch_mul256_benchmark,
    normal_batch_narrowing_right_shift_512_benchmark,
    fast_batch_narrowing_right_shift_512_benchmark,
    c_batch_narrowing_right_shift_512_benchmark,
    normal_batch_wrapping_mul256_benchmark,
    fast_batch_wrapping_mul256_benchmark,
);
criterion_main!(benches);
