////////////////////////////////////////////////////////////////
// секция форвард-декларации
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// секция заголовочного файла
////////////////////////////////////////////////////////////////
#include "gpu_vkn_library.h"
////////////////////////////////////////////////////////////////
// секция имплементации
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// секция для остального
////////////////////////////////////////////////////////////////
#include "../gpu_library.h"

#include "vkn_loader.h"
#include "vkn_commands.h"
#include "vkn_instance.h"
#include "vkn_physical_device.h"

#include "Extensions/vkn_ext_debug_utils_commands.h"
#include "Extensions/vkn_ext_debug_utils.h"

namespace CGDev {

	namespace GPU {

		namespace Private {

			namespace Vulkan {

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				GpuVknLibrary::GpuVknLibrary(void) noexcept {
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				GpuVknLibrary::~GpuVknLibrary(void) noexcept {
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				bool GpuVknLibrary::create(const GpuVknLibraryCreateInfo& create_info) noexcept {

					// ~~~~~~~~~~~~~~~~
					// СБОРКА
					// Build::s_vkn_debug
					// НАЧАЛО
					// ~~~~~~~~~~~~~~~~
					// проверяем валидность GpuVknLibraryCreateInfo
					// ~~~~~~~~~~~~~~~~

					if constexpr (Build::s_vkn_debug == true) {

						if (validationCreateInfo(create_info) == false) return false;

					}

					// ~~~~~~~~~~~~~~~~
					// СБОРКА
					// Build::s_vkn_debug
					// КОНЕЦ
					// ~~~~~~~~~~~~~~~~

					// ~~~~~~~~~~~~~~~~
					// сохраняем GpuVknLibraryCreateInfo
					// ~~~~~~~~~~~~~~~~

					m_create_info = create_info;

					// ~~~~~~~~~~~~~~~~
					// создаём VknLoader
					// ~~~~~~~~~~~~~~~~

					if (createVknLoader() == false) {

						Tools::addEntry(m_create_info.log, Tools::LogEntryError("createVknLoader() == false"));

						return false;
					}

					// ~~~~~~~~~~~~~~~~
					// создаём VknGlobalCommands 'глобального уровня'
					// ~~~~~~~~~~~~~~~~

					if (createVknCommandsGlobalLevel() == false) {

						Tools::addEntry(m_create_info.log, Tools::LogEntryError("createVknCommandsGlobalLevel() = StatusCode::FAIL"));

						return false;
					}
					
					// ~~~~~~~~~~~~~~~~
					// создаём VknInstance
					// ~~~~~~~~~~~~~~~~

					Status _status;

					_status = createVknInstance();

					//if (_status.m_code != StatusCode::SUCCESSFUL) {
					//	_status.append("createVknInstance() = false");
					//	return false;
					//}

					// ~~~~~~~~~~~~~~~~
					// создаём VknGlobalCommands 'уровня VkInstance'
					// ~~~~~~~~~~~~~~~~

					if (createVknCommandsInstanceLevel() == false) {

						Tools::addEntry(m_create_info.log, Tools::LogEntryError("createVknCommandsInstanceLevel() = false"));

						return false;
					}
						
					// ~~~~~~~~~~~~~~~~
					// СБОРКА
					// Build::s_vk_ext_debug_utils
					// НАЧАЛО
					// ~~~~~~~~~~~~~~~~
					// метод createVknExtDebugUtils предназначен для создания расширения 'VK_EXT_debug_utils'
					// соотвественно, метод должен вызываться только в сборке, с использованием расширения VK_EXT_debug_utils
					// ~~~~~~~~~~~~~~~~

					if constexpr (Build::s_vk_ext_debug_utils == true) {

						// ~~~~~~~~~~~~~~~~
						// пытаемся создать VknExtDebugUtils
						// ~~~~~~~~~~~~~~~~

						if (createVknExtDebugUtils() == false) {

							Tools::addEntry(m_create_info.log, Tools::LogEntryError("createVknExtDebugUtils() = false"));

							return false;
						}

					}

					// ~~~~~~~~~~~~~~~~
					// СБОРКА
					// Build::s_vk_ext_debug_utils
					// КОНЕЦ
					// ~~~~~~~~~~~~~~~~
					
					// ~~~~~~~~~~~~~~~~
					// создаём VknPhysicalDevice
					// ~~~~~~~~~~~~~~~~

					if (prepareVknPhysicalDevice() == false) {

						Tools::addEntry(m_create_info.log, Tools::LogEntryError("prepareVknPhysicalDevice() = false"));

						return false;
					}

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

						if (checkValidation() == false) {

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

				void GpuVknLibrary::destroy(void) noexcept {

					// ~~~~~~~~~~~~~~~~
					// 
					// ~~~~~~~~~~~~~~~~

					s_vkn_loader->destroy();

					delete s_vkn_loader;

					// ~~~~~~~~~~~~~~~~
					// очистка данных
					// ~~~~~~~~~~~~~~~~

					m_create_info			= {};
					s_vkn_loader			= nullptr;
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				bool GpuVknLibrary::checkValidation(void) noexcept {

					// ~~~~~~~~~~~~~~~~
					// с помощью Extensions::VknExtDebugUtilsPtr мы запрашиваем статус валидации через метод requestLastValidation
					// ~~~~~~~~~~~~~~~~

					// ~~~~~~~~~~~~~~~~
					// СБОРКА
					// Build::s_vk_ext_debug_utils
					// НАЧАЛО
					// ~~~~~~~~~~~~~~~~
					// 
					// ~~~~~~~~~~~~~~~~

					//if constexpr (Build::s_vk_ext_debug_utils == true) {

					//	bool _validation = false;

						//if (static_cast<Extensions::VknExtDebugUtilsPtr>(m_vkn_ext_debug_utils)->requestLastValidation(_validation) == false) {

						//	Tools::addEntry(m_create_info.log, Tools::LogEntryError("GpuVknLibrary::VknExtensionPtr::requestLastValidation = false"));

						//	return false;
						//}

					//	return true;
					//}

					// ~~~~~~~~~~~~~~~~
					// СБОРКА
					// Build::s_vk_ext_debug_utils
					// КОНЕЦ
					// ~~~~~~~~~~~~~~~~

					return true;
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				bool GpuVknLibrary::validationCreateInfo(const GpuVknLibraryCreateInfo& create_info) const noexcept {

					if (create_info.log == nullptr) {

						return false;
					}

					if (create_info.gpu_library == nullptr) {

						Tools::addEntry(create_info.log, Tools::LogEntryError("GpuVknLibraryCreateInfo::gpu_library = nullptr"));

						return false;
					}

					return true;
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				bool GpuVknLibrary::createVknLoader(void) noexcept {

					// ~~~~~~~~~~~~~~~~
					// создаём VknLoader, с помощью которого можно будет получить VknGlobalCommands глобального уровня
					// ~~~~~~~~~~~~~~~~
					
					// ~~~~~~~~~~~~~~~~
					// заполняем VknLoaderCreateInfo
					// ~~~~~~~~~~~~~~~~

					VknLoaderCreateInfo _vkn_loader_create_info			= {};
					_vkn_loader_create_info.log							= m_create_info.log;

					// ~~~~~~~~~~~~~~~~
					// создаём VknLoader
					// ~~~~~~~~~~~~~~~~

					auto _vkn_loader = new VknLoader();

					if (_vkn_loader->create(&_vkn_loader_create_info) == StatusCode::FAIL) {

						Tools::addEntry(m_create_info.log, Tools::LogEntryError("VknLoader->create() = false"));

						return false;
					}

					s_vkn_loader = std::move(_vkn_loader);

					return true;
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				Status GpuVknLibrary::createVknInstance(void) noexcept {

					Status							_status;

					// ~~~~~~~~~~~~~~~~
					// заполняем VknInstanceCreateInfo
					// ~~~~~~~~~~~~~~~~

					VknInstanceCreateInfo _vkn_instance_create_info					= {};
					_vkn_instance_create_info.log									= m_create_info.log;
					_vkn_instance_create_info.vkn_commands							= s_vkn_commands_global_level;
					_vkn_instance_create_info.application_name						= m_create_info.gpu_library->getCreateInfo().application_name;
					_vkn_instance_create_info.engine_name							= Private::Build::engine_name;
					_vkn_instance_create_info.layer_name_collection					= {};
					_vkn_instance_create_info.extension_name_collection				= {};

					// ~~~~~~~~~~~~~~~~
					// создаём VknInstance
					// ~~~~~~~~~~~~~~~~

					auto _vkn_instance = new VknInstance();

					//if (_vkn_instance->create(&_vkn_instance_create_info) != StatusCode::SUCCESSFUL) {

					//	Tools::addEntry(m_create_info.log, Tools::LogEntryError("VknInstance->create() = false"));

					//	return _status;
					//}

					m_vkn_instance = std::move(_vkn_instance);

					return _status;
				}
				
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				bool GpuVknLibrary::prepareVknPhysicalDevice(void) noexcept {

					// ~~~~~~~~~~~~~~~~
					// в вулкане VkPhysicalDevice не создаётся напрямую
					// VkPhysicalDevice получается перечислением, после создания VkInstance
					// поэтому для создания VknPhysicalDevice, сперва мы получим список VkPhysicalDevice
					// ~~~~~~~~~~~~~~~~
					
					// ~~~~~~~~~~~~~~~~
					// формируем двухмерный массив, имеющий вид:
					//			X					X					X
					//		Y	[VkPhysicalDevice]	[VkPhysicalDevice]	[VkPhysicalDevice]
					//		Y	[VkPhysicalDevice]	[VkPhysicalDevice]	[VkPhysicalDevice]
					//		Y	[VkPhysicalDevice]	[VkPhysicalDevice]	[VkPhysicalDevice]
					// ~~~~~~~~~~~~~~~~

					VkPhysicalDeviceArr2 _vk_physical_device_collection2;

					prepareNextForDeviceGroupDeviceCreateInfo(_vk_physical_device_collection2);
					
					// ~~~~~~~~~~~~~~~~
					// перебираем вышесозданный двухмерный массив
					// ~~~~~~~~~~~~~~~~			
				
					// ~~~~~~~~~~~~~~~~
					// перебираем VkPhysicalDeviceArr2
					// ~~~~~~~~~~~~~~~~

					for (size_t ct_vk_physical_device_arr1 = 0; ct_vk_physical_device_arr1 < _vk_physical_device_collection2.size(); ++ct_vk_physical_device_arr1) {
						const auto& it_vk_physical_device_arr1 = _vk_physical_device_collection2[ct_vk_physical_device_arr1];

						VknPhysicalDevicePtrArr1 _vkn_physical_device_collection1;
						
						// ~~~~~~~~~~~~~~~~
						// перебираем VkPhysicalDeviceArr1
						// ~~~~~~~~~~~~~~~~

						for (size_t ct_vk_physical_device = 0; ct_vk_physical_device < it_vk_physical_device_arr1.size(); ++ct_vk_physical_device) {
							const auto& it_vk_physical_device = it_vk_physical_device_arr1[ct_vk_physical_device];

							// ~~~~~~~~~~~~~~~~
							// подготавливаем VknPhysicalDevice
							// ~~~~~~~~~~~~~~~~

							VknPhysicalDevicePtr _vkn_physical_device = nullptr;

							// ~~~~~~~~~~~~~~~~
							// описываем и создаём VknPhysicalDevice
							// ~~~~~~~~~~~~~~~~

							// ~~~~~~~~~~~~~~~~
							// заполняем VknPhysicalDeviceCreateInfo
							// ~~~~~~~~~~~~~~~~

							VknPhysicalDeviceCreateInfo _vkn_physical_device_create_info		= {};
							_vkn_physical_device_create_info.vkn_commands						= s_vkn_commands_instance_level;
							_vkn_physical_device_create_info.vkn_instance						= m_vkn_instance;
							_vkn_physical_device_create_info.vk_physical_device					= it_vk_physical_device;

							// ~~~~~~~~~~~~~~~~
							// создаём VknPhysicalDevice
							// ~~~~~~~~~~~~~~~~

							_vkn_physical_device = new VknPhysicalDevice();

							//if (_vkn_physical_device->create(_vkn_physical_device_create_info) == false) {

							//	Tools::addEntry(m_create_info.log, Tools::LogEntryError("VknPhysicalDevice->create() = false"));

							//	delete _vkn_physical_device;

							//}

							// ~~~~~~~~~~~~~~~~
							// если успешно создался
							// сохраняем VknPhysicalDevice 
							// ~~~~~~~~~~~~~~~~

							//else {

							//	_vkn_physical_device_collection1.emplace_back(std::move(_vkn_physical_device));
							//}							

						} // for (size_t ct_vk_physical_device = 0;						

						// ~~~~~~~~~~~~~~~~
						// если не пустой
						// сохраняем VknPhysicalDevice 
						// ~~~~~~~~~~~~~~~~

						if (_vkn_physical_device_collection1.empty() == false) m_vkn_physical_device_collection2.emplace_back(std::move(_vkn_physical_device_collection1));

					} // for (size_t ct_vk_physical_device_arr1 = 0;

					// ~~~~~~~~~~~~~~~~
					// сохраняем созданные VknPhysicalDevice в другой список,
					// в котором хранятся доступные для использования VknPhysicalDevice
					// ~~~~~~~~~~~~~~~~

					m_vkn_physical_device_free_collection2 = m_vkn_physical_device_collection2;

					return true;
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				bool GpuVknLibrary::prepareNextForDeviceGroupDeviceCreateInfo(VkPhysicalDeviceArr2& vk_physical_device_collection2) noexcept {

					// ~~~~~~~~~~~~~~~~
					// в версии вулкана 1.0 нет поддержки совместимых груп физических устройст
					// поэтому VkPhysicalDevice получаем с помощью комманды вулкана vkEnumeratePhysicalDevices
					// которая возвращает простой список (массив) VkPhysicalDevice
					// ~~~~~~~~~~~~~~~~

					// ~~~~~~~~~~~~~~~~
					// СБОРКА
					// Build::vulkan_version
					// 1.0
					// НАЧАЛО
					// ~~~~~~~~~~~~~~~~

					if constexpr (Build::vulkan_version == Build::VulkanVersion::VERSION_10) {

						// ~~~~~~~~~~~~~~~~
						// получаем количество VkPhysicalDevice
						// ~~~~~~~~~~~~~~~~

						uint32_t _count = 0;

						auto _vk_result = s_vkn_commands_instance_level->vknEnumeratePhysicalDevices(&_count, VK_NULL_HANDLE);

						if (_vk_result != VK_SUCCESS) {

							if (_vk_result == VK_ERROR_OUT_OF_HOST_MEMORY) Tools::addEntry(m_create_info.log, Tools::LogEntryError("VK_ERROR_OUT_OF_HOST_MEMORY"));
							if (_vk_result == VK_ERROR_OUT_OF_DEVICE_MEMORY) Tools::addEntry(m_create_info.log, Tools::LogEntryError("VK_ERROR_OUT_OF_DEVICE_MEMORY"));
							if (_vk_result == VK_ERROR_INITIALIZATION_FAILED) Tools::addEntry(m_create_info.log, Tools::LogEntryError("VK_ERROR_INITIALIZATION_FAILED"));

							return false;
						}

						// ~~~~~~~~~~~~~~~~
						// получаем список VkPhysicalDevice
						// ~~~~~~~~~~~~~~~~

						VkPhysicalDeviceArr1 _vk_physical_device_collection1(_count);

						_vk_result = s_vkn_commands_instance_level->vknEnumeratePhysicalDevices(&_count, _vk_physical_device_collection1.data());

						if (_vk_result != VK_SUCCESS && _vk_result != VK_INCOMPLETE) {

							if (_vk_result == VK_ERROR_OUT_OF_HOST_MEMORY) Tools::addEntry(m_create_info.log, Tools::LogEntryError("VK_ERROR_OUT_OF_HOST_MEMORY"));
							if (_vk_result == VK_ERROR_OUT_OF_DEVICE_MEMORY) Tools::addEntry(m_create_info.log, Tools::LogEntryError("VK_ERROR_OUT_OF_DEVICE_MEMORY"));
							if (_vk_result == VK_ERROR_INITIALIZATION_FAILED) Tools::addEntry(m_create_info.log, Tools::LogEntryError("VK_ERROR_INITIALIZATION_FAILED"));

							return false;
						}

						vk_physical_device_collection2.emplace_back(std::move(_vk_physical_device_collection1));
					}

					// ~~~~~~~~~~~~~~~~
					// СБОРКА
					// Build::vulkan_version
					// 1.0
					// КОНЕЦ
					// ~~~~~~~~~~~~~~~~

					// ~~~~~~~~~~~~~~~~
					// начиная с 1.1 и выше, вулкан предоставляет информацию о совместимых группах VkPhysicalDevice
					// поэтому с помощью комманды вулкана vkEnumeratePhysicalDeviceGroups, получаем VkPhysicalDeviceGroupProperties
					// VkPhysicalDeviceGroupProperties содержит в том числе информацию об VkPhysicalDevice и их количестве в группе
					// ~~~~~~~~~~~~~~~~

					// ~~~~~~~~~~~~~~~~
					// СБОРКА
					// Build::vulkan_version
					// 1.1+
					// НАЧАЛО
					// ~~~~~~~~~~~~~~~~

					if constexpr (Build::vulkan_version >= Build::VulkanVersion::VERSION_11) {

						// ~~~~~~~~~~~~~~~~
						// получаем количество VkPhysicalDeviceGroupProperties
						// ~~~~~~~~~~~~~~~~

						uint32_t _count_group = 0;

						auto _vk_result = s_vkn_commands_instance_level->vknEnumeratePhysicalDeviceGroups(&_count_group, VK_NULL_HANDLE);

						if (_vk_result != VK_SUCCESS) {

							if (_vk_result == VK_ERROR_OUT_OF_HOST_MEMORY) Tools::addEntry(m_create_info.log, Tools::LogEntryError("VK_ERROR_OUT_OF_HOST_MEMORY"));
							if (_vk_result == VK_ERROR_OUT_OF_DEVICE_MEMORY) Tools::addEntry(m_create_info.log, Tools::LogEntryError("VK_ERROR_OUT_OF_DEVICE_MEMORY"));
							if (_vk_result == VK_ERROR_INITIALIZATION_FAILED) Tools::addEntry(m_create_info.log, Tools::LogEntryError("VK_ERROR_INITIALIZATION_FAILED"));

							return false;
						}

						// ~~~~~~~~~~~~~~~~
						// получаем список VkPhysicalDeviceGroupProperties
						// ~~~~~~~~~~~~~~~~

						VkPhysicalDeviceGroupPropertiesArr _vk_physical_device_group_properties_collection(_count_group);

						_vk_result = s_vkn_commands_instance_level->vknEnumeratePhysicalDeviceGroups(&_count_group, _vk_physical_device_group_properties_collection.data());

						if (_vk_result != VK_SUCCESS && _vk_result != VK_INCOMPLETE) {

							if (_vk_result == VK_ERROR_OUT_OF_HOST_MEMORY) Tools::addEntry(m_create_info.log, Tools::LogEntryError("VK_ERROR_OUT_OF_HOST_MEMORY"));
							if (_vk_result == VK_ERROR_OUT_OF_DEVICE_MEMORY) Tools::addEntry(m_create_info.log, Tools::LogEntryError("VK_ERROR_OUT_OF_DEVICE_MEMORY"));
							if (_vk_result == VK_ERROR_INITIALIZATION_FAILED) Tools::addEntry(m_create_info.log, Tools::LogEntryError("VK_ERROR_INITIALIZATION_FAILED"));

							return false;
						}

						// ~~~~~~~~~~~~~~~~
						// перебираем двухмерный массив, имеющий вид:
						//			X														X														X
						//		Y	[VkPhysicalDeviceGroupProperties::VkPhysicalDevice[0]]	[VkPhysicalDeviceGroupProperties::VkPhysicalDevice[0]]	[VkPhysicalDeviceGroupProperties::VkPhysicalDevice[0]]
						//		Y	[VkPhysicalDeviceGroupProperties::VkPhysicalDevice[1]]	[VkPhysicalDeviceGroupProperties::VkPhysicalDevice[1]]	[VkPhysicalDeviceGroupProperties::VkPhysicalDevice[1]]
						//		Y	[VkPhysicalDeviceGroupProperties::VkPhysicalDevice[2]]	[VkPhysicalDeviceGroupProperties::VkPhysicalDevice[2]]	[VkPhysicalDeviceGroupProperties::VkPhysicalDevice[2]]
						// ~~~~~~~~~~~~~~~~
						
						// ~~~~~~~~~~~~~~~~
						// перебираем VkPhysicalDeviceGroupPropertiesArr
						// ~~~~~~~~~~~~~~~~

						for (size_t ct_vk_physical_device_group = 0; ct_vk_physical_device_group < _vk_physical_device_group_properties_collection.size(); ++ct_vk_physical_device_group) {
							const auto& it_vk_physical_device_group = _vk_physical_device_group_properties_collection[ct_vk_physical_device_group];

							VkPhysicalDeviceArr1 _vk_physical_device_collection1;

							// ~~~~~~~~~~~~~~~~
							// перебираем VkPhysicalDeviceGroupProperties::VkPhysicalDevice[]
							// ~~~~~~~~~~~~~~~~

							for (size_t ct_vk_physical_device = 0; ct_vk_physical_device < it_vk_physical_device_group.physicalDeviceCount; ++ct_vk_physical_device) {
								const auto& it_vk_physical_device = it_vk_physical_device_group.physicalDevices[ct_vk_physical_device];

								_vk_physical_device_collection1.emplace_back(it_vk_physical_device);

							} // for (size_t ct_vk_physical_device = 0;

							vk_physical_device_collection2.emplace_back(std::move(_vk_physical_device_collection1));

						} // for (size_t ct_vk_physical_device_arr1 = 0;

					}

					// ~~~~~~~~~~~~~~~~
					// СБОРКА
					// Build::vulkan_version
					// 1.1+
					// КОНЕЦ
					// ~~~~~~~~~~~~~~~~

					return true;
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				bool GpuVknLibrary::createVknCommandsGlobalLevel(void) noexcept {

					// ~~~~~~~~~~~~~~~~
					// создаём VknGlobalCommands, который проинициализирует комманды вулкана
					// для которых не требуется ни VkInstance, ни VkDevice
					// если передаётся
					//		VknCommandsCreateInfo::vkn_loader = не nullptr
					//		VknCommandsCreateInfo::vkn_instance = nullptr
					//		VknCommandsCreateInfo::vkn_logical_device = nullptr
					// VknGlobalCommands инициализируется только глобальными функциями, а именно
					//		PFN_vkEnumerateInstanceLayerProperties
					//		PFN_vkEnumerateInstanceExtensionProperties
					//		PFN_vkCreateInstance
					//		PFN_vkEnumerateInstanceVersion
					// ~~~~~~~~~~~~~~~~
					
					// ~~~~~~~~~~~~~~~~
					// заполняем VknCommandsCreateInfo
					// ~~~~~~~~~~~~~~~~

					VknCommandsCreateInfo _vkn_commands_create_info			= {};
					_vkn_commands_create_info.log							= m_create_info.log;
					_vkn_commands_create_info.vkn_loader					= s_vkn_loader;
					_vkn_commands_create_info.vkn_instance					= nullptr;
					_vkn_commands_create_info.vkn_logical_device			= nullptr;

					// ~~~~~~~~~~~~~~~~
					// создаём VknGlobalCommands
					// ~~~~~~~~~~~~~~~~

					auto _vkn_commands = new VknGlobalCommands();

					if (_vkn_commands->create(&_vkn_commands_create_info) == StatusCode::FAIL) {

						Tools::addEntry(m_create_info.log, Tools::LogEntryError("VknGlobalCommands->create() = StatusCode::FAIL"));

						return false;
					}

					s_vkn_commands_global_level = std::move(_vkn_commands);

					return true;
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				bool GpuVknLibrary::createVknCommandsInstanceLevel(void) noexcept {

					// ~~~~~~~~~~~~~~~~
					// создаём VknGlobalCommands, который проинициализирует комманды вулкана
					// для которых требуется VkInstance
					// если передаётся
					//		VknCommandsCreateInfo::vkn_loader = не nullptr
					//		VknCommandsCreateInfo::vkn_instance = не nullptr
					//		VknCommandsCreateInfo::vkn_logical_device = nullptr
					// VknGlobalCommands инициализируется коммандами, уровня VkInstance
					// ~~~~~~~~~~~~~~~~

					// ~~~~~~~~~~~~~~~~
					// заполняем VknCommandsCreateInfo
					// ~~~~~~~~~~~~~~~~

					VknCommandsCreateInfo _vkn_commands_create_info				= {};
					_vkn_commands_create_info.log								= m_create_info.log;
					_vkn_commands_create_info.vkn_loader						= s_vkn_loader;
					_vkn_commands_create_info.vkn_instance						= m_vkn_instance;
					_vkn_commands_create_info.vkn_logical_device				= nullptr;

					// ~~~~~~~~~~~~~~~~
					// аллоцируем VknGlobalCommands
					// ~~~~~~~~~~~~~~~~

					VknCommandsPtr _vkn_commands = nullptr;

					if (allocate(&_vkn_commands) != StatusCode::SUCCESSFUL) {

						Tools::addEntry(m_create_info.log, Tools::LogEntryError("GpuVknLibrary->allocate() != StatusCode::SUCCESSFUL"));

						return false;
					}

					// ~~~~~~~~~~~~~~~~
					// создаём VknGlobalCommands
					// ~~~~~~~~~~~~~~~~

					if (_vkn_commands->create(&_vkn_commands_create_info) != StatusCode::SUCCESSFUL) {

						Tools::addEntry(m_create_info.log, Tools::LogEntryError("VknGlobalCommands->create() != StatusCode::SUCCESSFUL"));

						return false;
					}

					s_vkn_commands_instance_level = std::move(_vkn_commands);

					return true;
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				
				bool GpuVknLibrary::createVknExtDebugUtils(void) noexcept {

					// ~~~~~~~~~~~~~~~~
					// СБОРКА
					// Build::s_vk_ext_debug_utils
					// НАЧАЛО
					// ~~~~~~~~~~~~~~~~
					// данный метод (createVknExtDebugUtils) предназначен для создания расширения 'VK_EXT_debug_utils'
					// соотвественно, метод должен вызываться только в сборке, с использованием расширения VK_EXT_debug_utils
					// ~~~~~~~~~~~~~~~~

					if constexpr (Build::s_vk_ext_debug_utils == true) {

						// ~~~~~~~~~~~~~~~~
						// расширение VK_EXT_debug_utils предлагает дополнительный набор комманд
						// все эти дополнительные комманды представлены в VknExtDebugUtilsCommands
						// без VknExtDebugUtilsCommands, не получится создать VknExtDebugUtils
						// поэтому создаём VknExtDebugUtilsCommands, а затем VknExtDebugUtils
						// ~~~~~~~~~~~~~~~~

						// ~~~~~~~~~~~~~~~~
						// заполняем VknExtDebugUtilsCommandsCreateInfo
						// ~~~~~~~~~~~~~~~~

						Extensions::VknExtDebugUtilsCommandsCreateInfo _vkn_ext_debug_utils_commands_create_info = {};
						
						_vkn_ext_debug_utils_commands_create_info.vkn_commands = s_vkn_commands_global_level;
						

						// ~~~~~~~~~~~~~~~~
						// создаём VknExtDebugUtilsCommands
						// ~~~~~~~~~~~~~~~~

						auto _vkn_ext_debug_utils_commands = new Extensions::VknExtDebugUtilsCommands();

						//if (_vkn_ext_debug_utils_commands->create(_vkn_ext_debug_utils_commands_create_info) == false) {

						//	Tools::addEntry(m_create_info.log, Tools::LogEntryError("VknExtDebugUtilsCommands->create() = false"));

						//	return false;
						//}

						// ~~~~~~~~~~~~~~~~
						// заполняем VknExtDebugUtilsCreateInfo
						// ~~~~~~~~~~~~~~~~

						Extensions::VknExtDebugUtilsCreateInfo _vkn_ext_debug_utils_create_info = {};
						//_vkn_ext_debug_utils_create_info.log = m_create_info.log;
						_vkn_ext_debug_utils_create_info.vkn_debug_utils_commands = _vkn_ext_debug_utils_commands;

						// ~~~~~~~~~~~~~~~~
						// создаём VknExtDebugUtils
						// ~~~~~~~~~~~~~~~~

						auto _vkn_ext_debug_utils = new Extensions::VknExtDebugUtils();

						//if (_vkn_ext_debug_utils->create(_vkn_ext_debug_utils_create_info) == false) {

						//	Tools::addEntry(m_create_info.log, Tools::LogEntryError("VknExtDebugUtils->create() = false"));

						//	return false;
						//}

						m_vkn_ext_debug_utils = std::move(_vkn_ext_debug_utils);
					}

					// ~~~~~~~~~~~~~~~~
					// СБОРКА
					// Build::s_vk_ext_debug_utils
					// КОНЕЦ
					// ~~~~~~~~~~~~~~~~

					return true;
				}
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				VknStatus GpuVknLibrary::createVknQueueFamily(void) noexcept {

					VknStatus _status;

					uint32_t _count = 0;

					// --- Шаг 1: Получаем количество доступных семейств очередей ---
					// Первый вызов функции — просто узнаём, сколько семейств очередей поддерживает физическое устройство.
					//m_create_info.vkn_commands->vkGetPhysicalDeviceQueueFamilyProperties(m_vk_physical_device, &_count, nullptr);

					// --- Шаг 2: Выделяем память под свойства всех семейств очередей ---
					// Создаём вектор нужного размера, в который Vulkan запишет данные о каждом семействе.
					VkQueueFamilyPropertiesArr1 _vk_queue_family_properties_collection(_count);

					// --- Шаг 3: Получаем свойства всех семейств очередей ---
					// Второй вызов той же функции, теперь с выделенным буфером для записи данных.
					//m_create_info.vkn_commands->vkGetPhysicalDeviceQueueFamilyProperties(
					//	m_vk_physical_device,
					//	&_count,
					//	_vk_queue_family_properties_collection.data()
					//);

					// --- Шаг 4: Создаём внутренние объекты VknQueueFamily на основе полученных свойств ---
					// Проходим по каждому семейству, создаём объект и добавляем его в коллекцию.
					for (const auto& it : _vk_queue_family_properties_collection) {

						// Создаём структуру, которая будет передана в метод create()
						//VknQueueFamilyCreateInfo _vkn_queue_family_create_info;
						//_vkn_queue_family_create_info.vk_queue_family_properties = it;

						// Создаём экземпляр VknQueueFamily
						//VknQueueFamily _vkn_queue_family;

						// Вызываем метод create() для инициализации объекта на основе свойств
						//_status = _vkn_queue_family.create(_vkn_queue_family_create_info);

						// Если инициализация не удалась — возвращаем ошибку
						//if (_status.m_code != VknStatusCode::SUCCESSFUL) {
						//	_status.message_old += "\n\tVknQueueFamily::create() - fail.";
						//	return _status;
						//}

						// Добавляем объект в коллекцию (перемещением, без копирования)
						//m_vkn_queue_family_collection.emplace_back(std::move(_vkn_queue_family));
					}

					// --- Шаг 5: Успешное завершение ---
					_status.m_code = VknStatusCode::SUCCESSFUL;
					return _status;
				}
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			} // namespace Vulkan

		} // namespace Private

	} // namespace GPU

} // namespace CGDev