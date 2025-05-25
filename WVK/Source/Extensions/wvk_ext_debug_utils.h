#ifndef CGDEV_SOURCE_GPU_PRIVATE_VULKAN_EXTENSIONS__VKN_EXT_DEBUG_UTILS_H
#define CGDEV_SOURCE_GPU_PRIVATE_VULKAN_EXTENSIONS__VKN_EXT_DEBUG_UTILS_H
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
#include "../wvk_extension.h"
////////////////////////////////////////////////////////////////
// ������ ��� ����������
////////////////////////////////////////////////////////////////

namespace CGDev {

	//namespace GPU {

		//namespace Private {

			namespace wvk {

				namespace Extensions {

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief ������������ ���� ���������� ��������� �� Vulkan ����� VK_EXT_debug_utils.
					*
					* ��������� `DebugMessage` ������ ��������� ������ ���������, ����������������
					* �������� ������� Vulkan. ��� ������������ � ��������� ������ `VknExtDebugUtils`
					* ��� �������� ������� ��������� � ������������ �������.
					*
					* \sa VknExtDebugUtils::hasStatus
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					struct DebugMessage {
						VkDebugUtilsMessageSeverityFlagBitsEXT severity;  //!< ������� ����������� ��������� (������, �������������� � �.�.).
						VkDebugUtilsMessageTypeFlagsEXT type;             //!< ��� ��������� (���������, ������������������, �����).
						std::string message;                              //!< ��������� �������� ���������.
					};





					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief ����� ���������� � �������� �� ��������� ������� Vulkan.
					*
					* `VknDebugUtilsMode` � ��� ������������ � �������, ������������, �����
					* ��������� ������� ������� �������������. ��� ���������� ������ �����������
					* (������, ��������������, ����������, �����������) � ���� ���������
					* (���������, ������������������, �����).
					*
					* �������� ����� ������������� � ������� ��������� ��������:
					* @code
					* VknDebugUtilsMode mode = VknDebugUtilsMode::ERRORS_ONLY | VknDebugUtilsMode::VALIDATION;
					* @endcode
					*
					* \note ������ ENABLE_ENUM_FLAGS ��������� ������������ ��������� �������� � ���� �������������.
					*
					* \sa VknExtDebugUtils::create
					* \sa VknExtDebugUtils::convertDebugUtilsModeToVkFlags
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					enum class VknDebugUtilsMode : uint32_t {
						UNKNOWN = 0,                                    //!< ����� �� ����� ��� ����������.
						ERRORS_ONLY = 1 << 0,                           //!< ������ ��������� �� �������.
						WARNINGS_ONLY = 1 << 1,                         //!< ������ ��������������.
						INFO_ONLY = 1 << 2,                             //!< �������������� ���������.
						VERBOSE = 1 << 3,                               //!< ��������� (debug-level) ���������.
						ALL_SEVERITIES = ERRORS_ONLY | WARNINGS_ONLY | INFO_ONLY | VERBOSE, //!< ��� ������ �����������.

						VALIDATION = 1 << 4,                            //!< ���������, ��������� � ����������.
						PERFORMANCE = 1 << 5,                           //!< ���������������� (performance) ���������.
						GENERAL = 1 << 6,                               //!< ����� ���������.
						ALL_TYPES = VALIDATION | PERFORMANCE | GENERAL  //!< ��� ���� ���������.
					};
					ENABLE_ENUM_FLAGS(VknDebugUtilsMode);





					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief ��������� ���������� �������� ��� `VknExtDebugUtils`.
					*
					* �������� ��������� �� ������ ������ ���������� �������, � �����
					* ����� ������� `VknDebugUtilsMode`, ������� ����������, ����� ���������
					* ������ ���������������.
					*
					* ������������ ��� ������ `VknExtDebugUtils::create()`.
					*
					* \note ��������� `wvk_debug_utils_commands` ���������� � �� ������ ���� nullptr.
					*
					* \sa VknExtDebugUtils
					* \sa VknExtDebugUtils::create
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					struct VknExtDebugUtilsCreateInfo {
						VknExtDebugUtilsCommandsPtr wvk_debug_utils_commands = nullptr; //!< ��������� �� ������ ������� ����������.
						VknDebugUtilsMode mode = VknDebugUtilsMode::UNKNOWN;            //!< ����� ���������� ���������� ���������.
					};





					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief ���������� ������� Vulkan � ���������� VK_EXT_debug_utils.
					*
					* ����� `VknExtDebugUtils` ������������� ������ �������� � ����������
					* ����������� ����������� � Vulkan ����� ���������� `VK_EXT_debug_utils`.
					*
					* �� �������������:
					* - �������� ������� `VkDebugUtilsMessengerEXT`,
					* - ���������� ��������� �� ���� � ������ �����������,
					* - ���������� ��������� ������� ����������,
					* - ��������������� �������������� ������� ������������ � ����� Vulkan.
					*
					* ������������ ��� ������� ���������� ������� ��� ���������� Vulkan-����������.
					*
					* ������:
					* @code
					* VknExtDebugUtils debugUtils;
					* VknExtDebugUtilsCreateInfo info = {...};
					* debugUtils.create(info);
					* if (debugUtils.hasStatus(VknDebugUtilsMode::ERRORS_ONLY)) {
					*     std::cerr << "������ ������� ����������!" << std::endl;
					* }
					* @endcode
					*
					* \see VkDebugUtilsMessengerEXT
					* \see VknDebugUtilsMode
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					class VknExtDebugUtils : public VknExtension {

					public:

						//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						/*!	\brief
						*/
						//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						inline static const std::string& s_getName(void) noexcept;

					private:

						inline static const std::string s_name = "VK_EXT_debug_utils";





					public:

						//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						/*!	\brief
						*/
						//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						VknExtDebugUtils(void) noexcept;

						//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						/*!	\brief
						*/
						//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						~VknExtDebugUtils(void) noexcept;

						//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						/*!	\brief �������������� ���������� ��������� Vulkan � �������� �������������.
						*
						* ����� ��������� ������� ��������� `VknExtDebugUtilsCreateInfo` �� ���������� ���������,
						* ��������� ��������� �����, ���� ������ �������� �������� (`ValidationBuildInfo::enable == true`),
						* � �������� �������� `VkDebugUtilsMessengerEXT`.
						*
						* ���� ����� �� �������� ����������� �������� � ����� ���������� ������ `VknStatus` � ��������� ������.
						*
						* @param[in] create_info ���������, ���������� ��������� �������� ����������� ����������.
						* @return [out] WvkStatus ������ ������� ����������, ���������� ��� ���������� � ��������� ��������� �� ������.
						*
						* ������ �������������:
						* @code
						* VknExtDebugUtilsCreateInfo info = {};
						* info.mode = VknDebugUtilsMode::ALL;
						* info.wvk_debug_utils_commands = &debugCommands;
						*
						* VknExtDebugUtils debugUtils;
						* WvkStatus status = debugUtils.create(info);
						* if (!status) {
						*     std::cerr << "Failed to create debug utils: " << status.message_old << std::endl;
						* }
						* @endcode
						*/
						//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						WvkStatus create(const VknExtDebugUtilsCreateInfo& create_info) noexcept;

						//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						/*!	\brief
						*/
						//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						void destroy(void) noexcept;

					public:

						inline const VknExtDebugUtilsCreateInfo& getCreateInfo(void) const noexcept {
							return m_create_info;
						}

					public:

						//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						/*!	\brief ���������, �������� �� ��������� ��������� ��������� � ��������� ������� ����������� � ����.
						*
						* ��� ������� ��������� ����������, ���� �� ���������������� ��������� ������� Vulkan,
						* ��������������� ��������� ������ `VknDebugUtilsMode`, ������� �������� ����� ������� �����������
						* (����� ��� ERROR, WARNING � �.�.) � ���� ��������� (VALIDATION, PERFORMANCE � �.�.).
						*
						* @param[in] mode ����� ���������� ���������, ���������� ���������� ������ VknDebugUtilsMode.
						* @return true, ���� ������� ���� �� ���� ���������, ��������������� ��������.
						* @return false, ���� ����� ��������� ���.
						*
						* ������:
						* @code
						* VknExtDebugUtils debugUtils;
						* if (debugUtils.check(VknDebugUtilsMode::ERRORS_ONLY | VknDebugUtilsMode::VALIDATION)) {
						*     std::cerr << "Validation error detected!" << std::endl;
						* }
						* @endcode
						*/
						//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						bool hasStatus(const VknDebugUtilsMode& mode) const noexcept;

					private:

						//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						/*!	\brief ��������� ������������ ����� ��������� VknExtDebugUtilsCreateInfo.
						* 
						* ����� ������������ ��� ������� ��������� ������� ������ ����� ��������� ������� ������� Vulkan.
						* ���������� ������ ��� ���������� � ���������� ���������� (��. ValidationBuildInfo::enable).
						*
						* @return WvkStatus
						*         ���������� ������ �� �������� ��������:
						*         - VknStatusCode::SUCCESSFUL � ���� ��� �������� �������� �������.
						*         - VknStatusCode::FAIL � ���� ���������� ������, � ���������� ���������� � ���� `message`.
						* 
						* @code
						* VknExtDebugUtils debugUtils;
						* VknExtDebugUtilsCreateInfo info = {...};
						* debugUtils.create(info); // ������ ����� ������� validationCreateInfo()
						* @endcode
						*/
						//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						WvkStatus validationCreateInfo(void) const noexcept;
						
						//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						/*!	\brief ����������� ���������������� ����� ������� VknDebugUtilsMode � ��������������� ����� Vulkan.
						*
						* ��� ������� ������������ �������� ������������ VknDebugUtilsMode � ����������������
						* �������� ������� Vulkan `VkDebugUtilsMessageSeverityFlagsEXT` � `VkDebugUtilsMessageTypeFlagsEXT`.
						* ��� ������������ ��� �������� ������� `VkDebugUtilsMessengerEXT` ��� ��������, ����� ���������
						* ������� �������������: ������, ��������������, �������������� ��� ��������� ���������,
						* � ����� �� ����: �����, ������������� ��� ����������������.
						*
						* @param[in] mode ���������������� ����� �������, ������������ ������ ����������� � ���� ���������.
						* @param[out] outSeverity �������� ��������, � ������� ����� ����������� ����� ������� ����������� (`VkDebugUtilsMessageSeverityFlagsEXT`).
						* @param[out] outType �������� ��������, � ������� ����� ����������� ����� ����� ��������� (`VkDebugUtilsMessageTypeFlagsEXT`).
						*
						* @note ��� ������� ���������� ������� ����� ��� ����������� �������� ������ � `mode`.
						*       �������� `outSeverity` � `outType` �� ���������� ������ � ��������������, ��� ���������� ���
						*       �������������� �� �� ������, ���� �����.
						*
						* @see VknDebugUtilsMode
						* @see VkDebugUtilsMessageSeverityFlagBitsEXT
						* @see VkDebugUtilsMessageTypeFlagBitsEXT
						* 
						* @code
						* VknDebugUtilsMode mode = VknDebugUtilsMode::ERRORS_ONLY | VknDebugUtilsMode::VALIDATION;
						* VkDebugUtilsMessageSeverityFlagsEXT severity = 0;
						* VkDebugUtilsMessageTypeFlagsEXT type = 0;
						*
						* VknExtDebugUtils debugUtils;
						* debugUtils.convertDebugUtilsModeToVkFlags(mode, severity, type);
						* // ������ severity �������� VK_DEBUG_UTILS_MESSAGE_SEVERITY_ERROR_BIT_EXT
						* // � type �������� VK_DEBUG_UTILS_MESSAGE_TYPE_VALIDATION_BIT_EXT
						* @endcode
						*/
						//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						void convertDebugUtilsModeToVkFlags(const VknDebugUtilsMode& mode, VkDebugUtilsMessageSeverityFlagsEXT& outSeverity, VkDebugUtilsMessageTypeFlagsEXT& outType) const noexcept;

						//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						/*!	\brief ������ ���������� ��� ������� � �������������� ���������� Vulkan VK_EXT_debug_utils.
						* 
						* ���� ����� �������������� ��������� VkDebugUtilsMessengerCreateInfoEXT, ��������� � ������������ ����������, 
						* ������������� callback-������� ��� ��������� ��������� �� Vulkan � �������� ������� ����������.
						* ���� �������� ����������� �� ������, ������������ ��� ������.
						*
						* @return ���������� ������ ���� WvkStatus, ������� �������� ��� ���������� ��������.
						*
						* @code
						* // ������ �������������:
						* VknExtDebugUtils debugUtils;
						* WvkStatus status = debugUtils.createVkDebugUtilsMessenger();
						* if (status) {
						*     // ���������� ������� ������
						* } else {
						*     // ��������� ������
						* }
						* @endcode
						*/
						//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						WvkStatus createVkDebugUtilsMessenger(void) noexcept;

					private:

						//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						/*!	\brief
						*/
						//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						static VkBool32 s_callback(VkDebugUtilsMessageSeverityFlagBitsEXT messageSeverity, VkDebugUtilsMessageTypeFlagsEXT messageTypes, const VkDebugUtilsMessengerCallbackDataEXT* pCallbackData, void* pUserData) noexcept;

					private:

						VknExtDebugUtilsCreateInfo				m_create_info = {};
						VkDebugUtilsMessengerEXT				m_vk_debug_utils_messenger = nullptr;
						std::vector<DebugMessage>				m_debug_message_collection;

					}; // class VknExtDebugUtils

				} // namespace Extensions

			} // namespace wvk

		//} // namespace Private

	//} // namespace GPU

} // namespace CGDev

#include "wvk_ext_debug_utils.hpp"

#endif // CGDEV_SOURCE_GPU_PRIVATE_VULKAN_EXTENSIONS__VKN_EXT_DEBUG_UTILS_H
