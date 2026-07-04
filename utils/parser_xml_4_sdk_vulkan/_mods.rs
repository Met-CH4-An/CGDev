// SPDX-License-Identifier: None
// Copyright (c) 2026 None

#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// зависимости
// dependencies
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
mod event;
pub use event::*;

mod parser;
pub use parser::*;

mod preset_mask;
pub(crate) use preset_mask::*;

mod chunk_mask;
pub(crate) use chunk_mask::*;

fn main() {
    // создаем парсер
    let mut parser_ = Parser::create();

    loop {
        // новое состояние
        let state_ = parser_.nextEvent();

        println!("{}", state_.toStr());

        match state_.toType() {
            crate::EventType::END => {
                break;
            }
            (_) => {}
        }
    }    
}