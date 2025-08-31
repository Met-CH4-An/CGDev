// SPDX-License-Identifier: AGPL-3.0-or-later
////////////////////////////////////////////////////////////////
// секция форвард-декларации
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// секция заголовочного файла
////////////////////////////////////////////////////////////////
#include "wvk_shader.h"
////////////////////////////////////////////////////////////////
// секция имплементации
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

		WvkShader::WvkShader(void) noexcept {
		}

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		WvkShader::~WvkShader(void) noexcept {
		}

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		WvkStatus WvkShader::create(const WvkShaderCreateInfo& create_info) noexcept {
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
				return _status.set(VknStatusCode::FAIL, "\n\tWvkShader::validationCreateInfo() is fail.");
			}

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Создание пула комманд
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			_status = create();
			if (!_status) {
				destroy();
				return _status.set(VknStatusCode::FAIL, "\n\tWvkShader::create() is fail.");
			}

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Успех
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			m_valid = true;
			return _status.setOk();
		}

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		void WvkShader::destroy(void) noexcept {
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// удаление VkShader
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			for (const auto& _shader : m_vk_shaders) {
				if(_shader != VK_NULL_HANDLE)
					m_create_info.wvk_logical_device_ptr->getWvkDispatchTable()->wvkDestroyShader(_shader, nullptr);
			}

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// очистка данных
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			m_create_info = {};

			m_vk_shader_module = VK_NULL_HANDLE;
#if WVK_EXT_shader_object == WVK_ENABLE
			m_vk_shaders.clear();
#endif
		}

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		WvkStatus WvkShader::validationCreateInfo(const WvkShaderCreateInfo& create_info) noexcept {
			WvkStatus _status(VknStatusCode::SUCCESSFUL);

			if (create_info.wvk_logical_device_ptr == nullptr) {
				_status.setFail("\nWvkShaderCreateInfo::wvk_logical_device_ptr is nullptr.");
			}
			if (create_info.wvk_shader_stage_create_infos.empty()) {
				_status.setFail("\nWvkShaderCreateInfo::wvk_shader_stage_create_infos is empty.");
			}

			if (!_status) return _status;

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Успех
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			m_create_info = create_info;
			return _status.setOk();
		}

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		WvkStatus WvkShader::create(void) noexcept {
			WvkStatus _status;
#if WVK_EXT_shader_object == WVK_ENABLE
			VkShaderEXT m_vk_shader = VK_NULL_HANDLE;
#endif
			uint32_t _stage_count = static_cast<uint32_t>(m_create_info.wvk_shader_stage_create_infos.size());

			std::vector<VkShaderCreateInfoEXT> _vk_shader_create_infos;
			for (size_t ct_0 = 0; ct_0 < _stage_count; ++ct_0) {
				const auto& _vk_create_info = m_create_info.wvk_shader_stage_create_infos[ct_0];
				
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// следующая VkShaderStageFlags
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				VkShaderStageFlags _vk_next_stage = 0;
				if (ct_0 + 1 < _stage_count) {
					_vk_next_stage = static_cast<VkShaderStageFlagBits>(m_create_info.wvk_shader_stage_create_infos[ct_0 + 1].stage);
				}

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// VkShaderCreateInfoEXT
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				_vk_shader_create_infos.emplace_back(VkShaderCreateInfoEXT{
					.sType = VK_STRUCTURE_TYPE_SHADER_CREATE_INFO_EXT,
					.pNext = nullptr,
					.flags = 0,
					.stage = static_cast<VkShaderStageFlagBits>(_vk_create_info.stage),
					.nextStage = 0,
					.codeType = VK_SHADER_CODE_TYPE_SPIRV_EXT,
					.codeSize = _vk_create_info.code.size(),
					.pCode = reinterpret_cast<const uint32_t*>(_vk_create_info.code.data()),
					.pName = "main",
					.setLayoutCount = 0,
					.pSetLayouts = nullptr,
					.pushConstantRangeCount = 0,
					.pPushConstantRanges = nullptr,
					.pSpecializationInfo = nullptr,
					});
			}
			
			m_vk_shaders.resize(_stage_count);
			auto _vk_result = m_create_info.wvk_logical_device_ptr->getWvkDispatchTable()->wvkCreateShaders(_stage_count, _vk_shader_create_infos.data(), nullptr, m_vk_shaders.data());

			if (_vk_result != VK_SUCCESS) {
				switch (_vk_result) {
				case VK_INCOMPATIBLE_SHADER_BINARY_EXT:
					_status.setFail("\nwvkCreateShaders is VK_INCOMPATIBLE_SHADER_BINARY_EXT.");
					break;
				case VK_ERROR_INITIALIZATION_FAILED:
					_status.setFail("\nwvkCreateShaders is VK_ERROR_INITIALIZATION_FAILED.");
					break;
				case VK_ERROR_OUT_OF_DEVICE_MEMORY:
					_status.setFail("\nwvkCreateShaders is VK_ERROR_OUT_OF_DEVICE_MEMORY.");
					break;
				case VK_ERROR_OUT_OF_HOST_MEMORY:
					_status.setFail("\nwvkCreateShaders is VK_ERROR_OUT_OF_HOST_MEMORY.");
					break;
				case VK_ERROR_UNKNOWN:
					_status.setFail("\nwvkCreateShaders is VK_ERROR_UNKNOWN.");
					break;
				case VK_ERROR_VALIDATION_FAILED_EXT:
					_status.setFail("\nwvkCreateShaders is VK_ERROR_VALIDATION_FAILED_EXT.");
					break;
				default:
					_status.setFail("\nwvkCreateShaders is UNKNOWN ERROR.");
				}

				return _status;
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