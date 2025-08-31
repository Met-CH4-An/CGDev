// SPDX-License-Identifier: AGPL-3.0-or-later
#ifndef CGDEV_WVK_SOURCE_EXTENSION__WVK_DEBUG_UTILS_MESSENGER_H
#define CGDEV_WVK_SOURCE_EXTENSION__WVK_DEBUG_UTILS_MESSENGER_H
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
#include "../wvk_base_object.h"
////////////////////////////////////////////////////////////////
// ������ ��� ����������
////////////////////////////////////////////////////////////////

namespace CGDev {

	namespace wvk {

		namespace Extensions {

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			struct DebugMessage {
				VkDebugUtilsMessageSeverityFlagBitsEXT severity;  //!< ������� ����������� ��������� (������, �������������� � �.�.).
				VkDebugUtilsMessageTypeFlagsEXT type;             //!< ��� ��������� (���������, ������������������, �����).
				std::string message;                              //!< ��������� �������� ���������.
			}; // struct DebugMessage



			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
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
			}; // enum class VknDebugUtilsMode
			ENABLE_ENUM_FLAGS(VknDebugUtilsMode);



			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			struct WvkDebugUtilsMessengerCreateInfo {
				WvkInstancePtr wvk_instance_ptr = nullptr;
				VknDebugUtilsMode mode = VknDebugUtilsMode::UNKNOWN;
			}; // struct WvkDebugUtilsMessengerCreateInfo



			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			class WvkDebugUtilsMessenger : public GpuObject {

			public:

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*/
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				WvkDebugUtilsMessenger(void) noexcept;

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*/
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				~WvkDebugUtilsMessenger(void) noexcept;

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*/
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				WvkStatus create(const WvkDebugUtilsMessengerCreateInfo& create_info) noexcept;

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*/
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				void destroy(void) noexcept;

			// hpp
			public:

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*/
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				inline WvkStatus hasIssues(const VknDebugUtilsMode& mode = VknDebugUtilsMode::UNKNOWN) noexcept;

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*/
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				inline WvkStatus hasErrorsWarnings(void) noexcept;

			// cpp
			private:

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*/
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				WvkStatus validationCreateInfo(const WvkDebugUtilsMessengerCreateInfo& create_info) noexcept;
						
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*/
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				void convertDebugUtilsModeToVkFlags(const VknDebugUtilsMode& mode, VkDebugUtilsMessageSeverityFlagsEXT& outSeverity, VkDebugUtilsMessageTypeFlagsEXT& outType) const noexcept;

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*/
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				WvkStatus create(void) noexcept;

			private:

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*/
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				static VkBool32 s_callback(VkDebugUtilsMessageSeverityFlagBitsEXT messageSeverity, VkDebugUtilsMessageTypeFlagsEXT messageTypes, const VkDebugUtilsMessengerCallbackDataEXT* pCallbackData, void* pUserData) noexcept;

			private:

				WvkDebugUtilsMessengerCreateInfo m_create_info = {};
				VkDebugUtilsMessengerEXT m_vk_debug_utils_messenger = VK_NULL_HANDLE;
				std::vector<DebugMessage> m_debug_message_collection;
			}; // class WvkDebugUtilsMessenger

		} // namespace Extensions

	} // namespace wvk

} // namespace CGDev

#include "wvk_debug_utils_messenger.hpp"

#endif // CGDEV_WVK_SOURCE_EXTENSION__WVK_DEBUG_UTILS_MESSENGER_H
