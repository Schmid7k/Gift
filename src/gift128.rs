use cipher::{
    consts::{U16, U24, U32},
    AlgorithmName, BlockCipher, Key, KeyInit, KeySizeUser,
};
use core::fmt;

#[cfg(feature = "zeroize")]
use cipher::zeroize::{Zeroize, ZeroizeOnDrop};

use crate::{consts::{GIFT_P, GIFT_P_INV, GIFT_RC, GIFT_S, GIFT_S_INV}, primitives::{u32big, swapmovesingle, swapmove, quintuple_round}, key_schedule::{key_update, rearrange_rkey_0, rearrange_rkey_1, rearrange_rkey_2, rearrange_rkey_3, key_triple_update_0, key_double_update_1, key_triple_update_2, key_triple_update_3, key_double_update_4, key_triple_update_1, key_double_update_2, key_double_update_3, key_triple_update_4}};

fn precompute_rkeys(rkey: &mut [u32], key: &u16) {
    rkey[0] = u32big(&(key.to_be_bytes()[3] as u32));
    rkey[1] = u32big(&(key.to_be_bytes()[1] as u32));
    rkey[2] = u32big(&(key.to_be_bytes()[2] as u32));
    rkey[3] = u32big(&(key.to_be_bytes()[0] as u32));

    for i in (0..16).step_by(2) {
        rkey[i+4] = rkey[i+1];
        rkey[i+5] = key_update(&rkey[i]);
    }

    for i in (0..20).step_by(10) {
        rkey[i] = rearrange_rkey_0(&rkey[i]);
        rkey[i+1] = rearrange_rkey_0(&rkey[i+1]);
        rkey[i+2]	= rearrange_rkey_1(&rkey[i + 2]);
		rkey[i+3]	= rearrange_rkey_1(&rkey[i + 3]);
		rkey[i+4]	= rearrange_rkey_2(&rkey[i + 4]);
		rkey[i+5]	= rearrange_rkey_2(&rkey[i + 5]);
		rkey[i+6]	= rearrange_rkey_3(&rkey[i + 6]);
		rkey[i+7]	= rearrange_rkey_3(&rkey[i + 7]);
    }

    for i in (20..80).step_by(10) {
        rkey[i] = rkey[i-19];
        rkey[i+1] = key_triple_update_0(&rkey[i-20]);
		rkey[i+2] = key_double_update_1(&rkey[i-17]);
		rkey[i+3] = key_triple_update_1(&rkey[i-18]);
		rkey[i+4] = key_double_update_2(&rkey[i-15]);
		rkey[i+5] = key_triple_update_2(&rkey[i-16]);
		rkey[i+6] = key_double_update_3(&rkey[i-13]);
		rkey[i+7] = key_triple_update_3(&rkey[i-14]);
		rkey[i+8] = key_double_update_4(&rkey[i-11]);
		rkey[i+9] = key_triple_update_4(&rkey[i-12]);
		swapmovesingle(&mut rkey[i],  0x00003333, 16);
		swapmovesingle(&mut rkey[i],  0x55554444, 1);
		swapmovesingle(&mut rkey[i+1],  0x55551100, 1);
    }
}

fn packing(state: &mut [u32], input: &[u8]) {
    let mut s0 =	((input[6] as u32) << 24)	| ((input[7] as u32) << 16)	|
				((input[14] as u32) << 8)	| input[15] as u32;
	let mut s1 =	((input[4] as u32) << 24)	| ((input[5] as u32) << 16)	|
				((input[12] as u32) << 8)	| input[13] as u32;
	let mut s2 =	((input[2] as u32) << 24)	| ((input[3] as u32) << 16)	|
				((input[10] as u32 ) << 8)	| input[11] as u32;
	let mut s3 =	((input[0] as u32) << 24)	| ((input[1] as u32) << 16)	|
				((input[8] as u32) << 8)		| input[9] as u32;
    swapmovesingle(&mut s0, 0x0a0a0a0a, 3);
    swapmovesingle(&mut s0, 0x00cc00cc, 6);
    swapmovesingle(&mut s1, 0x0a0a0a0a, 3);
    swapmovesingle(&mut s1, 0x00cc00cc, 6);
    swapmovesingle(&mut s2, 0x0a0a0a0a, 3);
    swapmovesingle(&mut s2, 0x00cc00cc, 6);
    swapmovesingle(&mut s3, 0x0a0a0a0a, 3);
    swapmovesingle(&mut s3, 0x00cc00cc, 6);
    swapmove(&mut s0, &mut s1, 0x000f000f, 4);
    swapmove(&mut s0, &mut s2, 0x000f000f, 8);
    swapmove(&mut s0, &mut s3, 0x000f000f, 12);
    swapmove(&mut s1, &mut s2, 0x00f000f0, 4);
    swapmove(&mut s1, &mut s3, 0x00f000f0, 8);
    swapmove(&mut s2, &mut s3, 0x0f000f00, 4);
    (state[0], state[1], state[2], state[3]) = (s0, s1, s2, s3);
}

