#ifndef CGDEV_WVK_SOURCE__WVK_COMMAND_BUFFER_HPP
#define CGDEV_WVK_SOURCE__WVK_COMMAND_BUFFER_HPP
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
#include "wvk_logical_device.h"

namespace CGDev {

	namespace wvk {

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		inline VkCommandBuffer WvkCommandBuffer::getVkCommandBuffer(void) const noexcept {
			return m_vk_command_buffer;
		}

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		inline WvkStatus WvkCommandBuffer::reset(const VkCommandBufferResetFlags& flags) const noexcept {
			WvkStatus _status;

			auto _vk_result = m_create_info.wvk_logical_device_ptr->getDispatchTable()->wvkResetCommandBuffer(m_vk_command_buffer, flags);
			if (_vk_result != VK_SUCCESS) {
				switch (_vk_result) {
				case VK_ERROR_OUT_OF_DEVICE_MEMORY:
					_status.append("\nwvkResetCommandBuffer is VK_ERROR_OUT_OF_DEVICE_MEMORY.");
					break;
				case VK_ERROR_UNKNOWN:
					_status.append("\nwvkResetCommandBuffer is VK_ERROR_UNKNOWN.");
					break;
				case VK_ERROR_VALIDATION_FAILED_EXT:
					_status.append("\nwvkResetCommandBuffer is VK_ERROR_VALIDATION_FAILED.");
					break;
				}
				return _status.setFail();
			}

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Успех
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			return _status.setOk();
		}

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		inline WvkStatus WvkCommandBuffer::begin(const VkCommandBufferBeginInfo& info) const noexcept {
			WvkStatus _status;
			
			auto _vk_result = m_create_info.wvk_logical_device_ptr->getDispatchTable()->wvkBeginCommandBuffer(m_vk_command_buffer, &info);
			if (_vk_result != VK_SUCCESS) {
				switch (_vk_result) {
				case VK_ERROR_OUT_OF_DEVICE_MEMORY:
					_status.append("\nwvkResetCommandBuffer is VK_ERROR_OUT_OF_DEVICE_MEMORY.");
					break;
				case VK_ERROR_OUT_OF_HOST_MEMORY:
					_status.append("\nwvkResetCommandBuffer is VK_ERROR_OUT_OF_HOST_MEMORY.");
					break;
				case VK_ERROR_UNKNOWN:
					_status.append("\nwvkResetCommandBuffer is VK_ERROR_UNKNOWN.");
					break;
				case VK_ERROR_VALIDATION_FAILED_EXT:
					_status.append("\nwvkResetCommandBuffer is VK_ERROR_VALIDATION_FAILED.");
					break;
				}
				return _status.setFail();
			}

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Успех
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			return _status.setOk();
		}

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		inline WvkStatus WvkCommandBuffer::begin(const WvkCommandBufferBeginHelper& helper) const noexcept {
			WvkStatus _status;
			
			begin(helper.getBeginInfo());			

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Успех
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			return _status.setOk();
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		inline WvkStatus WvkCommandBuffer::end(void) const noexcept {
			WvkStatus _status;

			auto _vk_result = m_create_info.wvk_logical_device_ptr->getDispatchTable()->wvkEndCommandBuffer(m_vk_command_buffer);
			if (_vk_result != VK_SUCCESS) {
				switch (_vk_result) {
				case VK_ERROR_OUT_OF_DEVICE_MEMORY:
					_status.append("\nwvkResetCommandBuffer is VK_ERROR_OUT_OF_DEVICE_MEMORY.");
					break;
				case VK_ERROR_OUT_OF_HOST_MEMORY:
					_status.append("\nwvkResetCommandBuffer is VK_ERROR_OUT_OF_HOST_MEMORY.");
					break;
				case VK_ERROR_UNKNOWN:
					_status.append("\nwvkResetCommandBuffer is VK_ERROR_UNKNOWN.");
					break;
				case VK_ERROR_VALIDATION_FAILED_EXT:
					_status.append("\nwvkResetCommandBuffer is VK_ERROR_VALIDATION_FAILED.");
					break;
				}
				return _status.setFail();
			}

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Успех
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			return _status.setOk();
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

	} // namespace wvk

} // namespace CGDev

#endif // CGDEV_WVK_SOURCE__WVK_COMMAND_BUFFER_HPP
