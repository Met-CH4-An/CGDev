////////////////////////////////////////////////////////////////
// ������ �������-����������
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// ������ ������������� �����
////////////////////////////////////////////////////////////////
#include "wvk_instance_dispatch_table_test.h"
////////////////////////////////////////////////////////////////
// ������ �������������
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// ������ ��� ����������
////////////////////////////////////////////////////////////////

namespace CGDev {

    namespace tests {

        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

        TEST_F(WvkInstanceDispatchTableTest, CallsVulkan10) {
            uint32_t deviceCount = 0;
            s_wvk_instance_dispatch_table->wvkEnumeratePhysicalDevices(&deviceCount, nullptr);
            EXPECT_NE(deviceCount, 0);
        }

        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
       
        TEST_F(WvkInstanceDispatchTableTest, CallsVulkan11_WhenBuiltForVulkan10) {
            if constexpr (CGDev::wvk::Build::WvkBuildInfo::vulkan_api_version == CGDev::wvk::Build::VulkanVersion::VERSION_10) {
                uint32_t deviceCount = 0;
                s_wvk_instance_dispatch_table->wvkEnumeratePhysicalDeviceGroups(&deviceCount, nullptr);
                EXPECT_EQ(deviceCount, 0);
            }
        }

        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

        TEST_F(WvkInstanceDispatchTableTest, CallsVulkanExtension_WhenBuiltForVulkan10) {
            if constexpr (CGDev::wvk::Build::WvkBuildInfo::find("VK_KHR_device_group_creation")) {
                uint32_t deviceCount = 0;
                s_wvk_instance_dispatch_table->wvkEnumeratePhysicalDeviceGroups(&deviceCount, nullptr);
                EXPECT_NE(deviceCount, 0);
            }
        }

        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    } // namespace tests
}
