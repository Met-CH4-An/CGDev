// SPDX-License-Identifier: None
// Copyright (c) 2026 None

#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

// файл token.rs
// file token.rs
pub mod token;

// файл tokenizer.rs
// file tokenizer.rs
mod tokenizer;
pub use tokenizer::Tokenizer;

// файл chunk_mask.rs
// file chunk_mask.rs
pub mod chunk_mask;

// файл chunk_mask_register.rs
// file chunk_mask_register.rs
pub mod chunk_mask_register;

#[path = "backend/_mods.rs"]
mod backend;
pub use backend::backend::AVX2;
pub use backend::backend::AVX512;
