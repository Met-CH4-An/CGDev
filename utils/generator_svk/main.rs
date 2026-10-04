// SPDX-License-Identifier: None
// Copyright (c) 2026 None

#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// зависимости
// dependencies
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use std::sync::Arc;
use utils__generator_svk__lib::{Registry, TypesItemTypeItemStructView};
use utils__generator_svk__lib::{
    TypesViewVariant,
    TypesItemTypeViewVariant
};

fn main() {
    // Загружаем данные.
    // Loading data.
    let data_vec_ = loadDataFromFile("vk.xml").expect("Не удалось загрузить файл. Failed to upload file.");

    let data_rc_ = Arc::new(data_vec_);

    // Создаем генератор.
    // Create a generator.
    let mut registry_ = Registry::create();

    registry_.setData(data_rc_);
    
    // Получаем сгенерированный svk.
    // Get the generated svk.
    registry_.build().unwrap();

    svk_generating(registry_);
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

fn svk_generating(registry: Registry) {
    registry.types()
        .iter()
        .for_each(|types_view_| {
            println!("{}", types_view_.comment());
            types_view_.content()
                .iter()
                .for_each(|v| {
                    match v {
                        TypesViewVariant::TYPE(item_type_) => {
                            match &item_type_.variant() {
                                TypesItemTypeViewVariant::STRUCT(item_struct_) => {
                                    svk_struct_generating(item_struct_);
                                }

                                _ => {}
                            }

                        }
                        _ => {}
                    }
                })
        });
}

fn svk_struct_generating(item_struct: &TypesItemTypeItemStructView) {
    item_struct.name()
}

