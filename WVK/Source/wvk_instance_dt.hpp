#ifndef CGDEV_WVK_SOURCE__WVK_INSTANCE_DT_HPP
#define CGDEV_WVK_SOURCE__WVK_INSTANCE_DT_HPP
////////////////////////////////////////////////////////////////
// ������ �������-����������
////////////////////////////////////////////////////////////////
#include "_wvk.h"
////////////////////////////////////////////////////////////////
// ������ �������������
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// ������ ������������� ������
////////////////////////////////////////////////////////////////
#include "wvk_base_object.h"
////////////////////////////////////////////////////////////////
// ������ ��� ����������
////////////////////////////////////////////////////////////////
#include "wvk_loader.h"
#include "wvk_instance.h"
#include "Extensions/wvk_khr_get_physical_device_properties2_dt.hpp"

namespace CGDev {

	namespace wvk {

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		/*!	\brief
		*/
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		struct WvkInstanceDtCreateInfo {
			WvkInstancePtr wvk_instance = nullptr;
			WvkLoaderPtr wvk_loader = nullptr;
		}; // class WvkInstanceDtCreateInfo

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		/*!	\brief
		*/
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		class WvkInstanceDt : public GpuObject {

		public:

			inline VkResult wvkEnumeratePhysicalDevices(uint32_t* pPhysicalDeviceCount, VkPhysicalDevice* pPhysicalDevices)const noexcept {
				return m_create_info.wvk_instance->invokeWithVkInstanceFunction(m_vkEnumeratePhysicalDevices, pPhysicalDeviceCount, pPhysicalDevices); }

			// ~~~~~~~~~~~~~~~~
			// 1.1 or VK_KHR_device_group_creation
			// ~~~~~~~~~~~~~~~~
			inline VkResult wvkEnumeratePhysicalDeviceGroups(uint32_t* pPhysicalDeviceGroupCount, VkPhysicalDeviceGroupProperties* pPhysicalDeviceGroupProperties) const noexcept {
				return m_create_info.wvk_instance->invokeWithVkInstanceFunction(m_vkEnumeratePhysicalDeviceGroups, pPhysicalDeviceGroupCount, pPhysicalDeviceGroupProperties); }

			// ~~~~~~~~~~~~~~~~
			// 1.0
			// ~~~~~~~~~~~~~~~~
			inline void wvkGetPhysicalDeviceFeatures(VkPhysicalDevice physicalDevice, VkPhysicalDeviceFeatures* pFeatures) const noexcept {
				m_vkGetPhysicalDeviceFeatures(physicalDevice, pFeatures); }
			inline void wvkGetPhysicalDeviceProperties(VkPhysicalDevice physicalDevice, VkPhysicalDeviceProperties* pProperties) const noexcept {
				m_vkGetPhysicalDeviceProperties(physicalDevice, pProperties); }
			inline void wvkGetPhysicalDeviceFormatProperties(VkPhysicalDevice physicalDevice, VkFormat format, VkFormatProperties* pFormatProperties) const noexcept {
				m_vkGetPhysicalDeviceFormatProperties(physicalDevice, format, pFormatProperties); }
			inline VkResult wvkGetPhysicalDeviceImageFormatProperties(VkPhysicalDevice physicalDevice, VkFormat format, VkImageType type, VkImageTiling tiling, VkImageUsageFlags usage, VkImageCreateFlags flags, VkImageFormatProperties* pImageFormatProperties) const noexcept {
				m_vkGetPhysicalDeviceImageFormatProperties(physicalDevice, format, type, tiling, usage, flags, pImageFormatProperties); }
			inline void wvkGetPhysicalDeviceQueueFamilyProperties(VkPhysicalDevice physicalDevice, uint32_t* pQueueFamilyPropertyCount, VkQueueFamilyProperties* pQueueFamilyProperties) const noexcept {
				m_vkGetPhysicalDeviceQueueFamilyProperties(physicalDevice, pQueueFamilyPropertyCount, pQueueFamilyProperties); }
			inline void wvkGetPhysicalDeviceMemoryProperties(VkPhysicalDevice physicalDevice, VkPhysicalDeviceMemoryProperties* pMemoryProperties) const noexcept {
				m_vkGetPhysicalDeviceMemoryProperties(physicalDevice, pMemoryProperties); }
			inline void wvkGetPhysicalDeviceSparseImageFormatProperties(VkPhysicalDevice physicalDevice, VkFormat format, VkImageType type, VkSampleCountFlagBits samples, VkImageUsageFlags usage, VkImageTiling tiling, uint32_t* pPropertyCount, VkSparseImageFormatProperties* pProperties) const noexcept {
				m_vkGetPhysicalDeviceSparseImageFormatProperties(physicalDevice, format, type, samples, usage, tiling, pPropertyCount, pProperties); }

			// ~~~~~~~~~~~~~~~~
			// 1.1 or VK_KHR_get_physical_device_properties2
			// ~~~~~~~~~~~~~~~~
			inline void wvkGetPhysicalDeviceFeatures2(VkPhysicalDevice physicalDevice, VkPhysicalDeviceFeatures2* pFeatures) const noexcept {
				m_vkGetPhysicalDeviceFeatures2(physicalDevice, pFeatures); }
			inline void wvkGetPhysicalDeviceProperties2(VkPhysicalDevice physicalDevice, VkPhysicalDeviceProperties2* pProperties) const noexcept {
				m_vkGetPhysicalDeviceProperties2(physicalDevice, pProperties); }
			inline void wvkGetPhysicalDeviceFormatProperties2(VkPhysicalDevice physicalDevice, VkFormat format, VkFormatProperties2* pFormatProperties) const noexcept {
				m_vkGetPhysicalDeviceFormatProperties2(physicalDevice, format, pFormatProperties); }
			inline VkResult wvkGetPhysicalDeviceImageFormatProperties2(VkPhysicalDevice physicalDevice, const VkPhysicalDeviceImageFormatInfo2* pImageFormatInfo, VkImageFormatProperties2* pImageFormatProperties) const noexcept {
				m_vkGetPhysicalDeviceImageFormatProperties2(physicalDevice, pImageFormatInfo, pImageFormatProperties); }
			inline void wvkGetPhysicalDeviceQueueFamilyProperties2(VkPhysicalDevice physicalDevice, uint32_t* pQueueFamilyPropertyCount, VkQueueFamilyProperties2* pQueueFamilyProperties) const noexcept {
				m_vkGetPhysicalDeviceQueueFamilyProperties2(physicalDevice, pQueueFamilyPropertyCount, pQueueFamilyProperties); }
			inline void wvkGetPhysicalDeviceMemoryProperties2(VkPhysicalDevice physicalDevice, VkPhysicalDeviceMemoryProperties2* pMemoryProperties) const noexcept {
				m_vkGetPhysicalDeviceMemoryProperties2(physicalDevice, pMemoryProperties); }
			inline void wvkGetPhysicalDeviceSparseImageFormatProperties2(VkPhysicalDevice physicalDevice, const VkPhysicalDeviceSparseImageFormatInfo2* pFormatInfo, uint32_t* pPropertyCount, VkSparseImageFormatProperties2* pProperties) const noexcept {
				m_vkGetPhysicalDeviceSparseImageFormatProperties2(physicalDevice, pFormatInfo, pPropertyCount, pProperties); }

			// ~~~~~~~~~~~~~~~~
			// WvkSurface 1.1
			// ~~~~~~~~~~~~~~~~
			inline VkResult wvkGetPhysicalDeviceSurfaceCapabilities2KHR(VkPhysicalDevice physicalDevice, const VkPhysicalDeviceSurfaceInfo2KHR* pSurfaceInfo, VkSurfaceCapabilities2KHR* pSurfaceCapabilities) const noexcept {
				return m_vkGetPhysicalDeviceSurfaceCapabilities2KHR(physicalDevice, pSurfaceInfo, pSurfaceCapabilities); }


		private:

			// ~~~~~~~~~~~~~~~~
			// Vulkan 1.0
			// ~~~~~~~~~~~~~~~~
			PFN_vkDestroyInstance m_vkDestroyInstance = VK_NULL_HANDLE;
			PFN_vkEnumeratePhysicalDevices m_vkEnumeratePhysicalDevices = VK_NULL_HANDLE;

			PFN_vkCreateDevice m_vkCreateDevice = VK_NULL_HANDLE;
			PFN_vkEnumerateDeviceExtensionProperties m_vkEnumerateDeviceExtensionProperties = VK_NULL_HANDLE;
			PFN_vkEnumerateDeviceLayerProperties m_vkEnumerateDeviceLayerProperties = VK_NULL_HANDLE;
			PFN_vkGetDeviceProcAddr	m_vkGetDeviceProcAddr = VK_NULL_HANDLE;

			// ~~~~~~~~~~~~~~~~
			// 1.1 or VK_KHR_device_group_creation
			// ~~~~~~~~~~~~~~~~
			PFN_vkEnumeratePhysicalDeviceGroups m_vkEnumeratePhysicalDeviceGroups = VK_NULL_HANDLE;

			// ~~~~~~~~~~~~~~~~
			// 1.0
			// ~~~~~~~~~~~~~~~~
			PFN_vkGetPhysicalDeviceFeatures m_vkGetPhysicalDeviceFeatures = VK_NULL_HANDLE;			
			PFN_vkGetPhysicalDeviceProperties m_vkGetPhysicalDeviceProperties = VK_NULL_HANDLE;
			PFN_vkGetPhysicalDeviceFormatProperties m_vkGetPhysicalDeviceFormatProperties = VK_NULL_HANDLE;
			PFN_vkGetPhysicalDeviceImageFormatProperties m_vkGetPhysicalDeviceImageFormatProperties = VK_NULL_HANDLE;
			PFN_vkGetPhysicalDeviceQueueFamilyProperties m_vkGetPhysicalDeviceQueueFamilyProperties = VK_NULL_HANDLE;
			PFN_vkGetPhysicalDeviceMemoryProperties m_vkGetPhysicalDeviceMemoryProperties = VK_NULL_HANDLE;
			PFN_vkGetPhysicalDeviceSparseImageFormatProperties m_vkGetPhysicalDeviceSparseImageFormatProperties = VK_NULL_HANDLE;		

			// ~~~~~~~~~~~~~~~~
			// 1.1 or VK_KHR_get_physical_device_properties2
			// ~~~~~~~~~~~~~~~~
			PFN_vkGetPhysicalDeviceFeatures2 m_vkGetPhysicalDeviceFeatures2 = VK_NULL_HANDLE;
			PFN_vkGetPhysicalDeviceProperties2 m_vkGetPhysicalDeviceProperties2 = VK_NULL_HANDLE;
			PFN_vkGetPhysicalDeviceFormatProperties2 m_vkGetPhysicalDeviceFormatProperties2 = VK_NULL_HANDLE;
			PFN_vkGetPhysicalDeviceImageFormatProperties2 m_vkGetPhysicalDeviceImageFormatProperties2 = VK_NULL_HANDLE;
			PFN_vkGetPhysicalDeviceQueueFamilyProperties2 m_vkGetPhysicalDeviceQueueFamilyProperties2 = VK_NULL_HANDLE;
			PFN_vkGetPhysicalDeviceMemoryProperties2 m_vkGetPhysicalDeviceMemoryProperties2 = VK_NULL_HANDLE;
			PFN_vkGetPhysicalDeviceSparseImageFormatProperties2 m_vkGetPhysicalDeviceSparseImageFormatProperties2 = VK_NULL_HANDLE;

			// ~~~~~~~~~~~~~~~~
			// 1.1 or VK_KHR_get_surface_capabilities2
			// ~~~~~~~~~~~~~~~~
			PFN_vkGetPhysicalDeviceSurfaceCapabilities2KHR m_vkGetPhysicalDeviceSurfaceCapabilities2KHR = VK_NULL_HANDLE;

		public:

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			WvkInstanceDt(void) noexcept {}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			~WvkInstanceDt(void) noexcept {}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			* �������������� ������� ������� Vulkan-������� ��� ���������� `VkInstance`.
			*
			* ����� ��������� ���������� ��������� `WvkInstanceDispatchTableCreateInfo`, ��������� � �� ������������ (���� �������� ���������),
			* � ����� ��������� ��������� �� ����������� Vulkan-�������. ��������� ����� ������� � ������ `ALREADY_INITIALIZED`.
			*
			* @param[in] create_info
			*  ��������� � ����������� � �������������, ������������ ��� �������� ������� �������:
			*  - `wvk_loader` � ������ �������� �������;
			*  - `wvk_instance` � ������ ��� `VkInstance`.
			*
			* @return
			*  ���������� ������ `WvkStatus`:
			*  - `OK`, ���� ��� ���� ��������� �������;
			*  - `FAIL`, ���� ��������� ������ �������� ������� ��� ���������;
			*  - `ALREADY_INITIALIZED`, ���� ����� ��� ��� ������.
			*
			* @code
			* WvkInstanceDtCreateInfo info = { ... };
			* WvkInstanceDt dispatch_table;
			* WvkStatus status = dispatch_table.create(info);
			* if (!status) {
			*     std::cerr << status.what() << std::endl;
			*     return false;
			* }
			* @endcode
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			WvkStatus create(const WvkInstanceDtCreateInfo& create_info) noexcept {
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
				// ��� 3. ��������� ������� ���������� (���� ��������)
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				if constexpr (Build::ValidationBuildInfo::enable) {
					_status = validationCreateInfo();
					if (!_status) {
						reset(); // ������� ���������� ���������
						return _status.set(VknStatusCode::FAIL, "\n\tWvkInstanceDispatchTable::validationCreateInfo - fail.");
					}
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
			/*!	\
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			void destroy(void) noexcept {
				reset();
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			inline void setUp(Extensions::WvkKhrGetPhysicalDeviceProperties2DTPtr dt) {
				std::vector<WvkVulkanProcedureInfo> _procedures;

				auto maybe_add = [&](const char* name, void** ptr) {
					if (*ptr == nullptr) {
						_procedures.emplace_back(name, ptr);
					}
				};

				maybe_add("vkGetPhysicalDeviceFeatures2", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceFeatures2));
				maybe_add("vkGetPhysicalDeviceProperties2", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceProperties2));
				maybe_add("vkGetPhysicalDeviceFormatProperties2", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceFormatProperties2));
				maybe_add("vkGetPhysicalDeviceImageFormatProperties2", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceImageFormatProperties2));
				maybe_add("vkGetPhysicalDeviceQueueFamilyProperties2", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceQueueFamilyProperties2));
				maybe_add("vkGetPhysicalDeviceMemoryProperties2", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceMemoryProperties2));
				maybe_add("vkGetPhysicalDeviceSparseImageFormatProperties2", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceSparseImageFormatProperties2));

				dt->loadInto(_procedures);
			}

		private:

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief ��������� ��������� ����� ��������� `WvkInstanceDispatchTableCreateInfo`,
			* ����� ���������, ��� ���������� ��������� �� `WvkLoader` � `WvkInstance` �� ����� `nullptr`.
			*
			* ����� ��������� ����������� ����������� � ������������� ���������� ������ � ���������������������
			* �����������, ������� ����� �������� � �������������� ���������.
			*
			* @return
			*  ���������� ������ WvkStatus � �����:
			*  - `OK`, ���� ��� ����������� ���� ��������� ���������������� ���������;
			*  - `FAIL`, ���� ���� �� ���� ���� (loader ��� instance) �� ������.
			*
			* @code
			* WvkInstanceDt dispatch_table;
			* WvkStatus status = dispatch_table.validationCreateInfo();
			* if (!status) {
			*     std::cerr << status.what() << std::endl;
			*     return false;
			* }
			* @endcode
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			WvkStatus validationCreateInfo(void) const noexcept {
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
			/*!	\brief ��������� ��������� �� ��� ����������� ���������� � ���������-��������� ������� Vulkan
			* � �������������� ����������� ���������� `WvkLoader` � ������� `WvkInstance`.
			*
			* ����� ��������� ������ �������� Vulkan, ������� ���������� ��������� (������� �������
			* Vulkan 1.0, 1.1 � 1.2), � ������� ��� � `WvkLoader::loadProcedure`, ���������
			* ���������� ����� ����� `invokeWithVkInstance`, ����� �� ���������� `VkInstance`.
			*
			* @return
			*  ���������� ������ WvkStatus � �����:
			*  - `OK`, ���� ��� ��������� ���� ������� ���������;
			*  - `FAIL`, ���� �������� ���� �� ����� ��������� ����������� ��������.
			*
			* @code
			* WvkInstanceDt table;
			* WvkStatus status = table.loadProcedure();
			* if (!status) {
			*     std::cerr << status.what() << std::endl;
			* }
			* @endcode
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			WvkStatus loadProcedure(void) noexcept {
				WvkStatus _status;

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// ��� 1. ��������� ������ �������� Vulkan, ������� ����� ���������
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				std::vector<WvkVulkanProcedureInfo> _procedures = {
					// ~~~~~~~~~~~~~~~~
					// 1.0
					// ~~~~~~~~~~~~~~~~
					{ "vkDestroyInstance", reinterpret_cast<void**>(&m_vkDestroyInstance) },
					{ "vkEnumeratePhysicalDevices", reinterpret_cast<void**>(&m_vkEnumeratePhysicalDevices) },
					{ "vkCreateDevice", reinterpret_cast<void**>(&m_vkCreateDevice) },
					{ "vkEnumerateDeviceExtensionProperties", reinterpret_cast<void**>(&m_vkEnumerateDeviceExtensionProperties) },
					{ "vkEnumerateDeviceLayerProperties", reinterpret_cast<void**>(&m_vkEnumerateDeviceLayerProperties) },
					{ "vkGetDeviceProcAddr", reinterpret_cast<void**>(&m_vkGetDeviceProcAddr) },

					// ~~~~~~~~~~~~~~~~
					// 1.1 or VK_KHR_device_group_creation
					// ~~~~~~~~~~~~~~~~
					{ "vkEnumeratePhysicalDeviceGroups", reinterpret_cast<void**>(&m_vkEnumeratePhysicalDeviceGroups) },

					// ~~~~~~~~~~~~~~~~
					// 1.0
					// ~~~~~~~~~~~~~~~~
					{ "vkGetPhysicalDeviceFeatures", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceFeatures) },
					{ "vkGetPhysicalDeviceProperties", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceProperties) },
					{ "vkGetPhysicalDeviceFormatProperties", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceFormatProperties) },
					{ "vkGetPhysicalDeviceImageFormatProperties", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceImageFormatProperties) },
					{ "vkGetPhysicalDeviceMemoryProperties", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceMemoryProperties) },
					{ "vkGetPhysicalDeviceQueueFamilyProperties", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceQueueFamilyProperties) },
					{ "vkGetPhysicalDeviceSparseImageFormatProperties", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceSparseImageFormatProperties) },

					// ~~~~~~~~~~~~~~~~
					// 1.1 or VK_KHR_get_physical_device_properties2
					// ~~~~~~~~~~~~~~~~
					{ "vkGetPhysicalDeviceFeatures2", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceFeatures2) },
					{ "vkGetPhysicalDeviceProperties2", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceProperties2) },
					{ "vkGetPhysicalDeviceFormatProperties2", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceFormatProperties2) },
					{ "vkGetPhysicalDeviceImageFormatProperties2", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceImageFormatProperties2) },
					{ "vkGetPhysicalDeviceMemoryProperties2", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceMemoryProperties2) },
					{ "vkGetPhysicalDeviceQueueFamilyProperties2", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceQueueFamilyProperties2) },
					{ "vkGetPhysicalDeviceSparseImageFormatProperties2", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceSparseImageFormatProperties2) },
					
					// ~~~~~~~~~~~~~~~~
					// VkSurface 1.1
					// ~~~~~~~~~~~~~~~~
					{ "vkGetPhysicalDeviceSurfaceCapabilities2KHR", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceSurfaceCapabilities2KHR) }
				};

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// ��� 2. ��������� ��������� ����� WvkLoader � ��������� VkInstance
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				_status = m_create_info.wvk_instance->invokeWithVkInstanceMethod(
					&WvkLoader::loadProcedure,          // ��������� �� �����, ������� ��������� ���������
					m_create_info.wvk_loader,           // ������ WvkLoader
					_procedures                         // ������ ��������
				);

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

			void reset(void) noexcept {
				// ~~~~~~~~~~~~~~~~
				// Vulkan 1.0
				// ~~~~~~~~~~~~~~~~
				m_vkDestroyInstance = VK_NULL_HANDLE;
				m_vkEnumeratePhysicalDevices = VK_NULL_HANDLE;
				m_vkCreateDevice = VK_NULL_HANDLE;
				m_vkEnumerateDeviceExtensionProperties = VK_NULL_HANDLE;
				m_vkEnumerateDeviceLayerProperties = VK_NULL_HANDLE;
				m_vkGetDeviceProcAddr = VK_NULL_HANDLE;

				// ~~~~~~~~~~~~~~~~
				// 1.1 or VK_KHR_device_group_creation
				// ~~~~~~~~~~~~~~~~
				m_vkEnumeratePhysicalDeviceGroups = VK_NULL_HANDLE;

				// ~~~~~~~~~~~~~~~~
				// 1.0
				// ~~~~~~~~~~~~~~~~
				m_vkGetPhysicalDeviceFeatures = VK_NULL_HANDLE;
				m_vkGetPhysicalDeviceProperties = VK_NULL_HANDLE;
				m_vkGetPhysicalDeviceFormatProperties = VK_NULL_HANDLE;
				m_vkGetPhysicalDeviceImageFormatProperties = VK_NULL_HANDLE;
				m_vkGetPhysicalDeviceQueueFamilyProperties = VK_NULL_HANDLE;
				m_vkGetPhysicalDeviceMemoryProperties = VK_NULL_HANDLE;
				m_vkGetPhysicalDeviceSparseImageFormatProperties = VK_NULL_HANDLE;

				// ~~~~~~~~~~~~~~~~
				// 1.1 or VK_KHR_get_physical_device_properties2
				// ~~~~~~~~~~~~~~~~
				m_vkGetPhysicalDeviceFeatures2 = VK_NULL_HANDLE;
				m_vkGetPhysicalDeviceProperties2 = VK_NULL_HANDLE;
				m_vkGetPhysicalDeviceFormatProperties2 = VK_NULL_HANDLE;
				m_vkGetPhysicalDeviceImageFormatProperties2 = VK_NULL_HANDLE;
				m_vkGetPhysicalDeviceMemoryProperties2 = VK_NULL_HANDLE;
				m_vkGetPhysicalDeviceQueueFamilyProperties2 = VK_NULL_HANDLE;
				m_vkGetPhysicalDeviceSparseImageFormatProperties2 = VK_NULL_HANDLE;

				m_create_info = {};

				m_valid = false;
			}
			
		private:

			WvkInstanceDtCreateInfo m_create_info;
		}; // class WvkInstanceDt

	} // namespace wvk

} // namespace CGDev

#endif // CGDEV_WVK_SOURCE__WVK_INSTANCE_DT_HPP
