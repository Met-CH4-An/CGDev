#ifndef CGDEV_WVK_SOURCE_EXTENSIONS__WVK_KHR_SURFACE_DISPATCH_TABLE_H
#define CGDEV_WVK_SOURCE_EXTENSIONS__WVK_KHR_SURFACE_DISPATCH_TABLE_H
////////////////////////////////////////////////////////////////
// ������ �������-����������
////////////////////////////////////////////////////////////////
#include "_extensions.h"
////////////////////////////////////////////////////////////////
// ������ �������������
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// ������ ������������� ������
////////////////////////////////////////////////////////////////
#include "../wvk_base_object.h"
////////////////////////////////////////////////////////////////
// ������ ��� ����������
////////////////////////////////////////////////////////////////

namespace CGDev {

	namespace wvk {

		namespace Extensions {

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			struct WvkKhrSurfaceDispatchTableCreateInfo {
				WvkLoaderPtr wvk_loader = nullptr;
				WvkInstancePtr wvk_instance = nullptr;
			}; // WvkKhrSurfaceDispatchTableCreateInfo





			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			class WvkKhrSurfaceDispatchTable : public GpuObject {

			public:

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				inline static const std::string& s_getName(void) noexcept;

			private:

				inline static const std::string s_name = "VK_KHR_surface";

			public:
						
				inline void wvkDestroySurfaceKHR(VkSurfaceKHR surface, const VkAllocationCallbacks* pAllocator) const noexcept;
				inline VkResult wvkGetPhysicalDeviceSurfaceSupportKHR(VkPhysicalDevice physicalDevice, uint32_t queueFamilyIndex, VkSurfaceKHR surface, VkBool32* pSupported) const noexcept;
				inline VkResult wvkGetPhysicalDeviceSurfaceCapabilitiesKHR(VkPhysicalDevice physicalDevice, VkSurfaceKHR surface, VkSurfaceCapabilitiesKHR* pSurfaceCapabilities) const noexcept;
				inline VkResult wvkGetPhysicalDeviceSurfaceFormatsKHR(VkPhysicalDevice physicalDevice, VkSurfaceKHR surface, uint32_t* pSurfaceFormatCount, VkSurfaceFormatKHR* pSurfaceFormats) const noexcept;
				inline VkResult wvkGetPhysicalDeviceSurfacePresentModesKHR(VkPhysicalDevice physicalDevice, VkSurfaceKHR surface, uint32_t* pPresentModeCount, VkPresentModeKHR* pPresentModes) const noexcept;
						
			private:

				PFN_vkDestroySurfaceKHR m_vkDestroySurfaceKHR = VK_NULL_HANDLE;
				PFN_vkGetPhysicalDeviceSurfaceSupportKHR m_vkGetPhysicalDeviceSurfaceSupportKHR = VK_NULL_HANDLE;
				PFN_vkGetPhysicalDeviceSurfaceCapabilitiesKHR m_vkGetPhysicalDeviceSurfaceCapabilitiesKHR = VK_NULL_HANDLE;
				PFN_vkGetPhysicalDeviceSurfaceFormatsKHR m_vkGetPhysicalDeviceSurfaceFormatsKHR = VK_NULL_HANDLE;
				PFN_vkGetPhysicalDeviceSurfacePresentModesKHR m_vkGetPhysicalDeviceSurfacePresentModesKHR = VK_NULL_HANDLE;
					
			public:

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				WvkKhrSurfaceDispatchTable(void) noexcept;

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				~WvkKhrSurfaceDispatchTable(void) noexcept;

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				* �������������� ������ WvkKhrSurfaceDispatchTable, �������� ��������� � �������� Vulkan-������.
				*
				* @details
				* ����� ��������� ���������� ��������� �������� � ��������� ��� �������� ����:
				* ��������� ������ (��� ���������� ������ � ����������) � �������� Vulkan-�������,
				* ����������� ��� ������ � ������������� (surface).
				*
				* ���� ��������� ��� �������� ������� ����������� � �������, ����� ����������
				* ��������������� ������ � ��������� ����������.
				*
				* @param[in] create_info
				* ��������� � ����������� �������� �����������.
				*
				* @retval VknStatusCode::SUCCESSFUL
				* ����������� ������� �������, ��� ������� ���������.
				*
				* @retval VknStatusCode::FAIL
				* ������ ��������� ��� ��� �������� Vulkan-�������.
				*
				* @code
				* WvkKhrSurfaceDispatchTable surface;
				* WvkKhrSurfaceDispatchTableCreateInfo info = { ... };
				* WvkStatus status = surface.create(info);
				* if (!status) {
				*     // ��������� ������
				* }
				* @endcode
				*
				* @see WvkKhrSurfaceDispatchTable::validationCreateInfo
				* @see WvkKhrSurfaceDispatchTable::loadVulkanCommand
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				WvkStatus create(const WvkKhrSurfaceDispatchTableCreateInfo& create_info) noexcept;

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				void destroy(void) noexcept;
						
			private:

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				* ��������� ������������ ��������� WvkKhrSurfaceCreateInfo.
				*
				* @details
				* ����� ��������� ������� ��������� ������� ������, ����������� ��� ����������
				* ������ WvkKhrSurfaceDispatchTable. �� ������� ����� ����������� ������ ������� ���������
				* �� WvkCommands. ���� �� ����� nullptr, ������������ ������.
				*
				* @retval VknStatusCode::SUCCESSFUL
				* ��� ������������ ������ ������������.
				*
				* @retval VknStatusCode::FAIL
				* ��������� �� WvkCommands ����� nullptr.
				*
				* @code
				* WvkKhrSurfaceDispatchTable surface;
				* WvkStatus status = surface.validationCreateInfo();
				* if (!status) {
				*     // ��������� ������
				* }
				* @endcode
				*
				* @see WvkKhrSurfaceDispatchTable::loadVulkanCommand
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				WvkStatus validationCreateInfo(void) const noexcept;

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				* *��������� ������ �������� Vulkan ��� ���������� VK_KHR_surface.*
				*
				* ����� ���������� `WvkLoader::loadProcedure` ��� �������� ���������� �� �������, ��������� � `VK_KHR_surface`,
				* �������: `vkDestroySurfaceKHR`, `vkGetPhysicalDeviceSurfaceSupportKHR`, `vkGetPhysicalDeviceSurfaceCapabilitiesKHR`,
				* `vkGetPhysicalDeviceSurfaceFormatsKHR`, `vkGetPhysicalDeviceSurfacePresentModesKHR`.
				*
				* �������� ���������� ����� `invokeWithVkInstanceMethod`, ������� �������� ����� � ������� VkInstance.
				*
				* @return
				* ���������� ������ `WvkStatus`. � ������ ������ � `WvkStatus::setOk()`, ����� ��� ������ � ����������.
				*
				* @code
				* WvkKhrSurfaceDispatchTable table;
				* WvkStatus status = table.loadVulkanCommand();
				* if (!status) {
				*     std::cerr << "�� ������� ��������� ������� VK_KHR_surface: " << status.message() << std::endl;
				* }
				* @endcode
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				WvkStatus loadVulkanCommand(void) noexcept;

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				void reset(void) noexcept;

			private:

				WvkKhrSurfaceDispatchTableCreateInfo m_create_info = {};
			}; // class WvkKhrSurfaceDispatchTable

		} // namespace Extensions

	} // namespace wvk

} // namespace CGDev

#include "wvk_khr_surface_dispatch_table.hpp"

#endif // CGDEV_WVK_SOURCE_EXTENSIONS__WVK_KHR_SURFACE_DISPATCH_TABLE_H
