////////////////////////////////////////////////////////////////
// ������ �������-����������
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// ������ ������������� �����
////////////////////////////////////////////////////////////////
#include "wvk_ext_debug_utils.h"
////////////////////////////////////////////////////////////////
// ������ �������������
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// ������ ��� ����������
////////////////////////////////////////////////////////////////

namespace CGDev {

	//namespace GPU {

		//namespace Private {

			namespace wvk {

				namespace Extensions {
					
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

					WvkExtDebugUtils::WvkExtDebugUtils(void) noexcept
						: VknExtension(s_name) {
					}

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

					WvkExtDebugUtils::~WvkExtDebugUtils(void) noexcept {
					}

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

					WvkStatus WvkExtDebugUtils::create(const WvkExtDebugUtilsCreateInfo& create_info) noexcept {
						WvkStatus _status;

						// ��������� ���������� ��������� �������� �� ���������� ���� ������
						m_create_info = create_info;

						// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						// ���� ������ �������� ���������, �� ��������� ���������
						// WvkExtDebugUtilsCreateInfo �� ������������ ����������
						// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						//if constexpr (wvk::Build::ValidationBuildInfo::enable == true) {
							// ��������� ��������� ��������� ��������
							_status = validationCreateInfo();

							// ���� ���� �� ���� �������� ����������� � ���������� ������
							if (_status.m_code != VknStatusCode::SUCCESSFUL) {
								_status.m_code = VknStatusCode::FAIL;
								_status.append("\n\tVknExtDebugUtils::validationCreateInfo() - fail");
								return _status;
							}
						//}

						// ������ ������ VkDebugUtilsMessengerEXT ����� ��������������� �����
						_status = createVkDebugUtilsMessenger();
						if (_status.m_code != VknStatusCode::SUCCESSFUL) {
							_status.m_code = VknStatusCode::FAIL;
							_status.append("\n\tVknExtDebugUtils::createVkDebugUtilsMessenger() - fail");
							return _status;
						}

						// ���� �� ������ ������� � ���������� ������ SUCCESSFUL
						_status.m_code = VknStatusCode::SUCCESSFUL;
						return _status;
					}

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

					void WvkExtDebugUtils::destroy(void) noexcept {

						//m_create_info.wvk_debug_utils_commands->vkDestroyDebugUtilsMessengerEXT();

						// ~~~~~~~~~~~~~~~~
						// ������� ������
						// ~~~~~~~~~~~~~~~~

						m_create_info = {};
						m_vk_debug_utils_messenger = nullptr;
						m_debug_message_collection.clear();
					}

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

					bool WvkExtDebugUtils::hasStatus(const VknDebugUtilsMode& mode) const noexcept {
						// ����� ����������� � ����, ��������������� ����������� ������
						VkDebugUtilsMessageSeverityFlagsEXT _severity = 0;
						VkDebugUtilsMessageTypeFlagsEXT _type = 0;

						// ����������� ����� ������������ � ����� Vulkan
						convertDebugUtilsModeToVkFlags(mode, _severity, _type);

						// �������� �� ���� ����������� ���������� �������
						for (const auto& msg : m_debug_message_collection) {
							// ���������, ������������� �� ��������� ���� �� ������ �� �������� ������
							if ((msg.severity & _severity) && (msg.type & _type)) {
								return true; // ������� ��������������� ���������
							}
						}

						// ��� ���������, ��������������� �������� ��������
						return false;
					}

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

					WvkStatus WvkExtDebugUtils::validationCreateInfo(void) const noexcept {
						WvkStatus _status;

						// ----------------------------------------------
						// ��������: ������ ���� ������� �������� ��������� 
						// �� ������ ������ ��� ������ � �������� Vulkan
						// ----------------------------------------------
						//if (m_create_info.wvk_dispatch_table_ptr == nullptr) {
						//	_status.m_code = VknStatusCode::FAIL;
						//	_status.append("\n\tVknExtDebugUtilsCreateInfo::wvk_dispatch_table_ptr - nullptr.");
						//	return _status;
						//}

						// ----------------------------------------------
						// ��������: ����� ������� ������ ���� ���������� 
						// � ���������� ��������, �� UNKNOWN
						// ----------------------------------------------
						if (m_create_info.mode == VknDebugUtilsMode::UNKNOWN) {
							_status.m_code = VknStatusCode::FAIL;
							_status.append("\n\tVknExtDebugUtilsCreateInfo::mode = VknDebugUtilsMode::UNKNOWN");
							return _status;
						}

						// ------------------------------------------------
						// ��� �������� �������� �������, ���������� OK
						// ------------------------------------------------
						_status.m_code = VknStatusCode::SUCCESSFUL;
						return _status;
					}

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

