////////////////////////////////////////////////////////////////
// ������ �������-����������
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// ������ ������������� �����
////////////////////////////////////////////////////////////////
#include "hcpi_library_vulkan.h"
////////////////////////////////////////////////////////////////
// ������ �������������
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// ������ ��� ����������
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
					// ��������� HCPI_LIBRARY_VULKAN_CREATE_INFO �� ������������
					// ~~~~~~~~~~~~~~~~

					if (create_info.isValid() == false) {

						if(create_info.library != nullptr) Tools::addEntry(create_info.library->getLog(), Tools::LogEntryError("HCPI_LIBRARY_VULKAN_CREATE_INFO �� ���������"));

						return;
					}

					m_create_info = create_info;

					// ~~~~~~~~~~~~~~~~
					// ������ Loader
					// ~~~~~~~~~~~~~~~~

					createLoader();
					
					if (isValid() == false) {

						Tools::addEntry(m_create_info.library->getLog(), Tools::LogEntryError("�� ������� ������� Loader"));

						m_valid = false;

						return;
					}

					// ~~~~~~~~~~~~~~~~
					// �������������� ������ �������������� ���� � ����������
					// ~~~~~~~~~~~~~~~~

					prepareSupportedLayersAndExtensions();

					if (isValid() == false) {

						Tools::addEntry(m_create_info.library->getLog(), Tools::LogEntryError("�� ������� �������� ������ �������������� ���� � ����������"));

						m_valid = false;

						return;
					}
					
					// ~~~~~~~~~~~~~~~~
					// �������������� ������ �������������� ���� � ����������
					// ~~~~~~~~~~~~~~~~
					
					prepareEnabledLayersAndExtensions();

					if (isValid() == false) {

						Tools::addEntry(m_create_info.library->getLog(), Tools::LogEntryError("�� ������� �������� ������ �������������� ���� � ����������"));

						m_valid = false;

						return;
					}

					//prepareDebug();

					// ~~~~~~~~~~~~~~~~
					// ������ Instance
					// ~~~~~~~~~~~~~~~~

					createInstance();

					if (this->isValid() == false) {

						m_valid = false;

						return;
					}

					// ~~~~~~~~~~~~~~~~
					// ������ PhysicalDevice
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
					// ��������� LOADER_CREATE_INFO
					// ~~~~~~~~~~~~~~~~
					
					LOADER_CREATE_INFO _v_loader_create_info		= {};
					_v_loader_create_info.log						= m_create_info.library->getLog();

					// ~~~~~~~~~~~~~~~~
					// ������ Loader
					// ~~~~~~~~~~~~~~~~

					auto _loader = std::make_shared<Loader>();

					_loader->create(_v_loader_create_info);

					if (_loader->isValid() == false) {

						Tools::addEntry(m_create_info.library->getLog(), Tools::LogEntryError("�� ������� ������� Vloader"));

						m_valid = false;

						return;
					}

					m_loader = std::move(_loader);

					// ~~~~~~~~~~~~~~~~
					// ����������� �������� ������� � ����������
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
					// ����� vkEnumerateInstanceLayerProperties ����������� ���� �������,
					// ��������� � �������
					// ~~~~~~~~~~~~~~~~

					{
						// ~~~~~~~~~~~~~~~~
						// �������� ���������� VkLayerProperties
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
						// �������� ������ VkLayerProperties
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
						// ���������� _vk_layer_properties_collection � _layer_supported_by_library
						// ���� ����������
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
					// ����� vkEnumerateInstanceExtensionProperties ����������� ���������� �������,
					// ��������� � �������
					// ~~~~~~~~~~~~~~~~

					{
						// ~~~~~~~~~~~~~~~~
						// �������� ���������� VkExtensionProperties
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
						// �������� ������ VkExtensionProperties
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
						// ���������� _vk_extension_properties_collection � _extensions_supported_by_library
						// ���� ����������
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
					// ���������� ������ �������������� ���� (m_layer_supported_collection)
					// ~~~~~~~~~~~~~~~~

					for (size_t ct_0 = 0; ct_0 < m_layer_supported_collection.size(); ++ct_0) {
						const auto& it_layer = m_layer_supported_collection[ct_0];

						// ~~~~~~~~~~~~~~~~
						// � debug ������ � ������ �������������� ���� (m_layer_enabled_collection)
						// ��������� ���������� ����:
						// Layers::VkLayerKhronosValidation
						// ~~~~~~~~~~~~~~~~

						if constexpr (CGDev::RPI::Private::BuildSettings::g_type_build == CGDev::RPI::Private::BuildSettings::TypeBuild::DEBUG) {

							if (it_layer->getName() == Layers::VkLayerKhronosValidation::getName()) {

								m_layer_enabled_collection.emplace_back(it_layer);

							}
						}

						// ~~~~~~~~~~~~~~~~
						// ���������� ������ ������������� �������������� ������������ (HCPI_LIBRARY_CREATE_INFO::feature_name_collection)
						// � ������� ��� � ���� � ����������
						// ~~~~~~~~~~~~~~~~

						for (size_t ct_1 = 0; ct_1 < m_create_info.library->getCreateInfo().feature_name_collection.size(); ++ct_1) {
							const auto& it_name = m_create_info.library->getCreateInfo().feature_name_collection[ct_1];

							if (it_layer->getName() == it_name) {

								m_layer_enabled_collection.emplace_back(it_layer);

								//��������� ���������� �������, ������ ��� �����
								ct_1 = m_layer_supported_collection.size();
							}

						}

					} // for (size_t ct_1 = 0;

					// ~~~~~~~~~~~~~~~~
					// ���������� ������ �������������� ���������� (m_extension_supported_collection)
					// ~~~~~~~~~~~~~~~~

					for (size_t ct_0 = 0; ct_0 < m_extension_supported_collection.size(); ++ct_0) {
						const auto& it_extension = m_extension_supported_collection[ct_0];

						// ~~~~~~~~~~~~~~~~
						// � debug ������ � ������ �������������� ���������� (m_extension_enabled_collection)
						// ��������� ���������� ����������:
						// Extensions::VkExtDebugUtils
						// ~~~~~~~~~~~~~~~~

						if constexpr (CGDev::RPI::Private::BuildSettings::g_type_build == CGDev::RPI::Private::BuildSettings::TypeBuild::DEBUG) {

							if (it_extension->getName() == Extensions::VkExtDebugUtils::getName()) {

								m_extension_enabled_collection.emplace_back(it_extension);

							}
						}

						// ~~~~~~~~~~~~~~~~
						// ���������� ������ ������������� �������������� ������������ (HCPI_LIBRARY_CREATE_INFO::feature_name_collection)
						// � ������� ��� � ���� � ����������
						// ~~~~~~~~~~~~~~~~

						for (size_t ct_1 = 0; ct_1 < m_create_info.library->getCreateInfo().feature_name_collection.size(); ++ct_1) {
							const auto& it_name = m_create_info.library->getCreateInfo().feature_name_collection[ct_1];

							if (it_extension->getName() == it_name) {

								m_extension_enabled_collection.emplace_back(it_extension);

								//��������� ���������� �������, ������ ��� �����
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
					// ��������� INSTANCE_CREATE_INFO
					// ~~~~~~~~~~~~~~~~

					INSTANCE_CREATE_INFO _v_instance_create_info				= {};
					_v_instance_create_info.log									= m_create_info.library->getLog();
					_v_instance_create_info.library_vulkan_version				= BuildSettings::g_version;
					_v_instance_create_info.vulkan_command						= m_vulkan_command;
					_v_instance_create_info.layer_collection					= {};
					_v_instance_create_info.extension_collection				= {};

					// ~~~~~~~~~~~~~~~~
					// ������ Instance
					// ~~~~~~~~~~~~~~~~

					auto _instance = std::make_shared<CGDev::RPI::Private::Vulkan::Instance>();

					_instance->create(_v_instance_create_info);

					if (_instance->isValid() == false) {

						Tools::addEntry(m_create_info.library->getLog(), Tools::LogEntryError("�� ������� ������� Instance"));

						m_valid = false;

						return;
					}

					m_instance = std::move(_instance);

					// ~~~~~~~~~~~~~~~~
					// ����������� �������� ������� � ����������, ��������� ��������� Instance
					// ~~~~~~~~~~~~~~~~

					m_loader->requestVulkanCommand(m_vulkan_command, m_instance);

					m_valid = true;
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				void HCPILibraryVulkan::createDebug(void) noexcept {

					// ~~~~~~~~~~~~~~~~
					// ������� ������������� ������ ���� ������ ������� ����������
					// ~~~~~~~~~~~~~~~~

					//if constexpr (CGDev::RPI::Private::BuildSettings::g_type_build == CGDev::RPI::Private::BuildSettings::TypeBuild::DEBUG) {

						// ~~~~~~~~~~~~~~~~
						// ��������� VK_EXT_DEBUG_UTILS_CREATE_INFO
						// ~~~~~~~~~~~~~~~~

					//	Extensions::VK_EXT_DEBUG_UTILS_CREATE_INFO _create_info		= {};
					//	_create_info.log										= m_log;
					//	_create_info.vulkan_command								= m_vulkan_command;
					//	_create_info.instance									= m_instance;

						// ~~~~~~~~~~~~~~~~
						// ������ VK_EXT_DEBUG_UTILS_CREATE_INFO
						// ~~~~~~~~~~~~~~~~

					//	auto _ext_debug_utils = std::make_shared<Extensions::VkExtDebugUtils>();

					//	_ext_debug_utils->create(_create_info);

					//	if (_ext_debug_utils->isValid() == false) {

					//		Tools::addEntry(this->getLog(), Tools::LogEntryError("�� ������� ������� VkExtDebugUtils"));

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
					// ���� ������ ������� 1.0, �������� ������ VkPhysicalDevice ����� vkEnumeratePhysicalDevices
					// 
					// ~~~~~~~~~~~~~~~~

					if constexpr (BuildSettings::g_version == LibraryVulkanVersion::VERSION_10) {

						// ~~~~~~~~~~~~~~~~
						// ������� ���������� VkPhysicalDevice
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
						// ������� ������ VkPhysicalDevice
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
						// ������� ���������� VkPhysicalDeviceGroupProperties
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
						// ������� ������ VkPhysicalDeviceGroupProperties
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
					// ���������� ������ PhysicalDevice
					// ���������� ������ ������� ������
					// � ������ PhysicalDevice
					// ~~~~~~~~~~~~~~~~

					for (size_t ct_0 = 0; ct_0 < _group_count.size(); ++ct_0) {
						auto& it_element_count = _group_count[ct_0];

						PhysicalDeviceSptrArr _physical_device_group;

						for (size_t ct_1 = 0; ct_1 < it_element_count; ++ct_1) {
							
							// ~~~~~~~~~~~~~~~~
							// ��������� PHYSICAL_DEVICE_CREATE_INFO
							// ~~~~~~~~~~~~~~~~

							PHYSICAL_DEVICE_CREATE_INFO _physical_device_create_info		= {};
							_physical_device_create_info.log								= m_create_info.library->getLog();
							_physical_device_create_info.vulkan_command						= m_vulkan_command;
							_physical_device_create_info.instance							= m_instance;
							_physical_device_create_info.id_group							= ct_0;
							_physical_device_create_info.id_element							= ct_1;

							// ~~~~~~~~~~~~~~~~
							// ������ PHYSICAL_DEVICE_CREATE_INFO
							// ~~~~~~~~~~~~~~~~

							auto _physical_device = std::make_shared<PhysicalDevice>();

							_physical_device->create(_physical_device_create_info);

							if (_physical_device->isValid() == false) {

								Tools::addEntry(m_create_info.library->getLog(), Tools::LogEntryError("�� ������� ������� PhysicalDevice"));
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

						Tools::addEntry(m_create_info.library->getLog(), Tools::LogEntryError("�� ������� ������� �� ������ PhysicalDevice"));

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