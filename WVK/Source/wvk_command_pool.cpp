////////////////////////////////////////////////////////////////
// секция форвард-декларации
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// секция заголовочного файла
////////////////////////////////////////////////////////////////
#include "wvk_command_pool.h"
////////////////////////////////////////////////////////////////
// секция имплементации
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// секция для остального
////////////////////////////////////////////////////////////////
#include "wvk_commands.h"
#include "wvk_loader.h"
#include "wvk_physical_device.h"
#include "wvk_logical_device.h"

namespace CGDev {

	//namespace GPU {

		//namespace Private {

			namespace wvk {

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				bool VknCommandPoolCreateInfo::isValid(void) const noexcept {

					//if (log == nullptr) {

					//	return false;
					//}

					// ~~~~~~~~~~~~~~~~
					// VknCommandPoolCreateInfo::validation
					// ~~~~~~~~~~~~~~~~

					//if (validation == nullptr || validation->isValid() == false) {

					//	CGDev::Tools::addEntry(log, CGDev::Tools::LogEntryError("VknCommandPoolCreateInfo::validation fail"));

					//	return false;
					//}

					// ~~~~~~~~~~~~~~~~
					// VknCommandPoolCreateInfo::commands
					// ~~~~~~~~~~~~~~~~

					//if (commands == nullptr || commands->isValid() == false || commands->getCreateInfo().command_level != VknCommandsLevel::INSTANCE) {

					//	CGDev::Tools::addEntry(log, CGDev::Tools::LogEntryError("VknCommandPoolCreateInfo::commands fail"));

					//	return false;
					//}

					// ~~~~~~~~~~~~~~~~
					// VknCommandPoolCreateInfo::logical_device
					// ~~~~~~~~~~~~~~~~

					//if (logical_device == nullptr || logical_device->/isValid() == false) {

					//	CGDev::Tools::addEntry(log, CGDev::Tools::LogEntryError("VknCommandPoolCreateInfo::logical_device fail"));

					//	return false;
					//}

					// ~~~~~~~~~~~~~~~~
					// VknCommandPoolCreateInfo::iq_queue_family
					// ~~~~~~~~~~~~~~~~

					//if (iq_queue_family == std::numeric_limits<uint32_t>::std::max()) {

						//CGDev::Tools::addEntry(log, CGDev::Tools::LogEntryError("VknCommandPoolCreateInfo::iq_queue_family fail"));

						return false;
					//}

					return true;
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				VknCommandPool::VknCommandPool(void) noexcept {
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				VknCommandPool::~VknCommandPool(void) noexcept {
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				void VknCommandPool::create(const VknCommandPoolCreateInfo& create_info) noexcept {

					// ~~~~~~~~~~~~~~~~
					// отладочный код
					// проверяем VknCommandPoolCreateInfo на валидность
					// ~~~~~~~~~~~~~~~~

					//if constexpr (Private::Build::type_build == Private::Build::TypeBuild::DEBUG) {

						if (create_info.isValid() == false) {

							//if (create_info.log != nullptr) Tools::addEntry(create_info.log, Tools::LogEntryError("VknCommandPoolCreateInfo fail"));

							//m_valid = false;

							return;
						}

					//} // if constexpr (Private::Build::type_build == Private::Build::TypeBuild::DEBUG)

					m_create_info = create_info;

					// ~~~~~~~~~~~~~~~~
					// создаём VkCommandPool
					// ~~~~~~~~~~~~~~~~

					if (createVkCommandPool() == false) {

						//Tools::addEntry(create_info.log, Tools::LogEntryError("createVkCommandPool fail"));

						return;
					}

					//m_valid = true;

				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				void VknCommandPool::destroy(void) noexcept {

					//m_create_info.commands->vkDestroyCommandPool(m_create_info.logical_device->getVkDevice(), m_vk_command_pool, VK_NULL_HANDLE);

					// ~~~~~~~~~~~~~~~~
					// очистка данных
					// ~~~~~~~~~~~~~~~~

					//m_create_info = {};
					//m_vk_command_pool = VK_NULL_HANDLE;

					//m_valid = false;
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				bool VknCommandPool::allocateVkCommandBuffer(VkCommandBufferArr& vk_command_buffer_collection, const VkCommandBufferLevel& vk_command_buffer_level) const noexcept {

					if (vk_command_buffer_collection.empty() == true) {

						return true;
					}

					// ~~~~~~~~~~~~~~~~
					// описываем VkCommandBufferAllocateInfo
					// ~~~~~~~~~~~~~~~~
					
					VkCommandBufferAllocateInfo _vk_command_buffer_allocate_info = {};
					_vk_command_buffer_allocate_info.sType						= VK_STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO;
					_vk_command_buffer_allocate_info.pNext						= VK_NULL_HANDLE;
					_vk_command_buffer_allocate_info.commandPool				= m_vk_command_pool;
					_vk_command_buffer_allocate_info.level						= vk_command_buffer_level;
					_vk_command_buffer_allocate_info.commandBufferCount			= static_cast<uint32_t>(vk_command_buffer_collection.size());

					//auto _vk_result = m_create_info.commands->vkAllocateCommandBuffers(m_create_info.logical_device->getVkDevice(), &_vk_command_buffer_allocate_info, vk_command_buffer_collection.data());

					//if (_vk_result != VK_SUCCESS) {

					//	if (_vk_result == VK_ERROR_OUT_OF_HOST_MEMORY) Tools::addEntry(m_create_info.log, Tools::LogEntryError("VK_ERROR_OUT_OF_HOST_MEMORY"));
					//	if (_vk_result == VK_ERROR_OUT_OF_DEVICE_MEMORY) Tools::addEntry(m_create_info.log, Tools::LogEntryError("VK_ERROR_OUT_OF_DEVICE_MEMORY"));

					//	return false;
					//}

					// ~~~~~~~~~~~~~~~~
					// отладочный код
					// проверка на валидацию
					// ~~~~~~~~~~~~~~~~

					//if constexpr (Private::Build::type_build == Private::Build::TypeBuild::DEBUG) {

						bool _validation = false;

						//m_create_info.validation->requestLastValidation(_validation);

						//if (_validation == false) {

						//	Tools::addEntry(m_create_info.log, Tools::LogEntryError("vkAllocateCommandBuffers validation fail"));

						//	return false;
						//}
					//}

					return true;
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				bool VknCommandPool::freeVkCommandBuffer(VkCommandBufferArr& vk_command_buffer_collection) const noexcept {

					//m_create_info.commands->vkFreeCommandBuffers(m_create_info.logical_device->getVkDevice(), m_vk_command_pool, static_cast<uint32_t>(vk_command_buffer_collection.size()), vk_command_buffer_collection.data());

					return true;
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				bool VknCommandPool::createVkCommandPool(void) noexcept {

					// ~~~~~~~~~~~~~~~~
					// описываем VkCommandPoolCreateInfo
					// ~~~~~~~~~~~~~~~~

					VkCommandPoolCreateInfo _vk_command_pool_create_info			= {};
					_vk_command_pool_create_info.sType								= VK_STRUCTURE_TYPE_COMMAND_POOL_CREATE_INFO;
					_vk_command_pool_create_info.pNext								= VK_NULL_HANDLE;
					_vk_command_pool_create_info.flags;
					_vk_command_pool_create_info.queueFamilyIndex					= m_create_info.iq_queue_family;
					
					// ~~~~~~~~~~~~~~~~
					// создаём VkCommandPool
					// ~~~~~~~~~~~~~~~~

					//auto _vk_result = m_create_info.commands->vkCreateCommandPool(m_create_info.logical_device->getVkDevice(), &_vk_command_pool_create_info, VK_NULL_HANDLE, &m_vk_command_pool);

					//if (_vk_result != VK_SUCCESS) {

					//	if (_vk_result == VK_ERROR_OUT_OF_HOST_MEMORY) Tools::addEntry(m_create_info.log, Tools::LogEntryError("VK_ERROR_OUT_OF_HOST_MEMORY"));
					//	if (_vk_result == VK_ERROR_OUT_OF_DEVICE_MEMORY) Tools::addEntry(m_create_info.log, Tools::LogEntryError("VK_ERROR_OUT_OF_DEVICE_MEMORY"));

					//	return false;
					//}

					// ~~~~~~~~~~~~~~~~
					// отладочный код
					// проверка на валидацию
					// ~~~~~~~~~~~~~~~~

					//if constexpr (Private::Build::type_build == Private::Build::TypeBuild::DEBUG) {

						bool _validation = false;

						//m_create_info.validation->requestLastValidation(_validation);

						//if (_validation == false) {

						//	Tools::addEntry(m_create_info.log, Tools::LogEntryError("vkCreateCommandPool validation fail"));

						//	return false;
						//}
					//}

					return true;
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			} // namespace wvk

		//} // namespace Private

	//} // namespace GPU

} // namespace CGDev