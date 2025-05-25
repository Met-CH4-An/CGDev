////////////////////////////////////////////////////////////////
// секция форвард-декларации
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// секция заголовочного файла
////////////////////////////////////////////////////////////////
#include "gpu_vkn_device.h"
////////////////////////////////////////////////////////////////
// секция имплементации
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// секция для остального
////////////////////////////////////////////////////////////////
#include "../gpu_library.h"
#include "../gpu_device.h"

#include "gpu_vkn_library.h"
#include "vkn_physical_device.h"
#include "vkn_logical_device.h"

namespace CGDev {

	namespace GPU {

		namespace Private {

			namespace Vulkan {

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				GpuVknDevice::GpuVknDevice(void) noexcept {

				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				GpuVknDevice::~GpuVknDevice(void) noexcept {
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				bool GpuVknDevice::create(const GpuVknDeviceCreateInfo& create_info) noexcept {

					// ~~~~~~~~~~~~~~~~
					// сохраняем GpuVknDeviceCreateInfo
					// ~~~~~~~~~~~~~~~~

					m_create_info = create_info;

					prepareVknLogicalDevice();

					// ~~~~~~~~~~~~~~~~
					// создаём LogicalDevice
					// ~~~~~~~~~~~~~~~~

					//createLogicalDevice();

					//if (this->isValid() == false) {

					//	Tools::addEntry(this->getCreateInfo().library->getLog(), Tools::LogEntryError("Не удалось создать LogicalDevice"));

					//	m_valid = false;

					//	return;
					//}

					// ~~~~~~~~~~~~~~~~
					// 
					// ~~~~~~~~~~~~~~~~

					//m_vulkan_command = std::static_pointer_cast<HCPILibraryVulkan>(this->getCreateInfo().library)->getVulkanCommand();

					// ~~~~~~~~~~~~~~~~
					// создаём CommandPool
					// ~~~~~~~~~~~~~~~~
					
					//createCommandPool();

					//if (this->isValid() == false) {

					//	Tools::addEntry(this->getCreateInfo().library->getLog(), Tools::LogEntryError("Не удалось создать CommandPool"));

					//	m_valid = false;

					//	return;
					//}

					// ~~~~~~~~~~~~~~~~
					// создаём RenderMap
					// ~~~~~~~~~~~~~~~~

					//createRenderMap();

					//if (this->isValid() == false) {

					//	Tools::addEntry(this->getCreateInfo().library->getLog(), Tools::LogEntryError("Не удалось создать RenderMap"));

					//	m_valid = false;

						return true;
					//}


					//m_valid = true;
				}

				void GpuVknDevice::destroy(void) noexcept {

					// ~~~~~~~~~~~~~~~~
					// очистка данных
					// ~~~~~~~~~~~~~~~~

					m_create_info = {};
					m_vkn_logical_device = nullptr;
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				bool GpuVknDevice::execute(void) noexcept {

					const auto& _gpu_library = static_cast<GpuLibraryPtr>(m_create_info.gpu_device->getCreateInfo().gpu_library);
					const auto& _gpu_vkn_library = static_cast<GpuVknLibraryPtr>(_gpu_library->getImpl());

					// ~~~~~~~~~~~~~~~~
					// СБОРКА
					// Build::s_vkn_debug == true
					// НАЧАЛО
					// ~~~~~~~~~~~~~~~~
					// в отладочной сборке проверяем валидацию
					// ~~~~~~~~~~~~~~~~

					if constexpr (Build::s_vkn_debug == true) {

						// ~~~~~~~~~~~~~~~~
						// проверка на валидацию
						// ~~~~~~~~~~~~~~~~

						if (_gpu_vkn_library->checkValidation() == false) {

							return false;

						}

					}

					// ~~~~~~~~~~~~~~~~
					// СБОРКА
					// Build::s_vkn_debug == true
					// КОНЕЦ
					// ~~~~~~~~~~~~~~~~

					return true;
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				bool GpuVknDevice::prepareVknLogicalDevice(void) noexcept {

					auto _gpu_library		= static_cast<GpuLibraryPtr>(m_create_info.gpu_device->getCreateInfo().gpu_library);
					auto _gpu_vkn_library	= static_cast<GpuVknLibraryPtr>(_gpu_library->getImpl());

					// ~~~~~~~~~~~~~~~~
					// описываем VknLogicalDeviceQueueCreateInfo
					// ~~~~~~~~~~~~~~~~

					VknLogicalDeviceQueueCreateInfo _vkn_logical_device_queue_create_info		= {};
					//_vkn_logical_device_queue_create_info.queue_family							= 0;
					//_vkn_logical_device_queue_create_info.queue_count							= 1;
					//_vkn_logical_device_queue_create_info.priority_collection					= { 1.0f };

					// ~~~~~~~~~~~~~~~~
					// описываем VknLogicalDeviceQueueCreateInfo
					// ~~~~~~~~~~~~~~~~
					;
					VknLogicalDeviceCreateInfo _vkn_logical_device_create_info						= {};
					//_vkn_logical_device_create_info.log												= _gpu_library->getLog();
					//_vkn_logical_device_create_info.vkn_commands									= _gpu_vkn_library->getVknCommandsInstanceLevel();
					//_vkn_logical_device_create_info.vkn_instance									= _gpu_vkn_library->getVknInstance();
					//_vkn_logical_device_create_info.vkn_physical_device_collection					= _gpu_vkn_library->getVknPhysicalDevice()[0];
					//_vkn_logical_device_create_info.vkn_logical_device_queue_create_info_collection.emplace_back(_vkn_logical_device_queue_create_info);

					// ~~~~~~~~~~~~~~~~
					// создаём VknLogicalDevice
					// ~~~~~~~~~~~~~~~~
					
					m_vkn_logical_device = new VknLogicalDevice();

					if (m_vkn_logical_device->create(_vkn_logical_device_create_info) == false) {

						Tools::addEntry(_gpu_library->getLog(), Tools::LogEntryError("VknLogicalDevice::create() = false"));

						return false;
					}

					return true;
								
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				bool GpuVknDevice::prepareVknQueue(void) noexcept {
					
					// ~~~~~~~~~~~~~~~~
					// 
					// ~~~~~~~~~~~~~~~~

					for (size_t ct_vkn_logical_device_queue_create_info_collection = 0; ct_vkn_logical_device_queue_create_info_collection < m_vkn_logical_device->getCreateInfo().vkn_logical_device_queue_create_info_collection.size(); ++ct_vkn_logical_device_queue_create_info_collection) {
						const auto& it_vkn_logical_device_queue_create_info_collection = m_vkn_logical_device->getCreateInfo().vkn_logical_device_queue_create_info_collection[ct_vkn_logical_device_queue_create_info_collection];

						//it_vkn_logical_device_queue_create_info_collection.

					} // for (size_t ct_vkn_logical_device_queue_create_info_collection = 0;

					return true;

				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				//void DeviceVulkan::createCommandPool(void) noexcept {

					// ~~~~~~~~~~~~~~~~
					// описываем CommandPoolCreateInfo
					// ~~~~~~~~~~~~~~~~

					//CommandPoolCreateInfo _command_pool_create_info			= {};
					//_command_pool_create_info.log							= this->getCreateInfo().library->getLog();
					//_command_pool_create_info.logical_device				= m_logical_device;
			
					// ~~~~~~~~~~~~~~~~
					// создаём LOGICAL_DEVICE_CREATE_INFO
					// ~~~~~~~~~~~~~~~~

					//m_command_pool = std::make_shared<CommandPool>();

					//m_command_pool->create(_command_pool_create_info);

					//if (m_command_pool->isValid() == false) {

					//	Tools::addEntry(this->getCreateInfo().library->getLog(), Tools::LogEntryError("Не удалось создать CommandPool"));

					//	m_valid = false;

					//	return;
					//}

					//m_valid = true;
				//}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				//void DeviceVulkan::createRenderMap(void) noexcept {

					// ~~~~~~~~~~~~~~~~
					// описываем RenderMapCreateInfo
					// ~~~~~~~~~~~~~~~~

					//RenderMapCreateInfo _render_map_create_info						= {};
					//_render_map_create_info.log										= this->getCreateInfo().library->getLog();
					//_render_map_create_info.vulkan_command							= m_vulkan_command;
					//_render_map_create_info.command_pool							= m_command_pool;

					// ~~~~~~~~~~~~~~~~
					// создаём RenderMap
					// ~~~~~~~~~~~~~~~~

					//m_render_map = std::make_shared<RenderMap>();

					//m_render_map->create(_render_map_create_info);

					//if (m_render_map->isValid() == false) {

					//	Tools::addEntry(this->getCreateInfo().library->getLog(), Tools::LogEntryError("Не удалось подготовить RenderMap"));

					//	m_valid = false;

					//	return;
					//}

					//m_valid = true;
				//}

			} // namespace Vulkan

		} // namespace Private

	} // namespace GPU

} // namespace CGDev