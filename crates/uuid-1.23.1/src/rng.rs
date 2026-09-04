#![allow(dead_code, unused_imports)] // Keeps our cfg's from becoming too convoluted in here

trait Rng {
    fn u128() -> u128;
    fn u64() -> u64;
    fn u16() -> u16;
}

pub(crate) fn u128() -> u128 {
    imp::RngImp::u128()
}

pub(crate) fn u64() -> u64 {
    imp::RngImp::u64()
}

pub(crate) fn u16() -> u16 {
    imp::RngImp::u16()
}

mod imp
{
    /*
    Random support for non `wasm32-unknown-unknown` platforms. */
    use super::*;

    // Using `rand`
    pub(super) struct RngImp;

    impl Rng for RngImp {
        fn u128() -> u128 {
            rand::random()
        }

        fn u64() -> u64 {
            rand::random()
        }

        fn u16() -> u16 {
            rand::random()
        }
    }
}
