// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025 Metelev Met-CH4-An Andrey
// Project Lead: Metelev Andrey
// Main Technical Assistant: StarDev aka ChatGPT
////////////////////////////////////////////////////////////////
// секция форвард-декларации
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// секция заголовочного файла
////////////////////////////////////////////////////////////////
#include "wvk_physical_device_test.h"
////////////////////////////////////////////////////////////////
// секция имплементации
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// секция для остального
////////////////////////////////////////////////////////////////

namespace CGDev {

	namespace tests {

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// Проверка:
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		TEST_F(WvkPhysicalDeviceTest, requestProperties) {
			VkPhysicalDeviceProperties _props = {};
			wvk_phys_dev->requestProperties(_props);
			
			EXPECT_NE(_props.deviceID, 0);
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// Проверка:
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		TEST_F(WvkPhysicalDeviceTest, requestProperties_extended_one_arguments) {
			VkPhysicalDeviceVulkan11Properties _vulkan_props_11 = { VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_1_PROPERTIES };
			auto _res = wvk_phys_dev->requestProperties(_vulkan_props_11);

#if WVK_VULKAN_API_VERSION >= WVK_VULKAN_API_VERSION_11 || WVK_KHR_get_physical_device_properties2 == WVK_ENABLE
			EXPECT_EQ(_res, CGDev::wvk::VknStatusCode::SUCCESSFUL);
#else
			EXPECT_EQ(_res, CGDev::wvk::VknStatusCode::FEATURE_NOT_ENABLED);
#endif
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// Проверка:
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		TEST_F(WvkPhysicalDeviceTest, requestProperties_extended_two_arguments) {
			VkPhysicalDeviceVulkan11Properties _vulkan_props_11 = { VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_1_PROPERTIES };
			VkPhysicalDeviceVulkan12Properties _vulkan_props_12 = { VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_2_PROPERTIES };
			auto _res = wvk_phys_dev->requestProperties(_vulkan_props_11, _vulkan_props_12);

#if WVK_VULKAN_API_VERSION >= WVK_VULKAN_API_VERSION_11 || WVK_KHR_get_physical_device_properties2 == WVK_ENABLE
			EXPECT_EQ(_res, CGDev::wvk::VknStatusCode::SUCCESSFUL);
#else
			EXPECT_EQ(_res, CGDev::wvk::VknStatusCode::FEATURE_NOT_ENABLED);
#endif
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// Проверка:
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		TEST_F(WvkPhysicalDeviceTest, requestProperties_extended_three_arguments) {
			VkPhysicalDeviceVulkan11Properties _vulkan_props_11 = { VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_1_PROPERTIES };
			VkPhysicalDeviceVulkan12Properties _vulkan_props_12 = { VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_2_PROPERTIES };
			VkPhysicalDeviceDriverProperties _driver_props = { VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DRIVER_PROPERTIES };
			auto _res = wvk_phys_dev->requestProperties(_vulkan_props_11, _vulkan_props_12, _driver_props);

#if WVK_VULKAN_API_VERSION >= WVK_VULKAN_API_VERSION_11 || WVK_KHR_get_physical_device_properties2 == WVK_ENABLE
			EXPECT_EQ(_res, CGDev::wvk::VknStatusCode::SUCCESSFUL);
#else
			EXPECT_EQ(_res, CGDev::wvk::VknStatusCode::FEATURE_NOT_ENABLED);
#endif
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// Проверка:
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		TEST_F(WvkPhysicalDeviceTest, requestQueueFamilyProperties_base) {
			std::vector<VkQueueFamilyProperties> _queue_family_props = {};
			auto _res = wvk_phys_dev->requestQueueFamilyProperties(_queue_family_props);
			
			EXPECT_EQ(_res, CGDev::wvk::VknStatusCode::SUCCESSFUL);
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// Проверка:
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		TEST_F(WvkPhysicalDeviceTest, requestQueueFamilyProperties_extended_one_arguments) {
			std::vector<VkQueueFamilyGlobalPriorityProperties> _global_priority_props_1 = { VkQueueFamilyGlobalPriorityProperties(VK_STRUCTURE_TYPE_QUEUE_FAMILY_GLOBAL_PRIORITY_PROPERTIES) };
			auto _res = wvk_phys_dev->requestQueueFamilyProperties(_global_priority_props_1);

#if WVK_VULKAN_API_VERSION >= WVK_VULKAN_API_VERSION_11 || WVK_KHR_get_physical_device_properties2 == WVK_ENABLE
			EXPECT_EQ(_res, CGDev::wvk::VknStatusCode::SUCCESSFUL);
#else
			EXPECT_EQ(_res, CGDev::wvk::VknStatusCode::FEATURE_NOT_ENABLED);
#endif
			
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// Проверка:
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		TEST_F(WvkPhysicalDeviceTest, requestQueueFamilyProperties_extended_two_arguments) {
			std::vector<VkQueueFamilyGlobalPriorityProperties> _global_priority_props_1 = { VkQueueFamilyGlobalPriorityProperties(VK_STRUCTURE_TYPE_QUEUE_FAMILY_GLOBAL_PRIORITY_PROPERTIES) };
			std::vector<VkQueueFamilyGlobalPriorityProperties> _global_priority_props_2 = { VkQueueFamilyGlobalPriorityProperties(VK_STRUCTURE_TYPE_QUEUE_FAMILY_GLOBAL_PRIORITY_PROPERTIES) };

			auto _res = wvk_phys_dev->requestQueueFamilyProperties(_global_priority_props_1, _global_priority_props_2);

#if WVK_VULKAN_API_VERSION >= WVK_VULKAN_API_VERSION_11 || WVK_KHR_get_physical_device_properties2 == WVK_ENABLE
			EXPECT_EQ(_res, CGDev::wvk::VknStatusCode::SUCCESSFUL);
#else
			EXPECT_EQ(_res, CGDev::wvk::VknStatusCode::FEATURE_NOT_ENABLED);
#endif
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// Проверка:
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		TEST_F(WvkPhysicalDeviceTest, requestQueueFamilyProperties_extended_three_arguments) {
			std::vector<VkQueueFamilyGlobalPriorityProperties> _global_priority_props_1 = { VkQueueFamilyGlobalPriorityProperties(VK_STRUCTURE_TYPE_QUEUE_FAMILY_GLOBAL_PRIORITY_PROPERTIES) };
			std::vector<VkQueueFamilyGlobalPriorityProperties> _global_priority_props_2 = { VkQueueFamilyGlobalPriorityProperties(VK_STRUCTURE_TYPE_QUEUE_FAMILY_GLOBAL_PRIORITY_PROPERTIES) };
			std::vector<VkQueueFamilyGlobalPriorityProperties> _global_priority_props_3 = { VkQueueFamilyGlobalPriorityProperties(VK_STRUCTURE_TYPE_QUEUE_FAMILY_GLOBAL_PRIORITY_PROPERTIES) };

			auto _res = wvk_phys_dev->requestQueueFamilyProperties(_global_priority_props_1, _global_priority_props_2, _global_priority_props_3);

#if WVK_VULKAN_API_VERSION >= WVK_VULKAN_API_VERSION_11 || WVK_KHR_get_physical_device_properties2 == WVK_ENABLE
			EXPECT_EQ(_res, CGDev::wvk::VknStatusCode::SUCCESSFUL);
#else
			EXPECT_EQ(_res, CGDev::wvk::VknStatusCode::FEATURE_NOT_ENABLED);
#endif
		}
		
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// Проверка:
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		TEST_F(WvkPhysicalDeviceTest, checkCompatibility) {
			//std::vector<CGDev::wvk::WvkPhysicalDevicePtr> _wvk_phys_devs = {
			//	&wvk_physical_device,
			//	&wvk_physical_device };

			bool _compatibility = false;
			//auto _res = wvk_physical_device.checkCompatibility(_wvk_phys_devs, _compatibility);

//#if VULKAN_API_VERSION == VULKAN_API_VERSION_10 && WVK_KHR_device_group_creation == WVK_DISABLE
//			EXPECT_EQ(_res.isOk(), false);
//			EXPECT_EQ(_res.getCode(), CGDev::wvk::VknStatusCode::FEATURE_NOT_ENABLED);
#//endif
//#if (VULKAN_API_VERSION == VULKAN_API_VERSION_10 && WVK_KHR_device_group_creation == WVK_ENABLE) || VULKAN_API_VERSION >= VULKAN_API_VERSION_11
//			EXPECT_EQ(_compatibility, true);
//#endif
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// Проверка:
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		TEST_F(WvkPhysicalDeviceTest, checkCompatibility_fakePhysDevice) {
			//CGDev::wvk::WvkPhysicalDevice _fake_phys_device;
			//std::vector<CGDev::wvk::WvkPhysicalDevicePtr> _wvk_phys_devs = {
			//	&_fake_phys_device };

			//bool _compatibility = false;
			//auto _res = wvk_physical_device.checkCompatibility(_wvk_phys_devs, _compatibility);

//#if VULKAN_API_VERSION == VULKAN_API_VERSION_10 && WVK_KHR_device_group_creation == WVK_DISABLE
//			EXPECT_EQ(_res.isOk(), false);
//			EXPECT_EQ(_res.getCode(), CGDev::wvk::VknStatusCode::FEATURE_NOT_ENABLED);
//#endif
//#if (VULKAN_API_VERSION == VULKAN_API_VERSION_10 && WVK_KHR_device_group_creation == WVK_ENABLE) || VULKAN_API_VERSION >= VULKAN_API_VERSION_11
//			EXPECT_EQ(_compatibility, false);
//#endif
		}

	} // namespace tests

} // namespace CGDev