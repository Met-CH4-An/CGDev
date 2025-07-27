////////////////////////////////////////////////////////////////
// ������ �������-����������
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// ������ ������������� �����
////////////////////////////////////////////////////////////////
#include "wvk_ext_debug_utils_commands.h"
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

					VknExtDebugUtilsCommands::VknExtDebugUtilsCommands(void) noexcept {
					}

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

					VknExtDebugUtilsCommands::~VknExtDebugUtilsCommands(void) noexcept {
					}

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

					WvkStatus VknExtDebugUtilsCommands::create(const VknExtDebugUtilsCommandsCreateInfo& create_info) noexcept {
						WvkStatus _status;

						// ��������� ������� ��������� �������������.
						m_create_info = create_info;

						// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						// ���� ������ ������������ � ���������� ����������,
						// ��������� �������� ������������ ��������� CreateInfo.
						// ��� ��������� ������������� ������ �� ����� ����������.
						// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						//if constexpr (wvk::Build::ValidationBuildInfo::enable == true) {
							_status = validationCreateInfo();

							// ���� ��������� ��������� � ��������� � �������.
							if (_status.m_code != VknStatusCode::SUCCESSFUL) {
								_status.m_code = VknStatusCode::FAIL;
								_status.append("\n\tVknExtDebugUtilsCommands::validationCreateInfo() - fail");
								return _status;
							}
						//}

						// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						// �������� ��������� ������ ������� ���������� VK_EXT_debug_utils.
						// ������������ ��������������� ����� loadVulkanCommand().
						// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						_status = loadVulkanCommand();

						// ��������� ��������� ������ ��������.
						if (_status.m_code != VknStatusCode::SUCCESSFUL) {
							_status.m_code = VknStatusCode::FAIL;
							_status.append("\n\tVknExtDebugUtilsCommands::loadVulkanCommand() - fail");
							return _status;
						}

						// �������� �������������.
						_status.m_code = VknStatusCode::SUCCESSFUL;
						return _status;
					}

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

					void VknExtDebugUtilsCommands::destroy(void) noexcept {

						// ~~~~~~~~~~~~~~~~
						// ������� ������
						// ~~~~~~~~~~~~~~~~

						m_create_info = {};

						m_vkCreateDebugUtilsMessengerEXT = nullptr;
						m_vkDestroyDebugUtilsMessengerEXT = nullptr;
						m_vkSubmitDebugUtilsMessageEXT = nullptr;
					}

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

					WvkStatus VknExtDebugUtilsCommands::validationCreateInfo(void) const noexcept {
						WvkStatus _status;

						// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						// ���������, ��� ������������ ���� wvk_commands �� ����� nullptr.
						// ��� �������� ��� ����������� �������� Vulkan-�������.
						// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						if (m_create_info.wvk_commands == nullptr) {
							_status.m_code = VknStatusCode::FAIL;

							// ��������� �������� ������ � ���������.
							_status.append("\n\tVknExtDebugUtilsCommandsCreateInfo::wvk_commands = nullptr");

							return _status;
						}

						// ��� �������� �������� �������.
						_status.m_code = VknStatusCode::SUCCESSFUL;
						return _status;
					}

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

					WvkStatus VknExtDebugUtilsCommands::loadVulkanCommand(void) noexcept {
						// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						// ��� 1. ��������� ���������� ��� �������� ���������� ��������
						// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						WvkStatus _status;

						// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						// ��� 2. �������� ��������������� ������� tryLoadFunction,
						//        ��������� ������ Vulkan-�������, ������� ����� ���������
						// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						_status = m_create_info.wvk_commands->tryLoadFunction({
						//tryLoadFunction(m_create_info.wvk_commands, {
							{ "vkCreateDebugUtilsMessengerEXT", reinterpret_cast<void**>(&m_vkCreateDebugUtilsMessengerEXT) },
							{ "vkDestroyDebugUtilsMessengerEXT", reinterpret_cast<void**>(&m_vkDestroyDebugUtilsMessengerEXT) },
							{ "vkSubmitDebugUtilsMessageEXT", reinterpret_cast<void**>(&m_vkSubmitDebugUtilsMessageEXT) }
						});

						// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						// ��� 3. ���������� ��������� ������� �������� �������
						// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						return _status;
					}

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				} // namespace Extensions

			} // namespace wvk

		//} // namespace Private

	//} // namespace GPU

} // namespace CGDev