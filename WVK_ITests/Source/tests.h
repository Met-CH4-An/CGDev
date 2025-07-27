#pragma once
#include <iomanip>

#include "gtest/gtest.h"
#include "gmock/gmock.h"

// --gtest_filter=WvkPhysicalDeviceTest.RequestPhysicalDeviceProperties_FillsPropertiesCorrectly
#include <windows.h>

#include "wvk_loader.h"
#include "wvk_loader_dispatch_table.h"
#include "wvk_instance.h"
#include "wvk_instance_dispatch_table.h"
#include "wvk_physical_device.h"
#include "wvk_dispatch_table.hpp"
#include "wvk_queue_family.h"
#include "wvk_logical_device.h"

#include "Extensions/wvk_ext_debug_utils.h"
#include "Extensions/wvk_khr_get_physical_device_properties2_dispatch_table.hpp"
#include "Extensions/wvk_khr_surface_dt.hpp"
#include "Extensions/wvk_khr_get_surface_capabilities2_dt.hpp"
#include "Extensions/wvk_surface.h"

#include "Extensions/MSWindows/wvk_khr_win32_surface_dispatch_table.h"
#include "Extensions/MSWindows/wvk_surface_mswindows.h"

inline bool CreateTestWindow(HINSTANCE& outInstance, HWND& outHwnd);

class WvkBaseTest : public ::testing::Environment {

protected:

