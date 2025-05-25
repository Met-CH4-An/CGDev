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
		TEST_F(WvkPhysicalDeviceTest, print_vkPhysicalDeviceProperties) {
			VkPhysicalDeviceProperties _props;
			
			s_wvk_physical_device->requestPhysicalDeviceProperties(_props);

			std::cout << "=======================" << std::endl;
			std::cout << "Device Name: " << _props.deviceName << std::endl;
			std::cout << "API Version: "
				<< VK_VERSION_MAJOR(_props.apiVersion) << "."
				<< VK_VERSION_MINOR(_props.apiVersion) << "."
				<< VK_VERSION_PATCH(_props.apiVersion) << std::endl;
			std::cout << "Driver Version: " << _props.driverVersion << std::endl;
			std::cout << "Vendor ID: 0x" << std::hex << _props.vendorID << std::dec << std::endl;
			std::cout << "Device ID: 0x" << std::hex << _props.deviceID << std::dec << std::endl;

			std::string deviceType;
			switch (_props.deviceType) {
			case VK_PHYSICAL_DEVICE_TYPE_INTEGRATED_GPU: deviceType = "Integrated GPU"; break;
			case VK_PHYSICAL_DEVICE_TYPE_DISCRETE_GPU:   deviceType = "Discrete GPU";   break;
			case VK_PHYSICAL_DEVICE_TYPE_VIRTUAL_GPU:    deviceType = "Virtual GPU";    break;
			case VK_PHYSICAL_DEVICE_TYPE_CPU:            deviceType = "CPU";            break;
			default:                                     deviceType = "Other";          break;
			}
			std::cout << "Device Type: " << deviceType << std::endl;

			std::cout << "Limits:" << std::endl;
			std::cout << "  Max Image Dimension 2D: " << _props.limits.maxImageDimension2D << std::endl;
			std::cout << "  Max Uniform Buffer Range: " << _props.limits.maxUniformBufferRange << std::endl;
			std::cout << "  Max Push Constants Size: " << _props.limits.maxPushConstantsSize << std::endl;

			std::cout << "=======================" << std::endl;
						
			EXPECT_STRNE(_props.deviceName, "");
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// Проверка:
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		TEST_F(WvkPhysicalDeviceTest, print_VkPhysicalDeviceVulkan11Properties) {
			VkPhysicalDeviceVulkan11Properties _props = { VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_1_PROPERTIES };

			s_wvk_physical_device->requestPhysicalDeviceProperties(_props);

			std::cout << "VkPhysicalDeviceVulkan11Properties:\n";

			std::cout << "  deviceUUID: ";
			for (int i = 0; i < VK_UUID_SIZE; ++i) {
				std::cout << std::hex << std::setw(2) << std::setfill('0')
					<< static_cast<int>(_props.deviceUUID[i]);
			}
			std::cout << std::dec << "\n";

			std::cout << "  driverUUID: ";
			for (int i = 0; i < VK_UUID_SIZE; ++i) {
				std::cout << std::hex << std::setw(2) << std::setfill('0')
					<< static_cast<int>(_props.driverUUID[i]);
			}
			std::cout << std::dec << "\n";

			std::cout << "  deviceLUIDValid: " << (_props.deviceLUIDValid ? "true" : "false") << "\n";

			std::cout << "  subgroupSize: " << _props.subgroupSize << "\n";
			std::cout << "  subgroupSupportedStages: " << _props.subgroupSupportedStages << "\n";
			std::cout << "  subgroupSupportedOperations: " << _props.subgroupSupportedOperations << "\n";
			std::cout << "  subgroupQuadOperationsInAllStages: "
				<< (_props.subgroupQuadOperationsInAllStages ? "true" : "false") << "\n";

			std::cout << "  pointClippingBehavior: " << _props.pointClippingBehavior << "\n";
			std::cout << "  maxMultiviewViewCount: " << _props.maxMultiviewViewCount << "\n";
			std::cout << "  maxMultiviewInstanceIndex: " << _props.maxMultiviewInstanceIndex << "\n";
			std::cout << "  protectedNoFault: " << (_props.protectedNoFault ? "true" : "false") << "\n";

			std::cout << "  maxPerSetDescriptors: " << _props.maxPerSetDescriptors << "\n";
			std::cout << "  maxMemoryAllocationSize: " << _props.maxMemoryAllocationSize << "\n";

			EXPECT_NE(_props.deviceUUID[0], 0u) << "deviceUUID[0] should not be zero";
			EXPECT_NE(_props.deviceLUID[0], 0u) << "deviceLUID[0] should not be zero (если используется)";
		}

	} // namespace tests

} // namespace CGDev