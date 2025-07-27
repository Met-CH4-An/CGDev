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
			VkPhysicalDeviceProperties _props;
			
			wvk_physical_device.requestProperties(_props);
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// Проверка:
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		TEST_F(WvkPhysicalDeviceTest, requestProperties_out) {
			VkPhysicalDeviceVulkan11Properties _vulkan_props_11 = { VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_1_PROPERTIES };
			auto _res = wvk_physical_device.requestProperties(reinterpret_cast<VkBaseOutStructure*>(&_vulkan_props_11));

			VkPhysicalDeviceVulkan12Properties _vulkan_props_12 = { VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_2_PROPERTIES };
			_res = wvk_physical_device.requestProperties(reinterpret_cast<VkBaseOutStructure*>(&_vulkan_props_12));

			VkPhysicalDeviceDriverProperties _driver_props = { VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DRIVER_PROPERTIES };
			_res = wvk_physical_device.requestProperties(reinterpret_cast<VkBaseOutStructure*>(&_driver_props));
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// Проверка:
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		TEST_F(WvkPhysicalDeviceTest, requestProperties_out_template) {
			VkPhysicalDeviceVulkan11Properties _vulkan_props_11 = { VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_1_PROPERTIES };
			std::fill(std::begin(_vulkan_props_11.deviceUUID), std::end(_vulkan_props_11.deviceUUID), 0xCD);
			auto _res = wvk_physical_device.requestProperties(_vulkan_props_11);

			VkPhysicalDeviceVulkan12Properties _vulkan_props_12 = { VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_2_PROPERTIES };
			_vulkan_props_12.driverID = static_cast<VkDriverId>(0xCD);
			_res = wvk_physical_device.requestProperties(_vulkan_props_12);

			VkPhysicalDeviceDriverProperties _driver_props = { VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DRIVER_PROPERTIES };
			_driver_props.driverID = static_cast<VkDriverId>(0xCD);
			_res = wvk_physical_device.requestProperties(_driver_props);

			if constexpr (
				CGDev::wvk::Build::vulkan_api_version >= CGDev::wvk::Build::VulkanVersion::VERSION_11 ||
				CGDev::wvk::Build::find("VK_KHR_get_physical_device_properties2")) {

				bool _uuid_filled = std::any_of(
					std::begin(_vulkan_props_11.deviceUUID),
					std::end(_vulkan_props_11.deviceUUID),
					[](uint8_t b) { return b != 0xCD; }
				);
				EXPECT_TRUE(_uuid_filled) << "UUID остался равен 0xCD — структура не была перезаписана";
				EXPECT_NE(_vulkan_props_12.driverID, 0xCD);
				EXPECT_NE(_driver_props.driverID, 0xCD);
			}

			else {
				EXPECT_EQ(_res, CGDev::wvk::VknStatusCode::FEATURE_NOT_ENABLED);
			}
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// Проверка:
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		TEST_F(WvkPhysicalDeviceTest, requestQueueFamilyProperties) {
			std::vector<VkQueueFamilyProperties> _queue_family_props = {};
			wvk_physical_device.requestQueueFamilyProperties(_queue_family_props);
			
			EXPECT_EQ(_queue_family_props.empty(), false);
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// Проверка:
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		TEST_F(WvkPhysicalDeviceTest, requestQueueFamilyProperties_out) {
			std::vector<VkQueueFamilyGlobalPriorityProperties> _global_priority_props;
			auto _res = wvk_physical_device.requestQueueFamilyProperties(_global_priority_props, VK_STRUCTURE_TYPE_QUEUE_FAMILY_GLOBAL_PRIORITY_PROPERTIES);

			if constexpr (
				CGDev::wvk::Build::vulkan_api_version >= CGDev::wvk::Build::VulkanVersion::VERSION_11 ||
				CGDev::wvk::Build::find("VK_KHR_get_physical_device_properties2")) {

				EXPECT_EQ(_global_priority_props.empty(), false);
			}

			else {
				EXPECT_EQ(_res, CGDev::wvk::VknStatusCode::FEATURE_NOT_ENABLED);
			}
		}
		
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// Проверка:
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		TEST_F(WvkPhysicalDeviceTest, checkCompatibility) {
			std::vector<CGDev::wvk::WvkPhysicalDevicePtr> _wvk_phys_devs = {
				&wvk_physical_device,
				&wvk_physical_device };

			bool _compatibility = false;
			auto _res = wvk_physical_device.checkCompatibility(_wvk_phys_devs, _compatibility);

#if VULKAN_API_VERSION == VULKAN_API_VERSION_10 && WVK_KHR_device_group_creation == WVK_EXTENSION_DISABLE
			EXPECT_EQ(_res.isOk(), false);
			EXPECT_EQ(_res.getCode(), CGDev::wvk::VknStatusCode::FEATURE_NOT_ENABLED);
#endif
#if (VULKAN_API_VERSION == VULKAN_API_VERSION_10 && WVK_KHR_device_group_creation == WVK_EXTENSION_ENABLE) || VULKAN_API_VERSION >= VULKAN_API_VERSION_11
			EXPECT_EQ(_compatibility, true);
#endif
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// Проверка:
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		TEST_F(WvkPhysicalDeviceTest, checkCompatibility_fakePhysDevice) {
			CGDev::wvk::WvkPhysicalDevice _fake_phys_device;
			std::vector<CGDev::wvk::WvkPhysicalDevicePtr> _wvk_phys_devs = {
				&_fake_phys_device };

			bool _compatibility = false;
			auto _res = wvk_physical_device.checkCompatibility(_wvk_phys_devs, _compatibility);

#if VULKAN_API_VERSION == VULKAN_API_VERSION_10 && WVK_KHR_device_group_creation == WVK_EXTENSION_DISABLE
			EXPECT_EQ(_res.isOk(), false);
			EXPECT_EQ(_res.getCode(), CGDev::wvk::VknStatusCode::FEATURE_NOT_ENABLED);
#endif
#if (VULKAN_API_VERSION == VULKAN_API_VERSION_10 && WVK_KHR_device_group_creation == WVK_EXTENSION_ENABLE) || VULKAN_API_VERSION >= VULKAN_API_VERSION_11
			EXPECT_EQ(_compatibility, false);
#endif
		}

	} // namespace tests

} // namespace CGDev