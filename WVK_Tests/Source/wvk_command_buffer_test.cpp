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
#include "Extensions/wvk_debug_utils_messenger.h"

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
		TEST_F(WvkCommandBufferTest, resetFail) {
			CGDev::wvk::WvkCommandPoolCreateInfo _create_info = {
					.wvk_logical_device_ptr = wvk_logical_device_ptr.get(),
					.queue_family_index = 0,
			};

			auto _wvk_cmd_pool_ptr = std::make_unique<CGDev::wvk::WvkCommandPool>();
			_wvk_cmd_pool_ptr->create(_create_info);

			CGDev::wvk::WvkCommandBufferPtrVec1 _wvk_cmd_buffers(1, nullptr);
			_wvk_cmd_pool_ptr->allocateWvkCommandBuffers(_wvk_cmd_buffers, VK_COMMAND_BUFFER_LEVEL_PRIMARY);

			auto _wvk_result = _wvk_cmd_buffers[0]->reset();
			auto wvk_validation = wvk_debug_utils_messenger_ptr->hasStatus(CGDev::wvk::Extensions::VknDebugUtilsMode::ERRORS_ONLY);
			EXPECT_EQ(_wvk_result, true);
		}
		//VkDeviceGroupCommandBufferBeginInfo _vk_dev_group_cmd_buffer_begin_info = {
		//		.sType = VK_STRUCTURE_TYPE_DEVICE_GROUP_COMMAND_BUFFER_BEGIN_INFO,
		//		.pNext = nullptr,
		//		.deviceMask = 1u << 0,
		//};

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// Проверка:
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		TEST_F(WvkCommandBufferTest, begin_primary_usage) {
			CGDev::wvk::WvkCommandBufferPtrVec1 _wvk_cmd_buffers(1, nullptr);
			wvk_command_pool_ptr->allocateWvkCommandBuffers(_wvk_cmd_buffers, VK_COMMAND_BUFFER_LEVEL_PRIMARY);

			VkCommandBufferBeginInfo _begin_info = {
				.sType = VK_STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO,
				.pNext = nullptr,
				.flags = VK_COMMAND_BUFFER_USAGE_ONE_TIME_SUBMIT_BIT,
				.pInheritanceInfo = nullptr,
			};
			auto _wvk_result = _wvk_cmd_buffers[0]->begin(_begin_info);

			_wvk_cmd_buffers[0]->end();

			EXPECT_EQ(_wvk_result, true);
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// Проверка:
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		TEST_F(WvkCommandBufferTest, begin_primary_usageAndGroupMask) {
			CGDev::wvk::WvkCommandBufferPtrVec1 _wvk_cmd_buffers(1, nullptr);
			wvk_command_pool_ptr->allocateWvkCommandBuffers(_wvk_cmd_buffers, VK_COMMAND_BUFFER_LEVEL_PRIMARY);

			VkDeviceGroupCommandBufferBeginInfo _group_info = {
				.sType = VK_STRUCTURE_TYPE_DEVICE_GROUP_COMMAND_BUFFER_BEGIN_INFO,
				.pNext = nullptr,
				.deviceMask = 1u << 0,
			};

			VkCommandBufferBeginInfo _begin_info = {
				.sType = VK_STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO,
				.pNext = &_group_info,
				.flags = VK_COMMAND_BUFFER_USAGE_ONE_TIME_SUBMIT_BIT,
				.pInheritanceInfo = nullptr,
			};
			auto _wvk_result = _wvk_cmd_buffers[0]->begin(_begin_info);

			_wvk_cmd_buffers[0]->end();

			EXPECT_EQ(_wvk_result, true);
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// Проверка:
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		TEST_F(WvkCommandBufferTest, beginHelper_primary_usage) {
			CGDev::wvk::WvkCommandBufferPtrVec1 _wvk_cmd_buffers(1, nullptr);
			wvk_command_pool_ptr->allocateWvkCommandBuffers(_wvk_cmd_buffers, VK_COMMAND_BUFFER_LEVEL_PRIMARY);

			CGDev::wvk::WvkCommandBufferBeginHelper _helper;
			_helper.withUsage(VK_COMMAND_BUFFER_USAGE_ONE_TIME_SUBMIT_BIT);

			auto _wvk_result = _wvk_cmd_buffers[0]->begin(_helper);

			_wvk_cmd_buffers[0]->end();

			EXPECT_EQ(_wvk_result, true);
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// Проверка:
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		TEST_F(WvkCommandBufferTest, beginHelper_primary_usageAndGroupMask) {
			CGDev::wvk::WvkCommandBufferPtrVec1 _wvk_cmd_buffers(1, nullptr);
			wvk_command_pool_ptr->allocateWvkCommandBuffers(_wvk_cmd_buffers, VK_COMMAND_BUFFER_LEVEL_PRIMARY);

			CGDev::wvk::WvkCommandBufferBeginHelper _helper;
			_helper.withUsage(VK_COMMAND_BUFFER_USAGE_ONE_TIME_SUBMIT_BIT)
				.withGroupMask(1u << 0);

			auto _wvk_result = _wvk_cmd_buffers[0]->begin(_helper);

			_wvk_cmd_buffers[0]->end();

			EXPECT_EQ(_wvk_result, true);
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// Проверка:
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		TEST_F(WvkCommandBufferTest, beginHelper_secondary_usageAndOcclusion) {
			CGDev::wvk::WvkCommandBufferPtrVec1 _wvk_cmd_buffers(1, nullptr);
			wvk_command_pool_ptr->allocateWvkCommandBuffers(_wvk_cmd_buffers, VK_COMMAND_BUFFER_LEVEL_SECONDARY);

			CGDev::wvk::WvkCommandBufferBeginHelper _helper;
			_helper.withUsage(VK_COMMAND_BUFFER_USAGE_ONE_TIME_SUBMIT_BIT)
				.withOcclusionQuery(VK_TRUE);

			auto _wvk_result = _wvk_cmd_buffers[0]->begin(_helper);

			_wvk_cmd_buffers[0]->end();

			EXPECT_EQ(_wvk_result, true);
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// Проверка:
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		TEST_F(WvkCommandBufferTest, beginHelper_secondary_usageAndOcclusionAndRenderpass) {
			CGDev::wvk::WvkCommandBufferPtrVec1 _wvk_cmd_buffers(1, nullptr);
			wvk_command_pool_ptr->allocateWvkCommandBuffers(_wvk_cmd_buffers, VK_COMMAND_BUFFER_LEVEL_SECONDARY);

			CGDev::wvk::WvkCommandBufferBeginHelper _helper;
			_helper.withUsage(VK_COMMAND_BUFFER_USAGE_ONE_TIME_SUBMIT_BIT)
				.withOcclusionQuery(VK_TRUE)
				.withRenderPass(nullptr, nullptr);

			auto _wvk_result = _wvk_cmd_buffers[0]->begin(_helper);

			_wvk_cmd_buffers[0]->end();

			EXPECT_EQ(_wvk_result, true);
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// Проверка:
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		TEST_F(WvkCommandBufferTest, beginHelper_secondary_usageAndOcclusionAndRenderingColor) {
			CGDev::wvk::WvkCommandBufferPtrVec1 _wvk_cmd_buffers(1, nullptr);
			wvk_command_pool_ptr->allocateWvkCommandBuffers(_wvk_cmd_buffers, VK_COMMAND_BUFFER_LEVEL_SECONDARY);

			CGDev::wvk::WvkCommandBufferBeginHelper _helper;
			_helper.withUsage(VK_COMMAND_BUFFER_USAGE_ONE_TIME_SUBMIT_BIT)
				.withOcclusionQuery(VK_TRUE)
				.withRenderingColor({ nullptr, nullptr });

			auto _wvk_result = _wvk_cmd_buffers[0]->begin(_helper);

			_wvk_cmd_buffers[0]->end();

			EXPECT_EQ(_wvk_result, true);
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// Проверка:
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		TEST_F(WvkCommandBufferTest, beginHelper_secondary_usageAndOcclusionAndRenderingDepth) {
			CGDev::wvk::WvkCommandBufferPtrVec1 _wvk_cmd_buffers(1, nullptr);
			wvk_command_pool_ptr->allocateWvkCommandBuffers(_wvk_cmd_buffers, VK_COMMAND_BUFFER_LEVEL_SECONDARY);

			CGDev::wvk::WvkCommandBufferBeginHelper _helper;
			_helper.withUsage(VK_COMMAND_BUFFER_USAGE_ONE_TIME_SUBMIT_BIT)
				.withOcclusionQuery(VK_TRUE)
				.withRenderingDepth(nullptr);

			auto _wvk_result = _wvk_cmd_buffers[0]->begin(_helper);

			_wvk_cmd_buffers[0]->end();

			EXPECT_EQ(_wvk_result, true);
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// Проверка:
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		TEST_F(WvkCommandBufferTest, beginHelper_secondary_usageAndRenderpassAndRenderingColor) {
			CGDev::wvk::WvkCommandBufferPtrVec1 _wvk_cmd_buffers(1, nullptr);
			wvk_command_pool_ptr->allocateWvkCommandBuffers(_wvk_cmd_buffers, VK_COMMAND_BUFFER_LEVEL_SECONDARY);

			CGDev::wvk::WvkCommandBufferBeginHelper _helper;
			_helper.withUsage(VK_COMMAND_BUFFER_USAGE_ONE_TIME_SUBMIT_BIT | VK_COMMAND_BUFFER_USAGE_RENDER_PASS_CONTINUE_BIT)
				.withRenderPass(nullptr, nullptr)
				.withRenderingColor({ nullptr });

			auto _wvk_result = _wvk_cmd_buffers[0]->begin(_helper);

			_wvk_cmd_buffers[0]->end();

			EXPECT_EQ(_wvk_result, true);
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// Проверка:
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		TEST_F(WvkCommandBufferTest, beginHelper_primary_usageAndOcclusionAndRenderingDepth) {
			CGDev::wvk::WvkCommandBufferPtrVec1 _wvk_cmd_buffers(1, nullptr);
			wvk_command_pool_ptr->allocateWvkCommandBuffers(_wvk_cmd_buffers, VK_COMMAND_BUFFER_LEVEL_PRIMARY);

			CGDev::wvk::WvkCommandBufferBeginHelper _helper;
			_helper.withUsage(VK_COMMAND_BUFFER_USAGE_ONE_TIME_SUBMIT_BIT)
				.withOcclusionQuery(VK_TRUE)
				.withRenderingDepth(nullptr);

			auto _wvk_result = _wvk_cmd_buffers[0]->begin(_helper);

			_wvk_cmd_buffers[0]->end();

			EXPECT_EQ(_wvk_result, true);
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// Проверка:
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		TEST_F(WvkCommandBufferTest, beginAndHelper_primary_usageAndGroupAndOcclusionAndRenderingDepthAndLocation) {
			CGDev::wvk::WvkCommandBufferPtrVec1 _wvk_cmd_buffers(1, nullptr);
			wvk_command_pool_ptr->allocateWvkCommandBuffers(_wvk_cmd_buffers, VK_COMMAND_BUFFER_LEVEL_PRIMARY);

			VkDeviceGroupCommandBufferBeginInfo _group_info = {
				.sType = VK_STRUCTURE_TYPE_DEVICE_GROUP_COMMAND_BUFFER_BEGIN_INFO,
				.pNext = nullptr,
				.deviceMask = 1u << 0,
			};

			std::vector<uint32_t> _location = { 1,3 };
			VkRenderingAttachmentLocationInfo _location_info = {
				.sType = VK_STRUCTURE_TYPE_RENDERING_ATTACHMENT_LOCATION_INFO,
				.pNext = nullptr,
				.colorAttachmentCount = static_cast<uint32_t>(_location.size()),
				.pColorAttachmentLocations = _location.data(),
			};

			CGDev::wvk::WvkCommandBufferBeginHelper _helper;
			_helper.withUsage(VK_COMMAND_BUFFER_USAGE_ONE_TIME_SUBMIT_BIT)
				.withOcclusionQuery(VK_TRUE)
				.withRenderingDepth(nullptr)
				.withInheritanceNext(_location_info);

			auto _wvk_result = _wvk_cmd_buffers[0]->begin(_helper);

			_wvk_cmd_buffers[0]->end();

			EXPECT_EQ(_wvk_result, true);
		}

	} // namespace tests

} // namespace CGDev