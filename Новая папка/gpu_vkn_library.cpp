////////////////////////////////////////////////////////////////
// ������ �������-����������
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// ������ ������������� �����
////////////////////////////////////////////////////////////////
#include "gpu_vkn_library.h"
////////////////////////////////////////////////////////////////
// ������ �������������
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// ������ ��� ����������
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
					// ������
					// Build::s_vkn_debug
					// ������
					// ~~~~~~~~~~~~~~~~
					// ��������� ���������� GpuVknLibraryCreateInfo
					// ~~~~~~~~~~~~~~~~

					if constexpr (Build::s_vkn_debug == true) {

						if (validationCreateInfo(create_info) == false) return false;

					}

					// ~~~~~~~~~~~~~~~~
					// ������
					// Build::s_vkn_debug
					// �����
					// ~~~~~~~~~~~~~~~~

					// ~~~~~~~~~~~~~~~~
					// ��������� GpuVknLibraryCreateInfo
					// ~~~~~~~~~~~~~~~~

					m_create_info = create_info;

					// ~~~~~~~~~~~~~~~~
					// ������ VknLoader
					// ~~~~~~~~~~~~~~~~

					if (createVknLoader() == false) {

						Tools::addEntry(m_create_info.log, Tools::LogEntryError("createVknLoader() == false"));

						return false;
					}

					// ~~~~~~~~~~~~~~~~
					// ������ VknGlobalCommands '����������� ������'
					// ~~~~~~~~~~~~~~~~

					if (createVknCommandsGlobalLevel() == false) {

						Tools::addEntry(m_create_info.log, Tools::LogEntryError("createVknCommandsGlobalLevel() = StatusCode::FAIL"));

						return false;
					}
					
					// ~~~~~~~~~~~~~~~~
					// ������ VknInstance
					// ~~~~~~~~~~~~~~~~

					Status _status;

					_status = createVknInstance();

					//if (_status.m_code != StatusCode::SUCCESSFUL) {
					//	_status.append("createVknInstance() = false");
					//	return false;
					//}

					// ~~~~~~~~~~~~~~~~
					// ������ VknGlobalCommands '������ VkInstance'
					// ~~~~~~~~~~~~~~~~

					if (createVknCommandsInstanceLevel() == false) {

						Tools::addEntry(m_create_info.log, Tools::LogEntryError("createVknCommandsInstanceLevel() = false"));

						return false;
					}
						
					// ~~~~~~~~~~~~~~~~
					// ������
					// Build::s_vk_ext_debug_utils
					// ������
					// ~~~~~~~~~~~~~~~~
					// ����� createVknExtDebugUtils ������������ ��� �������� ���������� 'VK_EXT_debug_utils'
					// �������������, ����� ������ ���������� ������ � ������, � �������������� ���������� VK_EXT_debug_utils
					// ~~~~~~~~~~~~~~~~

					if constexpr (Build::s_vk_ext_debug_utils == true) {

						// ~~~~~~~~~~~~~~~~
						// �������� ������� VknExtDebugUtils
						// ~~~~~~~~~~~~~~~~

						if (createVknExtDebugUtils() == false) {

							Tools::addEntry(m_create_info.log, Tools::LogEntryError("createVknExtDebugUtils() = false"));

							return false;
						}

					}

					// ~~~~~~~~~~~~~~~~
					// ������
					// Build::s_vk_ext_debug_utils
					// �����
					// ~~~~~~~~~~~~~~~~
					
					// ~~~~~~~~~~~~~~~~
					// ������ VknPhysicalDevice
					// ~~~~~~~~~~~~~~~~

					if (prepareVknPhysicalDevice() == false) {

						Tools::addEntry(m_create_info.log, Tools::LogEntryError("prepareVknPhysicalDevice() = false"));

						return false;
					}

					// ~~~~~~~~~~~~~~~~
					// ������
					// Build::s_vkn_debug == true
					// ������
					// ~~~~~~~~~~~~~~~~
					// � ���������� ������ ��������� ���������
					// ~~~~~~~~~~~~~~~~

					if constexpr (Build::s_vkn_debug == true) {

						// ~~~~~~~~~~~~~~~~
						// �������� �� ���������
						// ~~~~~~~~~~~~~~~~

						if (checkValidation() == false) {

							return false;

						}

					}

					// ~~~~~~~~~~~~~~~~
					// ������
					// Build::s_vkn_debug == true
					// �����
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
					// ������� ������
					// ~~~~~~~~~~~~~~~~

					m_create_info			= {};
					s_vkn_loader			= nullptr;
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				bool GpuVknLibrary::checkValidation(void) noexcept {

					// ~~~~~~~~~~~~~~~~
					// � ������� Extensions::VknExtDebugUtilsPtr �� ����������� ������ ��������� ����� ����� requestLastValidation
					// ~~~~~~~~~~~~~~~~

					// ~~~~~~~~~~~~~~~~
					// ������
					// Build::s_vk_ext_debug_utils
					// ������
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
					// ������
					// Build::s_vk_ext_debug_utils
					// �����
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
					// ������ VknLoader, � ������� �������� ����� ����� �������� VknGlobalCommands ����������� ������
					// ~~~~~~~~~~~~~~~~
					
					// ~~~~~~~~~~~~~~~~
					// ��������� VknLoaderCreateInfo
					// ~~~~~~~~~~~~~~~~

					VknLoaderCreateInfo _vkn_loader_create_info			= {};
					_vkn_loader_create_info.log							= m_create_info.log;

					// ~~~~~~~~~~~~~~~~
					// ������ VknLoader
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
					// ��������� VknInstanceCreateInfo
					// ~~~~~~~~~~~~~~~~

					VknInstanceCreateInfo _vkn_instance_create_info					= {};
					_vkn_instance_create_info.log									= m_create_info.log;
					_vkn_instance_create_info.vkn_commands							= s_vkn_commands_global_level;
					_vkn_instance_create_info.application_name						= m_create_info.gpu_library->getCreateInfo().application_name;
					_vkn_instance_create_info.engine_name							= Private::Build::engine_name;
					_vkn_instance_create_info.layer_name_collection					= {};
					_vkn_instance_create_info.extension_name_collection				= {};

					// ~~~~~~~~~~~~~~~~
					// ������ VknInstance
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
					// � ������� VkPhysicalDevice �� �������� ��������
					// VkPhysicalDevice ���������� �������������, ����� �������� VkInstance
					// ������� ��� �������� VknPhysicalDevice, ������ �� ������� ������ VkPhysicalDevice
					// ~~~~~~~~~~~~~~~~
					
					// ~~~~~~~~~~~~~~~~
					// ��������� ���������� ������, ������� ���:
					//			X					X					X
					//		Y	[VkPhysicalDevice]	[VkPhysicalDevice]	[VkPhysicalDevice]
					//		Y	[VkPhysicalDevice]	[VkPhysicalDevice]	[VkPhysicalDevice]
					//		Y	[VkPhysicalDevice]	[VkPhysicalDevice]	[VkPhysicalDevice]
					// ~~~~~~~~~~~~~~~~

					VkPhysicalDeviceArr2 _vk_physical_device_collection2;

					prepareNextForDeviceGroupDeviceCreateInfo(_vk_physical_device_collection2);
					
					// ~~~~~~~~~~~~~~~~
					// ���������� ������������� ���������� ������
					// ~~~~~~~~~~~~~~~~			
				
					// ~~~~~~~~~~~~~~~~
					// ���������� VkPhysicalDeviceArr2
					// ~~~~~~~~~~~~~~~~

					for (size_t ct_vk_physical_device_arr1 = 0; ct_vk_physical_device_arr1 < _vk_physical_device_collection2.size(); ++ct_vk_physical_device_arr1) {
						const auto& it_vk_physical_device_arr1 = _vk_physical_device_collection2[ct_vk_physical_device_arr1];

						VknPhysicalDevicePtrArr1 _vkn_physical_device_collection1;
						
						// ~~~~~~~~~~~~~~~~
						// ���������� VkPhysicalDeviceArr1
						// ~~~~~~~~~~~~~~~~

						for (size_t ct_vk_physical_device = 0; ct_vk_physical_device < it_vk_physical_device_arr1.size(); ++ct_vk_physical_device) {
							const auto& it_vk_physical_device = it_vk_physical_device_arr1[ct_vk_physical_device];

							// ~~~~~~~~~~~~~~~~
							// �������������� VknPhysicalDevice
							// ~~~~~~~~~~~~~~~~

							VknPhysicalDevicePtr _vkn_physical_device = nullptr;

							// ~~~~~~~~~~~~~~~~
							// ��������� � ������ VknPhysicalDevice
							// ~~~~~~~~~~~~~~~~

							// ~~~~~~~~~~~~~~~~
							// ��������� VknPhysicalDeviceCreateInfo
							// ~~~~~~~~~~~~~~~~

							VknPhysicalDeviceCreateInfo _vkn_physical_device_create_info		= {};
							_vkn_physical_device_create_info.vkn_commands						= s_vkn_commands_instance_level;
							_vkn_physical_device_create_info.vkn_instance						= m_vkn_instance;
							_vkn_physical_device_create_info.vk_physical_device					= it_vk_physical_device;

							// ~~~~~~~~~~~~~~~~
							// ������ VknPhysicalDevice
							// ~~~~~~~~~~~~~~~~

							_vkn_physical_device = new VknPhysicalDevice();

							//if (_vkn_physical_device->create(_vkn_physical_device_create_info) == false) {

							//	Tools::addEntry(m_create_info.log, Tools::LogEntryError("VknPhysicalDevice->create() = false"));

							//	delete _vkn_physical_device;

							//}

							// ~~~~~~~~~~~~~~~~
							// ���� ������� ��������
							// ��������� VknPhysicalDevice 
							// ~~~~~~~~~~~~~~~~

							//else {

							//	_vkn_physical_device_collection1.emplace_back(std::move(_vkn_physical_device));
							//}							

						} // for (size_t ct_vk_physical_device = 0;						

						// ~~~~~~~~~~~~~~~~
						// ���� �� ������
						// ��������� VknPhysicalDevice 
						// ~~~~~~~~~~~~~~~~

						if (_vkn_physical_device_collection1.empty() == false) m_vkn_physical_device_collection2.emplace_back(std::move(_vkn_physical_device_collection1));

					} // for (size_t ct_vk_physical_device_arr1 = 0;

					// ~~~~~~~~~~~~~~~~
					// ��������� ��������� VknPhysicalDevice � ������ ������,
					// � ������� �������� ��������� ��� ������������� VknPhysicalDevice
					// ~~~~~~~~~~~~~~~~

					m_vkn_physical_device_free_collection2 = m_vkn_physical_device_collection2;

					return true;
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				bool GpuVknLibrary::prepareNextForDeviceGroupDeviceCreateInfo(VkPhysicalDeviceArr2& vk_physical_device_collection2) noexcept {

					// ~~~~~~~~~~~~~~~~
					// � ������ ������� 1.0 ��� ��������� ����������� ���� ���������� ��������
					// ������� VkPhysicalDevice �������� � ������� �������� ������� vkEnumeratePhysicalDevices
					// ������� ���������� ������� ������ (������) VkPhysicalDevice
					// ~~~~~~~~~~~~~~~~

					// ~~~~~~~~~~~~~~~~
					// ������
					// Build::vulkan_version
					// 1.0
					// ������
					// ~~~~~~~~~~~~~~~~

					if constexpr (Build::vulkan_version == Build::VulkanVersion::VERSION_10) {

						// ~~~~~~~~~~~~~~~~
						// �������� ���������� VkPhysicalDevice
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
						// �������� ������ VkPhysicalDevice
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
					// ������
					// Build::vulkan_version
					// 1.0
					// �����
					// ~~~~~~~~~~~~~~~~

					// ~~~~~~~~~~~~~~~~
					// ������� � 1.1 � ����, ������ ������������� ���������� � ����������� ������� VkPhysicalDevice
					// ������� � ������� �������� ������� vkEnumeratePhysicalDeviceGroups, �������� VkPhysicalDeviceGroupProperties
					// VkPhysicalDeviceGroupProperties �������� � ��� ����� ���������� �� VkPhysicalDevice � �� ���������� � ������
					// ~~~~~~~~~~~~~~~~

					// ~~~~~~~~~~~~~~~~
					// ������
					// Build::vulkan_version
					// 1.1+
					// ������
					// ~~~~~~~~~~~~~~~~

					if constexpr (Build::vulkan_version >= Build::VulkanVersion::VERSION_11) {

						// ~~~~~~~~~~~~~~~~
						// �������� ���������� VkPhysicalDeviceGroupProperties
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
						// �������� ������ VkPhysicalDeviceGroupProperties
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
						// ���������� ���������� ������, ������� ���:
						//			X														X														X
						//		Y	[VkPhysicalDeviceGroupProperties::VkPhysicalDevice[0]]	[VkPhysicalDeviceGroupProperties::VkPhysicalDevice[0]]	[VkPhysicalDeviceGroupProperties::VkPhysicalDevice[0]]
						//		Y	[VkPhysicalDeviceGroupProperties::VkPhysicalDevice[1]]	[VkPhysicalDeviceGroupProperties::VkPhysicalDevice[1]]	[VkPhysicalDeviceGroupProperties::VkPhysicalDevice[1]]
						//		Y	[VkPhysicalDeviceGroupProperties::VkPhysicalDevice[2]]	[VkPhysicalDeviceGroupProperties::VkPhysicalDevice[2]]	[VkPhysicalDeviceGroupProperties::VkPhysicalDevice[2]]
						// ~~~~~~~~~~~~~~~~
						
						// ~~~~~~~~~~~~~~~~
						// ���������� VkPhysicalDeviceGroupPropertiesArr
						// ~~~~~~~~~~~~~~~~

						for (size_t ct_vk_physical_device_group = 0; ct_vk_physical_device_group < _vk_physical_device_group_properties_collection.size(); ++ct_vk_physical_device_group) {
							const auto& it_vk_physical_device_group = _vk_physical_device_group_properties_collection[ct_vk_physical_device_group];

							VkPhysicalDeviceArr1 _vk_physical_device_collection1;

							// ~~~~~~~~~~~~~~~~
							// ���������� VkPhysicalDeviceGroupProperties::VkPhysicalDevice[]
							// ~~~~~~~~~~~~~~~~

							for (size_t ct_vk_physical_device = 0; ct_vk_physical_device < it_vk_physical_device_group.physicalDeviceCount; ++ct_vk_physical_device) {
								const auto& it_vk_physical_device = it_vk_physical_device_group.physicalDevices[ct_vk_physical_device];

								_vk_physical_device_collection1.emplace_back(it_vk_physical_device);

							} // for (size_t ct_vk_physical_device = 0;

							vk_physical_device_collection2.emplace_back(std::move(_vk_physical_device_collection1));

						} // for (size_t ct_vk_physical_device_arr1 = 0;

					}

					// ~~~~~~~~~~~~~~~~
					// ������
					// Build::vulkan_version
					// 1.1+
					// �����
					// ~~~~~~~~~~~~~~~~

					return true;
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				bool GpuVknLibrary::createVknCommandsGlobalLevel(void) noexcept {

					// ~~~~~~~~~~~~~~~~
					// ������ VknGlobalCommands, ������� ����������������� �������� �������
					// ��� ������� �� ��������� �� VkInstance, �� VkDevice
					// ���� ���������
					//		VknCommandsCreateInfo::vkn_loader = �� nullptr
					//		VknCommandsCreateInfo::vkn_instance = nullptr
					//		VknCommandsCreateInfo::vkn_logical_device = nullptr
					// VknGlobalCommands ���������������� ������ ����������� ���������, � ������
					//		PFN_vkEnumerateInstanceLayerProperties
					//		PFN_vkEnumerateInstanceExtensionProperties
					//		PFN_vkCreateInstance
					//		PFN_vkEnumerateInstanceVersion
					// ~~~~~~~~~~~~~~~~
					
					// ~~~~~~~~~~~~~~~~
					// ��������� VknCommandsCreateInfo
					// ~~~~~~~~~~~~~~~~

					VknCommandsCreateInfo _vkn_commands_create_info			= {};
					_vkn_commands_create_info.log							= m_create_info.log;
					_vkn_commands_create_info.vkn_loader					= s_vkn_loader;
					_vkn_commands_create_info.vkn_instance					= nullptr;
					_vkn_commands_create_info.vkn_logical_device			= nullptr;

					// ~~~~~~~~~~~~~~~~
					// ������ VknGlobalCommands
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
					// ������ VknGlobalCommands, ������� ����������������� �������� �������
					// ��� ������� ��������� VkInstance
					// ���� ���������
					//		VknCommandsCreateInfo::vkn_loader = �� nullptr
					//		VknCommandsCreateInfo::vkn_instance = �� nullptr
					//		VknCommandsCreateInfo::vkn_logical_device = nullptr
					// VknGlobalCommands ���������������� ����������, ������ VkInstance
					// ~~~~~~~~~~~~~~~~

					// ~~~~~~~~~~~~~~~~
					// ��������� VknCommandsCreateInfo
					// ~~~~~~~~~~~~~~~~

					VknCommandsCreateInfo _vkn_commands_create_info				= {};
					_vkn_commands_create_info.log								= m_create_info.log;
					_vkn_commands_create_info.vkn_loader						= s_vkn_loader;
					_vkn_commands_create_info.vkn_instance						= m_vkn_instance;
					_vkn_commands_create_info.vkn_logical_device				= nullptr;

					// ~~~~~~~~~~~~~~~~
					// ���������� VknGlobalCommands
					// ~~~~~~~~~~~~~~~~

					VknCommandsPtr _vkn_commands = nullptr;

					if (allocate(&_vkn_commands) != StatusCode::SUCCESSFUL) {

						Tools::addEntry(m_create_info.log, Tools::LogEntryError("GpuVknLibrary->allocate() != StatusCode::SUCCESSFUL"));

						return false;
					}

					// ~~~~~~~~~~~~~~~~
					// ������ VknGlobalCommands
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
					// ������
					// Build::s_vk_ext_debug_utils
					// ������
					// ~~~~~~~~~~~~~~~~
					// ������ ����� (createVknExtDebugUtils) ������������ ��� �������� ���������� 'VK_EXT_debug_utils'
					// �������������, ����� ������ ���������� ������ � ������, � �������������� ���������� VK_EXT_debug_utils
					// ~~~~~~~~~~~~~~~~

					if constexpr (Build::s_vk_ext_debug_utils == true) {

						// ~~~~~~~~~~~~~~~~
						// ���������� VK_EXT_debug_utils ���������� �������������� ����� �������
						// ��� ��� �������������� �������� ������������ � VknExtDebugUtilsCommands
						// ��� VknExtDebugUtilsCommands, �� ��������� ������� VknExtDebugUtils
						// ������� ������ VknExtDebugUtilsCommands, � ����� VknExtDebugUtils
						// ~~~~~~~~~~~~~~~~

						// ~~~~~~~~~~~~~~~~
						// ��������� VknExtDebugUtilsCommandsCreateInfo
						// ~~~~~~~~~~~~~~~~

						Extensions::VknExtDebugUtilsCommandsCreateInfo _vkn_ext_debug_utils_commands_create_info = {};
						
						_vkn_ext_debug_utils_commands_create_info.vkn_commands = s_vkn_commands_global_level;
						

						// ~~~~~~~~~~~~~~~~
						// ������ VknExtDebugUtilsCommands
						// ~~~~~~~~~~~~~~~~

						auto _vkn_ext_debug_utils_commands = new Extensions::VknExtDebugUtilsCommands();

						//if (_vkn_ext_debug_utils_commands->create(_vkn_ext_debug_utils_commands_create_info) == false) {

						//	Tools::addEntry(m_create_info.log, Tools::LogEntryError("VknExtDebugUtilsCommands->create() = false"));

						//	return false;
						//}

						// ~~~~~~~~~~~~~~~~
						// ��������� VknExtDebugUtilsCreateInfo
						// ~~~~~~~~~~~~~~~~

						Extensions::VknExtDebugUtilsCreateInfo _vkn_ext_debug_utils_create_info = {};
						//_vkn_ext_debug_utils_create_info.log = m_create_info.log;
						_vkn_ext_debug_utils_create_info.vkn_debug_utils_commands = _vkn_ext_debug_utils_commands;

						// ~~~~~~~~~~~~~~~~
						// ������ VknExtDebugUtils
						// ~~~~~~~~~~~~~~~~

						auto _vkn_ext_debug_utils = new Extensions::VknExtDebugUtils();

						//if (_vkn_ext_debug_utils->create(_vkn_ext_debug_utils_create_info) == false) {

						//	Tools::addEntry(m_create_info.log, Tools::LogEntryError("VknExtDebugUtils->create() = false"));

						//	return false;
						//}

						m_vkn_ext_debug_utils = std::move(_vkn_ext_debug_utils);
					}

					// ~~~~~~~~~~~~~~~~
					// ������
					// Build::s_vk_ext_debug_utils
					// �����
					// ~~~~~~~~~~~~~~~~

					return true;
				}
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				VknStatus GpuVknLibrary::createVknQueueFamily(void) noexcept {

					VknStatus _status;

					uint32_t _count = 0;

					// --- ��� 1: �������� ���������� ��������� �������� �������� ---
					// ������ ����� ������� � ������ �����, ������� �������� �������� ������������ ���������� ����������.
					//m_create_info.vkn_commands->vkGetPhysicalDeviceQueueFamilyProperties(m_vk_physical_device, &_count, nullptr);

					// --- ��� 2: �������� ������ ��� �������� ���� �������� �������� ---
					// ������ ������ ������� �������, � ������� Vulkan ������� ������ � ������ ���������.
					VkQueueFamilyPropertiesArr1 _vk_queue_family_properties_collection(_count);

					// --- ��� 3: �������� �������� ���� �������� �������� ---
					// ������ ����� ��� �� �������, ������ � ���������� ������� ��� ������ ������.
					//m_create_info.vkn_commands->vkGetPhysicalDeviceQueueFamilyProperties(
					//	m_vk_physical_device,
					//	&_count,
					//	_vk_queue_family_properties_collection.data()
					//);

					// --- ��� 4: ������ ���������� ������� VknQueueFamily �� ������ ���������� ������� ---
					// �������� �� ������� ���������, ������ ������ � ��������� ��� � ���������.
					for (const auto& it : _vk_queue_family_properties_collection) {

						// ������ ���������, ������� ����� �������� � ����� create()
						//VknQueueFamilyCreateInfo _vkn_queue_family_create_info;
						//_vkn_queue_family_create_info.vk_queue_family_properties = it;

						// ������ ��������� VknQueueFamily
						//VknQueueFamily _vkn_queue_family;

						// �������� ����� create() ��� ������������� ������� �� ������ �������
						//_status = _vkn_queue_family.create(_vkn_queue_family_create_info);

						// ���� ������������� �� ������� � ���������� ������
						//if (_status.m_code != VknStatusCode::SUCCESSFUL) {
						//	_status.message_old += "\n\tVknQueueFamily::create() - fail.";
						//	return _status;
						//}

						// ��������� ������ � ��������� (������������, ��� �����������)
						//m_vkn_queue_family_collection.emplace_back(std::move(_vkn_queue_family));
					}

					// --- ��� 5: �������� ���������� ---
					_status.m_code = VknStatusCode::SUCCESSFUL;
					return _status;
				}
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			} // namespace Vulkan

		} // namespace Private

	} // namespace GPU

} // namespace CGDev