    void SetUp() {
        if (s_wvk_loader == nullptr) {
            s_wvk_loader = std::make_unique<CGDev::wvk::WvkLoader>();

            CGDev::wvk::WvkLoaderCreateInfo _create_info;

            auto _wvk_res = s_wvk_loader->create(_create_info);

            ASSERT_EQ(_wvk_res.isOk(), true);
        }

        if (s_wvk_loader_dispatch_table == nullptr) {
            s_wvk_loader_dispatch_table = std::make_unique<CGDev::wvk::WvkLoaderDispatchTable>();

            CGDev::wvk::WvkLoaderDispatchTableCreateInfo _create_info;
            _create_info.wvk_loader = s_wvk_loader.get();

            auto _wvk_res = s_wvk_loader_dispatch_table->create(_create_info);

            ASSERT_EQ(_wvk_res.isOk(), true);
        }

        if (s_wvk_instance == nullptr) {
            s_wvk_instance = std::make_unique<CGDev::wvk::WvkInstance>();

            CGDev::wvk::WvkInstanceCreateInfo _create_info;
            _create_info.wvk_loader_dispatch_table = s_wvk_loader_dispatch_table.get();

            auto _wvk_res = s_wvk_instance->create(_create_info);

            ASSERT_EQ(_wvk_res.isOk(), true);
        }
        
        createExtensions();
        
        // WvkInstanceDispatchTable
        {
            CGDev::wvk::WvkInstanceDtCreateInfo _create_info;
            _create_info.wvk_loader = s_wvk_loader.get();
            _create_info.wvk_instance = s_wvk_instance.get();

            auto _wvk_res = wvk_instance_dispatch_table.create(_create_info);

            ASSERT_EQ(_wvk_res.isOk(), true);
        }

        // WvkExtDebugUtils
        {
            CGDev::wvk::Extensions::WvkExtDebugUtilsCreateInfo _create_info;
            _create_info.wvk_instance_dispatch_table = &wvk_instance_dispatch_table;
            _create_info.mode = CGDev::wvk::Extensions::VknDebugUtilsMode::ALL_SEVERITIES | CGDev::wvk::Extensions::VknDebugUtilsMode::ALL_TYPES;
            
            auto _wvk_res = wvk_ext_debug_utils.create(_create_info);
            ASSERT_EQ(_wvk_res.isOk(), true);
        }

        // WvkPhysicalDevice
        {
            uint32_t _count = 1;
            std::vector<VkPhysicalDevice> _vk_physical_device_collection1(1);
            wvk_instance_dispatch_table.wvkEnumeratePhysicalDevices(&_count, _vk_physical_device_collection1.data());

            CGDev::wvk::WvkPhysicalDeviceCreateInfo _create_info;
            _create_info.vk_physical_device = _vk_physical_device_collection1[0];
            _create_info.wvk_instance_dispatch_table = &wvk_instance_dispatch_table;

            auto _wvk_res = wvk_physical_device.create(_create_info);

            ASSERT_EQ(_wvk_res.isOk(), true);
        }

        // WvkQueueFamily
        {
            CGDev::wvk::WvkQueueFamilyCreateInfo _create_info;
            _create_info.index = 0;
            _create_info.instance_dt_ptr = &wvk_instance_dispatch_table;
            _create_info.wvk_physical_device_ptr = &wvk_physical_device;

            auto _wvk_res = wvk_queue_family.create(_create_info);

            ASSERT_EQ(_wvk_res.isOk(), true);
        }

        // WvkLogicalDevice
        {
            auto f16 = VkPhysicalDevice16BitStorageFeatures{
                .sType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_16BIT_STORAGE_FEATURES,
                .storageBuffer16BitAccess = VK_TRUE
            }; 
            auto f16_ = VkPhysicalDevice16BitStorageFeatures{
                .sType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_16BIT_STORAGE_FEATURES,
                .storageBuffer16BitAccess = VK_TRUE
            };
            auto f8 = VkPhysicalDevice8BitStorageFeatures{
                .sType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_8BIT_STORAGE_FEATURES,
                .storageBuffer8BitAccess = VK_TRUE
            };
            auto f11 = VkPhysicalDeviceVulkan11Features{
                .sType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_1_FEATURES,
				.multiview = VK_TRUE
            };
            auto f12 = VkPhysicalDeviceVulkan12Features{
                .sType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_2_FEATURES,
                .bufferDeviceAddressCaptureReplay = VK_TRUE
            };

            //auto f16 = CGDev::wvk::WvkLogicalDeviceFeature::make("VK_KHR_16bit_storage", );
            //VkPhysicalDevice16BitStorageFeatures f16{};
            //f16.sType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_16BIT_STORAGE_FEATURES;
            //f16.pNext = nullptr;
            //f16.storageBuffer16BitAccess = VK_TRUE;
            //CGDev::wvk::WvkLogicalDeviceFeature::make("VK_KHR_16bit_storage", f16);
            //CGDev::wvk::WvkLogicalDeviceFeatureVec1 m_wvk_logical_device_feature_collection;
            CGDev::wvk::WvkLogicalDeviceQueueCreateInfo _queue_create_info = {};
            _queue_create_info.wvk_queue_family_ptr = &wvk_queue_family;
            _queue_create_info.queue_count = 1;
            _queue_create_info.priority_collection = { 1.0f };
            
            CGDev::wvk::WvkLogicalDeviceCreateInfo _create_info;
            _create_info.wvk_instance_dispatch_table = &wvk_instance_dispatch_table;
            _create_info.wvk_physical_device_collection.push_back(&wvk_physical_device);
            _create_info.wvk_logical_device_queue_create_info_collection.push_back(_queue_create_info);
			_create_info.m_wvk_logical_device_feature_collection.push_back(CGDev::wvk::WvkLogicalDeviceFeature("VK_KHR_16bit_storage", f16));
            _create_info.m_wvk_logical_device_feature_collection.push_back(CGDev::wvk::WvkLogicalDeviceFeature("VK_KHR_8bit_storage", f8));
            _create_info.m_wvk_logical_device_feature_collection.push_back(CGDev::wvk::WvkLogicalDeviceFeature("VK_KHR_8bit_storage", f11));
            //_create_info.m_wvk_logical_device_feature_collection.push_back(CGDev::wvk::WvkLogicalDeviceFeature("VK_KHR_8bit_storage", f12));
            
            auto _wvk_res = wvk_logical_device.create(_create_info);

           //ASSERT_EQ(_wvk_res.isOk(), true);
        }

        if (s_wvk_surface == nullptr) {
            HINSTANCE _hInstance = NULL;
            HWND _hWnd = NULL;

            CreateTestWindow(_hInstance, _hWnd);

            s_wvk_surface = std::make_unique<CGDev::wvk::Extensions::WvkSurface>();

            CGDev::wvk::Extensions::mswindows::WvkSurfaceMSWindowsCreateInfo _create_info_mswindows;
            _create_info_mswindows.wvk_khr_win32_surface_dispatch_table = s_wvk_khr_win32_surface_dispatch_table.get();
            _create_info_mswindows.hInstance = _hInstance;
            _create_info_mswindows.hWnd = _hWnd;

            CGDev::wvk::Extensions::WvkSurfaceCreateInfo _create_info;
            _create_info.wvk_instance_dt_ptr = &wvk_instance_dispatch_table;
            _create_info.wvk_khr_surface_dt = s_wvk_khr_surface_dt.get();
            _create_info.wvk_khr_get_surface_capabilities2_dispatch_table = s_wvk_khr_get_surface_capabilities2_dispatch_table.get();
            _create_info.wvk_surface_platform_create_info = &_create_info_mswindows;

            auto _wvk_res = s_wvk_surface->create(_create_info);

            ASSERT_EQ(_wvk_res.isOk(), true);
        }   
    }

