mod expand;

use crate::Block;
use core::arch::x86_64::*;

#[macro_export]
macro_rules! aes128_store {
    ($to:expr, $from:expr) => {{
        use core::arch::x86_64::*;

        _mm_storeu_si128($to.as_mut_ptr() as *mut __m128i, $from);
    }};
}

#[macro_export]
macro_rules! aes128_store4 {
    ($to:expr, $from:expr) => {{
        use core::arch::x86_64::*;

        _mm_storeu_si128($to[0].as_mut_ptr() as *mut __m128i, $from[0]);
        _mm_storeu_si128($to[1].as_mut_ptr() as *mut __m128i, $from[1]);
        _mm_storeu_si128($to[2].as_mut_ptr() as *mut __m128i, $from[2]);
        _mm_storeu_si128($to[3].as_mut_ptr() as *mut __m128i, $from[3]);
    }};
}

macro_rules! aes128_xor4 {
    ($what:expr, $with:expr) => {{
        use core::arch::x86_64::*;

        $what[0] = _mm_xor_si128($what[0], $with);
        $what[1] = _mm_xor_si128($what[1], $with);
        $what[2] = _mm_xor_si128($what[2], $with);
        $what[3] = _mm_xor_si128($what[3], $with);
    }};
}

macro_rules! aes128_xor4x4 {
    ($what:expr, $with:expr) => {{
        use core::arch::x86_64::*;

        $what[0] = _mm_xor_si128($what[0], $with[0]);
        $what[1] = _mm_xor_si128($what[1], $with[1]);
        $what[2] = _mm_xor_si128($what[2], $with[2]);
        $what[3] = _mm_xor_si128($what[3], $with[3]);
    }};
}

macro_rules! aes128_encode4 {
    ($target:expr, $key:expr) => {{
        use core::arch::x86_64::*;

        $target[0] = _mm_aesenc_si128($target[0], $key);
        $target[1] = _mm_aesenc_si128($target[1], $key);
        $target[2] = _mm_aesenc_si128($target[2], $key);
        $target[3] = _mm_aesenc_si128($target[3], $key);
    }};
}

macro_rules! aes128_encode4_last {
    ($target:expr, $key:expr) => {{
        use core::arch::x86_64::*;

        $target[0] = _mm_aesenclast_si128($target[0], $key);
        $target[1] = _mm_aesenclast_si128($target[1], $key);
        $target[2] = _mm_aesenclast_si128($target[2], $key);
        $target[3] = _mm_aesenclast_si128($target[3], $key);
    }};
}

macro_rules! aes128_decode4 {
    ($target:expr, $key:expr) => {{
        use core::arch::x86_64::*;

        $target[0] = _mm_aesdec_si128($target[0], $key);
        $target[1] = _mm_aesdec_si128($target[1], $key);
        $target[2] = _mm_aesdec_si128($target[2], $key);
        $target[3] = _mm_aesdec_si128($target[3], $key);
    }};
}

macro_rules! aes128_decode4_last {
    ($target:expr, $key:expr) => {{
        use core::arch::x86_64::*;

        $target[0] = _mm_aesdeclast_si128($target[0], $key);
        $target[1] = _mm_aesdeclast_si128($target[1], $key);
        $target[2] = _mm_aesdeclast_si128($target[2], $key);
        $target[3] = _mm_aesdeclast_si128($target[3], $key);
    }};
}

#[macro_export]
macro_rules! aes128_load {
    ($var:expr) => {{
        use core::arch::x86_64::*;

        _mm_loadu_si128($var.as_ptr() as *const __m128i)
    }};
}

#[macro_export]
macro_rules! aes128_load4 {
    ($var0:expr, $var1:expr, $var2:expr, $var3:expr) => {{
        use core::arch::x86_64::*;

        [
            _mm_loadu_si128($var0.as_ptr() as *const __m128i),
            _mm_loadu_si128($var1.as_ptr() as *const __m128i),
            _mm_loadu_si128($var2.as_ptr() as *const __m128i),
            _mm_loadu_si128($var3.as_ptr() as *const __m128i),
        ]
    }};
    ($var:expr) => {
        aes128_load4!($var[0], $var[1], $var[2], $var[3])
    };
}

macro_rules! compare_eq4 {
    ($what:expr, $with:expr) => {{
        use core::arch::x86_64::*;

        let mut value = [0u128];
        _mm_storeu_si128(
            value.as_mut_ptr() as *mut __m128i,
            _mm_and_si128(
                _mm_and_si128(
                    _mm_cmpeq_epi64($what[0], $with[0]),
                    _mm_cmpeq_epi64($what[1], $with[1]),
                ),
                _mm_and_si128(
                    _mm_cmpeq_epi64($what[2], $with[2]),
                    _mm_cmpeq_epi64($what[3], $with[3]),
                ),
            ),
        );
        value == [u128::max_value()]
    }};
}

