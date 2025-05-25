#ifndef CGDEV_SOURCE_GPU_PRIVATE_VULKAN_EXTENSIONS__VKN_EXT_DEBUG_UTILS_COMMANDS_HPP
#define CGDEV_SOURCE_GPU_PRIVATE_VULKAN_EXTENSIONS__VKN_EXT_DEBUG_UTILS_COMMANDS_HPP
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

	//namespace GPU {

		//namespace Private {

			namespace wvk {

				namespace Extensions {

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

					inline VkResult VknExtDebugUtilsCommands::vknCreateDebugUtilsMessengerEXT(const VkDebugUtilsMessengerCreateInfoEXT* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkDebugUtilsMessengerEXT* pMessenger) const noexcept {
						return m_vkCreateDebugUtilsMessengerEXT(m_create_info.wvk_commands->getCreateInfo().wvk_instance->getVkInstance(), pCreateInfo, pAllocator, pMessenger);
					}

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

					inline void VknExtDebugUtilsCommands::vknDestroyDebugUtilsMessengerEXT(VkDebugUtilsMessengerEXT messenger, const VkAllocationCallbacks* pAllocator) const noexcept {
						m_vkDestroyDebugUtilsMessengerEXT(m_create_info.wvk_commands->getCreateInfo().wvk_instance->getVkInstance(), messenger, pAllocator);
					}

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

					inline void VknExtDebugUtilsCommands::vknSubmitDebugUtilsMessageEXT(VkDebugUtilsMessageSeverityFlagBitsEXT messageSeverity, VkDebugUtilsMessageTypeFlagsEXT messageTypes, const VkDebugUtilsMessengerCallbackDataEXT* pCallbackData) const noexcept {
						m_vkSubmitDebugUtilsMessageEXT(m_create_info.wvk_commands->getCreateInfo().wvk_instance->getVkInstance(), messageSeverity, messageTypes, pCallbackData);
					}

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

					inline const VknExtDebugUtilsCommandsCreateInfo& VknExtDebugUtilsCommands::getCreateInfo(void) const noexcept {
						return m_create_info;
					}

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				} // namespace Extensions

			} // namespace wvk

		//} // namespace Private

	//} // namespace GPU

} // namespace CGDev

#endif // CGDEV_SOURCE_GPU_PRIVATE_VULKAN_EXTENSIONS__VKN_EXT_DEBUG_UTILS_COMMANDS_HPP
