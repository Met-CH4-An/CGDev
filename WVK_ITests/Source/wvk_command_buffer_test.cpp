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
		TEST_F(WvkCommandBufferTest, begin) {
			CGDev::wvk::WvkCommandBufferPtrVec1 _wvk_cmd_buffers(1, nullptr);
			wvk_command_pool_ptr->allocateWvkCommandBuffers(_wvk_cmd_buffers, VK_COMMAND_BUFFER_LEVEL_PRIMARY);

			auto _wvk_result = _wvk_cmd_buffers[0]->begin();

			_wvk_cmd_buffers[0]->end();

			EXPECT_EQ(_wvk_result, true);
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// Проверка:
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		TEST_F(WvkCommandBufferTest, begin_flags) {
			CGDev::wvk::WvkCommandBufferPtrVec1 _wvk_cmd_buffers(1, nullptr);
			wvk_command_pool_ptr->allocateWvkCommandBuffers(_wvk_cmd_buffers, VK_COMMAND_BUFFER_LEVEL_PRIMARY);

			auto _wvk_result = _wvk_cmd_buffers[0]->begin(0);

			_wvk_cmd_buffers[0]->end();

			EXPECT_EQ(_wvk_result, true);
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// Проверка:
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		TEST_F(WvkCommandBufferTest, begin_flags_mask) {
			CGDev::wvk::WvkCommandBufferPtrVec1 _wvk_cmd_buffers(1, nullptr);
			wvk_command_pool_ptr->allocateWvkCommandBuffers(_wvk_cmd_buffers, VK_COMMAND_BUFFER_LEVEL_PRIMARY);

			uint32_t dev_mask = 1u << 1;
			auto _wvk_result = _wvk_cmd_buffers[0]->begin(0, dev_mask);

			_wvk_cmd_buffers[0]->end();

			EXPECT_EQ(_wvk_result, true);
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// Проверка:
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		TEST_F(WvkCommandBufferTest, begin_flags_mask_1arg) {
			CGDev::wvk::WvkCommandBufferPtrVec1 _wvk_cmd_buffers(1, nullptr);
			wvk_command_pool_ptr->allocateWvkCommandBuffers(_wvk_cmd_buffers, VK_COMMAND_BUFFER_LEVEL_PRIMARY);

			VkDeviceGroupCommandBufferBeginInfo _vk_dev_group_cmd_buffer_begin_info = {
				.sType = VK_STRUCTURE_TYPE_DEVICE_GROUP_COMMAND_BUFFER_BEGIN_INFO,
				.pNext = nullptr,
				.deviceMask = 0b10,
			};

			uint32_t dev_mask = 1u << 0;
			auto _wvk_result = _wvk_cmd_buffers[0]->begin(0, dev_mask, _vk_dev_group_cmd_buffer_begin_info);

			_wvk_cmd_buffers[0]->end();

			EXPECT_EQ(_wvk_result, true);
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// Проверка:
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		TEST_F(WvkCommandBufferTest, begin_flags_mask_1arg_2arg) {
			CGDev::wvk::WvkCommandBufferPtrVec1 _wvk_cmd_buffers(1, nullptr);
			wvk_command_pool_ptr->allocateWvkCommandBuffers(_wvk_cmd_buffers, VK_COMMAND_BUFFER_LEVEL_PRIMARY);

			VkDeviceGroupCommandBufferBeginInfo _vk_dev_group_cmd_buffer_begin_info = {
				.sType = VK_STRUCTURE_TYPE_DEVICE_GROUP_COMMAND_BUFFER_BEGIN_INFO,
				.pNext = nullptr,
				.deviceMask = 0b10,
			};

			VkDeviceGroupCommandBufferBeginInfo _vk_dev_group_cmd_buffer_begin_info_1 = {
				.sType = VK_STRUCTURE_TYPE_DEVICE_GROUP_COMMAND_BUFFER_BEGIN_INFO,
				.pNext = nullptr,
				.deviceMask = 0b10,
			};

			uint32_t dev_mask = 1u << 0;
			auto _wvk_result = _wvk_cmd_buffers[0]->begin(0, dev_mask, _vk_dev_group_cmd_buffer_begin_info, _vk_dev_group_cmd_buffer_begin_info_1);

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