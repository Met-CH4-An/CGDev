// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// зависимости
// dependencies
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use std::marker::PhantomData;
use crate::wvk::{ WvkEnvironment, WvkEnvironment_0_1_0_0 } ;
use crate::wvk_error::{ WvkError, WvkErrorType };
use crate::wvk_instance::WvkInstanceBuilder;

//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
///
//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
pub struct WvkInstance<TWvkEnvironment>
where TWvkEnvironment : WvkEnvironment {
    pub(crate) phantom_data: PhantomData<TWvkEnvironment>,

    // инстанс вулкана
    // volcano instance
    pub(crate) vk_instance : svk::VkInstance,

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // команды вулкана
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

    // vulkan 1.0
    
    // vulkan 1.1
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// приватная область
// private area
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

