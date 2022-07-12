use eint::{Eint, E256};
use fast_eint::widening_mul_256;
use proptest::prelude::*;
use rand_chacha::{
    rand_core::{RngCore, SeedableRng},
    ChaCha20Rng,
};

#[test]
fn test_single_mul_256() {
    let mut rng = ChaCha20Rng::seed_from_u64(100);

    let mut buf_a = vec![0u8; 32];
    rng.fill_bytes(&mut buf_a);
    let mut buf_b = vec![0u8; 32];
    rng.fill_bytes(&mut buf_b);

    let a = E256::get_unsafe(&buf_a);
    let b = E256::get_unsafe(&buf_b);
    let result = {
        let (lo, hi) = a.clone().widening_mul_u(b.clone());

        let mut buf = vec![0u8; 64];
        lo.put(&mut buf[0..32]);
        hi.put(&mut buf[32..64]);

        buf
    };

    let mut concat_buffer = [0u8; 128];
    concat_buffer[0..32].copy_from_slice(&buf_a[..]);
    concat_buffer[32..64].copy_from_slice(&buf_b[..]);

    widening_mul_256(&mut concat_buffer, 64, 0, 32, 1);

    assert_eq!(result, concat_buffer[64..128]);
}

#[test]
fn test_batch_mul_8_256() {
    let mut rng = ChaCha20Rng::seed_from_u64(123);
    let mut buf = vec![0u8; 32 * 8 * 2 + 64 * 8 + 16 * 2];
    rng.fill_bytes(&mut buf);
    let mut buf2 = buf.clone();

    for i in 0..8 {
        let a = E256::get_unsafe(&buf[i * 32..i * 32 + 32]);
        let b = E256::get_unsafe(&buf[32 * 8 + 16 + i * 32..32 * 8 + 16 + i * 32 + 32]);

        let (lo, hi) = a.widening_mul_u(b);

        lo.put(&mut buf[32 * 8 * 2 + 16 * 2 + i * 64..32 * 8 * 2 + 16 * 2 + i * 64 + 32]);
        hi.put(&mut buf[32 * 8 * 2 + 16 * 2 + i * 64 + 32..32 * 8 * 2 + 16 * 2 + i * 64 + 32 + 32]);
    }

    widening_mul_256(&mut buf2, 32 * 8 * 2 + 16 * 2, 0, 32 * 8 + 16, 8);

    assert_eq!(buf2, buf);
}

proptest! {
    #[test]
    fn random_batch_16_mul_256(
        a in prop::array::uniform16(prop::array::uniform32(0u8..)),
        b in prop::array::uniform16(prop::array::uniform32(0u8..)),
    ) {
        let mut expected = vec![0u8; 64 * 32];
        for i in 0..16 {
            let a = E256::get_unsafe(&a[i]);
            let b = E256::get_unsafe(&b[i]);

            let (lo, hi) = a.widening_mul_u(b);

            lo.put(&mut expected[i * 64..i * 64 + 32]);
            hi.put(&mut expected[i * 64 + 32..i * 64 + 64]);
        }

        // layout: b, c, a
        let mut buffer = vec![0u8; (32 + 32 + 64) * 32];
        for i in 0..16 {
            buffer[i * 32..i * 32 + 32].copy_from_slice(&a[i]);
            buffer[(32 + 64) * 32 + i * 32..(32 + 64) * 32 + i * 32 + 32].copy_from_slice(&b[i]);
        }

        widening_mul_256(&mut buffer, 32 * 32, 0, (32 + 64) * 32, 16);

        assert_eq!(expected, buffer[32 * 32..(32 + 64) * 32]);
    }
}