    void createExtensions() {
        if (s_wvk_khr_surface_dt == nullptr) {
            s_wvk_khr_surface_dt = std::make_unique<CGDev::wvk::Extensions::WvkKhrSurfaceDT>();

            CGDev::wvk::Extensions::WvkKhrSurfaceDTCreateInfo _create_info;
            _create_info.wvk_loader = s_wvk_loader.get();
            _create_info.wvk_instance = s_wvk_instance.get();

            auto _wvk_res = s_wvk_khr_surface_dt->create(_create_info);

            ASSERT_EQ(_wvk_res.isOk(), true);
        }

        if (s_wvk_khr_win32_surface_dispatch_table == nullptr) {
            s_wvk_khr_win32_surface_dispatch_table = std::make_unique<CGDev::wvk::Extensions::mswindows::WvkKhrWin32SurfaceDispatchTable>();

            CGDev::wvk::Extensions::mswindows::WvkKhrWin32SurfaceDispatchTableCreateInfo _create_info;
            _create_info.wvk_loader = s_wvk_loader.get();
            _create_info.wvk_instance = s_wvk_instance.get();

            auto _wvk_res = s_wvk_khr_win32_surface_dispatch_table->create(_create_info);

            ASSERT_EQ(_wvk_res.isOk(), true);
        }

        if (s_wvk_khr_get_surface_capabilities2_dispatch_table == nullptr) {
            s_wvk_khr_get_surface_capabilities2_dispatch_table = std::make_unique<CGDev::wvk::Extensions::WvkKhrGetSurfaceCapabilities2DT>();

            CGDev::wvk::Extensions::WvkKhrGetSurfaceCapabilities2DTCreateInfo _create_info;
            _create_info.wvk_loader = s_wvk_loader.get();
            _create_info.wvk_instance = s_wvk_instance.get();

            auto _wvk_res = s_wvk_khr_get_surface_capabilities2_dispatch_table->create(_create_info);

            ASSERT_EQ(_wvk_res.isOk(), true);
        }

        if (s_wvk_khr_get_phys_dev_props2_dt == nullptr) {
            s_wvk_khr_get_phys_dev_props2_dt = std::make_unique<CGDev::wvk::Extensions::WvkKhrGetPhysicalDeviceProperties2DispatchTable>();

            CGDev::wvk::Extensions::WvkKhrGetPhysicalDeviceProperties2DTCreateInfo _create_info;
            _create_info.wvk_loader = s_wvk_loader.get();
            _create_info.wvk_instance = s_wvk_instance.get();

            auto _wvk_res = s_wvk_khr_get_phys_dev_props2_dt->create(_create_info);

            ASSERT_EQ(_wvk_res.isOk(), true);
        }
    }

protected:

