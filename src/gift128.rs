use core::fmt;
use cipher::{
    consts::{U16, U24, U32},
    AlgorithmName, BlockCipher, Key, KeyInit, KeySizeUser
};

#[cfg(feature = "zeroize")]
use cipher::zeroize::{Zeroize, ZeroizeOnDrop};

use crate::consts::{GIFT_S, GIFT_S_INV, GIFT_P, GIFT_P_INV, GIFT_RC};

macro_rules! impl_gift {
    ($name:ident, $subkey_size:literal, $key_size:ty, $doc:literal) => {
        #[derive(Clone)]
        pub struct $name {
            /// Subkeys
            k: [u64; $subkey_size],
        }

        impl BlockCipher for $name {}

        impl KeySizeUser for $name {
            type KeySize = $key_size;
        }

        impl fmt::Debug for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(concat!stringify!($name), " { ... }")
            }
        }

        #[cfg(feature = "zeroize")]
        impl Drop for $name {
            fn drop(&mut self) {
                self.k.zeroize();
            }
        }

        impl ZeroizeOnDrop for $name {}

        cipher::impl_simple_block_encdec!(
            $name, U16, cipher, block,
            encrypt: {
                let b = block.get_in();
                let mut d1 = u64::from_be_bytes(b[0..8].try_into().unwrap());
                let mut d2 = u64::from_be_bytes(b[8..16].try_into().unwrap());

                d1 ^= cipher.k[0];
                d2 ^= cipher.k[1];

                BE::write_u64_into(&[d2, d1], block.get_out());
            }
            decrypt: {
                let b = block.get_int();
                let mut d1 = u64::from_be_bytes(b[0..8].try_into().unwrap());
                let mut d2 = u64::from_be_bytes(b[8..16].try_into().unwrap());

                BE::write_u64_into(&[d2, d1], block.get_out());
            }
        );
    };
}

impl_gift!(Gift128, 40, U16, "Gift-128 block cipher instance.");