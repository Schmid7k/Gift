pub(crate) fn u32big(x: &u32) -> u32 {
    (((*x) & 0x000000FF) << 24)
        | (((*x) & 0x0000FF00) << 8)
        | (((*x) & 0x00FF0000) >> 8)
        | (((*x) & 0xFF000000) >> 24)
}

pub(crate) fn ror(x: &u32, y: &u32) -> u32 {
    ((*x) >> (*y)) | (*x << (32 - (*y)))
}

pub(crate) fn byte_ror_2(x: &u32) -> u32 {
    (((x) >> 2) & 0x3f3f3f3f) | (((x) & 0x03030303) << 6)
}

pub(crate) fn byte_ror_4(x: &u32) -> u32 {
    (((x) >> 4) & 0x0f0f0f0f) | (((x) & 0x0f0f0f0f) << 4)
}

pub(crate) fn byte_ror_6(x: &u32) -> u32 {
    (((x) >> 6) & 0x03030303) | (((x) & 0x3f3f3f3f) << 2)
}

pub(crate) fn half_ror_4(&x: &u32) -> u32 {
    (((x) >> 4) & 0x0fff0fff) | (((x) & 0x000f000f) << 12)
}

pub(crate) fn half_ror_8(x: &u32) -> u32 {
    (((x) >> 8) & 0x00ff00ff) | (((x) & 0x00ff00ff) << 8)
}

pub(crate) fn half_ror_12(&x: &u32) -> u32 {
    (((x) >> 12) & 0x000f000f) | (((x) & 0x0fff0fff) << 4)
}

pub(crate) fn nibble_ror_1(x: &u32) -> u32 {
    (((x) >> 1) & 0x77777777) | (((x) & 0x11111111) << 3)
}

pub(crate) fn nibble_ror_2(x: &u32) -> u32 {
    (((x) >> 2) & 0x33333333) | (((x) & 0x33333333) << 2)
}

pub(crate) fn nibble_ror_3(&x: &u32) -> u32 {
    (((x) >> 3) & 0x11111111) | (((x) & 0x77777777) << 1)
}

pub(crate) fn swapmove(a: &mut u32, b: &mut u32, mask: u32, n: u8) {
    let tmp = (*b ^ (*a >> n)) & mask;
    *b ^= tmp;
    *a ^= tmp << n;
}

pub(crate) fn swapmovesingle(a: &mut u32, mask: u32, n: u8) {
    let tmp = (*a ^ (*a >> n)) & mask;
    *a ^= tmp;
    *a ^= tmp << n;
}

pub(crate) fn sbox(s0: &mut u32, s1: &mut u32, s2: &mut u32, s3: &mut u32) {
    *s1 ^= *s0 & *s2;
    *s0 ^= *s1 & *s3;
    *s2 ^= *s0 | *s1;
    *s3 ^= *s2;
    *s1 ^= *s3;
    *s3 ^= 0xffffffff;
    *s2 ^= *s0 & *s1;
}

pub(crate) fn inv_sbox(s0: &mut u32, s1: &mut u32, s2: &mut u32, s3: &mut u32) {
    *s2 ^= *s3 & *s1;
    *s0 ^= 0xffffffff;
    *s1 ^= *s0;
    *s0 ^= *s2;
    *s2 ^= *s3 | *s1;
    *s3 ^= *s1 & *s0;
    *s1 ^= *s3 & *s2;
}

pub(crate) fn quintuple_round(state: &mut [u32; 4], rkey: [u8; 10], rconst: [u8; 5]) {
    let mut s0 = state[0];
    let mut s1 = state[1];
    let mut s2 = state[2];
    let mut s3 = state[3];
    sbox(&mut s0, &mut s1, &mut s2, &mut s3);
    state[3] = nibble_ror_1(&state[3]);
    state[1] = nibble_ror_2(&state[1]);
    state[2] = nibble_ror_3(&state[2]);
    state[1] ^= rkey[0] as u32;
    state[2] ^= rkey[1] as u32;
    state[0] ^= rconst[0] as u32;
    sbox(&mut s3, &mut s1, &mut s2, &mut s0);
    state[0] = half_ror_4(&state[0]);
    state[1] = half_ror_8(&state[1]);
    state[2] = half_ror_12(&state[2]);
    state[1] ^= rkey[2] as u32;
    state[2] ^= rkey[3] as u32;
    state[3] ^= rconst[1] as u32;
    sbox(&mut s0, &mut s1, &mut s2, &mut s3);
    state[3] = ror(&state[3], &16);
    state[2] = ror(&state[2], &16);
    swapmovesingle(&mut s1, 0x55555555, 1);
    swapmovesingle(&mut s2, 0x00005555, 1);
    swapmovesingle(&mut s3, 0x55550000, 1);
    state[1] ^= rkey[4] as u32;
    state[2] ^= rkey[5] as u32;
    state[0] ^= rconst[2] as u32;
    sbox(&mut s3, &mut s1, &mut s2, &mut s0);
    state[0] = byte_ror_6(&state[0]);
    state[1] = byte_ror_4(&state[1]);
    state[2] = byte_ror_2(&state[2]);
    state[1] ^= rkey[6] as u32;
    state[2] ^= rkey[7] as u32;
    state[3] ^= rconst[3] as u32;
    sbox(&mut s0, &mut s1, &mut s2, &mut s3);
    state[3] = ror(&state[3], &24);
    state[1] = ror(&state[1], &16);
    state[2] = ror(&state[2], &8);
    state[1] ^= rkey[8] as u32;
    state[2] ^= rkey[9] as u32;
    state[0] ^= rconst[4] as u32;
    state.swap(0, 3);
}
