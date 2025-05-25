////////////////////////////////////////////////////////////////
// секция форвард-декларации
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// секция заголовочного файла
////////////////////////////////////////////////////////////////
#include "wvk_command_buffer.h"
////////////////////////////////////////////////////////////////
// секция имплементации
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// секция для остального
////////////////////////////////////////////////////////////////
#include "wvk_commands.h"
#include "wvk_logical_device.h"

namespace CGDev {

	//namespace GPU {

		//namespace Private {

			namespace wvk {

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				bool VknCommandBufferCreateInfo::isValid(void) const noexcept {

					//if (log == nullptr) {

					//	return false;
					//}

					// ~~~~~~~~~~~~~~~~
					// VknCommandBufferCreateInfo::validation
					// ~~~~~~~~~~~~~~~~

					//if (validation == nullptr || validation->isValid() == false) {

					//	CGDev::Tools::addEntry(log, CGDev::Tools::LogEntryError("VknCommandBufferCreateInfo::validation fail"));

					//	return false;
					//}

					// ~~~~~~~~~~~~~~~~
					// VknCommandBufferCreateInfo::commands
					// ~~~~~~~~~~~~~~~~

					//if (commands == nullptr || commands->isValid() == false || commands->getCreateInfo().command_level != VknCommandsLevel::INSTANCE) {

					//	CGDev::Tools::addEntry(log, CGDev::Tools::LogEntryError("VknCommandBufferCreateInfo::commands fail"));

					//	return false;
					//}

					// ~~~~~~~~~~~~~~~~
					// VknCommandBufferCreateInfo::logical_device
					// ~~~~~~~~~~~~~~~~

					//if (logical_device == nullptr || logical_device->isValid() == false) {

					//	CGDev::Tools::addEntry(log, CGDev::Tools::LogEntryError("VknCommandBufferCreateInfo::logical_device fail"));

					//	return false;
					//}

					// ~~~~~~~~~~~~~~~~
					// VknCommandBufferCreateInfo::vk_command_buffer
					// ~~~~~~~~~~~~~~~~

					//if (vk_command_buffer == VK_NULL_HANDLE) {

					//	CGDev::Tools::addEntry(log, CGDev::Tools::LogEntryError("VknCommandBufferCreateInfo::vk_command_buffer fail"));

					//	return false;
					//}

					return true;
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				VknCommandBuffer::VknCommandBuffer(void) noexcept {
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				VknCommandBuffer::~VknCommandBuffer(void) noexcept {
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				void VknCommandBuffer::create(const VknCommandBufferCreateInfo& create_info) noexcept {

					// ~~~~~~~~~~~~~~~~
					// отладочный код
					// проверяем VknCommandBufferCreateInfo на валидность
					// ~~~~~~~~~~~~~~~~

					//if constexpr (Private::Build::type_build == Private::Build::TypeBuild::DEBUG) {

						if (create_info.isValid() == false) {

							//if (create_info.log != nullptr) Tools::addEntry(create_info.log, Tools::LogEntryError("VknCommandBufferCreateInfo fail"));

							//m_valid = false;

							return;
						}

					//} // if constexpr (Private::Build::type_build == Private::Build::TypeBuild::DEBUG)

					m_create_info = create_info;

					// ~~~~~~~~~~~~~~~~
					// создаём VkCommandBuffer
					// ~~~~~~~~~~~~~~~~

					if (createVkCommandBuffer() == false) {

						//Tools::addEntry(create_info.log, Tools::LogEntryError("createVkCommandBuffer fail"));

						return;
					}

					//m_valid = true;

				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				void VknCommandBuffer::destroy(void) noexcept {

					// ~~~~~~~~~~~~~~~~
					// очистка данных
					// ~~~~~~~~~~~~~~~~

					m_create_info = {};
					me_vk_command_buffer = VK_NULL_HANDLE;

					//m_valid = false;
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				bool VknCommandBuffer::createVkCommandBuffer(void) noexcept {

					// ~~~~~~~~~~~~~~~~
					// VkCommandBuffer создаётся внешне, а именно в VknCommandPool 
					// и затем передаётся в VknCommandBuffer через VknCommandBufferCreateInfo
					// поэтому тут он просто сохраняется в me_vk_command_buffer
					// ~~~~~~~~~~~~~~~~

					me_vk_command_buffer = m_create_info.vk_command_buffer;

					return true;
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			} // namespace wvk

		//} // namespace Private

	//} // namespace GPU

} // namespace CGDev