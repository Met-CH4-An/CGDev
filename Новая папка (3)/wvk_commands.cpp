////////////////////////////////////////////////////////////////
// ������ �������-����������
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// ������ ������������� �����
////////////////////////////////////////////////////////////////
#include "wvk_commands.h"
////////////////////////////////////////////////////////////////
// ������ �������������
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// ������ ��� ����������
////////////////////////////////////////////////////////////////
#include "wvk_loader.h"
#include "wvk_instance.h"

namespace CGDev {

	//namespace GPU {

		//namespace Private {

			namespace wvk {

				WvkStatus WvkGlobalCommands::tryLoadFunction(std::vector<VknVulkanFunctionInfo> wvk_vulkan_procedure_collection1) const noexcept {
					WvkStatus _status;

					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					// ��� 1. �������� �� ������ ������ � ������ �������
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					for (const auto& it_0 : wvk_vulkan_procedure_collection1) {
						*it_0.targetPtr = vknGetInstanceProcAddr(it_0.name);

						// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						// ��� 2. ���������, ������� �� ��������� �������
						// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						if (*it_0.targetPtr == nullptr) {
							_status.m_code = VknStatusCode::FAIL;
							_status.append("\n\t%s - not found.", it_0.name);
							return _status;
						}
					}

					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					// ��� 3. ��� ������� ������� ���������
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					_status.m_code = VknStatusCode::SUCCESSFUL;
					return _status;
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				WvkGlobalCommands::WvkGlobalCommands(void) noexcept {
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				WvkGlobalCommands::~WvkGlobalCommands(void) noexcept {
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				WvkStatus WvkGlobalCommands::create(const WvkCommandsCreateInfo* create_info) noexcept {
					WvkStatus _status;

					// ~~~~~~~~~~~~~~~~
					// ������
					// Build::s_wvk_debug == true
					// ������
					// ~~~~~~~~~~~~~~~~
					// ��������� ���������� VknCommandsCreateInfo
					// ~~~~~~~~~~~~~~~~

					if constexpr (Build::s_wvk_debug == true) {
						_status = validationCreateInfo(create_info);
						
						if (!_status) {

							return _status.set(VknStatusCode::FAIL, "");
						}
					}

					// ~~~~~~~~~~~~~~~~
					// ������
					// Build::s_wvk_debug == true
					// �����
					// ~~~~~~~~~~~~~~~~

					// ~~~~~~~~~~~~~~~~
					// 
					// ~~~~~~~~~~~~~~~~
					
					m_create_info = *create_info;

					// ~~~~~~~~~~~~~~~~
					// ��������� '����������' �������� �������,
					// �������� ��� ������� �� ��������� �� VkInstance, �� VkDevice
					// ~~~~~~~~~~~~~~~~
					loadVulkanCommandLoaderLevel();
					//if (loadVulkanCommandLoaderLevel() == StatusCode::FAIL) {

						//Tools::addEntry(m_create_info.log, Tools::LogEntryError("loadVulkanCommandLoaderLevel() = false"));

					//	_status.set(VknStatusCode::SUCCESSFUL);
					//	return _status;
					//}

					// ~~~~~~~~~~~~~~~~
					// �������� �������� �������, ��� ������� ��������� VkInstance
					// ~~~~~~~~~~~~~~~~
					if(m_create_info.wvk_instance != nullptr) loadVulkanCommandInstanceLevel();
					//if (m_create_info.wvk_instance != nullptr && loadVulkanCommandInstanceLevel() == StatusCode::FAIL) {

						//Tools::addEntry(m_create_info.log, Tools::LogEntryError("loadVulkanCommandInstanceLevel() = false"));

					//	_status.set(VknStatusCode::SUCCESSFUL);
					//	return _status;
					//}

					return _status.setOk();

				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				void WvkGlobalCommands::destroy(void) noexcept {

					// ~~~~~~~~~~~~~~~~
					// ����������� ������������� GpuObject
					// ~~~~~~~~~~~~~~~~

					this->GpuObject::destroy();

					// ~~~~~~~~~~~~~~~~
					// ������� ������
					// ~~~~~~~~~~~~~~~~

					// ~~~~~~~~~~~~~~~~
					// VkInstance
					// 1.0
					// ~~~~~~~~~~~~~~~~

					m_vkDestroyInstance = nullptr;

					// ~~~~~~~~~~~~~~~~
					// VkPhysicalDevice
					// 1.0
					// ~~~~~~~~~~~~~~~~
					
					m_vkEnumeratePhysicalDevices = nullptr;
					m_vkGetPhysicalDeviceProperties = nullptr;
					m_vkGetPhysicalDeviceQueueFamilyProperties = nullptr;

					// ~~~~~~~~~~~~~~~~
					// VkPhysicalDevice
					// 1.1
					// ~~~~~~~~~~~~~~~~
							
					m_vkEnumeratePhysicalDeviceGroups = nullptr;
					m_vkGetPhysicalDeviceProperties2 = nullptr;
					m_vkGetPhysicalDeviceQueueFamilyProperties2 = nullptr;

					// ~~~~~~~~~~~~~~~~
					// VkDevice
					// 1.0
					// ~~~~~~~~~~~~~~~~
					
					m_vkCreateDevice = nullptr;
					m_vkDestroyDevice = nullptr;

					// ~~~~~~~~~~~~~~~~
					// vkCommandPool
					// 1.0 
					// ~~~~~~~~~~~~~~~~

					m_vkCreateCommandPool = nullptr;
					m_vkDestroyCommandPool = nullptr;
					m_vkAllocateCommandBuffers = nullptr;
					m_vkFreeCommandBuffers = nullptr;

					m_create_info = {};

				}
				
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				WvkStatus WvkGlobalCommands::validationCreateInfo(const WvkCommandsCreateInfo* create_info) const noexcept {
					WvkStatus _status;

					//if (create_info->log == nullptr) {

					//	return StatusCode::FAIL;
					//}

					if (create_info->wvk_loader == nullptr) {

						//Tools::addEntry(create_info->log, Tools::LogEntryError("WvkCommandsCreateInfo::wvk_loader = nullptr"));

						_status.set(VknStatusCode::FAIL);
						return _status;
					}

					return _status.setOk();

				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				WvkStatus WvkGlobalCommands::loadVulkanCommandLoaderLevel(void) noexcept {
					WvkStatus _status;

					// ~~~~~~~~~~~~~~~~
					// �� WvkLoader �������� �������� ������� PFN_vkGetInstanceProcAddr
					// ��������� ��� ��������, �������� ������� 4-� ������� �������, ��� ������� �� ��������� �� VkInstance, �� VkDevice
					// ~~~~~~~~~~~~~~~~

					//m_vkGetInstanceProcAddr = m_create_info.wvk_loader->getInstanceProcAddr();

					if (reinterpreted(m_vkEnumerateInstanceLayerProperties, "vkEnumerateInstanceLayerProperties", VK_NULL_HANDLE); m_vkEnumerateInstanceLayerProperties == nullptr) { return _status; }
					if (reinterpreted(m_vkEnumerateInstanceExtensionProperties, "vkEnumerateInstanceExtensionProperties", VK_NULL_HANDLE); m_vkEnumerateInstanceExtensionProperties == nullptr) { return _status; }
					if (reinterpreted(m_vkCreateInstance, "vkCreateInstance", VK_NULL_HANDLE); m_vkCreateInstance == nullptr) { return _status; }
					
					if (reinterpreted(m_vkEnumerateInstanceVersion, "vkEnumerateInstanceVersion", VK_NULL_HANDLE); m_vkEnumerateInstanceVersion == nullptr) { return _status; }

					_status.set(VknStatusCode::SUCCESSFUL);
					return _status;
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				WvkStatus WvkGlobalCommands::loadVulkanCommandInstanceLevel(void) noexcept {
					WvkStatus _status;

					// ~~~~~~~~~~~~~~~~
					// ��������� �������� �������, ��� ��������� �������� ���� ������ ��������� VkInstance
					// ~~~~~~~~~~~~~~~~
					
					// ~~~~~~~~~~~~~~~~
					// VkInstance
					// 1.0
					// ~~~~~~~~~~~~~~~~

					if (reinterpreted(m_vkDestroyInstance, "vkDestroyInstance", m_create_info.wvk_instance->getVkInstance()); m_vkDestroyInstance == nullptr) { return _status; }

					// ~~~~~~~~~~~~~~~~
					// VkPhysicalDevice
					// 1.0
					// ~~~~~~~~~~~~~~~~
				
					if (reinterpreted(m_vkEnumeratePhysicalDevices, "vkEnumeratePhysicalDevices", m_create_info.wvk_instance->getVkInstance()); m_vkEnumeratePhysicalDevices == nullptr) { return _status; }
					if (reinterpreted(m_vkGetPhysicalDeviceProperties, "vkGetPhysicalDeviceProperties", m_create_info.wvk_instance->getVkInstance()); m_vkGetPhysicalDeviceProperties == nullptr) { return _status; }
					if (reinterpreted(m_vkGetPhysicalDeviceQueueFamilyProperties, "vkGetPhysicalDeviceQueueFamilyProperties", m_create_info.wvk_instance->getVkInstance()); m_vkGetPhysicalDeviceQueueFamilyProperties == nullptr) { return _status; }

					// ~~~~~~~~~~~~~~~~
					// VkPhysicalDevice
					// 1.1
					// ~~~~~~~~~~~~~~~~
						
					if (reinterpreted(m_vkEnumeratePhysicalDeviceGroups, "vkEnumeratePhysicalDeviceGroups", m_create_info.wvk_instance->getVkInstance()); m_vkEnumeratePhysicalDeviceGroups == nullptr) { return _status; }
					if (reinterpreted(m_vkGetPhysicalDeviceProperties2, "vkGetPhysicalDeviceProperties2", m_create_info.wvk_instance->getVkInstance()); m_vkGetPhysicalDeviceProperties2 == nullptr) { return _status; }
					if (reinterpreted(m_vkGetPhysicalDeviceQueueFamilyProperties2, "vkGetPhysicalDeviceQueueFamilyProperties2", m_create_info.wvk_instance->getVkInstance()); m_vkGetPhysicalDeviceQueueFamilyProperties2 == nullptr) { return _status; }

					// ~~~~~~~~~~~~~~~~
					// VkDevice
					// 1.0
					// ~~~~~~~~~~~~~~~~

					if (reinterpreted(m_vkCreateDevice, "vkCreateDevice", m_create_info.wvk_instance->getVkInstance()); m_vkCreateDevice == nullptr) { return _status; }
					if (reinterpreted(m_vkDestroyDevice, "vkDestroyDevice", m_create_info.wvk_instance->getVkInstance()); m_vkDestroyDevice == nullptr) { return _status; }

					// ~~~~~~~~~~~~~~~~
					// vkCommandPool
					// 1.0
					// ~~~~~~~~~~~~~~~~

					if (reinterpreted(m_vkCreateCommandPool, "vkCreateCommandPool", m_create_info.wvk_instance->getVkInstance()); m_vkCreateCommandPool == nullptr) { return _status; }
					if (reinterpreted(m_vkDestroyCommandPool, "vkDestroyCommandPool", m_create_info.wvk_instance->getVkInstance()); m_vkDestroyCommandPool == nullptr) { return _status; }
					if (reinterpreted(m_vkAllocateCommandBuffers, "vkAllocateCommandBuffers", m_create_info.wvk_instance->getVkInstance()); m_vkAllocateCommandBuffers == nullptr) { return _status; }
					if (reinterpreted(m_vkFreeCommandBuffers, "vkFreeCommandBuffers", m_create_info.wvk_instance->getVkInstance()); m_vkFreeCommandBuffers == nullptr) { return _status; }

					// ~~~~~~~~~~~~~~~~
					// vkCommandBuffer
					// 1.0
					// ~~~~~~~~~~~~~~~~

					if (reinterpreted(m_vkCmdBeginRenderPass, "vkCmdBeginRenderPass", m_create_info.wvk_instance->getVkInstance()); m_vkCmdBeginRenderPass == nullptr) { return _status; }
					if (reinterpreted(m_vkCmdEndRenderPass, "vkCmdEndRenderPass", m_create_info.wvk_instance->getVkInstance()); m_vkCmdEndRenderPass == nullptr) { return _status; }

					// ~~~~~~~~~~~~~~~~
					// vkCommandBuffer
					// 1.3
					// ~~~~~~~~~~~~~~~~

					if (reinterpreted(m_vkCmdBeginRendering, "vkCmdBeginRendering", m_create_info.wvk_instance->getVkInstance()); m_vkCmdBeginRendering == nullptr) { return _status; }
					if (reinterpreted(m_vkCmdEndRendering, "vkCmdEndRendering", m_create_info.wvk_instance->getVkInstance()); m_vkCmdEndRendering == nullptr) { return _status; }

					// ~~~~~~~~~~~~~~~~
					// VkSurface. 1.1
					// ~~~~~~~~~~~~~~~~
					if (reinterpreted(m_vkGetPhysicalDeviceSurfaceCapabilities2KHR, "vkGetPhysicalDeviceSurfaceCapabilities2KHR", m_create_info.wvk_instance->getVkInstance()); m_vkGetPhysicalDeviceSurfaceCapabilities2KHR == nullptr) { return _status; }
					if (reinterpreted(m_vkGetPhysicalDeviceSurfaceFormats2KHR, "vkGetPhysicalDeviceSurfaceFormats2KHR", m_create_info.wvk_instance->getVkInstance()); m_vkGetPhysicalDeviceSurfaceFormats2KHR == nullptr) { return _status; }

					_status.set(VknStatusCode::SUCCESSFUL);
					return _status;
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			} // namespace wvk

		//} // namespace Private

	//} // namespace GPU

} // namespace CGDev