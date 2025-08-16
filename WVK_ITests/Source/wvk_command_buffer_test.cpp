////////////////////////////////////////////////////////////////
// секция форвард-декларации
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// секция заголовочного файла
////////////////////////////////////////////////////////////////
#include "wvk_command_buffer_test.h"
////////////////////////////////////////////////////////////////
// секция имплементации
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// секция для остального
////////////////////////////////////////////////////////////////
#include "wvk_command_pool.h"

namespace CGDev {

	namespace tests {

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// Проверка:
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		TEST_F(WvkCommandBufferTest, reset) {
			CGDev::wvk::WvkCommandPoolCreateInfo _create_info = {
					.wvk_logical_device_ptr = wvk_logical_device_ptr.get(),
					.queue_family_index = 0,
					.flags = VK_COMMAND_POOL_CREATE_RESET_COMMAND_BUFFER_BIT,
			};

			auto _wvk_cmd_pool_ptr = std::make_unique<CGDev::wvk::WvkCommandPool>();
			_wvk_cmd_pool_ptr->create(_create_info);

			CGDev::wvk::WvkCommandBufferPtrVec1 _wvk_cmd_buffers(1, nullptr);
			_wvk_cmd_pool_ptr->allocateWvkCommandBuffers(_wvk_cmd_buffers, VK_COMMAND_BUFFER_LEVEL_PRIMARY);

			auto _wvk_result = _wvk_cmd_buffers[0]->reset();

			EXPECT_EQ(_wvk_result, true);
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// Проверка:
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		TEST_F(WvkCommandBufferTest, reset_fail) {
			CGDev::wvk::WvkCommandPoolCreateInfo _create_info = {
					.wvk_logical_device_ptr = wvk_logical_device_ptr.get(),
					.queue_family_index = 0,
			};

			auto _wvk_cmd_pool_ptr = std::make_unique<CGDev::wvk::WvkCommandPool>();
			_wvk_cmd_pool_ptr->create(_create_info);

			CGDev::wvk::WvkCommandBufferPtrVec1 _wvk_cmd_buffers(1, nullptr);
			_wvk_cmd_pool_ptr->allocateWvkCommandBuffers(_wvk_cmd_buffers, VK_COMMAND_BUFFER_LEVEL_PRIMARY);

			auto _wvk_result = _wvk_cmd_buffers[0]->reset();

			EXPECT_EQ(_wvk_result, false);
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// Проверка:
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		TEST_F(WvkCommandBufferTest, begin_primary) {
			CGDev::wvk::WvkCommandBufferPtrVec1 _wvk_cmd_buffers(1, nullptr);
			wvk_command_pool_ptr->allocateWvkCommandBuffers(_wvk_cmd_buffers, VK_COMMAND_BUFFER_LEVEL_PRIMARY);

			auto _wvk_result = _wvk_cmd_buffers[0]->begin();

			_wvk_cmd_buffers[0]->end();

			EXPECT_EQ(_wvk_result, true);
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// Проверка:
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		TEST_F(WvkCommandBufferTest, begin_primary_VK_COMMAND_BUFFER_USAGE_RENDER_PASS_CONTINUE_BIT) {
			CGDev::wvk::WvkCommandBufferPtrVec1 _wvk_cmd_buffers(1, nullptr);
			wvk_command_pool_ptr->allocateWvkCommandBuffers(_wvk_cmd_buffers, VK_COMMAND_BUFFER_LEVEL_PRIMARY);

			auto _wvk_result = _wvk_cmd_buffers[0]->begin(VK_COMMAND_BUFFER_USAGE_RENDER_PASS_CONTINUE_BIT);

			_wvk_cmd_buffers[0]->end();

			EXPECT_EQ(_wvk_result, true);
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// Проверка:
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		TEST_F(WvkCommandBufferTest, begin_secondary) {
			CGDev::wvk::WvkCommandBufferPtrVec1 _wvk_cmd_buffers(1, nullptr);
			wvk_command_pool_ptr->allocateWvkCommandBuffers(_wvk_cmd_buffers, VK_COMMAND_BUFFER_LEVEL_SECONDARY);

			auto _wvk_result = _wvk_cmd_buffers[0]->begin();

			_wvk_cmd_buffers[0]->end();

			EXPECT_EQ(_wvk_result, false);
		}

	} // namespace tests

} // namespace CGDev