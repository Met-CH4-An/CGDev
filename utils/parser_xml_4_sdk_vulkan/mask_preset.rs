// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// зависимости
// dependencies
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
use {
    std::arch::x86_64::*,
};

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
///
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
pub(crate) struct MaskPreset {
    pub(crate) tab : __m256i,          // '\t'  = 0x09
    pub(crate) lf : __m256i,           // '\n'  = 0x0A
    pub(crate) cr : __m256i,           // '\r'  = 0x0D
    pub(crate) sp : __m256i,           // ' '   = 0x20
    pub(crate) em : __m256i,           // '!'   = 0x21
    pub(crate) quote : __m256i,        // '"'   = 0x22
    pub(crate) slash : __m256i,        // '/'   = 0x2F
    pub(crate) chevron_l : __m256i,    // '<'   = 0x3C
    pub(crate) chevron_r : __m256i,    // '>'   = 0x3E
    pub(crate) equal : __m256i,        // '='   = 0x3D
    pub(crate) qm : __m256i,           // '?'   = 0x3F
    pub(crate) letter_A : __m256i,     // 'A'   = 0x41
    pub(crate) letter_Z : __m256i,     // 'Z'   = 0x5A
}


// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// публичные методы
/// public methods
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
impl MaskPreset {
    
}


// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// защищённые методы
/// protected methods
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
impl MaskPreset {
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub(crate) fn create() -> Self {
        unsafe {
            Self {
                tab : _mm256_set1_epi8(b'\t' as i8),
                lf : _mm256_set1_epi8(b'\n' as i8),
                cr : _mm256_set1_epi8(b'\r' as i8),
                sp : _mm256_set1_epi8(b' ' as i8),
                em : _mm256_set1_epi8(b'!' as i8),
                quote : _mm256_set1_epi8(b'"' as i8),
                slash : _mm256_set1_epi8(b'/' as i8),
                chevron_l : _mm256_set1_epi8(b'<' as i8),
                chevron_r : _mm256_set1_epi8(b'>' as i8),
                equal : _mm256_set1_epi8(b'=' as i8),
                qm : _mm256_set1_epi8(b'?' as i8),
                letter_A : _mm256_set1_epi8(b'A' as i8),
                letter_Z : _mm256_set1_epi8(b'Z' as i8),
            }
        }
    }
}


// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// приватные методы
/// private methods
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
impl MaskPreset {
    
}