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

#include "Extensions/wvk_khr_surface_dispatch_table.h"
#include "Extensions/wvk_khr_get_surface_capabilities2_dispatch_table.h"
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

        if (s_wvk_instance_dispatch_table == nullptr) {
            s_wvk_instance_dispatch_table = std::make_unique<CGDev::wvk::WvkInstanceDispatchTable>();

            CGDev::wvk::WvkInstanceDispatchTableCreateInfo _create_info;
            _create_info.wvk_loader = s_wvk_loader.get();
            _create_info.wvk_instance = s_wvk_instance.get();

            auto _wvk_res = s_wvk_instance_dispatch_table->create(_create_info);

            ASSERT_EQ(_wvk_res.isOk(), true);
        }

        if (s_wvk_physical_device == nullptr) {
            uint32_t _count = 1;
            std::vector<VkPhysicalDevice> _vk_physical_device_collection1(1);
            s_wvk_instance_dispatch_table->wvkEnumeratePhysicalDevices(&_count, _vk_physical_device_collection1.data());

            s_wvk_physical_device = std::make_unique<CGDev::wvk::WvkPhysicalDevice>();

            CGDev::wvk::WvkPhysicalDeviceCreateInfo _create_info;
            _create_info.vk_physical_device = _vk_physical_device_collection1[0];
            _create_info.wvk_instance_dispatch_table = s_wvk_instance_dispatch_table.get();

            auto _wvk_res = s_wvk_physical_device->create(_create_info);

            ASSERT_EQ(_wvk_res.isOk(), true);
        }

        if (s_wvk_khr_surface_dispatch_table == nullptr) {
            s_wvk_khr_surface_dispatch_table = std::make_unique<CGDev::wvk::Extensions::WvkKhrSurfaceDispatchTable>();

            CGDev::wvk::Extensions::WvkKhrSurfaceDispatchTableCreateInfo _create_info;
            _create_info.wvk_loader = s_wvk_loader.get();
            _create_info.wvk_instance = s_wvk_instance.get();

            auto _wvk_res = s_wvk_khr_surface_dispatch_table->create(_create_info);

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
            s_wvk_khr_get_surface_capabilities2_dispatch_table = std::make_unique<CGDev::wvk::Extensions::WvkKhrGetSurfaceCapabilities2DispatchTable>();

            CGDev::wvk::Extensions::WvkKhrGetSurfaceCapabilities2DispatchTableCreateInfo _create_info;
            _create_info.wvk_loader = s_wvk_loader.get();
            _create_info.wvk_instance = s_wvk_instance.get();

            auto _wvk_res = s_wvk_khr_get_surface_capabilities2_dispatch_table->create(_create_info);

            ASSERT_EQ(_wvk_res.isOk(), true);
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
            _create_info.wvk_instance_dispatch_table = s_wvk_instance_dispatch_table.get();
            _create_info.wvk_khr_surface_dispatch_table = s_wvk_khr_surface_dispatch_table.get();
            _create_info.wvk_khr_get_surface_capabilities2_dispatch_table = s_wvk_khr_get_surface_capabilities2_dispatch_table.get();
            _create_info.wvk_surface_platform_create_info = &_create_info_mswindows;

            auto _wvk_res = s_wvk_surface->create(_create_info);

            ASSERT_EQ(_wvk_res.isOk(), true);
        }
    }

protected:

    inline static CGDev::wvk::WvkLoaderUptr s_wvk_loader = nullptr;
    inline static CGDev::wvk::WvkLoaderDispatchTableUptr s_wvk_loader_dispatch_table = nullptr;
    inline static CGDev::wvk::WvkInstanceUptr s_wvk_instance = nullptr;
    inline static CGDev::wvk::WvkInstanceDispatchTableUptr s_wvk_instance_dispatch_table = nullptr;
    inline static CGDev::wvk::WvkPhysicalDeviceUptr s_wvk_physical_device = nullptr;
    inline static CGDev::wvk::Extensions::WvkKhrSurfaceDispatchTableUptr s_wvk_khr_surface_dispatch_table = nullptr;
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

