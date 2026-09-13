// SPDX-License-Identifier: None
// Copyright (c) 2026 None

use crate::wvk::WVK_0_1_0_0;
use crate::dispatch_table::{WVK_DISPATCH_TABLE_GLOBAL, WVK_DISPATCH_TABLE_INSTANCE};
use crate::dispatch_table::wvk_dispatch_table_builder::WvkDispatchTableBuilder;

/*#[test]
fn wvk_dispatch_table_global__create() {
    let result_ = WvkDispatchTableBuilder::<WVK_0_1_0_0, WVK_DISPATCH_TABLE_GLOBAL>::s_create().build();
    
    if let Err(error_) = result_ {
        panic!("{}", error_.getMessage());
    }
    
}

#[test]
fn wvk_dispatch_table_instance__create() {
    let wvk_dispatch_table_global_ = WvkDispatchTableBuilder::<WVK_0_1_0_0, WVK_DISPATCH_TABLE_GLOBAL>::s_create().build().unwrap();

    // в вулкане можно описать своё приложение через VkApplicationInfo
    // In Vulkan, you can describe your application using VkApplicationInfo
    let vk_application_info_ = svk::VkApplicationInfo {
        sType : svk::VkStructureType::VK_STRUCTURE_TYPE_APPLICATION_INFO,
        pNext : std::ptr::null_mut(),
        pApplicationName : std::ptr::null(),
        applicationVersion : 0,
        pEngineName : std::ptr::null(),
        engineVersion : 0,
        apiVersion : 0,
    };

    // для создания VkInstance описываем его через VkInstanceCreateInfo
    // to create a VkInstance, we describe it using VkInstanceCreateInfo
    let vk_create_info_ = svk::VkInstanceCreateInfo {
        sType : svk::VkStructureType::VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
        pNext : std::ptr::null_mut(),
        flags : svk::VkInstanceCreateFlags(0),
        pApplicationInfo : &vk_application_info_,
        enabledLayerCount : 0,
        ppEnabledLayerNames : std::ptr::null(),
        enabledExtensionCount : 0,
        ppEnabledExtensionNames : std::ptr::null(),
    };

    let mut vk_instance_: svk::VkInstance = std::ptr::null_mut();
    let vk_result_ = wvk_dispatch_table_global_.vkCreateInstance(&vk_create_info_, std::ptr::null_mut(), &mut vk_instance_);

    if vk_result_ != svk::VkResult::VK_SUCCESS {
        panic!("Не удалось выполнить WvkDispatchTableBuilder::<WVK_0_1_0_0, WVK_DISPATCH_TABLE_GLOBAL>::vkCreateInstance.\n
        Failed to execute WvkDispatchTableBuilder::<WVK_0_1_0_0, WVK_DISPATCH_TABLE_GLOBAL>::vkCreateInstance.\n
        VkResult = {:?}", vk_result_);
    }

    let wvk_dispatch_table_instance_ = WvkDispatchTableBuilder::<WVK_0_1_0_0, WVK_DISPATCH_TABLE_INSTANCE>::s_create(vk_instance_, &wvk_dispatch_table_global_).build();

    if let Err(error_) = wvk_dispatch_table_instance_ {
        panic!("{}", error_.getMessage());
    }

    wvk_dispatch_table_instance_.unwrap().vkDestroyInstance(vk_instance_, std::ptr::null());
}*/