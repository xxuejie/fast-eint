use eint::{Eint, E256};
use fast_eint::msbc_256;
use proptest::prelude::*;
use rand_chacha::{
    rand_core::{RngCore, SeedableRng},
    ChaCha20Rng,
};

#[test]
fn test_single_sbc_256() {
    let mut rng = ChaCha20Rng::seed_from_u64(100);

    let mut buf_a = vec![0u8; 32];
    rng.fill_bytes(&mut buf_a);
    let mut buf_b = vec![0u8; 32];
    rng.fill_bytes(&mut buf_b);

    let a = E256::get(&buf_a);
    let b = E256::get(&buf_b);
    let result = {
        let (_, borrow) = a.overflowing_sub_u(b);
        borrow
    };

    let actual = msbc_256(buf_a.as_ptr(), buf_b.as_ptr());

    assert_eq!(result, actual);
}

proptest! {
    #[test]
    fn random_sbc_256(
        buf_a in prop::array::uniform32(0u8..),
        buf_b in prop::array::uniform32(0u8..),
    ) {
        let a = E256::get(&buf_a);
        let b = E256::get(&buf_b);
        let result = {
            let (_, borrow) = a.overflowing_sub_u(b);
            borrow
        };

        let actual = msbc_256(buf_a.as_ptr(), buf_b.as_ptr());

        assert_eq!(result, actual);
    }
}
