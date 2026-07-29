// SPDX-License-Identifier: None
// Copyright (c) 2026 None

use wvk::wvk::WVK_0_1_4_0;
use wvk::wvk_library::{ WvkLibraryBuilder };
use wvk::wvk_instance::{ WvkInstanceBuilder };

fn printVkPhysicalDeviceProperties(props_ref: &wvk::svk::svk_structures::VkPhysicalDeviceProperties) {
    let device_name_ = unsafe {
        std::ffi::CStr::from_ptr(props_ref.deviceName.as_ptr())
            .to_string_lossy()
            .into_owned()
    };

    let api_version_ = props_ref.apiVersion;
    let driver_version_ = props_ref.driverVersion;
    let vendor_id_ = props_ref.vendorID;
    let device_id_ = props_ref.deviceID;

    println!("deviceName: {}", device_name_);
    println!("apiVersion: {}", api_version_);
    println!("driverVersion: {}", driver_version_);
    println!("vendorID: {}", vendor_id_);
    println!("deviceID: {}", device_id_);

    println!("deviceType: {:?}", props_ref.deviceType);

    println!();
    println!("Limits:");
    println!("maxImageDimension1D: {}", props_ref.limits.maxImageDimension1D);
    println!("maxImageDimension2D: {}", props_ref.limits.maxImageDimension2D);
    println!("maxImageDimension3D: {}", props_ref.limits.maxImageDimension3D);
    println!("maxImageArrayLayers: {}", props_ref.limits.maxImageArrayLayers);
    println!("maxBoundDescriptorSets: {}", props_ref.limits.maxBoundDescriptorSets);

    println!();
    println!("Sparse properties:");
    println!(
        "residencyStandard2DBlockShape: {}",
        props_ref.sparseProperties.residencyStandard2DBlockShape
    );
    println!(
        "residencyStandard2DMultisampleBlockShape: {}",
        props_ref.sparseProperties.residencyStandard2DMultisampleBlockShape
    );
}

fn main() {
    println!("Пример получения списка физических устройств. Example of getting a list of physical devices.");

    let wvk_library_ = WvkLibraryBuilder::<WVK_0_1_4_0>::s_create().build().ok().unwrap();
    let wvk_instance_= WvkInstanceBuilder::<WVK_0_1_4_0>::s_create(&wvk_library_).build().ok().unwrap();

    let wvk_physical_devices_ = wvk_instance_.wvkEnumeratePhysicalDevices().ok().unwrap();

    println!("Пример получения свойств физических устройств через vkGetPhysicalDeviceProperties. An example of obtaining properties of physical devices via vkGetPhysicalDeviceProperties.");

    for wvk_physical_device_ in &wvk_physical_devices_ {
        let vk_properties_ = wvk_physical_device_.wvkGetPhysicalDeviceProperties();

        printVkPhysicalDeviceProperties(&vk_properties_);
    }

    println!("Пример получения свойств физических устройств через vkGetPhysicalDeviceProperties2. An example of obtaining properties of physical devices via vkGetPhysicalDeviceProperties2.");

    for wvk_physical_device_ in &wvk_physical_devices_ {
        let vk_properties_2_ = wvk_physical_device_.wvkGetPhysicalDeviceProperties2();

        printVkPhysicalDeviceProperties(&vk_properties_2_.properties);
    }

}