#ifndef CGDEV_SOURCE_GPU_PRIVATE_VULKAN__VKN_COMMAND_BUFFER_HPP
#define CGDEV_SOURCE_GPU_PRIVATE_VULKAN__VKN_COMMAND_BUFFER_HPP
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

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				inline const VknCommandBufferCreateInfo& VknCommandBuffer::getCreateInfo(void) const noexcept {

					return m_create_info;
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				inline const VkCommandBuffer& VknCommandBuffer::getVkCommandBuffer(void) const noexcept {

					return me_vk_command_buffer;
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			} // namespace wvk

		//} // namespace Private

	//} // namespace GPU

} // namespace CGDev

#endif // CGDEV_SOURCE_GPU_PRIVATE_VULKAN__VKN_COMMAND_BUFFER_HPP
