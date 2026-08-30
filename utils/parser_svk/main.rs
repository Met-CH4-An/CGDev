// SPDX-License-Identifier: None
// Copyright (c) 2026 None

use std::rc::Rc;

fn main() {
    // Загружаем данные.
    // Loading data.
    let data_vec_ = loadDataFromFile("vk.xml").expect("Не удалось загрузить файл. Failed to upload file.");

    let data_rc_ = Rc::new(data_vec_);

    // Создаем парсер.
    // Create a parser.
    let mut parser_ = utils__parser_svk__lib::Parser::s_create(data_rc_);
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