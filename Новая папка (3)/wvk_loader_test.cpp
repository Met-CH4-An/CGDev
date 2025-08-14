////////////////////////////////////////////////////////////////
// секция форвард-декларации
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// секция заголовочного файла
////////////////////////////////////////////////////////////////
#include "wvk_loader_test.h"
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

        TEST_F(WvkLoaderTest, loadProcedure_ok) {
            PFN_vkEnumerateInstanceVersion _vkEnumerateInstanceVersion = VK_NULL_HANDLE;
            PFN_vkEnumerateInstanceLayerProperties _vkEnumerateInstanceLayerProperties = VK_NULL_HANDLE;
            PFN_vkEnumerateInstanceExtensionProperties _vkEnumerateInstanceExtensionProperties = VK_NULL_HANDLE;
            PFN_vkCreateInstance _vkCreateInstance = VK_NULL_HANDLE;

            std::vector<CGDev::wvk::WvkVulkanProcedureInfo> _procedures = {
                { "vkEnumerateInstanceVersion", reinterpret_cast<void**>(&_vkEnumerateInstanceVersion) },
                { "vkEnumerateInstanceLayerProperties", reinterpret_cast<void**>(&_vkEnumerateInstanceLayerProperties) },
                { "vkEnumerateInstanceExtensionProperties", reinterpret_cast<void**>(&_vkEnumerateInstanceExtensionProperties) },
                { "vkCreateInstance", reinterpret_cast<void**>(&_vkCreateInstance) }
            };
            CGDev::wvk::WvkStatus _status = s_wvk_loader->loadProcedure(VK_NULL_HANDLE, _procedures);

            EXPECT_EQ(_status.isOk(), true);
            EXPECT_NE(_vkEnumerateInstanceVersion, VK_NULL_HANDLE);
            EXPECT_NE(_vkEnumerateInstanceLayerProperties, VK_NULL_HANDLE);
            EXPECT_NE(_vkEnumerateInstanceExtensionProperties, VK_NULL_HANDLE);
            EXPECT_NE(_vkCreateInstance, VK_NULL_HANDLE);
        }

        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

        TEST_F(WvkLoaderTest, loadProcedure_fail) {
            PFN_vkEnumerateInstanceVersion _vkEnumerateInstanceVersion = nullptr;
            PFN_vkEnumerateInstanceLayerProperties _vkEnumerateInstanceLayerProperties = nullptr;
            PFN_vkEnumerateInstanceExtensionProperties _vkEnumerateInstanceExtensionProperties = nullptr;
            PFN_vkCreateInstance _vkCreateInstance = nullptr;

            std::vector<CGDev::wvk::WvkVulkanProcedureInfo> _procedures = {
                { "vkEnumerateInstanceVersionFail", reinterpret_cast<void**>(&_vkEnumerateInstanceVersion) },
                { "vkEnumerateInstanceLayerPropertiesFail", reinterpret_cast<void**>(&_vkEnumerateInstanceLayerProperties) },
                { "vkEnumerateInstanceExtensionPropertiesFail", reinterpret_cast<void**>(&_vkEnumerateInstanceExtensionProperties) },
                { "vkCreateInstanceFail", reinterpret_cast<void**>(&_vkCreateInstance) }
            };
            CGDev::wvk::WvkStatus _status = s_wvk_loader->loadProcedure(VK_NULL_HANDLE, _procedures);

            EXPECT_EQ(_status.isOk(), false);
            EXPECT_EQ(_vkEnumerateInstanceVersion, VK_NULL_HANDLE);
            EXPECT_EQ(_vkEnumerateInstanceLayerProperties, VK_NULL_HANDLE);
            EXPECT_EQ(_vkEnumerateInstanceExtensionProperties, VK_NULL_HANDLE);
            EXPECT_EQ(_vkCreateInstance, VK_NULL_HANDLE);
        }

        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

    } // namespace tests
}
