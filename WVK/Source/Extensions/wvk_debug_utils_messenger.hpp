// SPDX-License-Identifier: AGPL-3.0-or-later
#ifndef CGDEV_WVK_SOURCE_EXTENSION__WVK_DEBUG_UTILS_MESSENGER_HPP
#define CGDEV_WVK_SOURCE_EXTENSION__WVK_DEBUG_UTILS_MESSENGER_HPP
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

namespace CGDev {

	namespace wvk {

		namespace Extensions {
					
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			inline WvkStatus WvkDebugUtilsMessenger::hasIssues(const VknDebugUtilsMode& mode) noexcept {
				WvkStatus _status;

				VkDebugUtilsMessageSeverityFlagsEXT _severity = 0;
				VkDebugUtilsMessageTypeFlagsEXT _type = 0;

				// Преобразуем режим пользователя в флаги Vulkan
				convertDebugUtilsModeToVkFlags(mode, _severity, _type);

				if (_severity == 0) _severity = VK_DEBUG_UTILS_MESSAGE_SEVERITY_ERROR_BIT_EXT | VK_DEBUG_UTILS_MESSAGE_SEVERITY_WARNING_BIT_EXT | VK_DEBUG_UTILS_MESSAGE_SEVERITY_INFO_BIT_EXT | VK_DEBUG_UTILS_MESSAGE_SEVERITY_VERBOSE_BIT_EXT;
				if (_type == 0) _type = VK_DEBUG_UTILS_MESSAGE_TYPE_GENERAL_BIT_EXT | VK_DEBUG_UTILS_MESSAGE_TYPE_VALIDATION_BIT_EXT | VK_DEBUG_UTILS_MESSAGE_TYPE_PERFORMANCE_BIT_EXT;

				for (const auto& msg : m_debug_message_collection) {
					if ((msg.severity & _severity) && (msg.type & _type)) {
						_status.append(msg.message.c_str());
						m_debug_message_collection.clear();
						return _status.setFail();
					}
				}

				return _status.setOk();
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			inline WvkStatus WvkDebugUtilsMessenger::hasErrorsWarnings(void) noexcept {
				return hasIssues(VknDebugUtilsMode::ERRORS_ONLY | VknDebugUtilsMode::WARNINGS_ONLY | VknDebugUtilsMode::VALIDATION | VknDebugUtilsMode::PERFORMANCE);
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		} // namespace Extensions

	} // namespace wvk

} // namespace CGDev

#endif // CGDEV_WVK_SOURCE_EXTENSION__WVK_DEBUG_UTILS_MESSENGER_HPP
