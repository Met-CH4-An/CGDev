////////////////////////////////////////////////////////////////
// секция форвард-декларации
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// секция заголовочного файла
////////////////////////////////////////////////////////////////
#include "wvk_surface_test.h"
////////////////////////////////////////////////////////////////
// секция имплементации
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// секция для остального
////////////////////////////////////////////////////////////////
#include "wvk_physical_device_test.h"

namespace CGDev {

	namespace tests {

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// Проверка:
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		TEST_F(WvkSurfaceTest, requestSupport) {
			bool _support = false;

			auto _status = s_wvk_surface->requestSupport(s_wvk_physical_device.get(), s_wvk_queue_family.get(), _support);

			EXPECT_EQ(_status.isOk(), true);
			EXPECT_EQ(_support, true);
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// Проверка:
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		TEST_F(WvkSurfaceTest, requestCapabilities) {
			VkSurfaceCapabilitiesKHR _caps = {};

			s_wvk_surface->requestCapabilities(s_wvk_physical_device.get(), _caps);

			EXPECT_GE(_caps.minImageCount, static_cast<uint32_t>(1));
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// Проверка:
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		TEST_F(WvkSurfaceTest, requestCapabilities_VkSurfaceProtectedCapabilitiesKHR) {
			VkSurfaceProtectedCapabilitiesKHR _caps = {};			
			s_wvk_surface->requestCapabilities(s_wvk_physical_device.get(), _caps);
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// Проверка:
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		TEST_F(WvkSurfaceTest, requestPresentModes) {
			std::vector<VkPresentModeKHR> _modes = {};
			s_wvk_surface->requestPresentModes(s_wvk_physical_device.get(), _modes);
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// Проверка:
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		TEST_F(WvkSurfaceTest, requestPresentModeCompatibility) {
			std::vector<VkPresentModeKHR> _out;
			s_wvk_surface->requestPresentModeCompatibility(s_wvk_physical_device.get(), VK_PRESENT_MODE_FIFO_KHR, _out);
		}
		
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// Проверка:
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		TEST_F(WvkSurfaceTest, requestScalingCompatibility) {
			VkSurfacePresentScalingCapabilitiesEXT _out = {};
			s_wvk_surface->requestScalingCompatibility(s_wvk_physical_device.get(), VK_PRESENT_MODE_FIFO_KHR, _out);
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// Проверка:
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		TEST_F(WvkSurfaceTest, requestCapabilities_castIn_castOut) {
			VkSurfacePresentModeEXT _in = {};
			_in.sType = VK_STRUCTURE_TYPE_SURFACE_PRESENT_MODE_EXT;
			_in.presentMode = VK_PRESENT_MODE_FIFO_KHR;
			_in.pNext = nullptr;

			VkSurfacePresentScalingCapabilitiesEXT _out = {};
			_out.sType = VK_STRUCTURE_TYPE_SURFACE_PRESENT_SCALING_CAPABILITIES_EXT;
			_out.pNext = nullptr;

			s_wvk_surface->requestCapabilities(s_wvk_physical_device.get(), reinterpret_cast<VkBaseInStructure*>(&_in), reinterpret_cast<VkBaseOutStructure*>(&_out));
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// Проверка:
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		TEST_F(WvkSurfaceTest, requestCapabilities_In_Out) {
			VkSurfacePresentModeEXT _in = {};
			_in.sType = VK_STRUCTURE_TYPE_SURFACE_PRESENT_MODE_EXT;
			_in.presentMode = VK_PRESENT_MODE_FIFO_KHR;
			_in.pNext = nullptr;

			VkSurfacePresentScalingCapabilitiesEXT _out = {};
			_out.sType = VK_STRUCTURE_TYPE_SURFACE_PRESENT_SCALING_CAPABILITIES_EXT;
			_out.pNext = nullptr;

			s_wvk_surface->requestCapabilities(s_wvk_physical_device.get(), _in, _out);
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// Проверка:
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		TEST_F(WvkSurfaceTest, requestFormats) {
			std::vector<VkSurfaceFormatKHR> _formats;
			s_wvk_surface->requestFormats(s_wvk_physical_device.get(), _formats);
		}
		
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// Проверка:
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		TEST_F(WvkSurfaceTest, requestFormats_in_out) {
			std::vector<VkImageCompressionPropertiesEXT> _compres = {};
			s_wvk_surface->requestFormats(s_wvk_physical_device.get(), nullptr, VK_STRUCTURE_TYPE_IMAGE_COMPRESSION_PROPERTIES_EXT,_compres);
		}

	} // namespace tests

} // namespace CGDev