fn unpacking(state: &[u32], output: &mut [u8]) {
    let (mut s0, mut s1, mut s2, mut s3) = (state[0], state[1], state[2], state[3]);

    swapmove(&mut s2, &mut s3, 0x0f000f00, 4);
    swapmove(&mut s1, &mut s3, 0x00f000f0, 8);
    swapmove(&mut s1,&mut  s2, 0x00f000f0, 4);
    swapmove(&mut s0,&mut  s3, 0x000f000f, 12);
    swapmove(&mut s0,&mut  s2, 0x000f000f, 8);
    swapmove(&mut s0,&mut  s1, 0x000f000f, 4);
    swapmovesingle(&mut s3, 0x00cc00cc, 6);
    swapmovesingle(&mut s3, 0x0a0a0a0a, 3);
    swapmovesingle(&mut s2, 0x00cc00cc, 6);
    swapmovesingle(&mut s2, 0x0a0a0a0a, 3);
    swapmovesingle(&mut s1, 0x00cc00cc, 6);
    swapmovesingle(&mut s1, 0x0a0a0a0a, 3);
    swapmovesingle(&mut s0, 0x00cc00cc, 6);
    swapmovesingle(&mut s0, 0x0a0a0a0a, 3);
	output[0] = (s3 >> 24) as u8; output[1] = ((s3 >> 16) & 0xff) as u8;
	output[2] = (s2 >> 24) as u8; output[3] = ((s2 >> 16) & 0xff) as u8;
	output[4] = (s1 >> 24) as u8; output[5] = ((s1 >> 16) & 0xff) as u8;
	output[6] = (s0 >> 24) as u8; output[7] = ((s0 >> 16) & 0xff) as u8;
	output[8] = ((s3 >> 8) & 0xff) as u8; output[9] = (s3 & 0xff) as u8;
	output[10] = ((s2 >> 8) & 0xff) as u8; output[11] = (s2 & 0xff) as u8;
	output[12] = ((s1 >> 8) & 0xff) as u8; output[13] = (s1 & 0xff) as u8;
	output[14] = ((s0 >> 8) & 0xff) as u8; output[15] = (s0 & 0xff) as u8;
}

macro_rules! impl_gift {
    ($name:ident, $subkey_size:literal, $key_size:ty, $doc:literal) => {
        #[derive(Clone)]
        pub struct $name {
            /// Subkeys
            k: [u32; $subkey_size],
        }

        impl BlockCipher for $name {}

        impl KeySizeUser for $name {
            type KeySize = $key_size;
        }

        impl fmt::Debug for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(concat!(stringify!($name), " { ... }"))
            }
        }

        #[cfg(feature = "zeroize")]
        impl Drop for $name {
            fn drop(&mut self) {
                self.k.zeroize();
            }
        }

        //impl ZeroizeOnDrop for $name {}

        cipher::impl_simple_block_encdec!(
            $name, U16, cipher, block,
            encrypt: {
                let b = block.get_in();
                let mut state = [0u32; 4];
                packing(&mut state, b);
                for i in (0..40).step_by(5) {
                    quintuple_round(&mut state, cipher.k[i*2], GIFT_RC[i]);
                }
                unpacking(&mut state, block.get_out());
            }
            decrypt: {
                let b = block.get_in();
                let mut d1 = u32::from_be_bytes(b[0..8].try_into().unwrap());
                let mut d2 = u32::from_be_bytes(b[8..16].try_into().unwrap());

            }
        );
    };
}

impl_gift!(Gift128, 40, U16, "Gift-128 block cipher instance.");
