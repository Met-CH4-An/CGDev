////////////////////////////////////////////////////////////////
// секция форвард-декларации
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// секция заголовочного файла
////////////////////////////////////////////////////////////////
#include "wvk_instance_dispatch_table_test.h"
////////////////////////////////////////////////////////////////
// секция имплементации
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// секция для остального
////////////////////////////////////////////////////////////////

namespace CGDev {

    namespace tests {

        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

        TEST_F(WvkInstanceDispatchTableTest, CallsVulkan10) {
            uint32_t deviceCount = 0;
            wvk_instance_dispatch_table.wvkEnumeratePhysicalDevices(&deviceCount, nullptr);
            EXPECT_NE(deviceCount, 0);
        }

        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
       
        TEST_F(WvkInstanceDispatchTableTest, CallsVulkan11_WhenBuiltForVulkan10) {
            if constexpr (CGDev::wvk::Build::vulkan_api_version == CGDev::wvk::Build::VulkanVersion::VERSION_10) {
                uint32_t deviceCount = 0;
                wvk_instance_dispatch_table.wvkEnumeratePhysicalDeviceGroups(&deviceCount, nullptr);
                EXPECT_EQ(deviceCount, 0);
            }
        }

        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

        TEST_F(WvkInstanceDispatchTableTest, CallsVulkanExtension_WhenBuiltForVulkan10) {
            if constexpr (CGDev::wvk::Build::find("VK_KHR_device_group_creation")) {
                uint32_t deviceCount = 0;
                wvk_instance_dispatch_table.wvkEnumeratePhysicalDeviceGroups(&deviceCount, nullptr);
                EXPECT_NE(deviceCount, 0);
            }
        }

        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    } // namespace tests
}
