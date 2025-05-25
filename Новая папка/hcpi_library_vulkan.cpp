////////////////////////////////////////////////////////////////
// секция форвард-декларации
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// секция заголовочного файла
////////////////////////////////////////////////////////////////
#include "hcpi_library_vulkan.h"
////////////////////////////////////////////////////////////////
// секция имплементации
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// секция для остального
////////////////////////////////////////////////////////////////
#include "loader.h"
#include "instance.h"
#include "physical_device.h"
#include "logical_device.h"
#include "Layers/vk_layer_khronos_validation.h"
#include "Extensions/vk_ext_debug_report.h"
#include "Extensions/vk_ext_debug_utils.h"

namespace CGDev {

	namespace RPI {

		namespace Private {

			namespace Vulkan {

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				HCPILibraryVulkan::HCPILibraryVulkan(void) noexcept {
					
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				HCPILibraryVulkan::~HCPILibraryVulkan(void) noexcept {
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				void HCPILibraryVulkan::create(const HCPI_LIBRARY_VULKAN_CREATE_INFO& create_info) noexcept {

					// ~~~~~~~~~~~~~~~~
					// проверяем HCPI_LIBRARY_VULKAN_CREATE_INFO на корректность
					// ~~~~~~~~~~~~~~~~

					if (create_info.isValid() == false) {

						if(create_info.library != nullptr) Tools::addEntry(create_info.library->getLog(), Tools::LogEntryError("HCPI_LIBRARY_VULKAN_CREATE_INFO не корректен"));

						return;
					}

					m_create_info = create_info;

					// ~~~~~~~~~~~~~~~~
					// создаём Loader
					// ~~~~~~~~~~~~~~~~

					createLoader();
					
					if (isValid() == false) {

						Tools::addEntry(m_create_info.library->getLog(), Tools::LogEntryError("Не удалось создать Loader"));

						m_valid = false;

						return;
					}

					// ~~~~~~~~~~~~~~~~
					// подготавливаем список поддерживаемых слоёв и расширений
					// ~~~~~~~~~~~~~~~~

					prepareSupportedLayersAndExtensions();

					if (isValid() == false) {

						Tools::addEntry(m_create_info.library->getLog(), Tools::LogEntryError("Не удалось получить список поддерживаемых слоёв и расширений"));

						m_valid = false;

						return;
					}
					
					// ~~~~~~~~~~~~~~~~
					// подготавливаем список активированных слоёв и расширений
					// ~~~~~~~~~~~~~~~~
					
					prepareEnabledLayersAndExtensions();

					if (isValid() == false) {

						Tools::addEntry(m_create_info.library->getLog(), Tools::LogEntryError("Не удалось получить список активированных слоёв и расширений"));

						m_valid = false;

						return;
					}

					//prepareDebug();

					// ~~~~~~~~~~~~~~~~
					// создаём Instance
					// ~~~~~~~~~~~~~~~~

					createInstance();

					if (this->isValid() == false) {

						m_valid = false;

						return;
					}

					// ~~~~~~~~~~~~~~~~
					// создаём PhysicalDevice
					// ~~~~~~~~~~~~~~~~

					createPhysicalDevice();

					if (isValid() == false) {

						m_valid = false;

						return;
					}

					m_valid = true;
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				void HCPILibraryVulkan::createLoader(void) noexcept {

					// ~~~~~~~~~~~~~~~~
					// описываем LOADER_CREATE_INFO
					// ~~~~~~~~~~~~~~~~
					
					LOADER_CREATE_INFO _v_loader_create_info		= {};
					_v_loader_create_info.log						= m_create_info.library->getLog();

					// ~~~~~~~~~~~~~~~~
					// создаём Loader
					// ~~~~~~~~~~~~~~~~

					auto _loader = std::make_shared<Loader>();

					_loader->create(_v_loader_create_info);

					if (_loader->isValid() == false) {

						Tools::addEntry(m_create_info.library->getLog(), Tools::LogEntryError("Не удалось создать Vloader"));

						m_valid = false;

						return;
					}

					m_loader = std::move(_loader);

					// ~~~~~~~~~~~~~~~~
					// запрашиваем комманды вулкана у загрузчика
					// ~~~~~~~~~~~~~~~~

					m_vulkan_command = std::make_shared<VulkanCommand>();

					m_loader->requestVulkanCommand(m_vulkan_command);

					m_valid = true;
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				void HCPILibraryVulkan::createVulkanCommand(void) noexcept {

					m_vulkan_command = std::make_shared<VulkanCommand>();

					m_loader->requestVulkanCommand(m_vulkan_command);
					
					m_valid = true;
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				void HCPILibraryVulkan::prepareSupportedLayersAndExtensions(void) noexcept {

					// ~~~~~~~~~~~~~~~~
					// через vkEnumerateInstanceLayerProperties запрашиваем слои вулкана,
					// доступные в системе
					// ~~~~~~~~~~~~~~~~

					{
						// ~~~~~~~~~~~~~~~~
						// получаем количество VkLayerProperties
						// ~~~~~~~~~~~~~~~~

						uint32_t _count = 0;

						auto _vk_result = m_vulkan_command->vkEnumerateInstanceLayerProperties(&_count, VK_NULL_HANDLE);

						if (_vk_result != VK_SUCCESS) {

							if (_vk_result == VK_ERROR_OUT_OF_HOST_MEMORY) Tools::addEntry(m_create_info.library->getLog(), Tools::LogEntryError("VK_ERROR_OUT_OF_HOST_MEMORY"));
							if (_vk_result == VK_ERROR_OUT_OF_DEVICE_MEMORY) Tools::addEntry(m_create_info.library->getLog(), Tools::LogEntryError("VK_ERROR_OUT_OF_DEVICE_MEMORY"));

							m_valid = false;

							return;
						}

						// ~~~~~~~~~~~~~~~~
						// получаем список VkLayerProperties
						// ~~~~~~~~~~~~~~~~

						VkLayerPropertiesArr _vk_layer_properties_collection = {};
						_vk_layer_properties_collection.resize(_count);						

						_vk_result = m_vulkan_command->vkEnumerateInstanceLayerProperties(&_count, _vk_layer_properties_collection.data());

						if (_vk_result != VK_SUCCESS &&
							_vk_result != VK_INCOMPLETE) {

							if (_vk_result == VK_ERROR_OUT_OF_HOST_MEMORY) Tools::addEntry(m_create_info.library->getLog(), Tools::LogEntryError("VK_ERROR_OUT_OF_HOST_MEMORY"));
							if (_vk_result == VK_ERROR_OUT_OF_DEVICE_MEMORY) Tools::addEntry(m_create_info.library->getLog(), Tools::LogEntryError("VK_ERROR_OUT_OF_DEVICE_MEMORY"));

							m_valid = false;

							return;
						}

						// ~~~~~~~~~~~~~~~~
						// перебираем _vk_layer_properties_collection и _layer_supported_by_library
						// ищем совпадения
						// ~~~~~~~~~~~~~~~~

						LayerSptrArr _layers_supported_by_library = {
							std::make_shared<Layers::VkLayerKhronosValidation>()
						};

						for (size_t ct_0 = 0; ct_0 < _vk_layer_properties_collection.size(); ++ct_0) {
							const auto& it_vk_layer_properties = _vk_layer_properties_collection[ct_0];

							for (size_t ct_1 = 0; ct_1 < _layers_supported_by_library.size(); ++ct_1) {
								const auto& it_layer_supported_by_library = _layers_supported_by_library[ct_1];

								if (it_vk_layer_properties.layerName == it_layer_supported_by_library->getName()) {

									m_layer_supported_collection.emplace_back(it_layer_supported_by_library);

									ct_1 = _layers_supported_by_library.size();
								}

							} // for (size_t ct_1 = 0;

						} // for (size_t ct_0 = 0;

					}

					// ~~~~~~~~~~~~~~~~
					// через vkEnumerateInstanceExtensionProperties запрашиваем расширения вулкана,
					// доступные в системе
					// ~~~~~~~~~~~~~~~~

					{
						// ~~~~~~~~~~~~~~~~
						// получаем количество VkExtensionProperties
						// ~~~~~~~~~~~~~~~~
						
						uint32_t _count = 0;

						auto _vk_result = m_vulkan_command->vkEnumerateInstanceExtensionProperties(VK_NULL_HANDLE, &_count, VK_NULL_HANDLE);

						if (_vk_result != VK_SUCCESS) {

							if (_vk_result == VK_ERROR_OUT_OF_HOST_MEMORY) Tools::addEntry(m_create_info.library->getLog(), Tools::LogEntryError("VK_ERROR_OUT_OF_HOST_MEMORY"));
							if (_vk_result == VK_ERROR_OUT_OF_DEVICE_MEMORY) Tools::addEntry(m_create_info.library->getLog(), Tools::LogEntryError("VK_ERROR_OUT_OF_DEVICE_MEMORY"));
							if (_vk_result == VK_ERROR_LAYER_NOT_PRESENT) Tools::addEntry(m_create_info.library->getLog(), Tools::LogEntryError("VK_ERROR_LAYER_NOT_PRESENT"));

							m_valid = false;

							return;
						}

						// ~~~~~~~~~~~~~~~~
						// получаем список VkExtensionProperties
						// ~~~~~~~~~~~~~~~~

						VkExtensionPropertiesArr _vk_extension_properties_collection = {};
						_vk_extension_properties_collection.resize(_count);

						_vk_result = m_vulkan_command->vkEnumerateInstanceExtensionProperties(VK_NULL_HANDLE, &_count, _vk_extension_properties_collection.data());

						if (_vk_result != VK_SUCCESS &&
							_vk_result != VK_INCOMPLETE) {

							if (_vk_result == VK_ERROR_OUT_OF_HOST_MEMORY) Tools::addEntry(m_create_info.library->getLog(), Tools::LogEntryError("VK_ERROR_OUT_OF_HOST_MEMORY"));
							if (_vk_result == VK_ERROR_OUT_OF_DEVICE_MEMORY) Tools::addEntry(m_create_info.library->getLog(), Tools::LogEntryError("VK_ERROR_OUT_OF_DEVICE_MEMORY"));
							if (_vk_result == VK_ERROR_LAYER_NOT_PRESENT) Tools::addEntry(m_create_info.library->getLog(), Tools::LogEntryError("VK_ERROR_LAYER_NOT_PRESENT"));

							m_valid = false;

							return;
						}

						// ~~~~~~~~~~~~~~~~
						// перебираем _vk_extension_properties_collection и _extensions_supported_by_library
						// ищем совпадения
						// ~~~~~~~~~~~~~~~~

						ExtensionSptrArr _extensions_supported_by_library = {
							std::make_shared<Extensions::VkExtDebugReport>(),
							std::make_shared<Extensions::VkExtDebugUtils>()
						};

						for (size_t ct_0 = 0; ct_0 < _vk_extension_properties_collection.size(); ++ct_0) {					
							const auto& it_vk_extension_properties = _vk_extension_properties_collection[ct_0];

							for (size_t ct_1 = 0; ct_1 < _extensions_supported_by_library.size(); ++ct_1) {						
								const auto& it_extensions_supported_by_library = _extensions_supported_by_library[ct_1];

								if (it_vk_extension_properties.extensionName == it_extensions_supported_by_library->getName()) {

									m_extension_supported_collection.emplace_back(it_extensions_supported_by_library);

									ct_1 = _extensions_supported_by_library.size();
								}

							} // for (size_t ct_1 = 0;

						} // for (size_t ct_0 = 0;
					}

					m_valid = true;
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				void HCPILibraryVulkan::prepareEnabledLayersAndExtensions(void) noexcept {

					// ~~~~~~~~~~~~~~~~
					// перебираем список поддерживаемых слоёв (m_layer_supported_collection)
					// ~~~~~~~~~~~~~~~~

					for (size_t ct_0 = 0; ct_0 < m_layer_supported_collection.size(); ++ct_0) {
						const auto& it_layer = m_layer_supported_collection[ct_0];

						// ~~~~~~~~~~~~~~~~
						// в debug сборке в список активированных слоёв (m_layer_enabled_collection)
						// добавляем отладочные слои:
						// Layers::VkLayerKhronosValidation
						// ~~~~~~~~~~~~~~~~

						if constexpr (CGDev::RPI::Private::BuildSettings::g_type_build == CGDev::RPI::Private::BuildSettings::TypeBuild::DEBUG) {

							if (it_layer->getName() == Layers::VkLayerKhronosValidation::getName()) {

								m_layer_enabled_collection.emplace_back(it_layer);

							}
						}

						// ~~~~~~~~~~~~~~~~
						// перебираем список запрашиваемых дополнительных возможностей (HCPI_LIBRARY_CREATE_INFO::feature_name_collection)
						// в вулкане тут и слои и расширения
						// ~~~~~~~~~~~~~~~~

						for (size_t ct_1 = 0; ct_1 < m_create_info.library->getCreateInfo().feature_name_collection.size(); ++ct_1) {
							const auto& it_name = m_create_info.library->getCreateInfo().feature_name_collection[ct_1];

							if (it_layer->getName() == it_name) {

								m_layer_enabled_collection.emplace_back(it_layer);

								//прерываем дальнейший перебор, потому что нашли
								ct_1 = m_layer_supported_collection.size();
							}

						}

					} // for (size_t ct_1 = 0;

					// ~~~~~~~~~~~~~~~~
					// перебираем список поддерживаемых расширений (m_extension_supported_collection)
					// ~~~~~~~~~~~~~~~~

					for (size_t ct_0 = 0; ct_0 < m_extension_supported_collection.size(); ++ct_0) {
						const auto& it_extension = m_extension_supported_collection[ct_0];

						// ~~~~~~~~~~~~~~~~
						// в debug сборке в список активированных расширений (m_extension_enabled_collection)
						// добавляем отладочные расширения:
						// Extensions::VkExtDebugUtils
						// ~~~~~~~~~~~~~~~~

						if constexpr (CGDev::RPI::Private::BuildSettings::g_type_build == CGDev::RPI::Private::BuildSettings::TypeBuild::DEBUG) {

							if (it_extension->getName() == Extensions::VkExtDebugUtils::getName()) {

								m_extension_enabled_collection.emplace_back(it_extension);

							}
						}

						// ~~~~~~~~~~~~~~~~
						// перебираем список запрашиваемых дополнительных возможностей (HCPI_LIBRARY_CREATE_INFO::feature_name_collection)
						// в вулкане тут и слои и расширения
						// ~~~~~~~~~~~~~~~~

						for (size_t ct_1 = 0; ct_1 < m_create_info.library->getCreateInfo().feature_name_collection.size(); ++ct_1) {
							const auto& it_name = m_create_info.library->getCreateInfo().feature_name_collection[ct_1];

							if (it_extension->getName() == it_name) {

								m_extension_enabled_collection.emplace_back(it_extension);

								//прерываем дальнейший перебор, потому что нашли
								ct_1 = m_extension_supported_collection.size();
							}

						} // for (size_t ct_1 = 0;

					} // for (size_t ct_0 = 0;

					m_valid = true;

				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				void HCPILibraryVulkan::createInstance(void) noexcept {

					// ~~~~~~~~~~~~~~~~
					// описываем INSTANCE_CREATE_INFO
					// ~~~~~~~~~~~~~~~~

					INSTANCE_CREATE_INFO _v_instance_create_info				= {};
					_v_instance_create_info.log									= m_create_info.library->getLog();
					_v_instance_create_info.library_vulkan_version				= BuildSettings::g_version;
					_v_instance_create_info.vulkan_command						= m_vulkan_command;
					_v_instance_create_info.layer_collection					= {};
					_v_instance_create_info.extension_collection				= {};

					// ~~~~~~~~~~~~~~~~
					// создаём Instance
					// ~~~~~~~~~~~~~~~~

					auto _instance = std::make_shared<CGDev::RPI::Private::Vulkan::Instance>();

					_instance->create(_v_instance_create_info);

					if (_instance->isValid() == false) {

						Tools::addEntry(m_create_info.library->getLog(), Tools::LogEntryError("Не удалось создать Instance"));

						m_valid = false;

						return;
					}

					m_instance = std::move(_instance);

					// ~~~~~~~~~~~~~~~~
					// запрашиваем комманды вулкана у загрузчика, используй созданный Instance
					// ~~~~~~~~~~~~~~~~

					m_loader->requestVulkanCommand(m_vulkan_command, m_instance);

					m_valid = true;
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				void HCPILibraryVulkan::createDebug(void) noexcept {

					// ~~~~~~~~~~~~~~~~
					// функция компилируется только если сборка проекта отладочная
					// ~~~~~~~~~~~~~~~~

					//if constexpr (CGDev::RPI::Private::BuildSettings::g_type_build == CGDev::RPI::Private::BuildSettings::TypeBuild::DEBUG) {

						// ~~~~~~~~~~~~~~~~
						// описываем VK_EXT_DEBUG_UTILS_CREATE_INFO
						// ~~~~~~~~~~~~~~~~

					//	Extensions::VK_EXT_DEBUG_UTILS_CREATE_INFO _create_info		= {};
					//	_create_info.log										= m_log;
					//	_create_info.vulkan_command								= m_vulkan_command;
					//	_create_info.instance									= m_instance;

						// ~~~~~~~~~~~~~~~~
						// создаём VK_EXT_DEBUG_UTILS_CREATE_INFO
						// ~~~~~~~~~~~~~~~~

					//	auto _ext_debug_utils = std::make_shared<Extensions::VkExtDebugUtils>();

					//	_ext_debug_utils->create(_create_info);

					//	if (_ext_debug_utils->isValid() == false) {

					//		Tools::addEntry(this->getLog(), Tools::LogEntryError("Не удалось создать VkExtDebugUtils"));

					//		m_valid = false;

					//		return;

					//	} // if (_ext_debug_utils->isValid()

					//} // if constexpr (BuildSettings::g_type_build == BuildSettings::TypeBuild::DEBUG)

					//m_valid = true;
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				void HCPILibraryVulkan::createPhysicalDevice(void) noexcept {

					std::vector<size_t> _group_count;

					// ~~~~~~~~~~~~~~~~
					// если версия вулкана 1.0, получаем список VkPhysicalDevice через vkEnumeratePhysicalDevices
					// 
					// ~~~~~~~~~~~~~~~~

					if constexpr (BuildSettings::g_version == LibraryVulkanVersion::VERSION_10) {

						// ~~~~~~~~~~~~~~~~
						// получем количество VkPhysicalDevice
						// ~~~~~~~~~~~~~~~~

						uint32_t _count = 0;

						auto _vk_result = m_vulkan_command->vkEnumeratePhysicalDevices(m_instance->getVkInstance(), &_count, VK_NULL_HANDLE);

						if (_vk_result != VK_SUCCESS) {

							if (_vk_result == VK_ERROR_OUT_OF_HOST_MEMORY) Tools::addEntry(m_create_info.library->getLog(), Tools::LogEntryError("VK_ERROR_OUT_OF_HOST_MEMORY"));
							if (_vk_result == VK_ERROR_OUT_OF_DEVICE_MEMORY) Tools::addEntry(m_create_info.library->getLog(), Tools::LogEntryError("VK_ERROR_OUT_OF_DEVICE_MEMORY"));
							if (_vk_result == VK_ERROR_INITIALIZATION_FAILED) Tools::addEntry(m_create_info.library->getLog(), Tools::LogEntryError("VK_ERROR_INITIALIZATION_FAILED"));

							m_valid = false;

							return;
						}

						// ~~~~~~~~~~~~~~~~
						// получем список VkPhysicalDevice
						// ~~~~~~~~~~~~~~~~

						VkPhysicalDeviceArr _vk_physical_device_colllection(_count);

						m_vulkan_command->vkEnumeratePhysicalDevices(m_instance->getVkInstance(), &_count, _vk_physical_device_colllection.data());

						if (_vk_result != VK_SUCCESS && _vk_result != VK_INCOMPLETE) {

							if (_vk_result == VK_ERROR_OUT_OF_HOST_MEMORY) Tools::addEntry(m_create_info.library->getLog(), Tools::LogEntryError("VK_ERROR_OUT_OF_HOST_MEMORY"));
							if (_vk_result == VK_ERROR_OUT_OF_DEVICE_MEMORY) Tools::addEntry(m_create_info.library->getLog(), Tools::LogEntryError("VK_ERROR_OUT_OF_DEVICE_MEMORY"));
							if (_vk_result == VK_ERROR_INITIALIZATION_FAILED) Tools::addEntry(m_create_info.library->getLog(), Tools::LogEntryError("VK_ERROR_INITIALIZATION_FAILED"));

							m_valid = false;

							return;
						}

						_group_count.push_back(_count);

					}

					if constexpr (BuildSettings::g_version >= LibraryVulkanVersion::VERSION_11) {
					
						// ~~~~~~~~~~~~~~~~
						// получем количество VkPhysicalDeviceGroupProperties
						// ~~~~~~~~~~~~~~~~

						uint32_t _count_group = 0;

						auto _vk_result = m_vulkan_command->vkEnumeratePhysicalDeviceGroups(m_instance->getVkInstance(), &_count_group, VK_NULL_HANDLE);

						if (_vk_result != VK_SUCCESS) {

							if (_vk_result == VK_ERROR_OUT_OF_HOST_MEMORY) Tools::addEntry(m_create_info.library->getLog(), Tools::LogEntryError("VK_ERROR_OUT_OF_HOST_MEMORY"));
							if (_vk_result == VK_ERROR_OUT_OF_DEVICE_MEMORY) Tools::addEntry(m_create_info.library->getLog(), Tools::LogEntryError("VK_ERROR_OUT_OF_DEVICE_MEMORY"));
							if (_vk_result == VK_ERROR_INITIALIZATION_FAILED) Tools::addEntry(m_create_info.library->getLog(), Tools::LogEntryError("VK_ERROR_INITIALIZATION_FAILED"));

							m_valid = false;

							return;
						}

						// ~~~~~~~~~~~~~~~~
						// получем список VkPhysicalDeviceGroupProperties
						// ~~~~~~~~~~~~~~~~

						VkPhysicalDeviceGroupPropertiesArr _vk_physical_device_group_properties(_count_group);

						_vk_result = m_vulkan_command->vkEnumeratePhysicalDeviceGroups(m_instance->getVkInstance(), &_count_group, _vk_physical_device_group_properties.data());

						if (_vk_result != VK_SUCCESS && _vk_result != VK_INCOMPLETE) {

							if (_vk_result == VK_ERROR_OUT_OF_HOST_MEMORY) Tools::addEntry(m_create_info.library->getLog(), Tools::LogEntryError("VK_ERROR_OUT_OF_HOST_MEMORY"));
							if (_vk_result == VK_ERROR_OUT_OF_DEVICE_MEMORY) Tools::addEntry(m_create_info.library->getLog(), Tools::LogEntryError("VK_ERROR_OUT_OF_DEVICE_MEMORY"));
							if (_vk_result == VK_ERROR_INITIALIZATION_FAILED) Tools::addEntry(m_create_info.library->getLog(), Tools::LogEntryError("VK_ERROR_INITIALIZATION_FAILED"));

							m_valid = false;

							return;
						}

						// ~~~~~~~~~~~~~~~~
						// 
						// ~~~~~~~~~~~~~~~~

						for (size_t ct_0 = 0; ct_0 < _vk_physical_device_group_properties.size(); ++ct_0) {
							const auto& it_vk_physical_device_group_properties = _vk_physical_device_group_properties[ct_0];

							_group_count.push_back(it_vk_physical_device_group_properties.physicalDeviceCount);
					
						}
						
					}
					
					// ~~~~~~~~~~~~~~~~
					// перебираем группы PhysicalDevice
					// перебираем каждый элемент группы
					// и создаём PhysicalDevice
					// ~~~~~~~~~~~~~~~~

					for (size_t ct_0 = 0; ct_0 < _group_count.size(); ++ct_0) {
						auto& it_element_count = _group_count[ct_0];

						PhysicalDeviceSptrArr _physical_device_group;

						for (size_t ct_1 = 0; ct_1 < it_element_count; ++ct_1) {
							
							// ~~~~~~~~~~~~~~~~
							// описываем PHYSICAL_DEVICE_CREATE_INFO
							// ~~~~~~~~~~~~~~~~

							PHYSICAL_DEVICE_CREATE_INFO _physical_device_create_info		= {};
							_physical_device_create_info.log								= m_create_info.library->getLog();
							_physical_device_create_info.vulkan_command						= m_vulkan_command;
							_physical_device_create_info.instance							= m_instance;
							_physical_device_create_info.id_group							= ct_0;
							_physical_device_create_info.id_element							= ct_1;

							// ~~~~~~~~~~~~~~~~
							// создаём PHYSICAL_DEVICE_CREATE_INFO
							// ~~~~~~~~~~~~~~~~

							auto _physical_device = std::make_shared<PhysicalDevice>();

							_physical_device->create(_physical_device_create_info);

							if (_physical_device->isValid() == false) {

								Tools::addEntry(m_create_info.library->getLog(), Tools::LogEntryError("Не удалось создать PhysicalDevice"));
							}
							else {

								_physical_device_group.emplace_back(std::move(_physical_device));
							}

						} // for (size_t ct_1 = 0;

						m_physical_device_group_collection.emplace_back(_physical_device_group);						

					} // for (size_t ct_0 = 0;				

					// ~~~~~~~~~~~~~~~~
					// 
					// ~~~~~~~~~~~~~~~~

					if (m_physical_device_group_collection.empty() == true) {

						Tools::addEntry(m_create_info.library->getLog(), Tools::LogEntryError("Не удалось создать ни одного PhysicalDevice"));

						m_valid = false;

						return;
					}

					m_valid = true;
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			} // namespace Vulkan

		} // namespace Private

	} // namespace RPI

} // namespace CGDev