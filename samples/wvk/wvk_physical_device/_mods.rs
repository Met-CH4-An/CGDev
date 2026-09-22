// SPDX-License-Identifier: None
// Copyright (c) 2026 None

use wvk::wvk::{WVK_0_1_1_0, WVK_0_1_4_0};
use wvk::wvk_library::{ WvkLibraryBuilder };
use wvk::wvk_instance::{ WvkInstanceBuilder };

fn printVkPhysicalDeviceProperties(props_ref: &wvk::svk::VkPhysicalDeviceProperties) {
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

    //println!("deviceType: {:?}", props_ref.deviceType);

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

pub fn printVkPhysicalDeviceVulkan11Properties(properties: &wvk::svk::VkPhysicalDeviceVulkan11Properties) {
    println!("VkPhysicalDeviceVulkan11Properties:");
    //println!("  sType: {:?}", properties.sType);
    //println!("  pNext: {:?}", properties.pNext);

    println!("  deviceUUID: {:?}", properties.deviceUUID);
    println!("  driverUUID: {:?}", properties.driverUUID);
    println!("  deviceLUID: {:?}", properties.deviceLUID);
    println!("  deviceNodeMask: {}", properties.deviceNodeMask);
    println!("  deviceLUIDValid: {}", properties.deviceLUIDValid);

    println!(
        "  subgroupSize: {}",
        properties.subgroupSize
    );

    //println!(
    //    "  subgroupSupportedStages: {:?}",
    //    properties.subgroupSupportedStages
    //);

    //println!(
    //    "  subgroupSupportedOperations: {:?}",
    //    properties.subgroupSupportedOperations
    //);

    println!(
        "  subgroupQuadOperationsInAllStages: {}",
        properties.subgroupQuadOperationsInAllStages
    );

    //println!(
    //    "  pointClippingBehavior: {:?}",
    //    properties.pointClippingBehavior
    //);

    println!(
        "  maxMultiviewViewCount: {}",
        properties.maxMultiviewViewCount
    );

    println!(
        "  maxMultiviewInstanceIndex: {}",
        properties.maxMultiviewInstanceIndex
    );

    println!(
        "  protectedNoFault: {}",
        properties.protectedNoFault
    );

    println!(
        "  maxPerSetDescriptors: {}",
        properties.maxPerSetDescriptors
    );

    println!(
        "  maxMemoryAllocationSize: {}",
        properties.maxMemoryAllocationSize
    );
}

pub fn printVkPhysicalDeviceVulkan12Properties(properties: &wvk::svk::VkPhysicalDeviceVulkan12Properties) {
    println!("VkPhysicalDeviceVulkan12Properties:");

    //println!("  driverID: {:?}", properties.driverID);

    println!("  driverName: {:?}", properties.driverName);
    println!("  driverInfo: {:?}", properties.driverInfo);

    //println!(
    //    "  conformanceVersion: {:?}",
    //    properties.conformanceVersion
    //);

    //println!(
    //    "  denormBehaviorIndependence: {:?}",
    //    properties.denormBehaviorIndependence
    //);

    //println!(
    //    "  roundingModeIndependence: {:?}",
    //    properties.roundingModeIndependence
    //);

    println!(
        "  shaderSignedZeroInfNanPreserveFloat16: {}",
        properties.shaderSignedZeroInfNanPreserveFloat16
    );

    println!(
        "  shaderSignedZeroInfNanPreserveFloat32: {}",
        properties.shaderSignedZeroInfNanPreserveFloat32
    );

    println!(
        "  shaderSignedZeroInfNanPreserveFloat64: {}",
        properties.shaderSignedZeroInfNanPreserveFloat64
    );

    println!(
        "  shaderDenormPreserveFloat16: {}",
        properties.shaderDenormPreserveFloat16
    );

    println!(
        "  shaderDenormPreserveFloat32: {}",
        properties.shaderDenormPreserveFloat32
    );

    println!(
        "  shaderDenormPreserveFloat64: {}",
        properties.shaderDenormPreserveFloat64
    );

    println!(
        "  shaderDenormFlushToZeroFloat16: {}",
        properties.shaderDenormFlushToZeroFloat16
    );

    println!(
        "  shaderDenormFlushToZeroFloat32: {}",
        properties.shaderDenormFlushToZeroFloat32
    );

    println!(
        "  shaderDenormFlushToZeroFloat64: {}",
        properties.shaderDenormFlushToZeroFloat64
    );

    println!(
        "  shaderRoundingModeRTEFloat16: {}",
        properties.shaderRoundingModeRTEFloat16
    );

    println!(
        "  shaderRoundingModeRTEFloat32: {}",
        properties.shaderRoundingModeRTEFloat32
    );

    println!(
        "  shaderRoundingModeRTEFloat64: {}",
        properties.shaderRoundingModeRTEFloat64
    );

    println!(
        "  shaderRoundingModeRTZFloat16: {}",
        properties.shaderRoundingModeRTZFloat16
    );

    println!(
        "  shaderRoundingModeRTZFloat32: {}",
        properties.shaderRoundingModeRTZFloat32
    );

    println!(
        "  shaderRoundingModeRTZFloat64: {}",
        properties.shaderRoundingModeRTZFloat64
    );

    println!(
        "  maxPerStageDescriptorUpdateAfterBindInputAttachments: {}",
        properties.maxPerStageDescriptorUpdateAfterBindInputAttachments
    );

    println!(
        "  maxPerStageUpdateAfterBindResources: {}",
        properties.maxPerStageUpdateAfterBindResources
    );

    println!(
        "  maxDescriptorSetUpdateAfterBindSamplers: {}",
        properties.maxDescriptorSetUpdateAfterBindSamplers
    );

    println!(
        "  maxDescriptorSetUpdateAfterBindUniformBuffers: {}",
        properties.maxDescriptorSetUpdateAfterBindUniformBuffers
    );

    println!(
        "  maxDescriptorSetUpdateAfterBindUniformBuffersDynamic: {}",
        properties.maxDescriptorSetUpdateAfterBindUniformBuffersDynamic
    );

    println!(
        "  maxDescriptorSetUpdateAfterBindStorageBuffers: {}",
        properties.maxDescriptorSetUpdateAfterBindStorageBuffers
    );

    println!(
        "  maxDescriptorSetUpdateAfterBindStorageBuffersDynamic: {}",
        properties.maxDescriptorSetUpdateAfterBindStorageBuffersDynamic
    );

    println!(
        "  maxDescriptorSetUpdateAfterBindSampledImages: {}",
        properties.maxDescriptorSetUpdateAfterBindSampledImages
    );

    println!(
        "  maxDescriptorSetUpdateAfterBindStorageImages: {}",
        properties.maxDescriptorSetUpdateAfterBindStorageImages
    );

    println!(
        "  maxDescriptorSetUpdateAfterBindInputAttachments: {}",
        properties.maxDescriptorSetUpdateAfterBindInputAttachments
    );

    //println!(
    //    "  supportedDepthResolveModes: {:?}",
    //    properties.supportedDepthResolveModes
    //);

    //println!(
    //    "  supportedStencilResolveModes: {:?}",
    //    properties.supportedStencilResolveModes
    //);

    println!(
        "  independentResolveNone: {}",
        properties.independentResolveNone
    );

    println!(
        "  independentResolve: {}",
        properties.independentResolve
    );

    println!(
        "  filterMinmaxSingleComponentFormats: {}",
        properties.filterMinmaxSingleComponentFormats
    );

    println!(
        "  filterMinmaxImageComponentMapping: {}",
        properties.filterMinmaxImageComponentMapping
    );

    println!(
        "  maxTimelineSemaphoreValueDifference: {}",
        properties.maxTimelineSemaphoreValueDifference
    );

    //println!(
    //    "  framebufferIntegerColorSampleCounts: {:?}",
    //    properties.framebufferIntegerColorSampleCounts
    //);
}

fn main() {
    println!("Пример получения списка физических устройств. Example of getting a list of physical devices.");

    let wvk_library_ = WvkLibraryBuilder::<WVK_0_1_4_0>::create().build().ok().unwrap();
    let wvk_instance_= WvkInstanceBuilder::<WVK_0_1_4_0>::create(&wvk_library_).build().ok().unwrap();

    let wvk_physical_devices_ = wvk_instance_.wvkEnumeratePhysicalDevices().ok().unwrap();

    println!("Пример получения свойств физических устройств через vkGetPhysicalDeviceProperties. An example of obtaining properties of physical devices via vkGetPhysicalDeviceProperties.");

    for wvk_physical_device_ in &wvk_physical_devices_ {
        let vk_properties_ = wvk_physical_device_.wvkGetPhysicalDeviceProperties();

        printVkPhysicalDeviceProperties(&vk_properties_);
    }

    println!("Пример получения свойств физических устройств через vkGetPhysicalDeviceProperties2. An example of obtaining properties of physical devices via vkGetPhysicalDeviceProperties2.");

    for wvk_physical_device_ in &wvk_physical_devices_ {
        let vk_properties_2_ = wvk_physical_device_.wvkGetPhysicalDeviceProperties2();

        printVkPhysicalDeviceVulkan12Properties(unsafe {&vk_properties_2_.vk_physical_device_vulkan_1_2_properties.unwrap().assume_init()});

        let asd1= unsafe {vk_properties_2_.vk_physical_device_vulkan_1_1_properties.as_ref().unwrap().assume_init_ref()};
        dbg!(asd1.deviceLUIDValid);

        let asd= unsafe {vk_properties_2_.vk_physical_device_descriptor_heap_tensor_properties_arm.as_ref().unwrap().assume_init_ref()};
        let mut count = asd.tensorCaptureReplayOpaqueDataSize;
        //printVkPhysicalDeviceProperties(&vk_properties_2_.properties);
        //vk_properties_2_.vk_physical_device_vulkan_1_1_properties;

        //dbg!(asd);
        dbg!(count);

        println!("asdasd");
        println!("{}", count);

        count += 3;

        println!("{}", count);
    }

}