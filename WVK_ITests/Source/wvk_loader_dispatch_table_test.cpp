////////////////////////////////////////////////////////////////
// ������ �������-����������
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// ������ ������������� �����
////////////////////////////////////////////////////////////////
#include "wvk_loader_dispatch_table_test.h"
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

        TEST_F(WvkLoaderDispatchTableTest, print_instanceLayerProperties) {
            // ��� 1: ����� ���������� ����
            uint32_t layer_count = 0;
            s_wvk_loader_dispatch_table.wvkEnumerateInstanceLayerProperties(&layer_count, nullptr);

            // ��� 2: �������� ������ ����
            std::vector<VkLayerProperties> layers(layer_count);
            s_wvk_loader_dispatch_table.wvkEnumerateInstanceLayerProperties(&layer_count, layers.data());

             // ��� 3: ������� ����������
            std::cout << "��������� Vulkan-����:\n";
            for (const auto& layer : layers) {
                std::cout << "----------------------------------------\n";
                std::cout << "���:        " << layer.layerName << "\n";
                std::cout << "��������:   " << layer.description << "\n";
                std::cout << "������ API: " << VK_VERSION_MAJOR(layer.specVersion) << "."
                    << VK_VERSION_MINOR(layer.specVersion) << "."
                    << VK_VERSION_PATCH(layer.specVersion) << "\n";
                std::cout << "������:     " << layer.implementationVersion << "\n";
            }
        }

        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

        TEST_F(WvkLoaderDispatchTableTest, print_instanceExtensionProperties) {
            // ��� 1: ����� ���������� ����������
            uint32_t extension_count = 0;
            s_wvk_loader_dispatch_table.wvkEnumerateInstanceExtensionProperties(nullptr, &extension_count, nullptr);

            // ��� 2: �������� ������ ����������
            std::vector<VkExtensionProperties> extensions(extension_count);
            s_wvk_loader_dispatch_table.wvkEnumerateInstanceExtensionProperties(nullptr, &extension_count, extensions.data());

            // ��� 3: ������� ����������
            std::cout << "��������� Vulkan-����������:\n";
            for (const auto& ext : extensions) {
                std::cout << "----------------------------------------\n";
                std::cout << "���:        " << ext.extensionName << "\n";
                std::cout << "������:     " << VK_VERSION_MAJOR(ext.specVersion) << "."
                    << VK_VERSION_MINOR(ext.specVersion) << "."
                    << VK_VERSION_PATCH(ext.specVersion) << "\n";
            }
        }

        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    } // namespace tests
}
