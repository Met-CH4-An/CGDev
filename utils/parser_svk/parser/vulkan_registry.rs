// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// зависимости
// dependencies
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use std::collections::HashMap;
use std::rc::Rc;
use crate::parser::vulkan_registry_enums::VulkanRegistryEnums;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
///
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
pub struct VulkanRegistry {
    /// Блок с константами.
    ///
    //constants
    /// Перечисления вулкана.
    /// Volcano listings.
    pub vulkan_enums_hmap: HashMap<u64, VulkanRegistryEnums>,
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
impl VulkanRegistry {
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // Публичные ассоциированные функции.
    // Public associated functions.
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn s_create() -> Self {
        Self{
            vulkan_enums_hmap: HashMap::<u64, VulkanRegistryEnums>::new(),
        }
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // Публичные методы.
    // Public methods.
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

    pub fn print(&self, data: Rc<Vec<u8>>) {
        let data_ptr_ = data.as_slice();
        let mut output_ = String::new();

        for enums_ in self.vulkan_enums_hmap.values() {
            let name_enums_str = unsafe { std::str::from_utf8_unchecked(&data_ptr_[*enums_.name_rng.start() ..=* enums_.name_rng.end()]) };
            let type_str = unsafe { std::str::from_utf8_unchecked(&data_ptr_[*enums_.type_rng.start() ..=* enums_.type_rng.end()]) };

            //extends="VkDescriptorUpdateTemplateType" name="VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_PUSH_DESCRIPTORS"
            if name_enums_str == "VkDescriptorUpdateTemplateType" {
                println!("asdasd");
                println!("asdasd");
                println!("asdasd");
                println!("asdasd");
            }



            if type_str == "constants" {
                for enum_ in &enums_.enums_enum_vec {
                    let name_str_ = unsafe { std::str::from_utf8_unchecked(&data_ptr_[*enum_.name_rng.start()..=*enum_.name_rng.end()]) };
                    let value_str_ = unsafe { std::str::from_utf8_unchecked(&data_ptr_[*enum_.value_rng.start()..=*enum_.value_rng.end()]) };
                    let type_str_ = unsafe { std::str::from_utf8_unchecked(&data_ptr_[*enum_.type_rng.start()..=*enum_.type_rng.end()]) };

                    let value_str_ = match value_str_ {
                        "(~0U)" => "u32::MAX",
                        "(~1U)" => "!1u32",
                        "(~2U)" => "!2u32",
                        "(~0ULL)" => "u64::MAX",
                        _ => value_str_
                    };

                    let type_str_ = match type_str_ {
                        "uint64_t" => "u64",
                        "uint32_t" => "u32",
                        "float" => "f32",
                        _ => type_str_
                    };

                    let output_str_ = &format!("pub const {}: {} = {};\n", name_str_, type_str_, value_str_);

                    output_.push_str(output_str_);
                }

                output_.push_str("\n\n");
            } // if type_str == "constants"

            if type_str == "enum" {
                output_.push_str(&format!("pub type {} = i32;\n", name_enums_str));
                output_.push_str(&format!("pub mod {}Value {{\n", name_enums_str));
                output_.push_str(&format!("\tuse crate::{};\n", name_enums_str));

                for enum_ in &enums_.enums_enum_vec {
                    let name_str_ = unsafe { std::str::from_utf8_unchecked(&data_ptr_[*enum_.name_rng.start()..=*enum_.name_rng.end()]) };
                    let value_str_ = unsafe { std::str::from_utf8_unchecked(&data_ptr_[*enum_.value_rng.start()..=*enum_.value_rng.end()]) };
                    let alias_str_ = unsafe { std::str::from_utf8_unchecked(&data_ptr_[*enum_.alias_rng.start()..=*enum_.alias_rng.end()]) };
                    let comment_str_ = unsafe { std::str::from_utf8_unchecked(&data_ptr_[*enum_.comment_rng.start()..=*enum_.comment_rng.end()]) };

                    let value_str_ = if alias_str_ != "<" {
                        alias_str_
                    } else {
                        value_str_
                    };

                    let output_str_ = &format!("\tpub const {}: {} = {};\n", name_str_, name_enums_str, value_str_);

                    output_.push_str(output_str_);
                }

                for enum_ex_ in &enums_.enums_enum_ex_vec {
                    let name_str = unsafe { std::str::from_utf8_unchecked(&data_ptr_[*enum_ex_.name_rng.start()..=*enum_ex_.name_rng.end()]) };
                    let extnumber_str = unsafe { std::str::from_utf8_unchecked(&data_ptr_[*enum_ex_.extnumber_rng.start()..=*enum_ex_.extnumber_rng.end()]) };
                    let offset_str = unsafe { std::str::from_utf8_unchecked(&data_ptr_[*enum_ex_.offset_rng.start()..=*enum_ex_.offset_rng.end()]) };
                    let dir_str = unsafe { std::str::from_utf8_unchecked(&data_ptr_[*enum_ex_.dir_rng.start()..=*enum_ex_.dir_rng.end()]) };
                    let comment_str = unsafe { std::str::from_utf8_unchecked(&data_ptr_[*enum_ex_.comment_rng.start()..=*enum_ex_.comment_rng.end()]) };

                    println!("{}", name_str);

                    if extnumber_str == "<" { continue; }
                    if offset_str == "<" { continue; }

                    let extnumber_: u64 = extnumber_str.parse().expect("asdasd");
                    let offset_: u64 = offset_str.parse().expect("asdasd");

                    let value_ = if dir_str == "-" {
                        1_000_000_000 + (extnumber_ - 1) * 1_000 - offset_
                    } else {
                        1_000_000_000 + (extnumber_ - 1) * 1_000 + offset_
                    };

                    let output_str_ = &format!("\tpub const {}: {} = {};\n", name_str, name_enums_str, value_);

                    output_.push_str(output_str_);
                }

                output_.push_str("}\n\n");
            } // if type_str == "constants"
        } // for enums_

        print!("asdas");
        print!("asdas");
        print!("asdas");
        print!("asdas");
        print!("asdas");
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn addEnums() {

    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // Приватные ассоциированные функции.
    // Private associated functions.
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // Приватные методы.
    // Private methods.
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
}
