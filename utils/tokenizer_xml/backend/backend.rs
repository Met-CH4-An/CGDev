// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// зависимости
// dependencies
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// маркеры версий
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use std::arch::x86_64::{_mm256_add_epi8, _mm256_set1_epi8, _mm256_sub_epi8};
use std::marker::PhantomData;
use crate::chunk_mask::ChunkMask;
use crate::chunk_mask_register::ChunkMaskRegister;

pub struct AVX2 {}
pub struct AVX512 {}

mod private {
    pub trait BackendPrivate {}
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
///
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
pub trait Backend: private::BackendPrivate {
    type CHUNK_TYPE1: Default + Copy
    + Into<u64>
    + From<u32>
    + PartialEq
    + std::ops::BitAnd<Output = Self::CHUNK_TYPE1>
    + std::ops::BitAndAssign
    + std::ops::BitOr<Output = Self::CHUNK_TYPE1>
    + std::ops::BitOrAssign
    + std::ops::BitXor<Output = Self::CHUNK_TYPE1>
    + std::ops::Not<Output = Self::CHUNK_TYPE1>
    + std::ops::Shl<usize, Output = Self::CHUNK_TYPE1>
    + std::ops::Shr<usize, Output = Self::CHUNK_TYPE1>
    + std::ops::Sub<Output = Self::CHUNK_TYPE1>
    + std::ops::SubAssign;

    type REGISTER_TYPE;

    const CHUNK_SIZE: usize;
    const ZERO: Self::CHUNK_TYPE1;
    const ONE: Self::CHUNK_TYPE1;

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    unsafe fn buildChunkMaskRegister() -> ChunkMaskRegister<Self>
    where
        Self: Sized;

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    unsafe fn buildChunk(chunk_mask_register: &mut ChunkMaskRegister<Self>, chunk: &mut ChunkMask<Self>, ptr: *const u8)
    where
        Self: Sized;

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    fn trailingZeros(mask: Self::CHUNK_TYPE1) -> u32;
}
pub trait BackendAVX2: Backend<CHUNK_TYPE1 = u32> {}
pub trait BackendAVX512: Backend<CHUNK_TYPE1 = u64> {}

impl private::BackendPrivate for AVX2 {}

impl private::BackendPrivate for AVX512 {}
impl Backend for AVX512 {
    type CHUNK_TYPE1 = u64;
    type REGISTER_TYPE = std::arch::x86_64::__m256i;
    const CHUNK_SIZE: usize = u64::BITS as usize;
    const ZERO: u64 = 0;
    const ONE: u64 = 1u64;
    unsafe fn buildChunkMaskRegister() -> ChunkMaskRegister<Self> {
        ChunkMaskRegister::<Self> {
            phantom_data: PhantomData,
            chevron_l: _mm256_set1_epi8(b'<' as i8),
            chevron_r: _mm256_set1_epi8(b'>' as i8),

            tab: _mm256_set1_epi8(b'\t' as i8),
            lf: _mm256_set1_epi8(b'\n' as i8),
            cr: _mm256_set1_epi8(b'\r' as i8),
            sp: _mm256_set1_epi8(b' ' as i8),

            quote: _mm256_set1_epi8(b'"' as i8),
            equal: _mm256_set1_epi8(b'=' as i8),

            letter_a: _mm256_sub_epi8(_mm256_set1_epi8(b'a' as i8), _mm256_set1_epi8(1)),
            letter_z: _mm256_add_epi8(_mm256_set1_epi8(b'z' as i8), _mm256_set1_epi8(1)),
            ascii_lowercase: _mm256_set1_epi8(0x20i8),

            slash: _mm256_set1_epi8(b'/' as i8),
            qm: _mm256_set1_epi8(b'?' as i8),
            em: _mm256_set1_epi8(b'!' as i8),
        }
    }
    unsafe fn buildChunk(chunk_mask_register: &mut ChunkMaskRegister<Self>, chunk: &mut ChunkMask<Self>, ptr: *const u8) {
        //chunk.buildAVX512(ptr);
    }
    fn trailingZeros(mask: u64) -> u32 {
        mask.trailing_zeros()
    }
}
impl BackendAVX512 for AVX512 {}