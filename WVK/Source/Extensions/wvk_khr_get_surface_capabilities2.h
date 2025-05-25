#ifndef CGDEV_WVK_SOURCE_EXTENSIONS__WVK_KHR_GET_SURFACE_CAPABILITIES2_H
#define CGDEV_WVK_SOURCE_EXTENSIONS__WVK_KHR_GET_SURFACE_CAPABILITIES2_H
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
#include "../wvk_commands.h"

namespace CGDev {

	namespace wvk {

		namespace Extensions {

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			struct WvkKhrGetSurfaceCapabilities2CreateInfo {
				WvkCommandsPtr wvk_commands = nullptr;
			}; // WvkKhrGetSurfaceCapabilities2CreateInfo





			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			class WvkKhrGetSurfaceCapabilities2 : public GpuObject {

			public:

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				inline static const std::string& s_getName(void) noexcept;

			private:

				inline static const std::string s_name = "VK_KHR_get_surface_capabilities2";

			public:

				inline VkResult vkGetPhysicalDeviceSurfaceCapabilities2KHR(VkPhysicalDevice physicalDevice, const VkPhysicalDeviceSurfaceInfo2KHR* pSurfaceInfo, VkSurfaceCapabilities2KHR* pSurfaceCapabilities) const noexcept;
				inline VkResult vkGetPhysicalDeviceSurfaceFormats2KHR(VkPhysicalDevice physicalDevice, const VkPhysicalDeviceSurfaceInfo2KHR* pSurfaceInfo, uint32_t* pSurfaceFormatCount, VkSurfaceFormat2KHR* pSurfaceFormats) const noexcept;				

			private:

				PFN_vkGetPhysicalDeviceSurfaceCapabilities2KHR m_vk_get_physical_device_surface_capabilities2_khr = VK_NULL_HANDLE;
				PFN_vkGetPhysicalDeviceSurfaceFormats2KHR m_vk_get_physical_device_surface_formats2_khr = VK_NULL_HANDLE;

			public:

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				WvkKhrGetSurfaceCapabilities2(void) noexcept;

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				~WvkKhrGetSurfaceCapabilities2(void) noexcept;

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				* �������������� ������ WvkKhrSurface, �������� ��������� � �������� Vulkan-������.
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
				* ��������� � ����������� �������� �����������. ������������ ����: wvk_commands.
				*
				* @retval VknStatusCode::SUCCESSFUL
				* ����������� ������� �������, ��� ������� ���������.
				*
				* @retval VknStatusCode::FAIL
				* ������ ��������� ��� ��� �������� Vulkan-�������.
				*
				* @code
				* WvkKhrSurface surface;
				* WvkKhrSurfaceCreateInfo info = { ... };
				* WvkStatus status = surface.create(info);
				* if (!status) {
				*     // ��������� ������
				* }
				* @endcode
				*
				* @see WvkKhrSurface::validationCreateInfo
				* @see WvkKhrSurface::loadVulkanCommand
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				WvkStatus create(const WvkKhrGetSurfaceCapabilities2CreateInfo& create_info) noexcept;

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				void destroy(void) noexcept;

			public:

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				inline const WvkKhrGetSurfaceCapabilities2CreateInfo& getCreateInfo(void) const noexcept;

			private:

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				* ��������� ������������ ��������� WvkKhrSurfaceCreateInfo.
				*
				* @details
				* ����� ��������� ������� ��������� ������� ������, ����������� ��� ����������
				* ������ WvkKhrSurface. �� ������� ����� ����������� ������ ������� ���������
				* �� WvkCommands. ���� �� ����� nullptr, ������������ ������.
				*
				* @retval VknStatusCode::SUCCESSFUL
				* ��� ������������ ������ ������������.
				*
				* @retval VknStatusCode::FAIL
				* ��������� �� WvkCommands ����� nullptr.
				*
				* @code
				* WvkKhrSurface surface;
				* WvkStatus status = surface.validationCreateInfo();
				* if (!status) {
				*     // ��������� ������
				* }
				* @endcode
				*
				* @see WvkKhrSurface::loadVulkanCommand
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				WvkStatus validationCreateInfo(void) const noexcept;

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				* ��������� ��������� �� Vulkan-�������, ��������� � KHR Surface �����������.
				*
				* @details
				* ���������� �������� �������� ������ �� ������� `wvk_commands`, ����� ��������
				* ��������� �� �������� ������� ���������� ������������� (Surface) � Vulkan.
				*
				* @return
				* ���������� ������ ���������� ��������:
				* - WvkStatus::ok(), ���� ��� ������� ���� ������� ���������.
				* - WvkStatus::fail(), ���� ��������� ������ ��� �������� ���� �� ����� �������.
				*
				* @code
				* WvkKhrSurface surface;
				* WvkStatus status = surface.loadVulkanCommand();
				* if (!status) {
				*     // ��������� ������
				* }
				* @endcode
				*
				* @see WvkCommands::tryLoadFunction
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				WvkStatus loadVulkanCommand(void) noexcept;

			private:

				WvkKhrGetSurfaceCapabilities2CreateInfo m_create_info = {};

			}; // class WvkKhrGetSurfaceCapabilities2

		} // namespace Extensions

	} // namespace wvk

} // namespace CGDev

#include "wvk_khr_get_surface_capabilities2.hpp"

#endif // CGDEV_WVK_SOURCE_EXTENSIONS__WVK_KHR_GET_SURFACE_CAPABILITIES2_H
