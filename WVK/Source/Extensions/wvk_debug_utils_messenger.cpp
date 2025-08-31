// SPDX-License-Identifier: AGPL-3.0-or-later
////////////////////////////////////////////////////////////////
// ������ �������-����������
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// ������ ������������� �����
////////////////////////////////////////////////////////////////
#include "wvk_debug_utils_messenger.h"
////////////////////////////////////////////////////////////////
// ������ �������������
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// ������ ��� ����������
////////////////////////////////////////////////////////////////
#include "../wvk_instance.h"
#include "../wvk_dispatch_table.h"

namespace CGDev {

	namespace wvk {

		namespace Extensions {
					
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			WvkDebugUtilsMessenger::WvkDebugUtilsMessenger(void) noexcept {
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			WvkDebugUtilsMessenger::~WvkDebugUtilsMessenger(void) noexcept {
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			WvkStatus WvkDebugUtilsMessenger::create(const WvkDebugUtilsMessengerCreateInfo& create_info) noexcept {
				WvkStatus _status;

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// �������� �� ��������� �������������
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				if (m_valid) {
					return _status.set(VknStatusCode::ALREADY_INITIALIZED);
				}

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// �������� ���������� ������� ���������
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				_status = validationCreateInfo(create_info);
				if (!_status) {
					destroy();
					return _status.set(VknStatusCode::FAIL, "\n\tWvkDebugUtilsMessenger::validationCreateInfo() is fail.");
				}

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// ��������
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				_status = create();
				if (!_status) {
					destroy();
					return _status.set(VknStatusCode::FAIL, "\n\tWvkDebugUtilsMessenger::create() is fail.");
				}

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// �����
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				m_valid = true;
				return _status.setOk();
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			void WvkDebugUtilsMessenger::destroy(void) noexcept {
				if(m_vk_debug_utils_messenger != VK_NULL_HANDLE)
					m_create_info.wvk_instance_ptr->getWvkDispatchTable()->wvkDestroyDebugUtilsMessenger(m_create_info.wvk_instance_ptr->getVkInstance(), m_vk_debug_utils_messenger, VK_NULL_HANDLE);

				// ~~~~~~~~~~~~~~~~
				// ������� ������
				// ~~~~~~~~~~~~~~~~
				m_create_info = {};
				m_vk_debug_utils_messenger = VK_NULL_HANDLE;
				m_debug_message_collection.clear();
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			WvkStatus WvkDebugUtilsMessenger::validationCreateInfo(const WvkDebugUtilsMessengerCreateInfo& create_info) noexcept {
				WvkStatus _status(VknStatusCode::SUCCESSFUL);

				if (create_info.wvk_instance_ptr == nullptr) {
					_status.setFail("\nWvkDebugUtilsMessengerCreateInfo::wvk_instance_ptr is nullptr.");
				}
				if (create_info.mode == VknDebugUtilsMode::UNKNOWN) {
					_status.setFail("\nWvkDebugUtilsMessengerCreateInfo::mode is UNKNOWN.");
				}
						
				if (!_status) {
					return _status;
				}
						
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// �����
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				m_create_info = create_info;
				return _status.setOk();
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			void WvkDebugUtilsMessenger::convertDebugUtilsModeToVkFlags(const VknDebugUtilsMode& mode, VkDebugUtilsMessageSeverityFlagsEXT& outSeverity, VkDebugUtilsMessageTypeFlagsEXT& outType) const noexcept {
						
				// --------------------------------------
				// ������ ����������� (Severity flags)
				// --------------------------------------

				// ��������� ���� ������, ���� ����� �������� ERRORS_ONLY
				if (has_flag(mode, VknDebugUtilsMode::ERRORS_ONLY)) {
					outSeverity |= VK_DEBUG_UTILS_MESSAGE_SEVERITY_ERROR_BIT_EXT;
				}

				// ��������� ���� ��������������, ���� ����� �������� WARNINGS_ONLY
				if (has_flag(mode, VknDebugUtilsMode::WARNINGS_ONLY)) {
					outSeverity |= VK_DEBUG_UTILS_MESSAGE_SEVERITY_WARNING_BIT_EXT;
				}

				// ��������� ���� �������������� ���������, ���� ������� INFO_ONLY
				if (has_flag(mode, VknDebugUtilsMode::INFO_ONLY)) {
					outSeverity |= VK_DEBUG_UTILS_MESSAGE_SEVERITY_INFO_BIT_EXT;
				}

				// ��������� ���� ���������� ������, ���� ������� VERBOSE
				if (has_flag(mode, VknDebugUtilsMode::VERBOSE)) {
					outSeverity |= VK_DEBUG_UTILS_MESSAGE_SEVERITY_VERBOSE_BIT_EXT;
				}

				// --------------------------------------
				// ���� ��������� (Message Type flags)
				// --------------------------------------

				// ��������� ��� ���������, ���� ������� VALIDATION
				if (has_flag(mode, VknDebugUtilsMode::VALIDATION)) {
					outType |= VK_DEBUG_UTILS_MESSAGE_TYPE_VALIDATION_BIT_EXT;
				}

				// ��������� ��� ������������������, ���� ������� PERFORMANCE
				if (has_flag(mode, VknDebugUtilsMode::PERFORMANCE)) {
					outType |= VK_DEBUG_UTILS_MESSAGE_TYPE_PERFORMANCE_BIT_EXT;
				}

				// ��������� ����� ���, ���� ������� GENERAL
				if (has_flag(mode, VknDebugUtilsMode::GENERAL)) {
					outType |= VK_DEBUG_UTILS_MESSAGE_TYPE_GENERAL_BIT_EXT;
				}
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			WvkStatus WvkDebugUtilsMessenger::create(void) noexcept {
				WvkStatus _status;

				VkDebugUtilsMessengerCreateInfoEXT _create_info = {
					.sType = VK_STRUCTURE_TYPE_DEBUG_UTILS_MESSENGER_CREATE_INFO_EXT,
					.pNext = nullptr,
					.flags = 0,
					.messageSeverity = 0,
					.messageType = 0,
					.pfnUserCallback = &s_callback,
					.pUserData = this,
				};

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// �������������� ������ ������� � ��������������� ����� Vulkan
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				convertDebugUtilsModeToVkFlags(m_create_info.mode,
					_create_info.messageSeverity,
					_create_info.messageType);

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// ��� 3. ������� �������� ����������� ��� �������
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				auto _vk_result = m_create_info.wvk_instance_ptr->getWvkDispatchTable()->wvkCreateDebugUtilsMessenger(m_create_info.wvk_instance_ptr->getVkInstance(), &_create_info, VK_NULL_HANDLE, &m_vk_debug_utils_messenger);
				
				if (_vk_result != VK_SUCCESS) {
					switch (_vk_result) {
					case VK_ERROR_OUT_OF_HOST_MEMORY:
						_status.setFail("\nwvkCreateDebugUtilsMessenger is VK_ERROR_OUT_OF_HOST_MEMORY.");
						break;
					default:
						_status.setFail("\nwvkCreateDebugUtilsMessenger is Unknown error.");
						break;
					}
					return _status;
				}

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// �����
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				return _status.setOk();
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			VkBool32 WvkDebugUtilsMessenger::s_callback(VkDebugUtilsMessageSeverityFlagBitsEXT messageSeverity, VkDebugUtilsMessageTypeFlagsEXT messageTypes, const VkDebugUtilsMessengerCallbackDataEXT* pCallbackData, void* pUserData) noexcept {

				auto* _this = static_cast<WvkDebugUtilsMessenger*>(pUserData);

				// ���������� ��������� � �������
				if (_this) {
					DebugMessage _msg;
					_msg.severity = messageSeverity;
					_msg.type = messageTypes;
					_msg.message = pCallbackData->pMessage;
					_this->m_debug_message_collection.push_back(std::move(_msg));
				}
				
				std::cerr << pCallbackData->pMessage << std::endl;
				
				return VK_FALSE; // �� ��������� ����������
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		} // namespace Extensions

	} // namespace wvk

} // namespace CGDev