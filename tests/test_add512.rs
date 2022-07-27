use eint::{Eint, E512};
use fast_eint::wrapping_add_512;
use proptest::prelude::*;
use rand_chacha::{
    rand_core::{RngCore, SeedableRng},
    ChaCha20Rng,
};

#[test]
fn test_single_add_512() {
    let mut rng = ChaCha20Rng::seed_from_u64(111);

    let mut buf_a = vec![0u8; 64];
    rng.fill_bytes(&mut buf_a);
    let mut buf_b = vec![0u8; 64];
    rng.fill_bytes(&mut buf_b);

    let a = E512::get(&buf_a);
    let b = E512::get(&buf_b);
    let result = {
        let c = a.clone().wrapping_add(b.clone());

        let mut buf = vec![0u8; 64];
        c.put(&mut buf[..]);

        buf
    };

    let mut result_buffer = [0u8; 64];

    wrapping_add_512(
        buf_a.as_ptr(),
        buf_b.as_ptr(),
        result_buffer.as_mut_ptr(),
        1,
    );

    assert_eq!(result, result_buffer);
}

#[test]
fn test_single_add_512_same_a() {
    let mut rng = ChaCha20Rng::seed_from_u64(111);

    let mut buf_a = vec![0u8; 64];
    rng.fill_bytes(&mut buf_a);
    let mut buf_b = vec![0u8; 64];
    rng.fill_bytes(&mut buf_b);

    let a = E512::get(&buf_a);
    let b = E512::get(&buf_b);
    let result = {
        let c = a.clone().wrapping_add(b.clone());

        let mut buf = vec![0u8; 64];
        c.put(&mut buf[..]);

        buf
    };

    wrapping_add_512(buf_a.as_ptr(), buf_b.as_ptr(), buf_a.as_mut_ptr(), 1);

    assert_eq!(result, buf_a);
}

#[test]
fn test_single_add_512_same_b() {
    let mut rng = ChaCha20Rng::seed_from_u64(111);

    let mut buf_a = vec![0u8; 64];
    rng.fill_bytes(&mut buf_a);
    let mut buf_b = vec![0u8; 64];
    rng.fill_bytes(&mut buf_b);

    let a = E512::get(&buf_a);
    let b = E512::get(&buf_b);
    let result = {
        let c = a.clone().wrapping_add(b.clone());

        let mut buf = vec![0u8; 64];
        c.put(&mut buf[..]);

        buf
    };

    wrapping_add_512(buf_a.as_ptr(), buf_b.as_ptr(), buf_b.as_mut_ptr(), 1);

    assert_eq!(result, buf_b);
}

#[test]
fn test_batch_add_8_512() {
    let mut rng = ChaCha20Rng::seed_from_u64(1234);

    let mut buf_a = vec![0u8; 64 * 8];
    let mut buf_b = vec![0u8; 64 * 8];
    let mut buf_c = vec![0u8; 64 * 8];
    rng.fill_bytes(&mut buf_a);
    rng.fill_bytes(&mut buf_b);
    rng.fill_bytes(&mut buf_c);
    let mut buf_expected = vec![0u8; 64 * 8];

    for i in 0..8 {
        let a = E512::get(&buf_a[i * 64..i * 64 + 64]);
        let b = E512::get(&buf_b[i * 64..i * 64 + 64]);

        let c = a.wrapping_add(b);

        c.put(&mut buf_expected[i * 64..i * 64 + 64]);
    }

    wrapping_add_512(buf_a.as_ptr(), buf_b.as_ptr(), buf_c.as_mut_ptr(), 8);

    assert_eq!(buf_c, buf_expected);
}

proptest! {
    #[test]
    fn random_batch_16_add_512(
        a in prop::array::uniform32(prop::array::uniform32(0u8..)),
        b in prop::array::uniform32(prop::array::uniform32(0u8..)),
    ) {
        let mut expected = vec![0u8; 16 * 64];
        for i in 0..16 {
            let mut buf_a = [0u8; 64];
            buf_a[0..32].copy_from_slice(&a[i]);
            buf_a[32..64].copy_from_slice(&a[i + 16]);

            let mut buf_b = [0u8; 64];
            buf_b[0..32].copy_from_slice(&b[i]);
            buf_b[32..64].copy_from_slice(&b[i + 16]);

            let a = E512::get(&buf_a);
            let b = E512::get(&buf_b);

            let c = a.wrapping_add(b);

            c.put(&mut expected[i * 64..i * 64 + 64]);
        }

        let mut buf_a = vec![0u8; 16 * 64];
        let mut buf_b = vec![0u8; 16 * 64];
        let mut buf_c = vec![0u8; 16 * 64];
        for i in 0..16 {
            buf_a[i * 64..i * 64 + 32].copy_from_slice(&a[i]);
            buf_a[i * 64 + 32..i * 64 + 64].copy_from_slice(&a[i + 16]);

            buf_b[i * 64..i * 64 + 32].copy_from_slice(&b[i]);
            buf_b[i * 64 + 32..i * 64 + 64].copy_from_slice(&b[i + 16]);
        }

        wrapping_add_512(buf_a.as_ptr(), buf_b.as_ptr(), buf_c.as_mut_ptr(), 16);

        assert_eq!(expected, buf_c);
    }
}
