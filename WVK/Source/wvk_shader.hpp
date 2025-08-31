// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025 Metelev Met-CH4-An Andrey
// Project Lead: Metelev Andrey
// Main Technical Assistant: StarDev aka ChatGPT
#ifndef CGDEV_WVK_SOURCE__WVK_SHADER_HPP
#define CGDEV_WVK_SOURCE__WVK_SHADER_HPP
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
		
		inline WvkStatus WvkShader::getBinary(std::vector<WvkShaderStageCreateInfo>& wvk_shader_stages) const noexcept {
			WvkStatus _status;

			for (const auto& _vk_shader : m_vk_shaders) {
				size_t _size = 0;
				auto _vk_result = m_create_info.wvk_logical_device_ptr->getWvkDispatchTable()->wvkGetShaderBinaryDataEXT(_vk_shader, &_size, nullptr);

				if (_vk_result != VK_SUCCESS) {
					switch (_vk_result) {
					case VK_ERROR_OUT_OF_DEVICE_MEMORY:
						_status.setFail("\nwvkGetShaderBinaryDataEXT is VK_ERROR_OUT_OF_DEVICE_MEMORY.");
						break;
					case VK_ERROR_OUT_OF_HOST_MEMORY:
						_status.setFail("\nwvkGetShaderBinaryDataEXT is VK_ERROR_OUT_OF_HOST_MEMORY.");
						break;
					case VK_ERROR_UNKNOWN:
						_status.setFail("\nwvkGetShaderBinaryDataEXT is VK_ERROR_UNKNOWN.");
						break;
					case VK_ERROR_VALIDATION_FAILED_EXT:
						_status.setFail("\nwvkGetShaderBinaryDataEXT is VK_ERROR_VALIDATION_FAILED_EXT.");
						break;
					default:
						_status.setFail("\nwvkGetShaderBinaryDataEXT is UNKNOWN ERROR.");
					}

					return _status;
				}

				std::vector<std::byte> _data(_size);
				_vk_result = m_create_info.wvk_logical_device_ptr->getWvkDispatchTable()->wvkGetShaderBinaryDataEXT(_vk_shader, &_size, _data.data());

				if (_vk_result != VK_SUCCESS) {
					switch (_vk_result) {
					case VK_INCOMPLETE:
						_status.setFail("\nwvkGetShaderBinaryDataEXT is VK_INCOMPLETE.");
						break;
					case VK_ERROR_OUT_OF_DEVICE_MEMORY:
						_status.setFail("\nwvkGetShaderBinaryDataEXT is VK_ERROR_OUT_OF_DEVICE_MEMORY.");
						break;
					case VK_ERROR_OUT_OF_HOST_MEMORY:
						_status.setFail("\nwvkGetShaderBinaryDataEXT is VK_ERROR_OUT_OF_HOST_MEMORY.");
						break;
					case VK_ERROR_UNKNOWN:
						_status.setFail("\nwvkGetShaderBinaryDataEXT is VK_ERROR_UNKNOWN.");
						break;
					case VK_ERROR_VALIDATION_FAILED_EXT:
						_status.setFail("\nwvkGetShaderBinaryDataEXT is VK_ERROR_VALIDATION_FAILED_EXT.");
						break;
					default:
						_status.setFail("\nwvkGetShaderBinaryDataEXT is UNKNOWN ERROR.");
					}

					return _status;
				}

				wvk_shader_stages.emplace_back(WvkShaderStageCreateInfo{
					.code = std::move(_data),
					});
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

#endif // CGDEV_WVK_SOURCE__WVK_SHADER_HPP
