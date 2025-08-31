// SPDX-License-Identifier: AGPL-3.0-or-later
////////////////////////////////////////////////////////////////
// секция форвард-декларации
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// секция заголовочного файла
////////////////////////////////////////////////////////////////
#include "wvk_swapchain.h"
////////////////////////////////////////////////////////////////
// секция имплементации
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// секция для остального
////////////////////////////////////////////////////////////////
#include "../wvk_logical_device.h"
#include "../wvk_dispatch_table.h"
#include "wvk_surface.h"

namespace CGDev {

	namespace wvk {

		namespace Extensions {

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			WvkSwapchain::WvkSwapchain(void) noexcept {
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			WvkSwapchain::~WvkSwapchain(void) noexcept {
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			WvkStatus WvkSwapchain::create(const WvkSwapchainCreateInfo& create_info) noexcept {
				WvkStatus _status;

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// Проверка на повторную инициализацию
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				if (m_valid) {
					return _status.set(VknStatusCode::ALREADY_INITIALIZED);
				}

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// Проверка валидности входной структуры
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				_status = validationCreateInfo(create_info);
				if (!_status) {
					destroy();
					return _status.set(VknStatusCode::FAIL, "\n\tWvkSwapchain::validationCreateInfo() is fail.");
				}

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// Создание
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				_status = create();
				if (!_status) {
					destroy();
					return _status.set(VknStatusCode::FAIL, "\n\tWvkSwapchain::create() is fail.");
				}

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// Успех
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				m_valid = true;
				return _status.setOk();
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			void WvkSwapchain::destroy(void) noexcept {
				if (m_vk_swapchain != VK_NULL_HANDLE)
					m_create_info.wvk_logical_device_ptr->getWvkDispatchTable()->wvkDestroySwapchainKHR(m_vk_swapchain, nullptr);

				// ~~~~~~~~~~~~~~~~
				// очистка данных
				// ~~~~~~~~~~~~~~~~
				m_create_info = {};
				m_vk_swapchain = VK_NULL_HANDLE;
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			WvkStatus WvkSwapchain::validationCreateInfo(const WvkSwapchainCreateInfo& create_info) noexcept {
				WvkStatus _status(VknStatusCode::SUCCESSFUL);

				if (create_info.wvk_logical_device_ptr == nullptr) {
					_status.setFail("\nWvkSwapchainCreateInfo::wvk_logical_device_ptr is nullptr.");
				}

				if (!_status) {
					return _status;
				}

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// Успех
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				m_create_info = create_info;
				return _status.setOk();
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			WvkStatus WvkSwapchain::create(void) noexcept {
				WvkStatus _status;

				VkSwapchainCreateInfoKHR _create_info = {
					.sType = VK_STRUCTURE_TYPE_SWAPCHAIN_CREATE_INFO_KHR,
					.pNext = nullptr,
					.flags = 0,
					.surface = m_create_info.wvk_surface_ptr->getVkSurface(),
					.minImageCount = 2,
					.imageFormat = VK_FORMAT_B8G8R8A8_SRGB,
					.imageColorSpace = VK_COLOR_SPACE_SRGB_NONLINEAR_KHR,
					.imageExtent = VkExtent2D{ .width = 800, .height = 600, },
					.imageArrayLayers = 1,
					.imageUsage = VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT,
					.imageSharingMode = VK_SHARING_MODE_EXCLUSIVE,
					.queueFamilyIndexCount = 0,
					.pQueueFamilyIndices = nullptr,
					.preTransform = VK_SURFACE_TRANSFORM_IDENTITY_BIT_KHR,
					.compositeAlpha = VK_COMPOSITE_ALPHA_OPAQUE_BIT_KHR,
					.presentMode = VK_PRESENT_MODE_FIFO_KHR,
					.clipped = VK_TRUE,
					.oldSwapchain = VK_NULL_HANDLE,
				};

				auto _vk_result = m_create_info.wvk_logical_device_ptr->getWvkDispatchTable()->wvkCreateSwapchainKHR(&_create_info, nullptr, &m_vk_swapchain);

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// Успех
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				return _status.setOk();
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		} // namespace Extensions

	} // namespace wvk

} // namespace CGDev