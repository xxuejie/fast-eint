use eint::{Eint, E512};
use fast_eint::narrowing_right_shift_512_c as narrowing_right_shift_512;
use proptest::prelude::*;
use rand_chacha::{
    rand_core::{RngCore, SeedableRng},
    ChaCha20Rng,
};

#[test]
fn test_single_nrshift_412() {
    let mut rng = ChaCha20Rng::seed_from_u64(100);

    let mut buf_a = vec![0u8; 64];
    rng.fill_bytes(&mut buf_a);

    let a = E512::get(&buf_a);
    let result = {
        let c = a.wrapping_shr(111);

        let mut buf = vec![0u8; 32];
        c.put_lo(&mut buf[..]);

        buf
    };

    let mut result_buffer = [0u8; 32];

    narrowing_right_shift_512(buf_a.as_ptr(), result_buffer.as_mut_ptr(), 111, 1);

    assert_eq!(result, result_buffer);
}

#[test]
fn test_batch_nrshift_8_256() {
    let mut rng = ChaCha20Rng::seed_from_u64(2222);

    let mut buf_a = vec![0u8; 64 * 8];
    let mut buf_c = vec![0u8; 32 * 8];
    rng.fill_bytes(&mut buf_a);
    rng.fill_bytes(&mut buf_c);
    let mut buf_expected = vec![0u8; 32 * 8];

    for i in 0..8 {
        let a = E512::get(&buf_a[i * 64..i * 64 + 64]);

        let c = a.wrapping_shr(342);

        c.put_lo(&mut buf_expected[i * 32..i * 32 + 32]);
    }

    narrowing_right_shift_512(buf_a.as_ptr(), buf_c.as_mut_ptr(), 342, 8);

    assert_eq!(buf_c, buf_expected);
}

#[test]
fn test_batch_nrshift_8_256_same() {
    let mut rng = ChaCha20Rng::seed_from_u64(3333);

    let mut buf_a = vec![0u8; 64 * 8];
    rng.fill_bytes(&mut buf_a);
    let mut buf_expected = vec![0u8; 32 * 8];

    for i in 0..8 {
        let a = E512::get(&buf_a[i * 64..i * 64 + 64]);

        let c = a.wrapping_shr(19);

        c.put_lo(&mut buf_expected[i * 32..i * 32 + 32]);
    }

    narrowing_right_shift_512(buf_a.as_ptr(), buf_a.as_mut_ptr(), 19, 8);

    assert_eq!(&buf_a[0..32 * 8], buf_expected);
}

proptest! {
    #[test]
    fn random_batch_16_add_512(
        a in prop::array::uniform32(prop::array::uniform32(0u8..)),
        shift in 0u32..1024u32,
    ) {
        let mut expected = vec![0u8; 16 * 32];
        for i in 0..16 {
            let mut buf_a = [0u8; 64];
            buf_a[0..32].copy_from_slice(&a[i]);
            buf_a[32..64].copy_from_slice(&a[i + 16]);

            let a = E512::get(&buf_a);

            let c = a.wrapping_shr(shift);

            c.put_lo(&mut expected[i * 32..i * 32 + 32]);
        }

        let mut buf_a = vec![0u8; 16 * 64];
        let mut buf_c = vec![0u8; 16 * 32];
        for i in 0..16 {
            buf_a[i * 64..i * 64 + 32].copy_from_slice(&a[i]);
            buf_a[i * 64 + 32..i * 64 + 64].copy_from_slice(&a[i + 16]);
        }

        narrowing_right_shift_512(buf_a.as_ptr(), buf_c.as_mut_ptr(), shift, 16);

        assert_eq!(expected, buf_c);
    }
}
