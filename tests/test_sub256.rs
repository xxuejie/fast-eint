use eint::{Eint, E256};
use fast_eint::wrapping_sub_256;
use proptest::prelude::*;
use rand_chacha::{
    rand_core::{RngCore, SeedableRng},
    ChaCha20Rng,
};

#[test]
fn test_single_sub_256() {
    let mut rng = ChaCha20Rng::seed_from_u64(100);

    let mut buf_a = vec![0u8; 32];
    rng.fill_bytes(&mut buf_a);
    let mut buf_b = vec![0u8; 32];
    rng.fill_bytes(&mut buf_b);

    let a = E256::get(&buf_a);
    let b = E256::get(&buf_b);
    let result = {
        let c = a.clone().wrapping_sub(b.clone());

        let mut buf = vec![0u8; 32];
        c.put(&mut buf[..]);

        buf
    };

    let mut result_buffer = [0u8; 32];

    wrapping_sub_256(
        buf_a.as_ptr(),
        buf_b.as_ptr(),
        result_buffer.as_mut_ptr(),
        1,
    );

    assert_eq!(result, result_buffer);
}

#[test]
fn test_single_sub_256_same_a() {
    let mut rng = ChaCha20Rng::seed_from_u64(12);

    let mut buf_a = vec![0u8; 32];
    rng.fill_bytes(&mut buf_a);
    let mut buf_b = vec![0u8; 32];
    rng.fill_bytes(&mut buf_b);

    let a = E256::get(&buf_a);
    let b = E256::get(&buf_b);
    let result = {
        let c = a.clone().wrapping_sub(b.clone());

        let mut buf = vec![0u8; 32];
        c.put(&mut buf[..]);

        buf
    };

    wrapping_sub_256(buf_a.as_ptr(), buf_b.as_ptr(), buf_a.as_mut_ptr(), 1);

    assert_eq!(result, buf_a);
}

#[test]
fn test_single_sub_256_same_b() {
    let mut rng = ChaCha20Rng::seed_from_u64(32);

    let mut buf_a = vec![0u8; 32];
    rng.fill_bytes(&mut buf_a);
    let mut buf_b = vec![0u8; 32];
    rng.fill_bytes(&mut buf_b);

    let a = E256::get(&buf_a);
    let b = E256::get(&buf_b);
    let result = {
        let c = a.clone().wrapping_sub(b.clone());

        let mut buf = vec![0u8; 32];
        c.put(&mut buf[..]);

        buf
    };

    wrapping_sub_256(buf_a.as_ptr(), buf_b.as_ptr(), buf_b.as_mut_ptr(), 1);

    assert_eq!(result, buf_b);
}

#[test]
fn test_batch_sub_8_256() {
    let mut rng = ChaCha20Rng::seed_from_u64(123);

    let mut buf_a = vec![0u8; 32 * 8];
    let mut buf_b = vec![0u8; 32 * 8];
    let mut buf_c = vec![0u8; 32 * 8];
    rng.fill_bytes(&mut buf_a);
    rng.fill_bytes(&mut buf_b);
    rng.fill_bytes(&mut buf_c);
    let mut buf_expected = vec![0u8; 32 * 8];

    for i in 0..8 {
        let a = E256::get(&buf_a[i * 32..i * 32 + 32]);
        let b = E256::get(&buf_b[i * 32..i * 32 + 32]);

        let c = a.wrapping_sub(b);

        c.put(&mut buf_expected[i * 32..i * 32 + 32]);
    }

    wrapping_sub_256(buf_a.as_ptr(), buf_b.as_ptr(), buf_c.as_mut_ptr(), 8);

    assert_eq!(buf_c, buf_expected);
}

proptest! {
    #[test]
    fn random_batch_16_sub_256(
        a in prop::array::uniform16(prop::array::uniform32(0u8..)),
        b in prop::array::uniform16(prop::array::uniform32(0u8..)),
    ) {
        let mut expected = vec![0u8; 16 * 32];
        for i in 0..16 {
            let a = E256::get(&a[i]);
            let b = E256::get(&b[i]);

            let c = a.wrapping_sub(b);

            c.put(&mut expected[i * 32..i * 32 + 32]);
        }

        let mut buf_a = vec![0u8; 16 * 32];
        let mut buf_b = vec![0u8; 16 * 32];
        let mut buf_c = vec![0u8; 16 * 32];
        for i in 0..16 {
            buf_a[i * 32..i * 32 + 32].copy_from_slice(&a[i]);
            buf_b[i * 32..i * 32 + 32].copy_from_slice(&b[i]);
        }

        wrapping_sub_256(buf_a.as_ptr(), buf_b.as_ptr(), buf_c.as_mut_ptr(), 16);

        assert_eq!(expected, buf_c);
    }
}
