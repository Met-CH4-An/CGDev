////////////////////////////////////////////////////////////////
// секция форвард-декларации
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// секция заголовочного файла
////////////////////////////////////////////////////////////////
#include "wvk_logical_device.h"
////////////////////////////////////////////////////////////////
// секция имплементации
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// секция для остального
////////////////////////////////////////////////////////////////
#include "wvk_commands.h"
#include "wvk_loader.h"
#include "wvk_instance.h"
#include "wvk_physical_device.h"
#include "wvk_queue_family.h"

namespace CGDev {

	//namespace GPU {

		//namespace Private {

			namespace wvk {

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				VknLogicalDevice::VknLogicalDevice(void) noexcept {
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				VknLogicalDevice::~VknLogicalDevice(void) noexcept {
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				bool VknLogicalDevice::create(const VknLogicalDeviceCreateInfo& create_info) noexcept {
					WvkStatus _status;

					m_create_info = create_info;

					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					// Проверка валидности структуры CreateInfo
					// Производится только если включена сборка с валидацией:
					// wvk::Build::ValidationBuildInfo::enable == true
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					if constexpr (Build::s_wvk_debug == true) {
						_status = validationCreateInfo(create_info);
						if (_status.m_code != VknStatusCode::SUCCESSFUL) {
							_status.append("\n\tVknLogicalDevice::validationCreateInfo() - fail.");
							return false;
						}
					}

					// ~~~~~~~~~~~~~~~~
					// подготавливаем VkDevice
					// ~~~~~~~~~~~~~~~~

					_status = createVkDevice();

					if (_status.m_code != VknStatusCode::SUCCESSFUL) {
						_status.m_code = VknStatusCode::FAIL;
						_status.append("\n\tVknLogicalDevice::createVkDevice - fail");

						return false;
					}

					return true;

				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				void VknLogicalDevice::destroy(void) noexcept {

					// ~~~~~~~~~~~~~~~~
					// 
					// ~~~~~~~~~~~~~~~~

					m_create_info.wvk_commands->vknDestroyDevice(m_vk_device, VK_NULL_HANDLE);

					// ~~~~~~~~~~~~~~~~
					// очистка данных
					// ~~~~~~~~~~~~~~~~

					m_create_info				= {};
					m_vk_device					= VK_NULL_HANDLE;
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				WvkStatus VknLogicalDevice::validationCreateInfo(const VknLogicalDeviceCreateInfo& create_info) const noexcept {
					WvkStatus _status;

					if (create_info.wvk_logical_device_queue_create_info_collection.empty() == true) {
						_status.m_code = VknStatusCode::FAIL;
						_status.append("\n\tVknLogicalDeviceCreateInfo::wvk_logical_device_queue_create_info_collection - empty.");
						return _status;
					}

					else {

						for (size_t i = 0; i < create_info.wvk_logical_device_queue_create_info_collection.size(); ++i) {
							const auto& it_logical_device_queue_create_info = create_info.wvk_logical_device_queue_create_info_collection[i];

							if (it_logical_device_queue_create_info.wvk_queue_family_ptr == nullptr) {
								_status.m_code = VknStatusCode::FAIL;
								_status.append("\n\tVknLogicalDeviceCreateInfo::wvk_queue_family_ptr - nullptr.");
								return _status;
							}
							if (it_logical_device_queue_create_info.queue_count.has_value() == false) {
								_status.m_code = VknStatusCode::FAIL;
								_status.append("\n\tVknLogicalDeviceCreateInfo::wvk_logical_device_queue_create_info_collection::queue_count - not value.");
								return _status;
							}
							if (it_logical_device_queue_create_info.priority_collection.empty() == true) {
								_status.m_code = VknStatusCode::FAIL;
								_status.append("\n\tVknLogicalDeviceCreateInfo::wvk_logical_device_queue_create_info_collection::priority_collection - empty.");
								return _status;
							}
						}
					}

					if (create_info.wvk_commands == nullptr) {
						_status.m_code = VknStatusCode::FAIL;
						_status.append("\n\tVknLogicalDeviceCreateInfo::wvk_commands - nullptr.");
						return _status;
					}
					
					if (create_info.wvk_physical_device_collection.empty() == true) {
						_status.m_code = VknStatusCode::FAIL;
						_status.append("\n\tVknLogicalDeviceCreateInfo::wvk_physical_device_collection - empty.");
						return _status;
					}
					
					_status.m_code = VknStatusCode::SUCCESSFUL;
					return _status;
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				void VknLogicalDevice::prepareVkQueueCreateInfo(VkDeviceQueueCreateInfoArr1& queue_create_info_collection1) const noexcept {
					// Перебираем каждую пользовательскую конфигурацию очереди
					for (const auto& it_0 : m_create_info.wvk_logical_device_queue_create_info_collection) {

						// Создаём и заполняем описание очереди
						VkDeviceQueueCreateInfo vk_queue_create_info{};
						vk_queue_create_info.sType = VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO;
						vk_queue_create_info.pNext = nullptr;
						vk_queue_create_info.flags = 0; // зарезервировано, всегда 0
						vk_queue_create_info.queueFamilyIndex = it_0.wvk_queue_family_ptr->getIndexFamily();		// индекс семейства очередей
						vk_queue_create_info.queueCount = it_0.queue_count.value();			// количество создаваемых очередей
						vk_queue_create_info.pQueuePriorities = it_0.priority_collection.data();	// приоритеты очередей (от 0.0f до 1.0f)

						// Добавляем в выходную коллекцию
						queue_create_info_collection1.emplace_back(std::move(vk_queue_create_info));
					}
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				WvkStatus VknLogicalDevice::createVkDevice(void) noexcept {
					WvkStatus _status;

					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					// 1. Подготавливаем коллекцию описаний очередей (VkDeviceQueueCreateInfo)
					//    Это обязательный шаг перед созданием логического устройства
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					VkDeviceQueueCreateInfoArr1 _vk_device_queue_create_info_collection1 = {};
					prepareVkQueueCreateInfo(_vk_device_queue_create_info_collection1);

					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					// 2. Заполняем структуру VkDeviceCreateInfo
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					VkDeviceCreateInfo _vk_device_create_info = {};
					_vk_device_create_info.sType = VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO;
					_vk_device_create_info.pNext = nullptr;
					_vk_device_create_info.flags = 0; // можно добавить флаги при необходимости
					_vk_device_create_info.queueCreateInfoCount = static_cast<uint32_t>(_vk_device_queue_create_info_collection1.size());
					_vk_device_create_info.pQueueCreateInfos = _vk_device_queue_create_info_collection1.data();
					_vk_device_create_info.enabledLayerCount = 0; // слои не используются напрямую
					_vk_device_create_info.ppEnabledLayerNames = nullptr;
					_vk_device_create_info.enabledExtensionCount = 0; // расширения пока не подключены
					_vk_device_create_info.ppEnabledExtensionNames = nullptr;
					_vk_device_create_info.pEnabledFeatures = nullptr; // можно добавить при необходимости

					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					// 3. Вызываем пользовательскую обёртку vknCreateDevice, чтобы создать логическое устройство
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*auto _vk_result = m_create_info.wvk_commands->vknCreateDevice(
						m_create_info.wvk_physical_device_collection[0]->getVkPhysicalDevice(),
						&_vk_device_create_info,
						VK_NULL_HANDLE,
						&m_vk_device
					);

					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					// 4. Обработка ошибок: если создание устройства не удалось
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					if (_vk_result != VK_SUCCESS) {
						switch (_vk_result) {
						case VK_ERROR_OUT_OF_HOST_MEMORY:
							_status.append("\n\tvknCreateDevice - VK_ERROR_OUT_OF_HOST_MEMORY");
							break;
						case VK_ERROR_OUT_OF_DEVICE_MEMORY:
							_status.append("\n\tvknCreateDevice - VK_ERROR_OUT_OF_DEVICE_MEMORY");
							break;
						case VK_ERROR_INITIALIZATION_FAILED:
							_status.append("\n\tvknCreateDevice - VK_ERROR_INITIALIZATION_FAILED");
							break;
						case VK_ERROR_EXTENSION_NOT_PRESENT:
							_status.append("\n\tvknCreateDevice - VK_ERROR_EXTENSION_NOT_PRESENT");
							break;
						case VK_ERROR_FEATURE_NOT_PRESENT:
							_status.append("\n\tvknCreateDevice - VK_ERROR_FEATURE_NOT_PRESENT");
							break;
						case VK_ERROR_TOO_MANY_OBJECTS:
							_status.append("\n\tvknCreateDevice - VK_ERROR_TOO_MANY_OBJECTS");
							break;
						case VK_ERROR_DEVICE_LOST:
							_status.append("\n\tvknCreateDevice - VK_ERROR_DEVICE_LOST");
							break;
						default:
							_status.append("\n\tvknCreateDevice - Unknown error");
							break;
						}
						_status.m_code = VknStatusCode::FAIL;
						return _status;
					}*/

					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					// 5. Если всё прошло успешно, возвращаем SUCCESS
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					_status.m_code = VknStatusCode::SUCCESSFUL;
					return _status;
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			} // namespace wvk

		//} // namespace Private

	//} // namespace GPU

} // namespace CGDev