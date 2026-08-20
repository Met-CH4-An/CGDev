// SPDX-License-Identifier: None
// Copyright (c) 2026 None

#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// зависимости
// dependencies
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
mod token;


#[path = "tokenizer/_mods.rs"]
pub(crate) mod tokenizer;

use std::hash::{Hash, Hasher};
use std::ops::Deref;
use std::rc::Rc;
pub use tokenizer::{Tokenizer, AVX2};
use crate::token::{Token, TokenType};

#[path = "parser/_mods.rs"]
pub(crate) mod parser;
use parser::Parser;

fn tokenizationBenchmark() {
    let data_ = loadDataFromFile("current.xml").ok().unwrap();

    let data_ = Rc::new(data_);

    let mut tokenizer_ = Tokenizer::<AVX2>::s_create();

    tokenizer_.setData(data_.clone());

    let instant_ = std::time::Instant::now();
    loop {
        let token_ = tokenizer_.nextToken1();

        //let value_str_ = unsafe { std::str::from_utf8_unchecked(&data_.as_slice()[*token_.asRange().start() ..= *token_.asRange().end()]) };

        //println!("{}", value_str_);

        //if value_str_ == "\"Slawek Grajewski @sgrajewski" {
        //    println!("Стопе нахуй");
        //}

        if let TokenType::END = token_.r#type {
            break;
        }
    }
    let duration_ = instant_.elapsed();
    println!("tokenizer: {:?}", duration_);
}

pub struct FileData {

}

fn main() {
    tokenizationBenchmark();

    let mut parser_ = Parser::s_create();

    let instant_ = std::time::Instant::now();
    let vulkan_registry_ = parser_.buildRegistry("current.xml").unwrap();
    let duration_ = instant_.elapsed();
    println!("parser: {:?}", duration_);

    let mut hasher_ = std::hash::DefaultHasher::new();
    //"VkGpaDeviceClockModeAMD".hash(&mut hasher_);
    "VkImageLayout".hash(&mut hasher_);
    let hash_ = hasher_.finish();

    //let vulkan_enums_ = vulkan_registry_.vulkan_enums_hmap.get("API Constants").unwrap();

    let vulkan_enums_ = vulkan_registry_
        .vulkan_enums_hmap
        .get(&hash_)
        .unwrap();

    let data_ = parser_.data_rc.as_slice();

    println!("========================================");
    println!("VULKAN ENUM");
    println!("========================================");

    println!(
        "Name: {}",
        unsafe {
            std::str::from_utf8_unchecked(
                &data_[vulkan_enums_.nameAsRange().clone()]
            )
        }
    );

    println!(
        "Type: {}",
        unsafe {
            std::str::from_utf8_unchecked(
                &data_[vulkan_enums_.typeAsRange().clone()]
            )
        }
    );

    println!(
        "Comment: {}",
        unsafe {
            std::str::from_utf8_unchecked(
                &data_[vulkan_enums_.commentAsRange().clone()]
            )
        }
    );

    println!();
    println!("Enumerators:");
    println!("----------------------------------------");

    for enum_ in vulkan_enums_.enumAsVec() {
        println!(
            "Name: {}",
            unsafe {
                std::str::from_utf8_unchecked(
                    &data_[enum_.nameAsRange().clone()]
                )
            }
        );

        println!(
            "Type: {}",
            unsafe {
                std::str::from_utf8_unchecked(
                    &data_[enum_.typeAsRange().clone()]
                )
            }
        );

        println!(
            "Value: {}",
            unsafe {
                std::str::from_utf8_unchecked(
                    &data_[enum_.valueAsRange().clone()]
                )
            }
        );

        println!(
            "Comment: {}",
            unsafe {
                std::str::from_utf8_unchecked(
                    &data_[enum_.commentAsRange().clone()]
                )
            }
        );

        println!("----------------------------------------");
    }
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

    return Ok(data_);
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Публичные ассоциированные функции.
// Public associated functions.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Публичные методы.
// Public methods.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Приватные ассоциированные функции.
// Private associated functions.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Приватные методы.
// Private methods.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~