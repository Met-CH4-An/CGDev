#pragma once
#include <iomanip>

#include "gtest/gtest.h"
#include "gmock/gmock.h"

// --gtest_filter=WvkPhysicalDeviceTest.RequestPhysicalDeviceProperties_FillsPropertiesCorrectly
#include <windows.h>

#include "Extensions/wvk_ext_debug_utils.h"
#include "Extensions/wvk_surface.h"

#include "Extensions/MSWindows/wvk_surface_mswindows.h"

inline bool CreateTestWindow(HINSTANCE& outInstance, HWND& outHwnd);

namespace CGDev {

    namespace tests {

        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        /*!	\brief
        */
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        class WvkBaseTest : public ::testing::Environment {

        protected:

            // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            /*!	\brief
            */
            // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            void SetUp() noexcept;

        protected:

            static CGDev::wvk::WvkInstanceUptr wvk_instance_ptr;
            static CGDev::wvk::WvkLogicalDeviceUptr wvk_logical_device_ptr;
            static CGDev::wvk::WvkCommandPoolUptr wvk_command_pool_ptr;

            inline static CGDev::wvk::Extensions::WvkExtDebugUtils wvk_ext_debug_utils;

            inline static CGDev::wvk::Extensions::WvkSurfaceUptr s_wvk_surface = nullptr;
        };

	} // namespace tests
} // namespace CGDev


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

