// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// зависимости
// dependencies
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use std::sync::Arc;
use utils__tokenizer_xml::{AVX2, Tokenizer};
use utils__tokenizer_xml::token::TokenType;
use crate::types::{Types, TypesElementVariant};
use crate::types_item_type::TypesItemTypeElementVariant;
use crate::type_body_with_enum::TypeBodyWithEnumElementVariant;
use crate::type_struct::TypeStructElementVariant;
use crate::enums::Enums;
use crate::commands::Commands;

macro_rules! snake_case {
    ($name:expr) => {{
        let mut result_ = String::with_capacity($name.len() + 8);

        for (i_, c_) in $name.chars().enumerate() {
            if c_.is_uppercase() {
                if i_ != 0 {
                    result_.push('_');
                }

                result_.extend(c_.to_lowercase());
            } else if c_.is_ascii_digit() {
                if i_ != 0 && !result_.ends_with('_') {
                    result_.push('_');
                }

                result_.push(c_);
            } else {
                result_.push(c_);
            }
        }

        result_
    }};
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
///
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
pub struct Registry {
    /// Данные.
    /// Data.
    pub(crate) data: Arc<Vec<u8>>,
    /// Токенайзер.
    /// Tokenizer.
    pub(crate) tokenizer: Tokenizer<AVX2>,
    pub(crate) typess: Vec<Types>,
    pub(crate) enums: Vec<Enums>,
    pub(crate) commandss: Vec<Commands>,
    //pub(crate) requires_cash: HashMap<u64, (usize, usize)>,
}

impl Registry {
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// Конструктор.
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn create() -> Self {
        Self {
            data: Arc::new(Vec::new()),
            tokenizer: Tokenizer::create(),
            typess: Vec::new(),
            enums: Vec::new(),
            commandss: Vec::new(),
            //requires_cash: HashMap::new(),
        }
    }
    
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// Конструктор.
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn createWithData(data: Arc<Vec<u8>>) -> Self {
        Self {
            data: data.clone(),
            tokenizer: Tokenizer::createWithData(data),
            typess: Vec::new(),
            enums: Vec::new(),
            commandss: Vec::new(),
            //registry_enums_vec: Vec::new(),
            //requires_cash: HashMap::new(),
        }
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// Установить новые данные.
    /// Set new data.
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn setData(&mut self, data: Arc<Vec<u8>>) {
        self.tokenizer.setData(data.clone());
        self.data = data;



    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn build(&mut self) -> Result<(), String> {
        loop {
            let token_ = self.tokenizer.nextToken1();

            // Ищем <types>
            if token_.asType() == TokenType::TAG_NAME && unsafe { token_.asStr(self.data.as_ptr()) } == "types" {
                let mut types_ = Types::create();
                types_.parse(&mut self.tokenizer, self.data.as_slice())?;

                self.typess.push(types_);
            }

            else if token_.asType() == TokenType::TAG_NAME && unsafe { token_.asStr(self.data.as_ptr()) } == "enums" {
                let mut enums_ = Enums::create();
                enums_.parse(&mut self.tokenizer, self.data.as_slice())?;

                self.enums.push(enums_);
            }

            else if token_.asType() == TokenType::TAG_NAME && unsafe { token_.asStr(self.data.as_ptr()) } == "enums" {
                let mut enums_ = Enums::create();
                enums_.parse(&mut self.tokenizer, self.data.as_slice())?;

                self.enums.push(enums_);
            }

            if token_.asType() == TokenType::INVALID {
                break;
            }
        }

        Ok(())
    }
}

impl Registry {
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    fn generateWvk(&self, data: &[u8], registry: &Registry) -> Result<String, String> {
        let mut output_wvk_ = String::new();

        let mut physical_device_properties_all_indices_ = Vec::<Vec<usize>>::new();

        registry.typess
            .iter()
            .enumerate()
            .for_each(|(id_types_, types_)| {
                let mut physical_device_properties_indices_ = Vec::<usize>::new();

                types_.elements
                    .iter()
                    .enumerate()
                    .for_each(|(id_type_, types_element_)|{
                        match &types_element_ {
                            TypesElementVariant::TYPE(type_) => {
                                match &type_.0 {
                                    TypesItemTypeElementVariant::BASE_TYPE(type_) => {
                                        let name_str_ = unsafe {std::str::from_utf8_unchecked(&data[*type_.type_body.type_body_name.name.start() ..= *type_.type_body.type_body_name.name.end()])};
                                        let type_str_ = if let Some(v) = type_.type_body.type_body_types.first() {
                                            unsafe {std::str::from_utf8_unchecked(&data[*v.r#type.start() ..= *v.r#type.end()])}
                                        }
                                        else {
                                            ""
                                        };

                                        //let type_str_ = unsafe {std::str::from_utf8_unchecked(&data[*type_.registry_type_body.registry_primitive_element_type_vec[0].type_rng.start() ..= *type_.registry_type_body.registry_primitive_element_type_vec[0].type_rng.end()])};
                                        //let comment_str_ = unsafe {std::str::from_utf8_unchecked(&data[*type_.registry_type_body..comment_rng.start() ..= *type_.type_body.comment_rng.end()])};

                                        let type_str_ = if type_str_ == "uint32_t" {
                                            "u32"
                                        }

                                        else if type_str_ == "uint64_t" {
                                            "u64"
                                        }

                                        else {
                                            //return;
                                            "*mut std::ffi::c_void"
                                        };

                                        output_wvk_.push_str(&format!("pub type {} = {}; // \n\n", name_str_, type_str_));
                                    }

                                    TypesItemTypeElementVariant::BITMASK(type_) => {
                                        let alias_str_ = unsafe {std::str::from_utf8_unchecked(&data[*type_.alias.start() ..= *type_.alias.end()])};
                                        let name_str_ = unsafe {std::str::from_utf8_unchecked(&data[*type_.name.start() ..= *type_.name.end()])};
                                        let name_body_str_ = unsafe {std::str::from_utf8_unchecked(&data[*type_.type_body.type_body_name.name.start() ..= *type_.type_body.type_body_name.name.end()])};

                                        let type_str_ = if let Some(v) = type_.type_body.type_body_types.first() {
                                            unsafe {std::str::from_utf8_unchecked(&data[*v.r#type.start() ..= *v.r#type.end()])}
                                        }
                                        else {
                                            ""
                                        };

                                        //let type_str_ = unsafe {std::str::from_utf8_unchecked(&data[*type_.registry_type_body.registry_primitive_element_type_vec[0].type_rng.start() ..= *type_.registry_type_body.registry_primitive_element_type_vec[0].type_rng.end()])};
                                        //let comment_str_ = unsafe {std::str::from_utf8_unchecked(&data[*type_.type_body.comment_rng.start() ..= *type_.type_body.comment_rng.end()])};

                                        if alias_str_.is_empty() {
                                            output_wvk_.push_str(&format!("#[derive(Copy, Clone, PartialEq)]\n"));
                                            output_wvk_.push_str(&format!("pub struct {}(pub {}); // \n", name_body_str_, type_str_));
                                            output_wvk_.push_str(&format!("impl std::ops::BitAnd for {} {{\n", name_body_str_));
                                            output_wvk_.push_str(&format!("\ttype Output = Self;\n\n"));
                                            output_wvk_.push_str(&format!("\tfn bitand(self, rhs: Self) -> Self {{\n"));
                                            output_wvk_.push_str(&format!("\t\treturn Self(self.0 & rhs.0);\n"));
                                            output_wvk_.push_str(&format!("\t}}\n"));
                                            output_wvk_.push_str(&format!("}}\n\n"));
                                        }

                                        else {
                                            output_wvk_.push_str(&format!("pub type {} = {}; // \n\n", name_str_, alias_str_));
                                        }
                                    }
                                    // #[repr(C)]
                                    // pub struct VkPhysicalDevice_T {
                                    //     _private: [u8; 0],
                                    // }
                                    TypesItemTypeElementVariant::HANDLE(type_) => {
                                        let name_str_ = unsafe {std::str::from_utf8_unchecked(&data[*type_.type_body.type_body_name.name.start() ..= *type_.type_body.type_body_name.name.end()])};
                                        let type_str_ = if let Some(v) = type_.type_body.type_body_types.first() {
                                            unsafe {std::str::from_utf8_unchecked(&data[*v.r#type.start() ..= *v.r#type.end()])}
                                        }
                                        else {
                                            ""
                                        };

                                        //let type_str_ = unsafe {std::str::from_utf8_unchecked(&data[*type_.registry_type_body.registry_primitive_element_type_vec[0].type_rng.start() ..= *type_.registry_type_body.registry_primitive_element_type_vec[0].type_rng.end()])};
                                        //let comment_str_ = unsafe {std::str::from_utf8_unchecked(&data[*type_.type_body.comment_rng.start() ..= *type_.type_body.comment_rng.end()])};

                                        if name_str_.is_empty() {
                                            return;
                                        }
                                        
                                        output_wvk_.push_str(&format!("#[repr(C)]\n"));
                                        output_wvk_.push_str(&format!("pub struct {} {{\n", name_str_));
                                        output_wvk_.push_str(&format!("\tprivate: [u8; 0]\n"));
                                        output_wvk_.push_str(&format!("}}\n"));
                                    }
                                    //physical_device_properties_structs_
                                    TypesItemTypeElementVariant::STRUCT(type_) => {
                                        let name_str_ = unsafe {std::str::from_utf8_unchecked(&data[*type_.name.start() ..= *type_.name.end()])};
                                        let struct_extends_str_ = unsafe {std::str::from_utf8_unchecked(&data[*type_.struct_extends.start() ..= *type_.struct_extends.end()])};

                                        if name_str_ == "VkLayerSettingEXT" {
                                            println!("VkLayerSettingEXT");
                                        }

                                        // Если структура является расширением для VkPhysicalDeviceProperties2. Запишем индексы в массив.
                                        // Дальше по этим структурам сгенерируем код для wvk_physical_device_x_properties::WvkPhysicalDeviceXProperties2.
                                        // If the structure is an extension of VkPhysicalDeviceProperties2, we'll store the indices in an array.
                                        // Next, we'll use these structures to generate code for wvk_physical_device_x_properties::WvkPhysicalDeviceXProperties2.
                                        if struct_extends_str_ == "VkPhysicalDeviceProperties2" {
                                            physical_device_properties_indices_.push(id_type_);
                                        }

                                        if name_str_.is_empty() {
                                            return;
                                        }
                                        output_wvk_.push_str(&format!("#[repr(C)]\n"));
                                        output_wvk_.push_str(&format!("pub struct {} {{\n", name_str_));

                                        type_.element_variants
                                            .iter()
                                            .for_each(|v| {
                                                match v {
                                                    TypeStructElementVariant::MEMBER(member_) => {
                                                        let name_str_ = unsafe {std::str::from_utf8_unchecked(&data[*member_.type_body_with_enum.type_body_with_enum_name.name.start() ..= *member_.type_body_with_enum.type_body_with_enum_name.name.end()])};
                                                        let type_str_ = unsafe {std::str::from_utf8_unchecked(&data[*member_.type_body_with_enum.type_body_with_enum_types[0].r#type.start() ..= *member_.type_body_with_enum.type_body_with_enum_types[0].r#type.end()])};
                                                        let prefix_str_ = unsafe {std::str::from_utf8_unchecked(&data[*member_.type_body_with_enum.type_body_with_enum_types[0].prefix.start() ..= *member_.type_body_with_enum.type_body_with_enum_types[0].prefix.end()])};
                                                        let postfix_str_ = unsafe {std::str::from_utf8_unchecked(&data[*member_.type_body_with_enum.type_body_with_enum_types[0].postfix.start() ..= *member_.type_body_with_enum.type_body_with_enum_types[0].postfix.end()])};

                                                        if name_str_ == "ppEnabledExtensionNames" {
                                                            println!("stope");
                                                        }

                                                        let prefix_str_ = if prefix_str_.starts_with("const") {
                                                            "const"
                                                        }
                                                        else { "" };

                                                        let postfix_str_ = if postfix_str_.find("* const*").is_some() {
                                                            "* const*"
                                                        }
                                                        else if postfix_str_.find("*").is_some() {
                                                            "*"
                                                        }
                                                        else { "" };

                                                        let prefix_rust_str = if prefix_str_ == "const" && postfix_str_ == "*" {
                                                            "*const "
                                                        }
                                                        else if prefix_str_ == "const" && postfix_str_ == "* const*" {
                                                            "*const *const"
                                                        }
                                                        else if prefix_str_ == "" && postfix_str_ == "*" {
                                                            "*mut "
                                                        }
                                                        else {
                                                            ""
                                                        };

                                                        let name_str_ = if name_str_ == "type" {
                                                            "r#type"
                                                        }

                                                        else {
                                                            name_str_
                                                        };

                                                        let type_str_ = if type_str_.starts_with("PFN_") {
                                                            &format!("crate::svk_commands::{}", type_str_)
                                                        }
                                                        else {
                                                            type_str_
                                                        };

                                                        let type_rust_str_ = if type_str_ == "void" {
                                                            "std::ffi::c_void"
                                                        }

                                                        else if type_str_ == "char" {
                                                            "i8"
                                                        }

                                                        else if type_str_ == "int8_t" {
                                                            "i8"
                                                        }

                                                        else if type_str_ == "uint8_t" {
                                                            "u8"
                                                        }

                                                        else if type_str_ == "int16_t" {
                                                            "i16"
                                                        }

                                                        else if type_str_ == "uint16_t" {
                                                            "u16"
                                                        }

                                                        else if type_str_ == "int32_t" {
                                                            "i32"
                                                        }

                                                        else if type_str_ == "uint32_t" {
                                                            "u32"
                                                        }

                                                        else if type_str_ == "int64_t" {
                                                            "i64"
                                                        }

                                                        else if type_str_ == "uint64_t" {
                                                            "u64"
                                                        }

                                                        else if type_str_ == "float" {
                                                            "f32"
                                                        }

                                                        else if type_str_ == "double" {
                                                            "f64"
                                                        }

                                                        else if type_str_ == "size_t" {
                                                            "usize"
                                                        }

                                                        else if type_str_ == "int" {
                                                            "i32"
                                                        }

                                                        else if type_str_ == "unsigned int" {
                                                            "u32"
                                                        }

                                                        else {
                                                            type_str_
                                                        };

                                                        let type_rust_str_ = if let Some(TypeBodyWithEnumElementVariant::ENUM(enum_)) = member_.type_body_with_enum.type_body_with_enum_element_variants.first() {
                                                            let enum_str_ = unsafe {std::str::from_utf8_unchecked(&data[*enum_.r#enum.start() ..= *enum_.r#enum.end()])};
                                                            let enum_str_ = format!("[{}; {} as usize]", type_rust_str_,enum_str_);
                                                            enum_str_
                                                        }

                                                        else {
                                                            type_rust_str_.to_string()
                                                        };

                                                        output_wvk_.push_str(&format!("\tpub {}: {} {},\n", name_str_, prefix_rust_str, type_rust_str_));
                                                    }

                                                    _ => {}
                                                }

                                            });

                                        output_wvk_.push_str(&format!("}}\n\n"));
                                    }

                                    _ => {}
                                }
                            }

                            TypesElementVariant::COMMENT_ELT(comment_elt_) => {

                            }
                        }
                    });

                physical_device_properties_all_indices_.push(physical_device_properties_indices_);
            });

        /*registry.registry_enums_vec
            .iter()
            .try_for_each(|enums_| -> Result<(), String> {
                let name_str = unsafe {std::str::from_utf8_unchecked(&data[*enums_.name_rng.start() ..= *enums_.name_rng.end()])};
                let type_str = unsafe {std::str::from_utf8_unchecked(&data[*enums_.type_rng.start() ..= *enums_.type_rng.end()])};
                let comment_str = unsafe {std::str::from_utf8_unchecked(&data[*enums_.comment_rng.start() ..= *enums_.comment_rng.end()])};

                if type_str == "enum" {

                    output_wvk_.push_str(&format!("pub struct {}(i32); //{}\n", name_str, comment_str));
                    output_wvk_.push_str(&format!("impl {} {{\n", name_str));

                    enums_.registry_enum_vec
                        .iter()
                        .for_each(|enum_| {
                            let alias_str_ = unsafe {std::str::from_utf8_unchecked(&data[*enum_.alias_rng.start() ..= *enum_.alias_rng.end()])};
                            let value_str_ = unsafe {std::str::from_utf8_unchecked(&data[*enum_.value_rng.start() ..= *enum_.value_rng.end()])};
                            let name_str_ = unsafe {std::str::from_utf8_unchecked(&data[*enum_.name_rng.start() ..= *enum_.name_rng.end()])};

                            if alias_str_.is_empty() {
                                output_wvk_.push_str(&format!("\tpub const {}: Self = Self({});\n", name_str_, value_str_));
                            }

                            else {
                                output_wvk_.push_str(&format!("\tpub const {}: Self = Self::{};\n", name_str_, alias_str_));
                            }
                        });

                    output_wvk_.push_str(&format!("}}\n\n"));
                }
                if type_str == "bitmask" {
                    let name_str__ = name_str.replace("FlagBits", "Flags");

                    output_wvk_.push_str(&format!("pub type {} = {}; //{}\n", name_str, name_str__, comment_str));
                    output_wvk_.push_str(&format!("impl {} {{\n", name_str));

                    let bitwidth_str_ = unsafe {std::str::from_utf8_unchecked(&data[*enums_.bitwidth_rng.start() ..= *enums_.bitwidth_rng.end()])};

                    if bitwidth_str_.is_empty() {
                        enums_.registry_enum_vec
                            .iter()
                            .try_for_each(|enum_| -> Result<(), String> {
                                let alias_str_ = unsafe {std::str::from_utf8_unchecked(&data[*enum_.alias_rng.start() ..= *enum_.alias_rng.end()])};
                                let bitpos_str_ = unsafe {std::str::from_utf8_unchecked(&data[*enum_.bitpos_rng.start() ..= *enum_.bitpos_rng.end()])};
                                let name_str_ = unsafe {std::str::from_utf8_unchecked(&data[*enum_.name_rng.start() ..= *enum_.name_rng.end()])};

                                let bitpos_ = if bitpos_str_.is_empty() {
                                    0
                                }

                                else {
                                    let bitpos_ = bitpos_str_.parse::<u32>()
                                        .map_err(|e| format!("{}", e))?;

                                    1u32 << bitpos_
                                };

                                if alias_str_.is_empty() {
                                    output_wvk_.push_str(&format!("\tpub const {}: {} = {}({});\n", name_str_, name_str__, name_str__, bitpos_));
                                }

                                else {
                                    output_wvk_.push_str(&format!("\tpub const {}: {} = Self::{};\n", name_str_, name_str__, alias_str_));
                                }

                                Ok(())
                            })?;
                    }

                    else if bitwidth_str_ == "64" {
                        enums_.registry_enum_vec
                            .iter()
                            .try_for_each(|enum_| -> Result<(), String> {
                                let alias_str_ = unsafe {std::str::from_utf8_unchecked(&data[*enum_.alias_rng.start() ..= *enum_.alias_rng.end()])};
                                let bitpos_str_ = unsafe {std::str::from_utf8_unchecked(&data[*enum_.bitpos_rng.start() ..= *enum_.bitpos_rng.end()])};
                                let name_str_ = unsafe {std::str::from_utf8_unchecked(&data[*enum_.name_rng.start() ..= *enum_.name_rng.end()])};

                                let bitpos_ = if bitpos_str_.is_empty() {
                                    0
                                }

                                else {
                                    let bitpos_ = bitpos_str_.parse::<u64>()
                                        .map_err(|e| format!("{}", e))?;

                                    1u64 << bitpos_
                                };

                                if alias_str_.is_empty() {
                                    output_wvk_.push_str(&format!("\tpub const {}: {} = {}({});\n", name_str_, name_str__, name_str__, bitpos_));
                                }

                                else {
                                    output_wvk_.push_str(&format!("\tpub const {}: {} = Self::{};\n", name_str_, name_str__, alias_str_));
                                }

                                Ok(())
                            })?;
                    }

                    output_wvk_.push_str(&format!("}}\n\n"));
                }

                Ok(())
            })?;

        self.generateWvkPhysicalDeviceXProperties2(registry, physical_device_properties_all_indices_);*/

        Ok(output_wvk_)
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// Функция генерирует содержимое для:
    /// struct WvkPhysicalDeviceXProperties2 {
    ///     ...
    /// }
    /// и
    /// pub fn create() -> Self {
    ///     Self {
    ///         ...
    ///     }
    /// }
    ///
    /// This function generates content for:
    /// struct WvkPhysicalDeviceXProperties2 {
    ///     ...
    /// }
    /// and
    /// pub fn create() -> Self {
    ///     Self {
    ///         ...
    ///     }
    /// }
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    fn generateWvkPhysicalDeviceXProperties2(&self, registry: &Registry, indices: Vec::<Vec<usize>>) {
        let mut struct_field_output_ = String::new();       // Содержимое структуры WvkPhysicalDeviceXProperties2.
        let mut struct_create_output_ = String::new();      // Содержимое функции WvkPhysicalDeviceXProperties2::create().

        // Итерируем индексы для RegistryTypes
        /*indices
            .iter()
            .enumerate()
            .for_each(|(id_types_, types_)| {
                // Итерируем индексы для RegistryType
                types_
                    .iter()
                    .for_each(|id_type_| {
                        let types_element_ = &registry.registry_types_vec[id_types_].registry_types_element_variant_vec[*id_type_];

                        // Индексы формируются извне и указывают конкретно на RegistryTypesElement::TYPE -> RegistryTypeType::TYPE_STRUCT.
                        match &types_element_ {
                            RegistryTypesElementVariant::TYPE(type_) => {
                                match &type_.registry_type_element_variant {
                                    RegistryTypeElementVariant::STRUCT(type_) => {
                                        let name_ = unsafe {std::str::from_utf8_unchecked(&self.data.as_slice()[*type_.name_rng.start() ..= *type_.name_rng.end()])};
                                        let name1_ = name_;
                                        let mut name_field_ = String::with_capacity(name_.len() * 2);

                                        // Преобразуем название структуры в имя для поля структуры. VkStructName10EXT -> vk_struct_name_10_ext

                                        // Префиксы.

                                        let (name_, prefix_) = if let Some(v) = name_.strip_prefix("Vk") {
                                            (v, "vk")
                                        }

                                        else {
                                            (name_, "")
                                        };

                                        // Суффиксы.

                                        let (name_, suffix_) = if let Some(v) = name_.strip_suffix("KHR") {
                                            (v, "_khr")
                                        }

                                        else if let Some(v) = name_.strip_suffix("EXT") {
                                            (v, "_ext")
                                        }

                                        else if let Some(v) = name_.strip_suffix("NV") {
                                            (v, "_nv")
                                        }

                                        else if let Some(v) = name_.strip_suffix("AMD") {
                                            (v, "_amd")
                                        }

                                        else if let Some(v) = name_.strip_suffix("VALVE") {
                                            (v, "_valve")
                                        }

                                        else if let Some(v) = name_.strip_suffix("GOOGLE") {
                                            (v, "_google")
                                        }

                                        else if let Some(v) = name_.strip_suffix("ANDROID") {
                                            (v, "_android")
                                        }

                                        else if let Some(v) = name_.strip_suffix("MSFT") {
                                            (v, "_msft")
                                        }

                                        else if let Some(v) = name_.strip_suffix("MESA") {
                                            (v, "_mesa")
                                        }

                                        else if let Some(v) = name_.strip_suffix("ARM") {
                                            (v, "_arm")
                                        }

                                        else if let Some(v) = name_.strip_suffix("QCOM") {
                                            (v, "_qcom")
                                        }

                                        else if let Some(v) = name_.strip_suffix("NVX") {
                                            (v, "_nvx")
                                        }

                                        else if let Some(v) = name_.strip_suffix("AMDX") {
                                            (v, "_amdx")
                                        }

                                        else if let Some(v) = name_.strip_suffix("OHOS") {
                                            (v, "_ohos")
                                        }

                                        else {
                                            (name_, "")
                                        };

                                        // Добавляем префикс.

                                        name_field_.push_str(prefix_);

                                        // Итерируем имя и заменяем [uppercase] на ['_' + lowercase]

                                        let mut chars_ = name_.chars().peekable();

                                        while let Some(v) = chars_.next() {
                                            if v == 'P'
                                                && chars_.next_if_eq(&'C').is_some()
                                                && chars_.next_if_eq(&'I').is_some() {

                                                name_field_.push_str("_pci");
                                            }

                                            else if v == 'I'
                                                && chars_.next_if_eq(&'D').is_some() {

                                                name_field_.push_str("_id");
                                            }

                                            else if v.is_ascii_uppercase() {
                                                name_field_.push('_');
                                                name_field_.push(v.to_ascii_lowercase());
                                            }

                                            else if v.is_ascii_digit() {
                                                name_field_.push('_');
                                                name_field_.push(v);
                                            }

                                            else {
                                                name_field_.push(v);
                                            }
                                        } // while let Some(v) = chars_.next() {

                                        name_field_.push_str(suffix_);

                                        match &type_.element_variant_vec[0] {
                                            RegistryTypeStructElementVariant::MEMBER(member_) => {
                                                let values_member_ = unsafe {std::str::from_utf8_unchecked(&self.data.as_slice()[*member_.values_rng.start() ..= *member_.values_rng.end()])};

                                                struct_field_output_.push_str(&format!("\tpub {}: Option<MaybeUninit<svk::{}>>,\n", name_field_, name1_));
                                                struct_create_output_.push_str(&format!("\t\t\t{}: Some(maybe_init!(svk::{}, svk::VkStructureType::{})),\n", name_field_, name1_, values_member_));
                                            }

                                            _ => {}
                                        }
                                    }

                                    _ => {}
                                }
                            }
                            _ => {}

                        } // match &types_element_ {
                    }); // .for_each(|id_type_| {
            }); // .for_each(|(id_types_, types_)| {

        println!("asdasdas");*/
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    fn generatingExtensionFiles(&self, registry: &Registry) {
        
    }
}

