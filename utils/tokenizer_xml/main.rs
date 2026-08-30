// SPDX-License-Identifier: None
// Copyright (c) 2026 None

#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// зависимости
// dependencies
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use std::rc::Rc;
use std::time::Duration;
use utils__tokenizer_xml::token::TokenType;
use utils__tokenizer_xml::{AVX2, Tokenizer};

fn main() {
    let data_ = loadDataFromFile("vk.xml").ok().unwrap();

    let data_ = Rc::new(data_);

    let mut tokenizer_ = Tokenizer::<AVX2>::s_create();

    tokenizer_.setData(data_.clone());

    let mut duration_ = Duration::new(0,0);
    for _try_ in 0 ..= 9 {
        tokenizer_.reset();

        let instant_ = std::time::Instant::now();
        loop {
            let token_ = tokenizer_.nextToken1();

            //let value_str_ = unsafe { std::str::from_utf8_unchecked(&data_.as_slice()[*token_.asRange().start() ..= *token_.asRange().end()]) };

            //println!("{}", value_str_);

            if let TokenType::INVALID = token_.asType() {
                break;
            }
        }
        duration_ += instant_.elapsed();
    }

    duration_ /= 10;
    println!("tokenizer: {:?}", duration_);
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
///
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
fn loadDataFromFile(name : &str) -> Result<Vec<u8>, ()> {
    // путь до файла с спецификацией вулкана - ../../external/vulkan/cargo.toml/
    // path to the file with the volcano specification - ../../external/vulkan/cargo.toml/
    let path_ = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent().unwrap()
        .parent().unwrap()
        .join("external")
        .join("vulkan")
        .join(name);

    let data_ = std::fs::read(path_)
        .map_err(|std_error| {
            println!("{}", std_error);
            ()
        }
        )?;

    Ok(data_)
}