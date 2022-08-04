use crate::primitives::{swapmovesingle};

pub(crate) fn reaarrange_rkey_0(x: &mut u32) {
    swapmovesingle(x, 0x00550055, 9);
	swapmovesingle(x, 0x000f000f, 12);
	swapmovesingle(x, 0x00003333, 18);
	swapmovesingle(x, 0x000000ff, 24);
}

pub(crate) fn reaarrange_rkey_1(x: &mut u32) {
    swapmovesingle(x, 0x11111111, 3);
	swapmovesingle(x, 0x03030303, 6);
	swapmovesingle(x,  0x000f000f, 12);
	swapmovesingle(x, 0x000000ff, 24);
}

pub(crate) fn reaarrange_rkey_2(x: &mut u32) {
    swapmovesingle(x, 0x0000aaaa, 15);
	swapmovesingle(x, 0x00003333, 18);
	swapmovesingle(x, 0x0000f0f0, 12);
	swapmovesingle(x, 0x000000ff, 24);
}

pub(crate) fn reaarrange_rkey_3(x: &mut u32) {
    swapmovesingle(x, 0x0a0a0a0a, 3);
	swapmovesingle(x, 0x00cc00cc, 6);
	swapmovesingle(x, 0x0000f0f0, 12);
	swapmovesingle(x, 0x000000ff, 24);
}

pub(crate) fn key_update(x: &u32) -> u32 {
    (((*x) >> 12) & 0x0000000f)	| (((*x) & 0x00000fff) << 4) |
	(((*x) >> 2) & 0x3fff0000)	| (((*x) & 0x00030000) << 14)
}

