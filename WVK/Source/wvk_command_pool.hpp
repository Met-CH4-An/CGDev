// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025 Metelev Met-CH4-An Andrey
// Project Lead: Metelev Andrey
// Main Technical Assistant: StarDev aka ChatGPT
#ifndef CGDEV_WVK_SOURCE__WVK_COMMAND_POOL_HPP
#define CGDEV_WVK_SOURCE__WVK_COMMAND_POOL_HPP
////////////////////////////////////////////////////////////////
// секция форвард-декларации
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// секция имплементации
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// секция родительского класса
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// секция для остального
////////////////////////////////////////////////////////////////
#include "wvk_dispatch_table.h"
#include "wvk_command_buffer.h"

namespace CGDev {

	namespace wvk {

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		 
		inline const VkCommandPool& WvkCommandPool::getVkCommandPool(void) const noexcept {
			return m_vk_command_pool;
		}

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		inline WvkStatus WvkCommandPool::trim(const VkCommandPoolTrimFlags& flags) const noexcept {
			WvkStatus _status;

#if WVK_VULKAN_API_VERSION >= WVK_VULKAN_API_VERSION_11 || WVK_KHR_maintenance1 == WVK_ENABLE
			m_dispatch_table_ptr->wvkTrimCommandPool(m_vk_command_pool, flags);

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Успех
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			return _status.setOk();
#else
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Функционал не доступен
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			return _status.set(VknStatusCode::FEATURE_NOT_ENABLED, "Vulkan 1.1 or WVK_KHR_maintenance1 is not available.");
#endif
		}

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		inline WvkStatus WvkCommandPool::reset(const VkCommandPoolResetFlags& flags) const noexcept {
			WvkStatus _status;

			auto _vk_result = m_dispatch_table_ptr->wvkResetCommandPool(m_vk_command_pool, flags);
			if (_vk_result != VK_SUCCESS) {
				switch (_vk_result) {
				case VK_ERROR_OUT_OF_DEVICE_MEMORY:
					_status.append("\nwvkResetCommandPool is VK_ERROR_OUT_OF_DEVICE_MEMORY.");
					break;
				case VK_ERROR_UNKNOWN:
					_status.append("\nwvkResetCommandPool is VK_ERROR_UNKNOWN.");
					break;
				case VK_ERROR_VALIDATION_FAILED_EXT:
					_status.append("\nwvkResetCommandPool is VK_ERROR_VALIDATION_FAILED.");
					break;
				}

				return _status.setFail("\n\tvknCreateInstance is fail.");
			}

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Успех
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			return _status.setOk();
		}

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		inline WvkStatus WvkCommandPool::allocateWvkCommandBuffers(std::vector<WvkCommandBufferPtr>& wvk_cmd_buffers, const VkCommandBufferLevel& level) const noexcept {
			WvkStatus _status;

			uint32_t _count = static_cast<uint32_t>(wvk_cmd_buffers.size());

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Описываем и аллоцируем VkCommandBuffer
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			VkCommandBufferAllocateInfo _allocate_info = {
				.sType = VK_STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO,
				.pNext = nullptr,
				.commandPool = m_vk_command_pool,
				.level = level,
				.commandBufferCount = _count,
			};

			std::vector<VkCommandBuffer> _vk_cmd_buffers(_count, VK_NULL_HANDLE);
			auto _vk_result = m_create_info.wvk_logical_device_ptr->getWvkDispatchTable()->wvkAllocateCommandBuffers(&_allocate_info, _vk_cmd_buffers.data());

			if (_vk_result != VK_SUCCESS) {
				switch (_vk_result) {
				case VK_ERROR_OUT_OF_DEVICE_MEMORY:
					_status.append("\nwvkAllocateCommandBuffers is VK_ERROR_OUT_OF_DEVICE_MEMORY.");
					break;
				case VK_ERROR_OUT_OF_HOST_MEMORY:
					_status.append("\nwvkAllocateCommandBuffers is VK_ERROR_OUT_OF_HOST_MEMORY.");
					break;
				case VK_ERROR_UNKNOWN:
					_status.append("\nwvkAllocateCommandBuffers is VK_ERROR_UNKNOWN.");
					break;
				case VK_ERROR_VALIDATION_FAILED_EXT:
					_status.append("\nwvkAllocateCommandBuffers is VK_ERROR_VALIDATION_FAILED.");
					break;
				}

				return _status.setFail("\n\tvknCreateInstance is fail.");
			}

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Описываем и аллоцируем VkCommandBuffer
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			for(size_t ct_0 = 0; ct_0 < _count; ++ct_0) {
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// Описываем и создаём WvkCommandBuffer
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				WvkCommandBufferCreateInfo _create_info = {
					.wvk_logical_device_ptr = m_create_info.wvk_logical_device_ptr,
					.vk_cmd_buffer = _vk_cmd_buffers[ct_0],
				};

				auto _wvk_cmd_buffer_ptr = new WvkCommandBuffer();
				_status = _wvk_cmd_buffer_ptr->create(_create_info);

				if (!_status) {
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					// Проходим по созданным WvkCommandBuffer
					// уничтожаем и удаляем их
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					for (auto* buf : wvk_cmd_buffers) {
						if (buf != nullptr) {
							buf->destroy();
							delete buf;
						}
					}

					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					// Освождаем VkCommandBuffer
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					m_create_info.wvk_logical_device_ptr->getWvkDispatchTable()->wvkFreeCommandBuffers(m_vk_command_pool, _count, _vk_cmd_buffers.data());

					wvk_cmd_buffers.clear();

					return _status.setFail("\nWvkCommandBuffer::create is fail.");
				}

				wvk_cmd_buffers[ct_0] = _wvk_cmd_buffer_ptr;
			} // for(size_t ct_0 = 0;
			
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Успех
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			return _status.setOk();
		}

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		inline WvkStatus WvkCommandPool::freeWvkCommandBuffers(std::vector<WvkCommandBufferPtr>& wvk_cmd_buffers) const noexcept {
			WvkStatus _status;

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Преобразовываем 
			// std::vector<WvkCommandBufferPtr> в std::vector<VkCommandBuffer>
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			std::vector<VkCommandBuffer> _vk_cmd_buffers;
			for (auto&& buf : wvk_cmd_buffers
				| std::views::filter([](auto& b) { return b != nullptr; })
				| std::views::transform([](auto& b) { return b->getVkCommandBuffer(); }))
			{
				_vk_cmd_buffers.push_back(buf);
			}
			m_create_info.wvk_logical_device_ptr->getWvkDispatchTable()->wvkFreeCommandBuffers(m_vk_command_pool, static_cast<uint32_t>(_vk_cmd_buffers.size()), _vk_cmd_buffers.data());
			
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Проходим по созданным WvkCommandBuffer
			// уничтожаем и удаляем их
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			for (auto* buf : wvk_cmd_buffers) {
				if (buf != nullptr) {
					buf->destroy();
					delete buf;
				}
			}
			

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Успех
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			return _status.setOk();
		}

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

	} // namespace wvk

} // namespace CGDev

#endif // CGDEV_WVK_SOURCE__WVK_COMMAND_POOL_HPP
