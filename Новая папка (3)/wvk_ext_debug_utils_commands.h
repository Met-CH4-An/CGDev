#ifndef CGDEV_SOURCE_GPU_PRIVATE_VULKAN_EXTENSIONS__VKN_EXT_DEBUG_UTILS_COMMANDS_H
#define CGDEV_SOURCE_GPU_PRIVATE_VULKAN_EXTENSIONS__VKN_EXT_DEBUG_UTILS_COMMANDS_H
////////////////////////////////////////////////////////////////
// ������ �������-����������
////////////////////////////////////////////////////////////////
#include "_extensions.h"
////////////////////////////////////////////////////////////////
// ������ �������������
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// ������ ������������� ������
////////////////////////////////////////////////////////////////
#include "../wvk_base_commands.h"
////////////////////////////////////////////////////////////////
// ������ ��� ����������
////////////////////////////////////////////////////////////////
#include "../wvk_commands.h"

namespace CGDev {

	//namespace GPU {

		//namespace Private {

			namespace wvk {

				namespace Extensions {

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					struct VknExtDebugUtilsCommandsCreateInfo {					
						WvkInstanceDispatchTablePtr wvk_commands = nullptr;
					}; // VknExtDebugUtilsCommandsCreateInfo





					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					class VknExtDebugUtilsCommands : public VknBaseCommands {

					public:

						inline VkResult vknCreateDebugUtilsMessengerEXT(const VkDebugUtilsMessengerCreateInfoEXT* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkDebugUtilsMessengerEXT* pMessenger) const noexcept;
						inline void vknDestroyDebugUtilsMessengerEXT(VkDebugUtilsMessengerEXT messenger, const VkAllocationCallbacks* pAllocator) const noexcept;
						inline void vknSubmitDebugUtilsMessageEXT(VkDebugUtilsMessageSeverityFlagBitsEXT messageSeverity, VkDebugUtilsMessageTypeFlagsEXT messageTypes, const VkDebugUtilsMessengerCallbackDataEXT* pCallbackData) const noexcept;

					private:

						PFN_vkCreateDebugUtilsMessengerEXT									m_vkCreateDebugUtilsMessengerEXT = nullptr;
						PFN_vkDestroyDebugUtilsMessengerEXT									m_vkDestroyDebugUtilsMessengerEXT = nullptr;
						PFN_vkSubmitDebugUtilsMessageEXT									m_vkSubmitDebugUtilsMessageEXT = nullptr;

					public:

						//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						/*!	\brief
						*/
						//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						VknExtDebugUtilsCommands(void) noexcept;

						//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						/*!	\brief
						*/
						//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						~VknExtDebugUtilsCommands(void) noexcept;

						//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						/*!	\brief �������������� ������ � ��������� ������� ���������� `VK_EXT_debug_utils`.
						*
						* ����� ��������� ���������� ��������� `VknExtDebugUtilsCommandsCreateInfo`, ��������� �
						* �������� (��� ���������� ������ � ����������), � ����� �������� ��������� ��� �����������
						* ������� ���������� `VK_EXT_debug_utils`. ������������ ���������� ����� `loadVulkanCommand()`.
						*
						* ��� ������������� ������ �� ����� ����� ������������ ������ `VknStatusCode::FAIL` � ���������
						* ��������� ��������.
						*
						* @param [in] create_info ��������� � ����������� �������������, ������� ������������ ���������
						* �� `VknCommands`, ����������� ��� �������� ������� �������.
						*
						* @return [out] ������ ���������� ��������. ��� ������� �������� �������� ������.
						*
						* @code
						* VknExtDebugUtilsCommands debug_utils_cmds;
						* VknExtDebugUtilsCommandsCreateInfo info;
						* info.wvk_commands = &commands;
						* info.mode = VknDebugUtilsMode::ALL_SEVERITIES | VknDebugUtilsMode::ALL_TYPES;
						*
						* WvkStatus status = debug_utils_cmds.create(info);
						* if (status.m_code != VknStatusCode::SUCCESSFUL) {
						*     std::cerr << "������ ������������� DebugUtils: " << status.message_old << std::endl;
						* }
						* @endcode
						*
						* @sa VknExtDebugUtilsCommandsCreateInfo
						* @sa VknExtDebugUtilsCommands::validationCreateInfo
						* @sa VknExtDebugUtilsCommands::loadVulkanCommand
						*/
						//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						WvkStatus create(const VknExtDebugUtilsCommandsCreateInfo& create_info) noexcept;

						//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						/*!	\brief
						*/
						//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						void destroy(void) noexcept;

					public:

						//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						/*!	\brief ���������� ������ �� ��������� ���������� �������������.
						*
						* ����� ������������� ������ � ����������� ������� `VknExtDebugUtilsCommandsCreateInfo`,
						* ������� ��� ������� ��� �������� ������� � ������� ������ `create()`.
						*
						* ������������, ���� ���������� �������� ������� ��������� ��� ���������,
						* ��������� � ������������� ������� ���������� `VK_EXT_debug_utils`.
						*
						* @return [out] ����������� ������ �� ��������� `VknExtDebugUtilsCommandsCreateInfo`.
						*
						* @code
						* const auto& info = debug_utils_cmds.getCreateInfo();
						* if (info.wvk_debug_utils_commands) {
						*     // ���������� ��������� ������� ���������
						* }
						* @endcode
						*
						* @sa VknExtDebugUtilsCommandsCreateInfo
						* @sa VknExtDebugUtilsCommands::create
						*/
						//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						inline const VknExtDebugUtilsCommandsCreateInfo& getCreateInfo(void) const noexcept;

					private:

						//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						/*!	\brief ��������� ��������� ������� ������, ����������� ��� ������ ����������� ������.
						*
						* ����� ��������� ������������ ��������� `VknExtDebugUtilsCreateInfo`, ���������� ��� ��������
						* ������� `VknExtDebugUtilsCommands`. � ���������, �� ��������������, ��� ���� `wvk_commands`
						* �� �������� `nullptr`, ��� ��� ��� ���������� ���������� ��� �������� ������� ����������
						* `VK_EXT_debug_utils`.
						*
						* � ������, ���� `wvk_commands` �� ����������, ����� ���������� ��� ������ `VknStatusCode::FAIL`
						* � ��������� � ���������, ����� ������ ���� ���������.
						*
						* @return [out] ������ ���������� ���������. ��� ������ �������� ��������� � ��������� �������.
						*
						* @code
						* VknExtDebugUtilsCommands debug_utils_cmds;
						* WvkStatus status = debug_utils_cmds.validationCreateInfo();
						* if (status.m_code != VknStatusCode::SUCCESSFUL) {
						*     std::cerr << "������ ��������� CreateInfo: " << status.message_old << std::endl;
						* }
						* @endcode
						*
						* @sa VknExtDebugUtilsCreateInfo
						*/
						//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						WvkStatus validationCreateInfo(void) const noexcept;

						//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						/*!	\brief ��������� ��������� �� ������� ���������� VK_EXT_debug_utils.
						* 
						* ����� ��������� ��� ������� �� ���������� VK_EXT_debug_utils:
						* - vkCreateDebugUtilsMessengerEXT
						* - vkDestroyDebugUtilsMessengerEXT
						* - vkSubmitDebugUtilsMessageEXT
						* 
						* ��� �������� ������������ ��������������� ������� tryLoadFunction, 
						* ������� ���������� ���� "��� ������� � ��������� ��� ������".
						* 
						* @return WvkStatus 
						* - VKN_SUCCESS � ���� ��� ������� ������� ���������.
						* - ���� ��� ������ � ���� ���� �� ���� ������� �� ���� ���������.
						* 
						* @note ����� �� ���������� ���������� (noexcept).
						* 
						* @code
						* VknExtDebugUtilsCommands debugUtils;
						* WvkStatus status = debugUtils.loadVulkanCommand();
						* if (status != VKN_SUCCESS) {
						*     // ��������� ������
						* }
						* @endcode
						*/
						//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						WvkStatus loadVulkanCommand(void) noexcept;

					private:

						VknExtDebugUtilsCommandsCreateInfo											m_create_info = {};
						
					}; // class VknExtDebugUtilsCommands

				} // namespace Extensions

			} // namespace wvk

		//} // namespace Private

	//} // namespace GPU

} // namespace CGDev

#include "wvk_ext_debug_utils_commands.hpp"

#endif // CGDEV_SOURCE_GPU_PRIVATE_VULKAN_EXTENSIONS__VKN_EXT_DEBUG_UTILS_COMMANDS_H
