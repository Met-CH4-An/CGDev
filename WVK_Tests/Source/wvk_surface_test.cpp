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
			//bool _support = false;

			//auto _status = s_wvk_surface->requestSupport(&wvk_physical_device, &wvk_queue_family, _support);

			//EXPECT_EQ(_status.isOk(), true);
			//EXPECT_EQ(_support, true);
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// Проверка:
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		TEST_F(WvkSurfaceTest, requestCapabilities) {
			//VkSurfaceCapabilitiesKHR _caps = {};

			//s_wvk_surface->requestCapabilities(&wvk_physical_device, _caps);

			//EXPECT_GE(_caps.minImageCount, static_cast<uint32_t>(1));
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// Проверка:
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		TEST_F(WvkSurfaceTest, requestCapabilities_VkSurfaceProtectedCapabilitiesKHR) {
			//VkSurfaceProtectedCapabilitiesKHR _caps = {};			
			//s_wvk_surface->requestCapabilities(&wvk_physical_device, _caps);
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// Проверка:
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		TEST_F(WvkSurfaceTest, requestPresentModes) {
			//std::vector<VkPresentModeKHR> _modes = {};
			//s_wvk_surface->requestPresentModes(&wvk_physical_device, _modes);
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// Проверка:
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		TEST_F(WvkSurfaceTest, requestPresentModeCompatibility) {
			//std::vector<VkPresentModeKHR> _out;
			//s_wvk_surface->requestPresentModeCompatibility(&wvk_physical_device, VK_PRESENT_MODE_FIFO_KHR, _out);
		}
		
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// Проверка:
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		TEST_F(WvkSurfaceTest, requestScalingCompatibility) {
			//VkSurfacePresentScalingCapabilitiesEXT _out = {};
			//s_wvk_surface->requestScalingCompatibility(&wvk_physical_device, VK_PRESENT_MODE_FIFO_KHR, _out);
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// Проверка:
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		TEST_F(WvkSurfaceTest, requestCapabilities_castIn_castOut) {
			//VkSurfacePresentModeEXT _in = {};
			//_in.sType = VK_STRUCTURE_TYPE_SURFACE_PRESENT_MODE_EXT;
			//_in.presentMode = VK_PRESENT_MODE_FIFO_KHR;
			//_in.pNext = nullptr;

			//VkSurfacePresentScalingCapabilitiesEXT _out = {};
			//_out.sType = VK_STRUCTURE_TYPE_SURFACE_PRESENT_SCALING_CAPABILITIES_EXT;
			//_out.pNext = nullptr;

			//s_wvk_surface->requestCapabilities(&wvk_physical_device, reinterpret_cast<VkBaseInStructure*>(&_in), reinterpret_cast<VkBaseOutStructure*>(&_out));
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// Проверка:
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		TEST_F(WvkSurfaceTest, requestCapabilities_In_Out) {
			//VkSurfacePresentModeEXT _in = {};
			//_in.sType = VK_STRUCTURE_TYPE_SURFACE_PRESENT_MODE_EXT;
			//_in.presentMode = VK_PRESENT_MODE_FIFO_KHR;
			//_in.pNext = nullptr;

			//VkSurfacePresentScalingCapabilitiesEXT _out = {};
			//_out.sType = VK_STRUCTURE_TYPE_SURFACE_PRESENT_SCALING_CAPABILITIES_EXT;
			//_out.pNext = nullptr;

			//s_wvk_surface->requestCapabilities(&wvk_physical_device, _in, _out);
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// Проверка:
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		TEST_F(WvkSurfaceTest, requestFormats) {
			//std::vector<VkSurfaceFormatKHR> _formats;
			//s_wvk_surface->requestFormats(&wvk_physical_device, _formats);
		}
		
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// Проверка:
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		TEST_F(WvkSurfaceTest, requestFormats_in_out) {
			//std::vector<VkImageCompressionPropertiesEXT> _compres = {};
			//s_wvk_surface->requestFormats(&wvk_physical_device, nullptr, VK_STRUCTURE_TYPE_IMAGE_COMPRESSION_PROPERTIES_EXT,_compres);
		}

	} // namespace tests

} // namespace CGDev