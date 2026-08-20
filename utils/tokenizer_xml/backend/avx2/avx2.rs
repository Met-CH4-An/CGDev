// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// зависимости
// dependencies
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use std::arch::x86_64::{__m256i, _mm256_set1_epi8, _mm256_sub_epi8, _mm256_add_epi8, _mm256_cmpeq_epi8, _mm256_movemask_epi8, _mm256_or_si256, _mm256_cmpgt_epi8, _mm256_and_si256, _mm256_loadu_epi8};
use std::marker::PhantomData;
use crate::backend::backend::{AVX2};
use crate::backend::backend::{Backend, BackendAVX2};
use crate::chunk_mask::ChunkMask;
use crate::chunk_mask_register::ChunkMaskRegister;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
///
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
impl Backend for AVX2 {
    type CHUNK_TYPE1 = u32;
    type REGISTER_TYPE = __m256i;
    const CHUNK_SIZE: usize = u32::BITS as usize;

    const ZERO: u32 = 0u32;
    const ONE: u32 = 1u32;

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    unsafe fn buildChunkMaskRegister() -> ChunkMaskRegister<Self> {
        ChunkMaskRegister::<Self>{
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

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    #[inline(always)]
    unsafe fn buildChunk(chunk_mask_register: &mut ChunkMaskRegister<Self>, chunk: &mut ChunkMask<Self>, ptr: *const u8) {
        // регистр 256, 32 байта по 8 бит, загружаем данные
        // register 256, 32 bytes of 8 bits, load data
        let src_ = _mm256_loadu_epi8(ptr as *const i8);

        chunk.l_chevron_mask = buildMaskAVX2(src_, chunk_mask_register.chevron_l);
        chunk.r_chevron_mask = buildMaskAVX2(src_, chunk_mask_register.chevron_r);
        chunk.equal_mask = buildMaskAVX2(src_, chunk_mask_register.equal);
        chunk.quote_mask = buildMaskAVX2(src_, chunk_mask_register.quote);
        chunk.separators_mask = buildMask4AVX2(src_, chunk_mask_register.sp, chunk_mask_register.tab, chunk_mask_register.lf, chunk_mask_register.cr);
        chunk.letters_digitals_mask = buildMaskRngAVX2(src_, chunk_mask_register.ascii_lowercase, chunk_mask_register.letter_a, chunk_mask_register.letter_z);
        chunk.forward_slash = buildMaskAVX2(src_, chunk_mask_register.slash);
        chunk.question_mark = buildMaskAVX2(src_, chunk_mask_register.qm);
        chunk.exclamation_mark = buildMaskAVX2(src_, chunk_mask_register.em);
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    fn trailingZeros(mask: u32) -> u32 {
        mask.trailing_zeros()
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
///
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
impl BackendAVX2 for AVX2 {}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
///
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
#[inline(always)]
unsafe fn buildMaskAVX2(src: __m256i, dst: __m256i) -> u32 {
    // Сравниваем src (входные данные) с 1 символами.
    // Compare src (input data) with 1 characters.
    let cmpeq_ = _mm256_cmpeq_epi8(src, dst);

    // Строим маску.
    // Build a mask.
    let mask_ =  _mm256_movemask_epi8(cmpeq_);

    mask_ as u32
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
///
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
#[inline(always)]
unsafe fn buildMask3AVX2(src: __m256i, dst_1: __m256i, dst_2: __m256i, dst_3: __m256i) -> u32 {
    // Сравниваем src (входные данные) с 3 символами.
    // Compare src (input data) with 3 characters.
    let cmpeq__1 = _mm256_cmpeq_epi8(src, dst_1);
    let cmpeq__2 = _mm256_cmpeq_epi8(src, dst_2);
    let cmpeq__3 = _mm256_cmpeq_epi8(src, dst_3);

    // Объединяем результат.
    // Combine the result.
    let or_ = _mm256_or_si256(cmpeq__1,
                              _mm256_or_si256(cmpeq__2, cmpeq__3));

    // Строим маску.
    // Build a mask.
    let mask_ = _mm256_movemask_epi8(or_);

    mask_ as u32
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
///
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
#[inline(always)]
unsafe fn buildMask4AVX2(src: __m256i, dst_1: __m256i, dst_2: __m256i, dst_3: __m256i, dst_4: __m256i) -> u32 {
    // Сравниваем src (входные данные) с 4 символами.
    // Compare src (input data) with 4 characters.
    let cmpeq_1 = _mm256_cmpeq_epi8(src, dst_1);
    let cmpeq_2 = _mm256_cmpeq_epi8(src, dst_2);
    let cmpeq_3 = _mm256_cmpeq_epi8(src, dst_3);
    let cmpeq_4 = _mm256_cmpeq_epi8(src, dst_4);

    // Объединяем результат.
    // Combine the result.
    let or_ = _mm256_or_si256(
        _mm256_or_si256(cmpeq_1, cmpeq_2),
        _mm256_or_si256(cmpeq_3, cmpeq_4));

    // Строим маску.
    // Build a mask.
    let mask_ =_mm256_movemask_epi8(or_);

    mask_ as u32
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
///
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
#[inline(always)]
unsafe fn buildMaskRngAVX2(src: __m256i, ascii_lowercase : __m256i, a : __m256i, z : __m256i) -> u32 {
    // Преобразуем сорс в нижний регистр.
    // Делаем or 0x20 [0010_0000].
    // В результате A [0100_0000] становится a [0110_0000]: 0100_0000 or 0010_0000 = 0110_0000
    // В ascii_lowercase установленный в регистр 0x20.
    // Convert the source to lowercase.
    // Do or 0x20 [0010_0000].
    // As a result, A [0100_0000] becomes a [0110_0000]: 0100_0000 or 0010_0000 = 0110_0000
    // In ascii_lowercase, set to register 0x20.
    let or_ = _mm256_or_si256(src, ascii_lowercase);

    // Сравниваем полученный нижний регистр с нижней границей диапазона через gt (greater than).
    // Фиксируем все символы, которые больше нижней границы.
    // Compare the resulting lowercase with the lower range bound using gt (greater than).
    // Mark all characters greater than the lower bound.
    let cmp_1_ = _mm256_cmpgt_epi8(or_, a);

    // Сравниваем верхнюю границу диапазона с полученным нижним регистром через gt (greater than).
    // Фиксируем все символы, которые меньше верхней границы.
    // Compare the upper range bound with the resulting lowercase using gt (greater than).
    // Mark all characters less than the upper bound.
    let cmp_2_ = _mm256_cmpgt_epi8(z, or_);

    // Совмещаем результат через логическое И.
    // Combine the result using logical AND.
    let and_ = _mm256_and_si256(cmp_1_, cmp_2_);

    // Строим маску.
    // Build a mask.
    let mask_ =_mm256_movemask_epi8(and_);

    mask_ as u32
}