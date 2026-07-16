// SPDX-License-Identifier: None
// Copyright (c) 2026 None

use std::marker::PhantomData;
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// зависимости
// dependencies
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
///
//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
pub struct WvkInstance<TWvkVersion> {
    pub(crate) phantom_data: PhantomData<TWvkVersion>,

    // инстанс вулкана
    // volcano instance
    pub(crate) vk_instance : svk::VkInstance,

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // команды вулкана
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

    // vulkan 1.0
    
    // vulkan 1.1
}

