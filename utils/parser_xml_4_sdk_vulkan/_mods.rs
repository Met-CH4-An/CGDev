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

use std::fmt::format;
pub use token::*;

mod preset_mask;
pub(crate) use preset_mask::*;

mod chunk_mask;
mod parser;
pub use parser::*;
//mod parse_item;

#[path = "parse_items/_mods.rs"]
pub mod parse_items;

mod tag_attribute;
mod tag;

fn main() {
    let mut svk_enums_data = String::new();
    svk_enums_data.push_str("// SPDX-License-Identifier: None\n// Copyright (c) 2026 None\n");

    let data_ = loadDataFromFile("1.4.356.xml").ok().unwrap();

    let mut parser_ = Parser::new().ok().unwrap();

    let mut parse_enums_as_enum_ = false;
    let mut parse_enums_as_bitmask_ = false;
    let mut parse_enums_as_bitmask_64_ = false;
    let mut parse_enums_as_constant_ = false;
    let mut type_ = String::new();
    let mut asd : u32 = 0;
    'tag: loop {
        asd += 1;

        let (tag_, end_ ) = parser_.nextToken(&data_);

        //println!("{}", tag_.name(&data_));



        if tag_.name(&data_) == "enums" && !tag_.isClosed() {
            for attribute in tag_.iter() {
                if attribute.name(&data_) == "type" && attribute.value(&data_) == "enum" {
                    parse_enums_as_enum_ = true;
                }

                if attribute.name(&data_) == "type" && attribute.value(&data_) == "bitmask" {
                    parse_enums_as_bitmask_ = true;
                }

                if attribute.name(&data_) == "bitwidth" && attribute.value(&data_) == "64" {
                    parse_enums_as_bitmask_ = false;
                    parse_enums_as_bitmask_64_ = true;
                }

                if attribute.name(&data_) == "type" && attribute.value(&data_) == "constants" {
                    continue 'tag;
                    parse_enums_as_constant_ = true;
                }
            }

            //if parse_enums_as_enum_ {

                for attribute in tag_.iter() {
                    if attribute.name(&data_) == "name" {
                        if parse_enums_as_enum_ {
                            svk_enums_data.push_str(&format!("pub type {} = i32;\n", attribute.value(&data_)));
                        }

                        if parse_enums_as_bitmask_ {
                            svk_enums_data.push_str(&format!("pub type {} = u32;\n", attribute.value(&data_)));
                        }

                        if parse_enums_as_bitmask_64_ {
                            svk_enums_data.push_str(&format!("pub type {} = u64;\n", attribute.value(&data_)));
                        }

                        svk_enums_data.push_str(&format!("pub mod {}Value {{\n", attribute.value(&data_)));
                        svk_enums_data.push_str(&format!("\tuse crate::{};\n\n", attribute.value(&data_)));
                        type_ = format!("{}", attribute.value(&data_));
                    }
                }

                continue 'tag;
            //}
        }

        if tag_.name(&data_) == "enums" && tag_.isClosed() && ( parse_enums_as_enum_ || parse_enums_as_bitmask_ || parse_enums_as_bitmask_64_) {
            svk_enums_data.push_str(&format!("}}\n\n"));

            parse_enums_as_enum_ = false;
            parse_enums_as_bitmask_ = false;
            parse_enums_as_bitmask_64_ = false;
            parse_enums_as_constant_ = false;
        }

        if parse_enums_as_enum_ || parse_enums_as_bitmask_ {
            let mut name_ = String::new();
            let mut value_ = String::new();

            for attribute in tag_.iter() {
                if attribute.name(&data_) == "name" {
                    name_ = format!("{}", attribute.value(&data_));
                }

                if attribute.name(&data_) == "value" {
                    value_ = format!("{}", attribute.value(&data_));
                }

                if attribute.name(&data_) == "bitpos" && parse_enums_as_bitmask_64_ {
                    let a = format!("{}", attribute.value(&data_));
                    let b : u64 = a.parse().unwrap();
                    let c : u64 = 1u64 << b;
                    let d = format!("{}", c);

                    value_ = format!("{}", d);
                }

                if attribute.name(&data_) == "bitpos" && parse_enums_as_bitmask_{
                    let a = format!("{}", attribute.value(&data_));
                    let b : u32 = a.parse().unwrap();
                    let c : u32 = 1u32 << b;
                    let d = format!("{}", c);

                    value_ = format!("{}", d);
                }

            }

            if name_.is_empty() || value_.is_empty() || type_.is_empty() {continue;}
            svk_enums_data.push_str(&format!("\tpub const {} : {} = {};\n", name_, type_, value_));
        }

        if end_ {
            break;
        }

    }

    println!("end");
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