					void WvkExtDebugUtils::convertDebugUtilsModeToVkFlags(const VknDebugUtilsMode& mode, VkDebugUtilsMessageSeverityFlagsEXT& outSeverity, VkDebugUtilsMessageTypeFlagsEXT& outType) const noexcept {
						
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

					WvkStatus WvkExtDebugUtils::createVkDebugUtilsMessenger(void) noexcept {
						WvkStatus _status;

						// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						// ��� 1. ������������� ��������� VkDebugUtilsMessengerCreateInfoEXT
						// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						// �������������� ��������� VkDebugUtilsMessengerCreateInfoEXT ������
						VkDebugUtilsMessengerCreateInfoEXT _debug_utils_messenger_create_info = {};
						_debug_utils_messenger_create_info.sType = VK_STRUCTURE_TYPE_DEBUG_UTILS_MESSENGER_CREATE_INFO_EXT;
						_debug_utils_messenger_create_info.pNext = nullptr;
						_debug_utils_messenger_create_info.flags = 0;

						// ������ ����������� � ���� ��������� ����� ����������� ���� �� ������ ������
						_debug_utils_messenger_create_info.messageSeverity = 0;
						_debug_utils_messenger_create_info.messageType = 0;

						// ��������� callback-������� � ������� this � �������� ���������������� ������
						_debug_utils_messenger_create_info.pfnUserCallback = &s_callback;
						_debug_utils_messenger_create_info.pUserData = this;

						// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						// ��� 2. �������������� ������ ������� � ��������������� ����� Vulkan
						// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						// ����������� ����� ������� � ��������������� ����� Vulkan
						convertDebugUtilsModeToVkFlags(m_create_info.mode,
							_debug_utils_messenger_create_info.messageSeverity,
							_debug_utils_messenger_create_info.messageType);

						// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						// ��� 3. ������� �������� ����������� ��� �������
						// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						// �������� ������� ���������� ����� ��������� ���������
						//VkResult _vk_result = m_create_info.wvk_debug_utils_commands->vknCreateDebugUtilsMessengerEXT(
						//	&_debug_utils_messenger_create_info,
						//	nullptr,
						//	&m_vk_debug_utils_messenger);

						// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						// ��� 4. ��������� ���������� �������� �����������
						// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						// ��������� ����������
						//if (_vk_result != VK_SUCCESS) {
						//	_status.m_code = VknStatusCode::FAIL;

							// ������� ���������� �������� ������, ���� ��������
						//	switch (_vk_result) {
						//	case VK_ERROR_OUT_OF_HOST_MEMORY:
						//		_status.append("\n\tvknCreateDebugUtilsMessengerEXT - VK_ERROR_OUT_OF_HOST_MEMORY");
						//		break;
						//	default:
						//		_status.append("\n\tvknCreateDebugUtilsMessengerEXT - Unknown error");
						//		break;
						//	}

						//	return _status;
						//}

						// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						// ��� 5. �������� ����������
						// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						// �������� ����������
						_status.m_code = VknStatusCode::SUCCESSFUL;
						return _status;
					}

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

					VkBool32 WvkExtDebugUtils::s_callback(VkDebugUtilsMessageSeverityFlagBitsEXT messageSeverity, VkDebugUtilsMessageTypeFlagsEXT messageTypes, const VkDebugUtilsMessengerCallbackDataEXT* pCallbackData, void* pUserData) noexcept {

						auto* _this = static_cast<WvkExtDebugUtils*>(pUserData);

						// ���������� ��������� � �������
						if (_this) {
							DebugMessage _msg;
							_msg.severity = messageSeverity;
							_msg.type = messageTypes;
							_msg.message = pCallbackData->pMessage;
							_this->m_debug_message_collection.push_back(std::move(_msg));
						}

						return VK_FALSE; // �� ��������� ����������
					}

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				} // namespace Extensions

			} // namespace wvk

		//} // namespace Private

	//} // namespace GPU

} // namespace CGDev