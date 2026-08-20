// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// зависимости
// dependencies
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use std::rc::Rc;
use crate::parser::binding_constant::BindingConstant;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// example:
/// // comment
/// pub const NAME: type = value; // comment
/// pub const NAME: type = value; // comment
/// pub const NAME: type = value; // comment
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
pub struct BindingConstants {
    /// Массив с описанием констант.
    /// Array with description of constants.
    pub(in crate::parser) binding_constant_vec: Vec<BindingConstant>,
    /// Дополнительный комментарий для печати.
    /// Additional comment for printing.
    pub(in crate::parser) comment: String,
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
impl BindingConstants {
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // Публичные ассоциированные функции.
    // Public associated functions.
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// Конструктор.
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub(in crate::parser) fn s_create() -> Self {
        Self {
            binding_constant_vec: Vec::new(),
            comment: String::new(),
        }
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // Публичные методы.
    // Public methods.
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    //
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub unsafe fn appendsString(&self, data: &[u8], output: &mut String) {
        for binding_constant_ in &self.binding_constant_vec {
            binding_constant_.appendsString(data, output);
        }

        output.push_str("\n\n");
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