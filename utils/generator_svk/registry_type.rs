// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// зависимости
// dependencies
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use crate::type_base_type::RegistryTypeBaseType;
use crate::registry_type_bitmask::RegistryTypeBitmask;
use crate::registry_type_define::RegistryTypeDefine;
use crate::registry_type_enum::RegistryTypeEnum;
use crate::registry_type_funcpointer::RegistryTypeFuncpointer;
use crate::registry_type_handle::RegistryTypeHandle;
use crate::registry_type_include::RegistryTypeInclude;
use crate::registry_type_requires::RegistryTypeRequires;
use crate::registry_type_struct::RegistryTypeStruct;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// Type =
///     element type {
///         TypeBasetype
///       | TypeBitmask
///       | TypeDefine
///       | TypeEnum
///       | TypeFuncpointer
///       | TypeHandle
///       | TypeInclude
///       | TypeRequires
///       | TypeStruct
///     }
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
pub(crate) enum RegistryTypeElementVariant{
    UNKNOWN,
    BASE_TYPE(RegistryTypeBaseType),
    BITMASK(RegistryTypeBitmask),
    DEFINE(RegistryTypeDefine),
    ENUM(RegistryTypeEnum),
    FUNC_POINTER(RegistryTypeFuncpointer),
    HANDLE(RegistryTypeHandle),
    INCLUDE(RegistryTypeInclude),
    REQUIRES(RegistryTypeRequires),
    STRUCT(RegistryTypeStruct),
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// Type =
///     element type {
///         TypeBasetype
///       | TypeBitmask
///       | TypeDefine
///       | TypeEnum
///       | TypeFuncpointer
///       | TypeHandle
///       | TypeInclude
///       | TypeRequires
///       | TypeStruct
///     }
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
pub(crate) struct RegistryType {
    pub(crate) registry_type_element_variant: RegistryTypeElementVariant,
}

impl RegistryType {
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// Конструктор.
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub(crate) fn create() -> Self {
        Self {
            registry_type_element_variant: RegistryTypeElementVariant::UNKNOWN,
        }
    }
}