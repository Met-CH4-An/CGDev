#ifndef CGDEV_WVK_SOURCE_EXTENSIONS__WVK_SURFACE_H
#define CGDEV_WVK_SOURCE_EXTENSIONS__WVK_SURFACE_H
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
			struct WvkSurfacePlatformCreateInfo {
			}; // struct WvkSurfacePlatformCreateInfo

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief 
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			struct WvkSurfaceCreateInfo {
				WvkInstanceDispatchTablePtr wvk_instance_dispatch_table = nullptr;
				WvkKhrSurfaceDispatchTablePtr wvk_khr_surface_dispatch_table = nullptr;
				WvkSurfacePlatformCreateInfo* wvk_surface_platform_create_info = nullptr;
			}; // struct WvkSurfaceCreateInfo

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			class WvkSurface : public GpuObject {

			public:

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				WvkSurface(void) noexcept;

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				~WvkSurface(void) noexcept;

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				WvkStatus create(const WvkSurfaceCreateInfo& create_info) noexcept;

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				void destroy(void) noexcept;

			public:

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				* *����������� ����������� ����������� ��� ���������� ����������� ����������.*
				*
				* @details
				* *����� �������� ������� `vkGetPhysicalDeviceSurfaceCapabilitiesKHR` ����� ��������������� ������� �������,
				* ����� �������� ������ � ������������ ����������� (��������, �������������� �������, ���������� ����������� � �.�.).*
				*
				* @param[in]  wvk_physical_device
				* *��������� �� ������ ����������� ����������, �� ����� �������� ���������� �����.*
				*
				* @param[out] vk_surface_capabilities_khr
				* *���������, � ������� ����� �������� ����������� �����������. ������ ���� ��������.*
				*
				* @retval WvkStatus
				* *������ ��������. � ������ ������� ������������ ��������� �������� ������.*
				*
				* @code
				* WvkSurface surface;
				* VkSurfaceCapabilitiesKHR caps{};
				* WvkStatus status = surface.requestCapabilities(myPhysicalDevice, caps);
				* if (!status) {
				*     std::cerr << status.toString();
				* }
				* @endcode
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				inline WvkStatus requestCapabilities(const WvkPhysicalDevicePtr wvk_physical_device, VkSurfaceCapabilitiesKHR& vk_surface_capabilities_khr) const noexcept;

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				* �������� ������ �������������� ������� ����������� (VkPresentModeKHR) ��� ������� �����������.
				*
				* @param[in]  wvk_physical_device   ��������� �� ���������� ���������� Vulkan (������ WvkPhysicalDevicePtr).
				* @param[out] vk_present_modes      ������, � ������� ����� �������� �������������� ������ �����������.
				*
				* @return ���������� ������ ���������� ��������.
				*
				* ��������� ������:
				* - VK_ERROR_OUT_OF_HOST_MEMORY
				* - VK_ERROR_OUT_OF_DEVICE_MEMORY
				* - VK_ERROR_SURFACE_LOST_KHR
				* - VK_INCOMPLETE (��� ������ ������)
				*
				* @code
				* WvkSurface surface = ...;
				* WvkPhysicalDevicePtr device = ...;
				* std::vector<VkPresentModeKHR> modes;
				* WvkStatus status = surface.requestPresentModes(device, modes);
				* if (status) {
				*     // �������
				*     printPresentModes(modes);
				* } else {
				*     std::cerr << status.message() << std::endl;
				* }
				* @endcode
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				inline WvkStatus requestPresentModes(const WvkPhysicalDevicePtr wvk_physical_device, std::vector<VkPresentModeKHR>& vk_present_modes) const noexcept;

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*     ����������� ������ ������� �����������, ����������� � �������� `VkPresentModeKHR`.
				*
				* @param [in]  wvk_physical_device
				*     ��������� �� ������ ����������� ����������, �� ������� ����������� ������.
				*
				* @param [in]  vk_present_mode
				*     ����� �����������, ��� �������� ��������� �������� ������ ����������� �������.
				*
				* @param [out] out
				*     ������, � ������� ����� ������� ������ ����������� ������� ����������� (`VkPresentModeKHR`).
				*
				* @return
				*     ������ `WvkStatus`, ���������� ���������� ��������.
				*
				* @code
				* std::vector<VkPresentModeKHR> compatible_modes;
				* WvkStatus status = wvk_surface->requestPresentModeCompatibility(
				*     wvk_physical_device,
				*     VK_PRESENT_MODE_MAILBOX_KHR,
				*     compatible_modes
				* );
				* if (!status) {
				*     // ��������� ������
				* }
				* @endcode
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				inline WvkStatus requestPresentModeCompatibility(const WvkPhysicalDevicePtr wvk_physical_device, const VkPresentModeKHR& vk_present_mode, std::vector<VkPresentModeKHR>& out) const noexcept;

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*     ����������� ����������� ��������������� ����������� (scaling capabilities)
				*     ��� ��������� ������ ����������� (`VkPresentModeKHR`) ����� ������� `pNext`.
				*
				* @param [in]  wvk_physical_device
				*     ��������� �� ������ ����������� ������������� ����������� ����������.
				*
				* @param [in]  vk_present_mode
				*     ����� �����������, ��� �������� ��������� �������� ���������� � ���������������.
				*
				* @param [out] out
				*     ��������� `VkSurfacePresentScalingCapabilitiesEXT`, ���� ����� ��������
				*     ���������� � �������������� ������� ���������������.
				*
				* @return
				*     ������ `WvkStatus`, ���������� ��������� ��������.
				*
				* @code
				* VkSurfacePresentScalingCapabilitiesEXT scaling_caps = {};
				* WvkStatus status = wvk_surface->requestScalingCompatibility(
				*     wvk_physical_device,
				*     VK_PRESENT_MODE_FIFO_KHR,
				*     scaling_caps
				* );
				* if (!status) {
				*     // ��������� ������
				* }
				* @endcode
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				WvkStatus requestScalingCompatibility(const WvkPhysicalDevicePtr wvk_physical_device, const VkPresentModeKHR& vk_present_mode, VkSurfacePresentScalingCapabilitiesEXT& out) const noexcept;

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*     ��������� ������ ����������� ���������� � surface capabilities � �������������� ��������� `VkBaseInStructure` � `VkBaseOutStructure`.
				*
				* @param [in]  wvk_physical_device
				*     ��������� �� ���������� ������ ����������� ���������� Vulkan.
				*
				* @param [in]  in
				*     ��������� �� ������� ������� ��������, �������������� �� `VkBaseInStructure`.
				*     ����� ���� `nullptr`, ���� �������������� ��������� �� ���������.
				*
				* @param [out] out
				*     ��������� �� ��������� ��� ������� ��������, �������������� �� `VkBaseOutStructure`,
				*     ���� ����� �������� ���������� �������.
				*
				* @return
				*     ������ `WvkStatus`, ���������� ��������� ��������.
				*     ��� ������ ������ �������� ��������� � ��� `FAIL`.
				*
				* @code
				* VkSurfaceProtectedCapabilitiesKHR _caps = { VK_STRUCTURE_TYPE_SURFACE_PROTECTED_CAPABILITIES_KHR };
				* WvkStatus status = wvk_surface->requestCapability(wvk_physical_device, nullptr, &_caps);
				* if (!status) {
				*     // ��������� ������
				* }
				* @endcode
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				inline WvkStatus requestCapabilities(const WvkPhysicalDevicePtr wvk_physical_device, const VkBaseInStructure* in, VkBaseOutStructure* out) const noexcept;

				inline WvkStatus requestCapabilities(const WvkPhysicalDevicePtr wvk_physical_device, VkSurfaceProtectedCapabilitiesKHR& out) const noexcept;

				template<typename In, typename Out>
				inline WvkStatus requestCapabilities(const WvkPhysicalDevicePtr wvk_physical_device, const In& in, Out& out) const noexcept;

			private:

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				WvkStatus validationCreateInfo(void) const noexcept;

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief 
				* ������ VkSurfaceKHR � ����������� �� ������� ���������.
				* 
				* @details
				* �� ������ ������ ����������� ��������� ������ ��������� MSWindows. ������������
				* ��������� WvkSurfaceMSWindowsCreateInfo, ������� �������� ����������� ���� � ��������,
				* � ����� ��������� �� ���������� WvkKhrWin32Surface.
				* 
				* @return WvkStatus 
				* ���������� ������ ����������. ���� �������� ������ ������� � `OK`. � ��������� ������ � ��� ������.
				* 
				* @code
				* WvkSurface surface;
				* surface.setCreateInfo(...); // ���������� ������������-��������� ����������
				* WvkStatus status = surface.createVkSurface();
				* if (!status.isOk()) {
				*     log->error(status.message());
				* }
				* @endcode
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				WvkStatus createVkSurface(void) noexcept;

			private:

				WvkSurfaceCreateInfo m_create_info = {};
				VkSurfaceKHR m_vk_surface = VK_NULL_HANDLE;
				struct WvkSurfaceImpl;
				std::unique_ptr<WvkSurfaceImpl> m_surface_impl = nullptr;
			}; // class WvkSurface

		} // namespace Extensions

	} // namespace wvk

} // namespace CGDev

#include "wvk_surface.hpp"

#endif // CGDEV_WVK_SOURCE_EXTENSIONS__WVK_SURFACE_H
