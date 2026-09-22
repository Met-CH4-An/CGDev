// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// зависимости
// dependencies
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use crate::registry_type_base_type::RegistryTypeBaseType;
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
pub(crate) enum RegistryTypeElementDeclaration{
    UNKNOWN,
    TYPE_BASE_TYPE(RegistryTypeBaseType),
    TYPE_BITMASK(RegistryTypeBitmask),
    TYPE_DEFINE(RegistryTypeDefine),
    TYPE_ENUM(RegistryTypeEnum),
    TYPE_FUNC_POINTER(RegistryTypeFuncpointer),
    TYPE_HANDLE(RegistryTypeHandle),
    TYPE_INCLUDE(RegistryTypeInclude),
    TYPE_REQUIRES(RegistryTypeRequires),
    TYPE_STRUCT(RegistryTypeStruct),
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
    pub(crate) registry_type_element_declaration: RegistryTypeElementDeclaration,
}

impl RegistryType {
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// Конструктор.
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub(crate) fn create() -> Self {
        Self {
            registry_type_element_declaration: RegistryTypeElementDeclaration::UNKNOWN,
        }
    }
}