////////////////////////////////////////////////////////////////
// ������ �������-����������
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// ������ ������������� �����
////////////////////////////////////////////////////////////////
#include "wvk_instance_dispatch_table.h"
////////////////////////////////////////////////////////////////
// ������ �������������
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// ������ ��� ����������
////////////////////////////////////////////////////////////////
#include "wvk_loader.h"

namespace CGDev {

	namespace wvk {

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		WvkInstanceDispatchTable::WvkInstanceDispatchTable(void) noexcept {}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		WvkInstanceDispatchTable::~WvkInstanceDispatchTable(void) noexcept {}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		WvkStatus WvkInstanceDispatchTable::create(const WvkInstanceDtCreateInfo& create_info) noexcept {
			WvkStatus _status;

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// ��� 1. �������� �� ��������� �������������
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			if (m_valid) {
				return _status.set(VknStatusCode::ALREADY_INITIALIZED);
			}

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// ��� 2. ��������� ��������� ��������
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			m_create_info = create_info;

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// ��� 3. ��������� ������� ����������
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			_status = validationCreateInfo();
			if (!_status) {
				reset(); // ������� ���������� ���������
				return _status.set(VknStatusCode::FAIL, "\n\tWvkInstanceDispatchTable::validationCreateInfo - fail.");
			}

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// ��� 4. �������� ���������� �� Vulkan-�������
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			_status = loadProcedure();
			if (!_status) {
				reset(); // ������� ���������� ���������
				return _status.set(VknStatusCode::FAIL, "\n\tWvkInstanceDispatchTable::loadProcedure - fail.");
			}

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// ��� 5. ������������� ���� �������� �������������
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			m_valid = true;

			return _status.setOk();
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		void WvkInstanceDispatchTable::destroy(void) noexcept {
			reset();
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		WvkStatus WvkInstanceDispatchTable::validationCreateInfo(void) const noexcept {
			WvkStatus _status;

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// ��� 1. �������� ��������� �� WvkLoader
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			if (m_create_info.wvk_loader == nullptr) {
				return _status.set(VknStatusCode::FAIL, "\n\tWvkInstanceDispatchTableCreateInfo::wvk_loader - nullptr.");
			}

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// ��� 2. �������� ��������� �� WvkInstance
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			else if (m_create_info.wvk_instance == nullptr) {
				return _status.set(VknStatusCode::FAIL, "\n\tWvkInstanceDispatchTableCreateInfo::wvk_instance - nullptr.");
			}

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// ��� 3. ��� ���� ������� � ���������� OK
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			return _status.setOk();
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		WvkStatus WvkInstanceDispatchTable::loadProcedure(void) noexcept {
			WvkStatus _status;

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// ��� 1. ��������� ������ �������� Vulkan, ������� ����� ���������
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			std::vector<WvkVulkanProcedureInfo> _procedures = {

				// ~~~~~~~~~~~~~~~~
				// 1.0
				// ~~~~~~~~~~~~~~~~

				{ "vkDestroyInstance", reinterpret_cast<void**>(&m_vkDestroyInstance) },
				{ "vkCreateDevice", reinterpret_cast<void**>(&m_vkDestroyInstance) },
				{ "vkEnumerateDeviceLayerProperties", reinterpret_cast<void**>(&m_vkDestroyInstance) },
				{ "vkEnumerateDeviceExtensionProperties", reinterpret_cast<void**>(&m_vkDestroyInstance) },
				{ "vkGetDeviceProcAddr", reinterpret_cast<void**>(&m_vkDestroyInstance) },

				// =======================================
				// [Category]: Physical Device
				// =======================================

				// ~~~~~~~~~~~~~~~~
				// [Version] 1.0
				// ~~~~~~~~~~~~~~~~	
				{ "vkEnumeratePhysicalDevices", reinterpret_cast<void**>(&m_vkEnumeratePhysicalDevices) },
				{ "vkGetPhysicalDeviceFeatures", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceFeatures) },
				{ "vkGetPhysicalDeviceProperties", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceProperties) },
				{ "vkGetPhysicalDeviceFormatProperties", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceFormatProperties) },
				{ "vkGetPhysicalDeviceImageFormatProperties", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceImageFormatProperties) },
				{ "vkGetPhysicalDeviceMemoryProperties", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceMemoryProperties) },
				{ "vkGetPhysicalDeviceQueueFamilyProperties", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceQueueFamilyProperties) },
				{ "vkGetPhysicalDeviceSparseImageFormatProperties", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceSparseImageFormatProperties) },

				// ~~~~~~~~~~~~~~~~
				// [Version] 1.1 / WVK_KHR_device_group_creation
				// ~~~~~~~~~~~~~~~~
#if VULKAN_API_VERSION >= VULKAN_API_VERSION_11
				{ "vkEnumeratePhysicalDeviceGroups", reinterpret_cast<void**>(&m_vkEnumeratePhysicalDeviceGroups) },
