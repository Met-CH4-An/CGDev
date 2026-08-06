// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// зависимости
// dependencies
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use std::arch::x86_64::{__m256i, _mm256_and_si256, _mm256_cmpeq_epi8, _mm256_cmpgt_epi8, _mm256_loadu_epi8, _mm256_movemask_epi8, _mm256_or_si256 };
use crate::tokenizer::tokenizer::{TokenizerBackendAVX512};
use crate::tokenizer::tokenizer_chunk_mask::TokenizerChunkMask;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// Приватные ассоциированные функции.
/// Private associated functions.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
impl<TBackend> TokenizerChunkMask<TBackend>
where
    TBackend: TokenizerBackendAVX512 {
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    unsafe fn s_buildMask(src: __m256i, dst: __m256i) -> u64 {
        // Сравниваем src (входные данные) с 1 символами.
        // Compare src (input data) with 1 characters.
        //let cmpeq_ = _mm256_cmpeq_epi8(src, dst);

        // Строим маску.
        // Build a mask.
        //let mask_ =  _mm256_movemask_epi8(cmpeq_);

        0 as u64
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    unsafe fn s_buildMask3(src: __m256i, dst_1: __m256i, dst_2: __m256i, dst_3: __m256i) -> u64 {
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

        0 as u64
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    unsafe fn s_buildMask4(src: __m256i, dst_1: __m256i, dst_2: __m256i, dst_3: __m256i, dst_4: __m256i) -> u64 {
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

        0 as u64
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    unsafe fn s_buildMaskRng(src: __m256i, ascii_lowercase : __m256i, a : __m256i, z : __m256i) -> u64 {
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

        0 as u64
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// Приватные методы.
/// Private methods.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
impl<TBackend> TokenizerChunkMask<TBackend>
where
    TBackend: TokenizerBackendAVX512 {
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// Функция строит чанк, используя avx2.
    /// Функция не проверяет диапазоны, а работает только с входными данными.
    /// Ответственность за проверки лежат на вызывающем коде. Поэтому unsafe.
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub(crate) unsafe fn build(&mut self, data_cptr: *const u8) {
        unsafe {
            // регистр 256, 32 байта по 8 бит, загружаем данные
            // register 256, 32 bytes of 8 bits, load data
            let src_ = _mm256_loadu_epi8(data_cptr as *const i8);

            self.l_chevron_mask = Self::s_buildMask(src_, self.registers.chevron_l);
            self.r_chevron_mask = Self::s_buildMask(src_, self.registers.chevron_r);
            self.equal_mask = Self::s_buildMask(src_, self.registers.equal);
            self.quote_mask = Self::s_buildMask(src_, self.registers.quote);
            self.separators_mask = Self::s_buildMask4(src_, self.registers.sp, self.registers.tab, self.registers.lf, self.registers.cr);
            self.letters_digitals_mask = Self::s_buildMaskRng(src_, self.registers.ascii_lowercase, self.registers.letter_a, self.registers.letter_z);
            self.forward_slash = Self::s_buildMask(src_, self.registers.slash);
            self.question_mark = Self::s_buildMask(src_, self.registers.qm);
            self.exclamation_mark = Self::s_buildMask(src_, self.registers.em);

        }
    }
}