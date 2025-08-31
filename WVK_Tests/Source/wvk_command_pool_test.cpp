// SPDX-License-Identifier: AGPL-3.0-or-later
////////////////////////////////////////////////////////////////
// секция форвард-декларации
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// секция заголовочного файла
////////////////////////////////////////////////////////////////
#include "wvk_command_pool_test.h"
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
		TEST_F(WvkCommandPoolTest, createInvalidCreateInfo) {
			CGDev::wvk::WvkCommandPoolCreateInfo _create_info = {
				.wvk_logical_device_ptr = nullptr,
			};

			auto _wvk_command_pool_ptr = std::make_unique<CGDev::wvk::WvkCommandPool>();
			auto _wvk_status = _wvk_command_pool_ptr->create(_create_info);
			
			EXPECT_EQ(_wvk_status, CGDev::wvk::VknStatusCode::FAIL);
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// Проверка:
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		TEST_F(WvkCommandPoolTest, trim) {
			auto _wvk_status = wvk_command_pool_ptr->trim();

#if WVK_VULKAN_API_VERSION >= WVK_VULKAN_API_VERSION_11 || WVK_KHR_maintenance1 == WVK_ENABLE
			EXPECT_EQ(_wvk_status, CGDev::wvk::VknStatusCode::SUCCESSFUL);
#else
			EXPECT_EQ(_wvk_status, CGDev::wvk::VknStatusCode::FEATURE_NOT_ENABLED);
#endif
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// Проверка:
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		TEST_F(WvkCommandPoolTest, reset) {
			auto _wvk_status = wvk_command_pool_ptr->reset(VK_COMMAND_POOL_RESET_RELEASE_RESOURCES_BIT);

			EXPECT_EQ(_wvk_status, CGDev::wvk::VknStatusCode::SUCCESSFUL);
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// Проверка:
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		TEST_F(WvkCommandPoolTest, allocate) {
			std::vector<CGDev::wvk::WvkCommandBufferPtr> _wvk_cmd_buffers(1, nullptr);
			auto _wvk_status = wvk_command_pool_ptr->allocateWvkCommandBuffers(_wvk_cmd_buffers, VK_COMMAND_BUFFER_LEVEL_PRIMARY);

			EXPECT_EQ(_wvk_status, CGDev::wvk::VknStatusCode::SUCCESSFUL);
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// Проверка:
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		TEST_F(WvkCommandPoolTest, free) {
			std::vector<CGDev::wvk::WvkCommandBufferPtr> _wvk_cmd_buffers(1, nullptr);
			auto _wvk_status = wvk_command_pool_ptr->allocateWvkCommandBuffers(_wvk_cmd_buffers, VK_COMMAND_BUFFER_LEVEL_PRIMARY);
			_wvk_status = wvk_command_pool_ptr->freeWvkCommandBuffers(_wvk_cmd_buffers);

			EXPECT_EQ(_wvk_status, CGDev::wvk::VknStatusCode::SUCCESSFUL);
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// Проверка:
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		TEST_F(WvkCommandPoolTest, free_nullptr) {
			std::vector<CGDev::wvk::WvkCommandBufferPtr> _wvk_cmd_buffers(1, nullptr);
			auto _wvk_status = wvk_command_pool_ptr->freeWvkCommandBuffers(_wvk_cmd_buffers);

			EXPECT_EQ(_wvk_status, CGDev::wvk::VknStatusCode::SUCCESSFUL);
		}

	} // namespace tests

} // namespace CGDev