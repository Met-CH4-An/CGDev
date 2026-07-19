// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// зависимости
// dependencies
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use std::marker::PhantomData;
use crate::wvk::WvkEnvironment;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
///
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
pub struct WvkLibrary<TWvkEnvironment>
where TWvkEnvironment : WvkEnvironment {
    pub(in crate::wvk_library) phantom : PhantomData<TWvkEnvironment>,

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // команды вулкана
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    
    // vulkan 1.0
    pub(in crate::wvk_library) vkGetInstanceProcAddr : svk::PFN_vkGetInstanceProcAddr,
    pub(in crate::wvk_library) vkEnumerateInstanceLayerProperties : svk::PFN_vkEnumerateInstanceLayerProperties,
    pub(in crate::wvk_library) vkEnumerateInstanceExtensionProperties : svk::PFN_vkEnumerateInstanceExtensionProperties,
    pub(in crate::wvk_library) vkCreateInstance : svk::PFN_vkCreateInstance,
    
    // vulkan 1.1
    pub(in crate::wvk_library) vkEnumerateInstanceVersion : svk::PFN_vkEnumerateInstanceVersion,
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// приватная область
// private area
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~



/*#[cfg(test)]
mod tests {
    use crate::wvk;
    use crate::wvk::WvkEnvironment_0_1_4_0;
    use super::*;

    #[test]
    fn wvk_library__create__ok() {
        let wvk_library_ = WvkLibrary::<wvk::WVK_0_1_4_0>::create().ok();
        assert!(wvk_library_.is_some());
    }

    #[test]
    fn wvk_library__wvkEnumerateInstanceExtensionProperties__null_byte_parameter() {
        let wvk_library_ = WvkLibrary::<wvk::WVK_0_1_4_0>::create().ok().unwrap();

        let wvk_error_ = wvk_library_.wvkEnumerateInstanceExtensionProperties(Some("layer\0name")).err();
        println!("{}", wvk_error_.as_ref().unwrap().getMessage());
        assert!(wvk_error_.is_some(), "message");
    }
}*/