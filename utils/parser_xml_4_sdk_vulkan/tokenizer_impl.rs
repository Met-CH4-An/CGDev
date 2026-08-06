// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// зависимости
// dependencies
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/*
use std::arch::x86_64::{__m256i, _mm256_and_si256, _mm256_cmpeq_epi8, _mm256_cmpgt_epi8, _mm256_loadu_epi8, _mm256_movemask_epi8, _mm256_or_si256};
use crate::tokenizer_chunk_mask::ChunkMask;
use crate::tokenizer_chunk_mask_register::PresetMask;
use crate::Tokenizer;
use crate::tokenizer::TokenizerState;



// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// приватная область
//
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// Приватные методы.
/// Private methods.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
impl Tokenizer {
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub(crate) fn nextChunk(&mut self, data : &[u8]) {
        let pos_ = self.current_chunk_position + 32;

        if self.current_chunk_position + 64 <= data.len() {
            self.current_chunk_position += 32;

            let data_ptr_ = unsafe { data.as_ptr().add(self.current_chunk_position) };

            unsafe { Self::s_buildChunk(data_ptr_, &self.preset_mask, &mut self.current_masks); }

            return;
        }

        self.state = TokenizerState::END;
    }
}*/