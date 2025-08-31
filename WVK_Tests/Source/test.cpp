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
#include "tests.h"
////////////////////////////////////////////////////////////////
// секция имплементации
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// секция для остального
////////////////////////////////////////////////////////////////
#include "wvk_instance.h"
#include "wvk_logical_device.h"
#include "wvk_command_pool.h"
#include "wvk_shader.h"

#include "Extensions/wvk_debug_utils_messenger.h"

::testing::Environment* g_vk_env = nullptr;

GTEST_API_ int main(int argc, char** argv) {
	printf("Running main() from %s\n", __FILE__);
	
	testing::InitGoogleTest(&argc, argv);
	setlocale(LC_ALL, "");
	SetConsoleOutputCP(CP_UTF8);  // Установка UTF-8 для вывода в консоль

	g_vk_env = ::testing::AddGlobalTestEnvironment(new CGDev::tests::WvkBaseTest());

	return RUN_ALL_TESTS();
}

namespace CGDev {

    namespace tests {

        CGDev::wvk::WvkInstanceUptr WvkBaseTest::wvk_instance_ptr;
        CGDev::wvk::WvkLogicalDeviceUptr WvkBaseTest::wvk_logical_device_ptr;
        CGDev::wvk::WvkCommandPoolUptr WvkBaseTest::wvk_command_pool_ptr;
        CGDev::wvk::WvkShaderUptr WvkBaseTest::wvk_shader_ptr;
        CGDev::wvk::Extensions::WvkDebugUtilsMessengerUptr WvkBaseTest::wvk_debug_utils_messenger_ptr;

        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

        void WvkBaseTest::SetUp() noexcept {

            // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            // WvkInstance
            // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            {
                CGDev::wvk::WvkInstanceCreateInfo _create_info;

                wvk_instance_ptr = std::make_unique<CGDev::wvk::WvkInstance>();
                auto _wvk_res = wvk_instance_ptr->create(_create_info);

                ASSERT_EQ(_wvk_res, true);
            }

            // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            // WvkDebugUtilsMessenger
            // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            {
                CGDev::wvk::Extensions::WvkDebugUtilsMessengerCreateInfo _create_info = {
                    .wvk_instance_ptr = wvk_instance_ptr.get(),
                    .mode = CGDev::wvk::Extensions::VknDebugUtilsMode::ALL_SEVERITIES | CGDev::wvk::Extensions::VknDebugUtilsMode::ALL_TYPES,
                };

                wvk_debug_utils_messenger_ptr = std::make_unique<CGDev::wvk::Extensions::WvkDebugUtilsMessenger>();
                auto _wvk_res = wvk_debug_utils_messenger_ptr->create(_create_info);

                //ASSERT_EQ(_wvk_res, true);
            }

            //std::vector<VkQueueFamilyProperties> _props;
            //wvk_instance_ptr->getWvkPhysicalDevices()[0][0].get()->requestQueueFamilyProperties(_props);

            // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            // WvkLogicalDevice
            // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            {
                CGDev::wvk::WvkLogicalDeviceQueueCreateInfo _queue_create_info = {
                    .index = 0,
                    .queue_count = 1,
                    .priority_collection = { 1.0f },
                };

                CGDev::wvk::WvkLogicalDeviceCreateInfo _create_info = {
					.wvk_instance_ptr = wvk_instance_ptr.get(),
                    .physical_device_group_index = 0,
                    .physical_device_indices = { 0 },
                    .wvk_logical_device_queue_create_infos = { _queue_create_info },
                    .m_vk_physical_device_features = {
                        .inheritedQueries = VK_TRUE },
                    .m_wvk_logical_device_feature_collection = {},
                };

                wvk_logical_device_ptr = std::make_unique<CGDev::wvk::WvkLogicalDevice>();
                auto _wvk_res = wvk_logical_device_ptr->create(_create_info);

                ASSERT_EQ(_wvk_res, true);
            }

            // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            // WvkCommandPool
            // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            {
                CGDev::wvk::WvkCommandPoolCreateInfo _create_info = {
                    .wvk_logical_device_ptr = wvk_logical_device_ptr.get(),
                    .queue_family_index = 0,
					.flags = VK_COMMAND_POOL_CREATE_RESET_COMMAND_BUFFER_BIT,
                };

                wvk_command_pool_ptr = std::make_unique<CGDev::wvk::WvkCommandPool>();
                auto _wvk_res = wvk_command_pool_ptr->create(_create_info);
                
                ASSERT_EQ(_wvk_res, true);
            }

            // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            // WvkCommandBuffer
            // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            {
                CGDev::wvk::WvkCommandBufferPtrVec1 _wvk_cmd_buffers(1, nullptr);

                auto _wvk_res = wvk_command_pool_ptr->allocateWvkCommandBuffers(_wvk_cmd_buffers, VK_COMMAND_BUFFER_LEVEL_PRIMARY);

                ASSERT_EQ(_wvk_res, true);
            }

            // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            // WvkShader
            // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            {
                auto _create_info = CGDev::wvk::WvkShaderHelper::graphics(wvk_logical_device_ptr.get())
                    .vertex("D:\\Laboratory\\CGDev\\Extern\\triangle_vs.spv")
                    .fragment("D:\\Laboratory\\CGDev\\Extern\\triangle_fs.spv")
                    .build();

                wvk_shader_ptr = std::make_unique<CGDev::wvk::WvkShader>();
                auto _wvk_res = wvk_shader_ptr->create(_create_info);

                ASSERT_EQ(_wvk_res, true);
            }
        }

        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

    } // namespace tests
} // namespace CGDev
   