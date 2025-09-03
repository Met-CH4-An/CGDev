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
#include "base.h"
////////////////////////////////////////////////////////////////
// секция имплементации
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// секция для остального
////////////////////////////////////////////////////////////////
#include "shader_object.h"
#include "wvk_fence.h"

int main() {
	auto _sample = std::make_unique<CGDev::samples::ShaderObject>();
	_sample->onInit();

	MSG msg{};
	bool running = true;

	while (running) {
		// Обработка всех сообщений Windows
		while (PeekMessage(&msg, nullptr, 0, 0, PM_REMOVE)) {
			if (msg.message == WM_QUIT) {
				running = false;
				break;
			}
			TranslateMessage(&msg);
			DispatchMessage(&msg);
		}

		// Рендер только если окно не закрыто
		if (running) {
			_sample->onRender();
		}
	}


	return 0;
}

namespace CGDev {

	namespace samples {

		static LRESULT CALLBACK WindowProc(HWND hwnd, UINT msg, WPARAM wParam, LPARAM lParam) {
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

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		Base::Base(void) noexcept {
			const wchar_t* className = L"TestWindowClass";

			// Получаем дескриптор текущего инстанса
			m_hinstance = GetModuleHandle(nullptr);

			// Регистрация класса окна
			WNDCLASS wc = {};
			wc.lpfnWndProc = WindowProc;
			wc.hInstance = m_hinstance;
			wc.lpszClassName = className;
			RegisterClass(&wc);

			// Устанавливаем клиентскую область 800x600
			RECT rect = { 0, 0, 800, 600 };
			AdjustWindowRect(&rect, WS_OVERLAPPEDWINDOW, FALSE);
			int width = rect.right - rect.left;
			int height = rect.bottom - rect.top;

			// Создание окна
			m_hwnd = CreateWindowEx(
				0, className, L"Test Window",
				WS_OVERLAPPEDWINDOW,
				CW_USEDEFAULT, CW_USEDEFAULT, width, height,
				nullptr, nullptr, m_hinstance, nullptr
			);

			ShowWindow(m_hwnd, SW_SHOW);

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// WvkInstance
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			{
				CGDev::wvk::WvkInstanceCreateInfo _create_info;

				wvk_instance_ptr = std::make_unique<CGDev::wvk::WvkInstance>();
				wvk_instance_ptr->create(_create_info);


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
					.m_vk_physical_device_features = {
						.inheritedQueries = VK_TRUE },
					.m_wvk_logical_device_feature_collection = {},
				};

				wvk_logical_device_ptr = std::make_unique<CGDev::wvk::WvkLogicalDevice>();
				wvk_logical_device_ptr->create(_create_info);
			}

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// WvSurface
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			{
				CGDev::wvk::Extensions::WvkSurfaceCreateInfo _create_info = {};
				_create_info.wvk_instance_ptr = wvk_instance_ptr.get();
				_create_info.win32.hwnd = m_hwnd;
				_create_info.win32.hinstance = m_hinstance;

				wvk_surface_ptr = std::make_unique<CGDev::wvk::Extensions::WvkSurface>();
				wvk_surface_ptr->create(_create_info);
			}

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// WvSwapchain
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			{
				CGDev::wvk::Extensions::WvkSwapchainCreateInfo _create_info = {
					.wvk_logical_device_ptr = wvk_logical_device_ptr.get(),
					.wvk_surface_ptr = wvk_surface_ptr.get(),
				};

				wvk_swapchain_ptr = std::make_unique<CGDev::wvk::Extensions::WvkSwapchain>();
				wvk_swapchain_ptr->create(_create_info);
			}

			{
				CGDev::wvk::WvkCommandPoolCreateInfo _create_info = {
					.wvk_logical_device_ptr = wvk_logical_device_ptr.get(),
					.queue_family_index = 0,
					.flags = VK_COMMAND_POOL_CREATE_RESET_COMMAND_BUFFER_BIT,
				};
				wvk_command_pool_ptr = std::make_unique<CGDev::wvk::WvkCommandPool>();
				wvk_command_pool_ptr->create(_create_info);
			}

			{ 
				wvk_command_buffers.resize(2);
				wvk_command_pool_ptr->allocateWvkCommandBuffers(wvk_command_buffers, VK_COMMAND_BUFFER_LEVEL_PRIMARY);
				CGDev::wvk::WvkCommandBufferCreateInfo _create_info = {
					.wvk_logical_device_ptr = wvk_logical_device_ptr.get(), 
					
				};
			}			

			{
				CGDev::wvk::WvkFenceCreateInfo _create_info = {
					.wvk_logical_device_ptr = wvk_logical_device_ptr.get(),
				};
				auto _wvk_fence_ptr = std::make_unique<CGDev::wvk::WvkFence>();
				_wvk_fence_ptr->create(_create_info);

				wvk_fence_ptrs.push_back(std::move(_wvk_fence_ptr));

				_wvk_fence_ptr = std::make_unique<CGDev::wvk::WvkFence>();
				_wvk_fence_ptr->create(_create_info);

				wvk_fence_ptrs.push_back(std::move(_wvk_fence_ptr));

				_wvk_fence_ptr = std::make_unique<CGDev::wvk::WvkFence>();
				_wvk_fence_ptr->create(_create_info);

				wvk_fence_ptrs.push_back(std::move(_wvk_fence_ptr));
			}

			m_vk_images = wvk_swapchain_ptr->getImages1();
			m_vk_image_views = wvk_swapchain_ptr->getImages();
		}

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		Base::~Base(void) noexcept {

		}

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

	} // namespace samples

} // namespace CGDev