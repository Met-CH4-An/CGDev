// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025 Metelev Met-CH4-An Andrey
// Project Lead: Metelev Andrey
// Main Technical Assistant: StarDev aka ChatGPT
#ifndef CGDEV_WVK_SOURCE__WVK_INSTANCE_HPP
#define CGDEV_WVK_SOURCE__WVK_INSTANCE_HPP
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

namespace CGDev {

	namespace wvk {

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		inline WvkStatus WvkInstance::requestLayerProperties(std::vector<VkLayerProperties>& vk_layer_properties) const noexcept {
			WvkStatus _status;

			vk_layer_properties.clear();

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Получение количества доступных слоёв
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			uint32_t _count = 0;
			VkResult _vk_result = m_wvk_global_dispatch_table_ptr->wvkEnumerateInstanceLayerProperties(&_count, nullptr);

			if (_vk_result != VK_SUCCESS) {
				switch (_vk_result) {
				case VK_ERROR_OUT_OF_HOST_MEMORY:
					_status << "vkEnumerateInstanceLayerProperties is VK_ERROR_OUT_OF_HOST_MEMORY.";
					break;
				case VK_ERROR_OUT_OF_DEVICE_MEMORY:
					_status << "vkEnumerateInstanceLayerProperties is VK_ERROR_OUT_OF_DEVICE_MEMORY.";
					break;
				default:
					_status << "vkEnumerateInstanceLayerProperties is Unknown error.";
					break;
				}
				return _status.setFail("vknEnumerateInstanceLayerProperties is fail");
			}

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Выделение памяти и получение свойств слоёв
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			vk_layer_properties.resize(_count);
			_vk_result = m_wvk_global_dispatch_table_ptr->wvkEnumerateInstanceLayerProperties(&_count, vk_layer_properties.data());

			if (_vk_result != VK_SUCCESS) {
				if (_vk_result == VK_INCOMPLETE) {
					_status << "vkEnumerateInstanceLayerProperties is VK_INCOMPLETE.";
				}
				else {
					switch (_vk_result) {
					case VK_ERROR_OUT_OF_HOST_MEMORY:
						_status << "vkEnumerateInstanceLayerProperties is VK_ERROR_OUT_OF_HOST_MEMORY.";
						break;
					case VK_ERROR_OUT_OF_DEVICE_MEMORY:
						_status << "vkEnumerateInstanceLayerProperties is VK_ERROR_OUT_OF_DEVICE_MEMORY.";
						break;
					default:
						_status << "vkEnumerateInstanceLayerProperties is Unknown error.";
						break;
					}
					return _status.setFail("vknEnumerateInstanceLayerProperties is fail");
				}
			}

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Успех
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			return _status.setOk();
		}

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		inline WvkStatus WvkInstance::requestExtensionProperties(std::vector<VkExtensionProperties>& vk_extension_properties) const noexcept {
			WvkStatus _status;

			vk_extension_properties.clear();

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Получение количества доступных расширений
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			uint32_t _count = 0;
			VkResult _vk_result = m_wvk_global_dispatch_table_ptr->wvkEnumerateInstanceExtensionProperties(nullptr, &_count, nullptr);

			if (_vk_result != VK_SUCCESS) {
				switch (_vk_result) {
				case VK_ERROR_OUT_OF_HOST_MEMORY:
					_status << "vkEnumerateInstanceLayerProperties is VK_ERROR_OUT_OF_HOST_MEMORY.";
					break;
				case VK_ERROR_OUT_OF_DEVICE_MEMORY:
					_status << "vkEnumerateInstanceLayerProperties is VK_ERROR_OUT_OF_DEVICE_MEMORY.";
					break;
				default:
					_status << "vkEnumerateInstanceLayerProperties is Unknown error.";
					break;
				}
				return _status.setFail("vknEnumerateInstanceExtensionProperties is fail.");
			}

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Выделение памяти и получение свойств расширений
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			vk_extension_properties.resize(_count);
			_vk_result = m_wvk_global_dispatch_table_ptr->wvkEnumerateInstanceExtensionProperties(nullptr, &_count, vk_extension_properties.data());

			if (_vk_result != VK_SUCCESS) {
				if (_vk_result == VK_INCOMPLETE) {
					_status << "vkEnumerateInstanceLayerProperties is VK_INCOMPLETE.";
				}
				else {
					switch (_vk_result) {
					case VK_ERROR_OUT_OF_HOST_MEMORY:
						_status << "vkEnumerateInstanceLayerProperties is VK_ERROR_OUT_OF_HOST_MEMORY.";
						break;
					case VK_ERROR_OUT_OF_DEVICE_MEMORY:
						_status << "vkEnumerateInstanceLayerProperties is VK_ERROR_OUT_OF_DEVICE_MEMORY.";
						break;
					default:
						_status << "vkEnumerateInstanceLayerProperties is Unknown error.";
						break;
					}
					return _status.setFail("vknEnumerateInstanceExtensionProperties is fail.");
				}
			}

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Успех
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			return _status.setOk();
		}

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		inline const WvkPhysicalDeviceUptrVec2& WvkInstance::getWvkPhysicalDevices(void) const noexcept {
			return m_wvk_physical_devices;
		}

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		inline const WvkDispatchTableUptr& WvkInstance::getWvkDispatchTable(void) const noexcept {
			return m_wvk_instance_dispatch_table_ptr;
		}

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		inline const VkInstance& WvkInstance::getVkInstance(void) const noexcept {
			return m_vk_instance;
		}

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

	} // namespace wvk

} // namespace CGDev

#endif // CGDEV_WVK_SOURCE__WVK_INSTANCE_HPP
