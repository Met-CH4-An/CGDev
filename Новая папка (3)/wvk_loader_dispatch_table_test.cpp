////////////////////////////////////////////////////////////////
// секция форвард-декларации
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// секция заголовочного файла
////////////////////////////////////////////////////////////////
#include "wvk_loader_dispatch_table_test.h"
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

        TEST_F(WvkLoaderDispatchTableTest, print_instanceLayerProperties) {
            // Шаг 1: Узнаём количество слоёв
            uint32_t layer_count = 0;
            s_wvk_loader_dispatch_table.wvkEnumerateInstanceLayerProperties(&layer_count, nullptr);

            // Шаг 2: Получаем список слоёв
            std::vector<VkLayerProperties> layers(layer_count);
            s_wvk_loader_dispatch_table.wvkEnumerateInstanceLayerProperties(&layer_count, layers.data());

             // Шаг 3: Выводим информацию
            std::cout << "Доступные Vulkan-слои:\n";
            for (const auto& layer : layers) {
                std::cout << "----------------------------------------\n";
                std::cout << "Имя:        " << layer.layerName << "\n";
                std::cout << "Описание:   " << layer.description << "\n";
                std::cout << "Версия API: " << VK_VERSION_MAJOR(layer.specVersion) << "."
                    << VK_VERSION_MINOR(layer.specVersion) << "."
                    << VK_VERSION_PATCH(layer.specVersion) << "\n";
                std::cout << "Версия:     " << layer.implementationVersion << "\n";
            }
        }

        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

        TEST_F(WvkLoaderDispatchTableTest, print_instanceExtensionProperties) {
            // Шаг 1: Узнаём количество расширений
            uint32_t extension_count = 0;
            s_wvk_loader_dispatch_table.wvkEnumerateInstanceExtensionProperties(nullptr, &extension_count, nullptr);

            // Шаг 2: Получаем список расширений
            std::vector<VkExtensionProperties> extensions(extension_count);
            s_wvk_loader_dispatch_table.wvkEnumerateInstanceExtensionProperties(nullptr, &extension_count, extensions.data());

            // Шаг 3: Выводим информацию
            std::cout << "Доступные Vulkan-расширения:\n";
            for (const auto& ext : extensions) {
                std::cout << "----------------------------------------\n";
                std::cout << "Имя:        " << ext.extensionName << "\n";
                std::cout << "Версия:     " << VK_VERSION_MAJOR(ext.specVersion) << "."
                    << VK_VERSION_MINOR(ext.specVersion) << "."
                    << VK_VERSION_PATCH(ext.specVersion) << "\n";
            }
        }

        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    } // namespace tests
}