pub fn por_encode_pipelined_x4_low_level(
    keys_reg: [__m128i; 11],
    blocks_reg: &mut [__m128i; 4],
    feedbacks_reg: [__m128i; 4],
    aes_iterations: usize,
) {
    unsafe {
        aes128_xor4x4!(blocks_reg, feedbacks_reg);

        for _ in 0..aes_iterations {
            aes128_xor4!(blocks_reg, keys_reg[0]);

            aes128_encode4!(blocks_reg, keys_reg[1]);
            aes128_encode4!(blocks_reg, keys_reg[2]);
            aes128_encode4!(blocks_reg, keys_reg[3]);
            aes128_encode4!(blocks_reg, keys_reg[4]);
            aes128_encode4!(blocks_reg, keys_reg[5]);
            aes128_encode4!(blocks_reg, keys_reg[6]);
            aes128_encode4!(blocks_reg, keys_reg[7]);
            aes128_encode4!(blocks_reg, keys_reg[8]);
            aes128_encode4!(blocks_reg, keys_reg[9]);

            aes128_encode4_last!(blocks_reg, keys_reg[10]);
        }
    }
}

pub fn por_decode_pipelined_x4_low_level(
    keys_reg: [__m128i; 11],
    blocks_reg: &mut [__m128i; 4],
    feedbacks_reg: [__m128i; 4],
    aes_iterations: usize,
) {
    unsafe {
        for _ in 0..aes_iterations {
            aes128_xor4!(blocks_reg, keys_reg[10]);

            aes128_decode4!(blocks_reg, keys_reg[9]);
            aes128_decode4!(blocks_reg, keys_reg[8]);
            aes128_decode4!(blocks_reg, keys_reg[7]);
            aes128_decode4!(blocks_reg, keys_reg[6]);
            aes128_decode4!(blocks_reg, keys_reg[5]);
            aes128_decode4!(blocks_reg, keys_reg[4]);
            aes128_decode4!(blocks_reg, keys_reg[3]);
            aes128_decode4!(blocks_reg, keys_reg[2]);
            aes128_decode4!(blocks_reg, keys_reg[1]);

            aes128_decode4_last!(blocks_reg, keys_reg[0]);
        }

        aes128_xor4x4!(blocks_reg, feedbacks_reg);
    }
}

pub fn pot_prove_low_level(
    keys_reg: [__m128i; 11],
    mut block_reg: __m128i,
    inner_iterations: usize,
) -> __m128i {
    unsafe {
        for _ in 0..inner_iterations {
            block_reg = _mm_xor_si128(block_reg, keys_reg[0]);
            block_reg = _mm_aesenc_si128(block_reg, keys_reg[1]);
            block_reg = _mm_aesenc_si128(block_reg, keys_reg[2]);
            block_reg = _mm_aesenc_si128(block_reg, keys_reg[3]);
            block_reg = _mm_aesenc_si128(block_reg, keys_reg[4]);
            block_reg = _mm_aesenc_si128(block_reg, keys_reg[5]);
            block_reg = _mm_aesenc_si128(block_reg, keys_reg[6]);
            block_reg = _mm_aesenc_si128(block_reg, keys_reg[7]);
            block_reg = _mm_aesenc_si128(block_reg, keys_reg[8]);
            block_reg = _mm_aesenc_si128(block_reg, keys_reg[9]);

            block_reg = _mm_aesenclast_si128(block_reg, keys_reg[10]);
        }
    }

    block_reg
}

pub fn pot_verify_pipelined_x4_low_level(
    keys_reg: [__m128i; 11],
    expected_reg: [__m128i; 4],
    mut blocks_reg: [__m128i; 4],
    aes_iterations: usize,
) -> bool {
    unsafe {
        for _ in 0..aes_iterations {
            aes128_xor4!(blocks_reg, keys_reg[10]);

            aes128_decode4!(blocks_reg, keys_reg[9]);
            aes128_decode4!(blocks_reg, keys_reg[8]);
            aes128_decode4!(blocks_reg, keys_reg[7]);
            aes128_decode4!(blocks_reg, keys_reg[6]);
            aes128_decode4!(blocks_reg, keys_reg[5]);
            aes128_decode4!(blocks_reg, keys_reg[4]);
            aes128_decode4!(blocks_reg, keys_reg[3]);
            aes128_decode4!(blocks_reg, keys_reg[2]);
            aes128_decode4!(blocks_reg, keys_reg[1]);

            aes128_decode4_last!(blocks_reg, keys_reg[0]);
        }

        compare_eq4!(expected_reg, blocks_reg)
    }
}

pub type ExpandedKeys = [__m128i; 11];

pub fn expand(key: &Block) -> (ExpandedKeys, ExpandedKeys) {
    expand::expand(key)
}
