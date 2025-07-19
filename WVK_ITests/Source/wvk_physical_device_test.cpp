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
			
			s_wvk_physical_device->requestProperties(_props);
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// Проверка:
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		TEST_F(WvkPhysicalDeviceTest, requestProperties_out) {
			VkPhysicalDeviceVulkan11Properties _vulkan_props_11 = { VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_1_PROPERTIES };
			s_wvk_physical_device->requestProperties(reinterpret_cast<VkBaseOutStructure*>(&_vulkan_props_11));

			VkPhysicalDeviceVulkan12Properties _vulkan_props_12 = { VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_2_PROPERTIES };
			s_wvk_physical_device->requestProperties(reinterpret_cast<VkBaseOutStructure*>(&_vulkan_props_12));

			VkPhysicalDeviceDriverProperties _driver_props = { VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DRIVER_PROPERTIES };
			s_wvk_physical_device->requestProperties(reinterpret_cast<VkBaseOutStructure*>(&_driver_props));
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// Проверка:
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		TEST_F(WvkPhysicalDeviceTest, requestProperties_out_template) {
			VkPhysicalDeviceVulkan11Properties _vulkan_props_11 = { VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_1_PROPERTIES };
			s_wvk_physical_device->requestProperties(_vulkan_props_11);

			VkPhysicalDeviceVulkan12Properties _vulkan_props_12 = { VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_2_PROPERTIES };
			s_wvk_physical_device->requestProperties(_vulkan_props_12);

			VkPhysicalDeviceDriverProperties _driver_props = { VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DRIVER_PROPERTIES };
			s_wvk_physical_device->requestProperties(_driver_props);
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// Проверка:
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		TEST_F(WvkPhysicalDeviceTest, requestQueueFamilyProperties) {
			CGDev::wvk::VkQueueFamilyPropertiesVec1 _queue_family_props = {};
			s_wvk_physical_device->requestQueueFamilyProperties(_queue_family_props);
			EXPECT_GE(_queue_family_props.size(), 1);
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// Проверка:
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		TEST_F(WvkPhysicalDeviceTest, requestQueueFamilyProperties_out) {
			std::vector<VkQueueFamilyGlobalPriorityProperties> _global_priority_props;
			s_wvk_physical_device->requestQueueFamilyProperties(_global_priority_props, VK_STRUCTURE_TYPE_QUEUE_FAMILY_GLOBAL_PRIORITY_PROPERTIES);
			//EXPECT_GE(_queue_family_props.size(), 1);
		}
		
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// Проверка:
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		TEST_F(WvkPhysicalDeviceTest, checkCompatibility) {
			std::vector<CGDev::wvk::WvkPhysicalDevicePtr> _wvk_phys_devs = {
			s_wvk_physical_device.get(),
			s_wvk_physical_device.get() };

			bool _compatibility = false;
			auto _res = s_wvk_physical_device->checkCompatibility(_wvk_phys_devs, _compatibility);

			if constexpr (
				CGDev::wvk::Build::WvkBuildInfo::vulkan_api_version >= CGDev::wvk::Build::VulkanVersion::VERSION_11 ||
				CGDev::wvk::Build::WvkBuildInfo::find("VK_KHR_device_group_creation")) {
				EXPECT_EQ(_compatibility, true);
				EXPECT_EQ(_res.isOk(), true);

			}

			else {
				EXPECT_EQ(_res.getCode(), CGDev::wvk::VknStatusCode::FEATURE_NOT_ENABLED);
			}
		}

	} // namespace tests

} // namespace CGDev