    inline static CGDev::wvk::WvkLoaderUptr s_wvk_loader = nullptr;
    inline static CGDev::wvk::WvkLoaderDispatchTableUptr s_wvk_loader_dispatch_table = nullptr;
    inline static CGDev::wvk::WvkInstanceUptr s_wvk_instance = nullptr;
    inline static CGDev::wvk::WvkInstanceDispatchTable wvk_instance_dispatch_table;
    inline static CGDev::wvk::WvkPhysicalDevice wvk_physical_device;
    inline static std::unique_ptr<CGDev::wvk::WvkDispatchTable> s_wvk_dispatch_table = nullptr;
    inline static CGDev::wvk::WvkQueueFamily wvk_queue_family;
    inline static CGDev::wvk::WvkLogicalDevice wvk_logical_device;
    
    inline static CGDev::wvk::Extensions::WvkExtDebugUtils wvk_ext_debug_utils;
    inline static CGDev::wvk::Extensions::WvkKhrGetPhysicalDeviceProperties2DispatchTableUptr s_wvk_khr_get_phys_dev_props2_dt = nullptr;
    inline static CGDev::wvk::Extensions::WvkKhrSurfaceDTUptr s_wvk_khr_surface_dt = nullptr;
    inline static CGDev::wvk::Extensions::WvkKhrGetSurfaceCapabilities2DispatchTableUptr s_wvk_khr_get_surface_capabilities2_dispatch_table = nullptr;
    inline static CGDev::wvk::Extensions::WvkSurfaceUptr s_wvk_surface = nullptr;
    
    inline static CGDev::wvk::Extensions::mswindows::WvkKhrWin32SurfaceDispatchTableUptr s_wvk_khr_win32_surface_dispatch_table = nullptr;
};




// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Шаг 1. Глобальный флаг для завершения цикла
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
static bool g_running = true;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Шаг 2. Обработка сообщений окна (закрытие по ESC или крестик)
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
inline static LRESULT CALLBACK WindowProc(HWND hwnd, UINT msg, WPARAM wParam, LPARAM lParam) {
    switch (msg) {
    case WM_DESTROY:
        PostQuitMessage(0);
        return 0;
    case WM_KEYDOWN:
        if (wParam == VK_ESCAPE) {
            PostQuitMessage(0);
            return 0;
        }
        break;
    }
    return DefWindowProc(hwnd, msg, wParam, lParam);
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Шаг 3. Функция создания окна
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
inline bool CreateTestWindow(HINSTANCE& outInstance, HWND& outHwnd) {
    const wchar_t* className = L"TestWindowClass";

    // Получаем дескриптор текущего инстанса
    HINSTANCE hInstance = GetModuleHandle(nullptr);

    // Регистрация класса окна
    WNDCLASS wc = {};
    wc.lpfnWndProc = WindowProc;
    wc.hInstance = hInstance;
    wc.lpszClassName = className;
    RegisterClass(&wc);

    // Устанавливаем клиентскую область 800x600
    RECT rect = { 0, 0, 800, 600 };
    AdjustWindowRect(&rect, WS_OVERLAPPEDWINDOW, FALSE);
    int width = rect.right - rect.left;
    int height = rect.bottom - rect.top;

    // Создание окна
    HWND hwnd = CreateWindowEx(
        0, className, L"Test Window",
        WS_OVERLAPPEDWINDOW,
        CW_USEDEFAULT, CW_USEDEFAULT, width, height,
        nullptr, nullptr, hInstance, nullptr
    );

    if (!hwnd) return false;

    ShowWindow(hwnd, SW_SHOW);

    // Возврат параметров
    outInstance = hInstance;
    outHwnd = hwnd;
    return true;
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Шаг 4. Пример использования в интеграционном тесте
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
//inline int WINAPI WinMain(HINSTANCE, HINSTANCE, LPSTR, int) {
//    HWND hwnd;
//    HINSTANCE hInstance;
//    if (!CreateTestWindow(hInstance, hwnd)) return -1;

    // Главный цикл сообщений
//    MSG msg = {};
//    while (g_running) {
//        while (PeekMessage(&msg, nullptr, 0, 0, PM_REMOVE)) {
//            if (msg.message == WM_QUIT) {
//                g_running = false;
//                break;
//            }
 //           TranslateMessage(&msg);
//            DispatchMessage(&msg);
//        }

        // Здесь может быть твоя Vulkan-интеграция
//    }

//    return 0;
//}