#endif
#if VULKAN_API_VERSION == VULKAN_API_VERSION_10 && WVK_KHR_device_group_creation == WVK_EXTENSION_ENABLE
				{ "vkEnumeratePhysicalDeviceGroupsKHR", reinterpret_cast<void**>(&m_vkEnumeratePhysicalDeviceGroupsKHR) },
#endif 

				// ~~~~~~~~~~~~~~~~
				// [Version] 1.1 / VK_KHR_get_physical_device_properties2
				// ~~~~~~~~~~~~~~~~
#if VULKAN_API_VERSION >= VULKAN_API_VERSION_11
				{ "vkGetPhysicalDeviceFeatures2", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceFeatures2) },
				{ "vkGetPhysicalDeviceProperties2", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceProperties2) },
				{ "vkGetPhysicalDeviceFormatProperties2", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceFormatProperties2) },
				{ "vkGetPhysicalDeviceImageFormatProperties2", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceImageFormatProperties2) },
				{ "vkGetPhysicalDeviceMemoryProperties2", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceMemoryProperties2) },
				{ "vkGetPhysicalDeviceQueueFamilyProperties2", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceQueueFamilyProperties2) },
				{ "vkGetPhysicalDeviceSparseImageFormatProperties2", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceSparseImageFormatProperties2) },
#endif
#if VULKAN_API_VERSION == VULKAN_API_VERSION_10 && WVK_KHR_get_physical_device_properties2 == WVK_EXTENSION_ENABLE
				{ "vkGetPhysicalDeviceFeatures2KHR", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceFeatures2KHR) },
				{ "vkGetPhysicalDeviceProperties2KHR", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceProperties2KHR) },
				{ "vkGetPhysicalDeviceFormatProperties2KHR", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceFormatProperties2KHR) },
				{ "vkGetPhysicalDeviceImageFormatProperties2KHR", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceImageFormatProperties2KHR) },
				{ "vkGetPhysicalDeviceMemoryProperties2KHR", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceMemoryProperties2KHR) },
				{ "vkGetPhysicalDeviceQueueFamilyProperties2KHR", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceQueueFamilyProperties2KHR) },
				{ "vkGetPhysicalDeviceSparseImageFormatProperties2KHR", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceSparseImageFormatProperties2KHR) },
#endif 

				// ~~~~~~~~~~~~~~~~
				// [Version] 1.1 / VK_KHR_get_surface_capabilities2
				// ~~~~~~~~~~~~~~~~
#if VULKAN_API_VERSION >= VULKAN_API_VERSION_11
				{ "vkGetPhysicalDeviceSurfaceCapabilities2KHR", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceSurfaceCapabilities2KHR) },
				{ "vkGetPhysicalDeviceSurfaceFormats2KHR", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceSurfaceFormats2KHR) },
#endif
#if VULKAN_API_VERSION == VULKAN_API_VERSION_10 && WVK_KHR_get_surface_capabilities2 == WVK_EXTENSION_ENABLE
				{ "vkGetPhysicalDeviceSurfaceCapabilities2KHR", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceSurfaceCapabilities2KHR) },
				{ "vkGetPhysicalDeviceSurfaceFormats2KHR", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceSurfaceFormats2KHR) },
#endif

				// =======================================
				// [Category]: Logical Device
				// =======================================

				// ~~~~~~~~~~~~~~~~
				// [Version] 1.0
				// ~~~~~~~~~~~~~~~~

				{ "vkGetDeviceProcAddr", reinterpret_cast<void**>(&m_vkGetDeviceProcAddr) },
				{ "vkCreateDevice", reinterpret_cast<void**>(&m_vkCreateDevice) },

				// =======================================
				// [Category]: Debug
				// =======================================

				// ~~~~~~~~~~~~~~~~
				// [Extension] VK_EXT_debug_utils
				// ~~~~~~~~~~~~~~~~
#if WVK_EXT_debug_utils == WVK_EXTENSION_ENABLE
				{ "vkCreateDebugUtilsMessengerEXT", reinterpret_cast<void**>(&m_vkCreateDebugUtilsMessengerEXT) },
				{ "vkDestroyDebugUtilsMessengerEXT", reinterpret_cast<void**>(&m_vkDestroyDebugUtilsMessengerEXT) },
				{ "vkSubmitDebugUtilsMessageEXT", reinterpret_cast<void**>(&m_vkSubmitDebugUtilsMessageEXT) }
#endif
			};

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// ��� 2. ����� ������ �������� �������� ����� ��������� ���������
			//        ����� ��������������, ��� wvk_loader ���������� vkGetInstanceProcAddr(VkInstance, ...)
			_status = m_create_info.wvk_loader->loadProcedure(m_create_info.wvk_instance->getVkInstance(), _procedures);

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// ��� 3. �������� ������� ����� ��������
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			if (!_status) {
				return _status.set(VknStatusCode::FAIL, "\n\tWvkLoader::loadProcedure - fail.");
			}

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// ��� 4. �������� ����������
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			return _status.setOk();
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		void WvkInstanceDispatchTable::reset(void) noexcept {
			WvkDispatchTable::reset();
		
			m_create_info = {};

			m_valid = false;
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

	} // namespace wvk

} // namespace CGDev