// SPDX-License-Identifier: None
// Copyright (c) 2026 None

#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// зависимости
// dependencies
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
mod event;
pub use event::*;

mod parser;
pub use parser::*;

fn main() {
    // создаем парсер
    let mut parser_ = Parser::create();

    loop {
        // новое состояние
        let state_ = parser_.parse();

        //match state_ {
        //    ParseState::END => {
        //        break;
        //    }
        //}
    }    
}