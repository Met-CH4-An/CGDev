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
                    .m_vk_physical_device_features = {},
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
                };

                wvk_command_pool_ptr = std::make_unique<CGDev::wvk::WvkCommandPool>();
                auto _wvk_res = wvk_command_pool_ptr->create(_create_info);
                
                ASSERT_EQ(_wvk_res, true);
            }
        }

        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

    } // namespace tests
} // namespace CGDev
   