use eint::{Eint, E256};
use fast_eint::wrapping_mul_256;
use proptest::prelude::*;
use rand_chacha::{
    rand_core::{RngCore, SeedableRng},
    ChaCha20Rng,
};

#[test]
fn test_single_wrapping_mul_256() {
    let mut rng = ChaCha20Rng::seed_from_u64(100);

    let mut buf_a = vec![0u8; 32];
    rng.fill_bytes(&mut buf_a);
    let mut buf_b = vec![0u8; 32];
    rng.fill_bytes(&mut buf_b);

    let a = E256::get(&buf_a);
    let b = E256::get(&buf_b);
    let expected = {
        let c = a.clone().wrapping_mul(b.clone());

        let mut buf = vec![0u8; 32];
        c.put(&mut buf);
        buf
    };

    let mut result = vec![0u8; 32];

    wrapping_mul_256(buf_a.as_ptr(), buf_b.as_ptr(), result.as_mut_ptr(), 1);

    assert_eq!(result, expected);
}

#[test]
fn test_batch_wrapping_mul_8_256() {
    let mut rng = ChaCha20Rng::seed_from_u64(123);
    let mut buf_a = vec![0u8; 32 * 8];
    let mut buf_b = vec![0u8; 32 * 8];
    let mut buf_expected = vec![0u8; 32 * 8];
    rng.fill_bytes(&mut buf_a);
    rng.fill_bytes(&mut buf_b);

    for i in 0..8 {
        let a = E256::get(&buf_a[i * 32..i * 32 + 32]);
        let b = E256::get(&buf_b[i * 32..i * 32 + 32]);

        let c = a.wrapping_mul(b);
        c.put(&mut buf_expected[i * 32..i * 32 + 32]);
    }

    let mut buf_result = vec![0u8; 32 * 8];

    wrapping_mul_256(buf_a.as_ptr(), buf_b.as_ptr(), buf_result.as_mut_ptr(), 8);

    assert_eq!(buf_expected, buf_result);
}

proptest! {
    #[test]
    fn random_batch_16_wrapping_mul_256(
        a in prop::array::uniform16(prop::array::uniform32(0u8..)),
        b in prop::array::uniform16(prop::array::uniform32(0u8..)),
    ) {
        let mut buf_a = vec![0u8; 32 * 16];
        let mut buf_b = vec![0u8; 32 * 16];
        let mut buf_expected = vec![0u8; 32 * 16];
        for i in 0..16 {
            buf_a[i * 32..i * 32 + 32].copy_from_slice(&a[i]);
            buf_b[i * 32..i * 32 + 32].copy_from_slice(&b[i]);

            let a = E256::get(&a[i]);
            let b = E256::get(&b[i]);

            let c = a.wrapping_mul(b);
            c.put(&mut buf_expected[i * 32..i * 32 + 32]);
        }

        let mut buf_result = vec![0u8; 32 * 16];

        wrapping_mul_256(buf_a.as_ptr(), buf_b.as_ptr(), buf_result.as_mut_ptr(), 16);

        assert_eq!(buf_expected, buf_result);
    }
}
