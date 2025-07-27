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
				WvkInstanceDispatchTablePtr wvk_instance_dt_ptr = nullptr;
				WvkKhrSurfaceDTPtr wvk_khr_surface_dt = nullptr;
				WvkKhrGetSurfaceCapabilities2DispatchTablePtr wvk_khr_get_surface_capabilities2_dispatch_table = nullptr;
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
				*     ����������� ������ �������������� �������� ����������� (VkSurfaceFormatKHR) ��� ���������� ����������� ����������.
				*
				* @details
				*     ����� ��������� ����������� ����� vkGetPhysicalDeviceSurfaceFormatsKHR ����� dispatch table:
				*     ������� �������� ���������� ��������� ��������, ����� � ���� �������.
				*     ���� ���������� VK_KHR_surface �� ������������, ���������� ������ FEATURE_NOT_ENABLED.
				*     � ������ ������ ���������� ��������� ��������� � ��� FAIL.
				*
				*     �������� ������:
				*     - ���������, ������������ �� ���������� VK_KHR_surface.
				*     - ������� �������� ������ � ����������� ���������� �������������� ��������.
				*     - ���� ���������� ������ ����, �������� ������ ��� ������ ��������.
				*     - �������� �������� vkGetPhysicalDeviceSurfaceFormatsKHR ��� ��������� ����� ��������.
				*     - ������������ ��������� ������ (� ��� ����� VK_INCOMPLETE) � ���������� ������.
				*
				* @param[in]  wvk_physical_device_ptr
				*     ��������� �� ������ ����������� ���������� Vulkan (WvkPhysicalDevicePtr).
				*
				* @param[out] out
				*     ������, � ������� ����� �������� �������������� ������� ����������� (VkSurfaceFormatKHR).
				*
				* @retval WvkStatus
				*     - VknStatusCode::SUCCESSFUL � ���� ������� ������� ��������.
				*     - VknStatusCode::FAIL � ���� ��������� ������ ��� ������ Vulkan API.
				*     - VknStatusCode::FEATURE_NOT_ENABLED � ���� VK_KHR_surface �� �����������.
				*
				* @code
				* std::vector<VkSurfaceFormatKHR> formats;
				* WvkStatus status = surface.requestFormats(physicalDevice, formats);
				* if (!status) {
				*     std::cerr << status.getMessage() << std::endl;
				* }
				* @endcode
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				WvkStatus requestFormats(const WvkPhysicalDevicePtr wvk_physical_device_ptr, std::vector<VkSurfaceFormatKHR>& out) const noexcept;

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*     ��������� ��������� ����������� (present support) ��� ���������� ��������� �������� �� ������ surface.
				*
				* @details
				*     ����� �������� vkGetPhysicalDeviceSurfaceSupportKHR (����� dispatch table) ��� ��������,
				*     ����� �� ��������� ��������� �������� ����������� ���������� ��������� ����������� �� ������� surface.
				*     ���� ���������� VK_KHR_surface �� ������������, ���������� ������ FEATURE_NOT_ENABLED.
				*     � ������ ������ � �������� out ������������ true (���� ��������� ����) ��� false (���� ���).
				*
				*     �������� ������:
				*     - ��������� ������� ��������� VK_KHR_surface.
				*     - �������� vkGetPhysicalDeviceSurfaceSupportKHR ��� ���������� ����������� ���������� � ��������� ��������.
				*     - ������������ ��������� ������ � ���������� ������.
				*
				* @param[in]  wvk_physical_device_ptr
				*     ��������� �� ������ ����������� ���������� Vulkan (WvkPhysicalDevicePtr).
				*
				* @param[in]  wvk_queue_family_ptr
				*     ��������� �� ������ ��������� �������� (WvkQueueFamilyPtr), ��� �������� ����������� ��������� �����������.
				*
				* @param[out] out
				*     ������ ��������: true, ���� ��������� �������� ������������ ����������� �� ������ surface, ����� false.
				*
				* @retval WvkStatus
				*     - VknStatusCode::SUCCESSFUL � ���� �������� ��������� �������.
				*     - VknStatusCode::FAIL � ���� ��������� ������ ��� ������ Vulkan API.
				*     - VknStatusCode::FEATURE_NOT_ENABLED � ���� VK_KHR_surface �� �����������.
				*
				* @code
				* bool presentSupported = false;
				* WvkStatus status = surface.requestSupport(physicalDevice, queueFamily, presentSupported);
				* if (status && presentSupported) {
				*     // ������� ������������ present
				* } else if (!status) {
				*     std::cerr << status.getMessage() << std::endl;
				* }
				* @endcode
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				WvkStatus requestSupport(const WvkPhysicalDevicePtr wvk_physical_device_ptr, const WvkQueueFamilyPtr wvk_queue_family_ptr, bool& out) const noexcept;

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				* �������� ������ �������������� ������� ����������� (VkPresentModeKHR) ��� ������� �����������.
				*
				* @param[in]  wvk_physical_device_ptr   ��������� �� ���������� ���������� Vulkan (������ WvkPhysicalDevicePtr).
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
				WvkStatus requestPresentModes(const WvkPhysicalDevicePtr wvk_physical_device_ptr, std::vector<VkPresentModeKHR>& vk_present_modes) const noexcept;

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*     ����������� �������� ����������� ����������� (surface capabilities) ��� ���������� ����������� ����������.
				*
				* @details
				*     ����� �������� ������� vkGetPhysicalDeviceSurfaceCapabilitiesKHR ����� dispatch table, ����� ��������
				*     ��������� ����������� (��������, ����������� � ������������ ���������� �����������, �������, �������������� ������������� � �.�.).
				*     ���� ���������� VK_KHR_surface �� ������������, ���������� ������ FEATURE_NOT_ENABLED.
				*     � ������ ������ ���������� ��������� ��������� � ��� FAIL.
				*
				*     �������� ������:
				*     - ���������, ������������ �� ���������� VK_KHR_surface.
				*     - �������� vkGetPhysicalDeviceSurfaceCapabilitiesKHR ��� ������� ����������� � ����������� ����������.
				*     - ������������ ��������� ������ � ���������� ������.
				*
				* @param[in]  wvk_physical_device_ptr
				*     ��������� �� ������ ����������� ���������� Vulkan (WvkPhysicalDevicePtr).
				*
				* @param[out] vk_surface_capabilities_khr
				*     ��������� VkSurfaceCapabilitiesKHR, � ������� ����� �������� ����������� �����������.
				*
				* @retval WvkStatus
				*     - VknStatusCode::SUCCESSFUL � ���� ������ �������� �������.
				*     - VknStatusCode::FAIL � ���� ��������� ������ ��� ������ Vulkan API.
				*     - VknStatusCode::FEATURE_NOT_ENABLED � ���� VK_KHR_surface �� �����������.
				*
				* @code
				* VkSurfaceCapabilitiesKHR caps{};
				* WvkStatus status = surface.requestCapabilities(physicalDevice, caps);
				* if (!status) {
				*     std::cerr << status.getMessage() << std::endl;
				* }
				* @endcode
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				WvkStatus requestCapabilities(const WvkPhysicalDevicePtr wvk_physical_device_ptr, VkSurfaceCapabilitiesKHR& vk_surface_capabilities_khr) const noexcept;

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*     ����������� ������ ������� �����������, ����������� � �������� `VkPresentModeKHR`.
				*
				* @param [in]  wvk_physical_device_ptr
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
				*     wvk_physical_device_ptr,
				*     VK_PRESENT_MODE_MAILBOX_KHR,
				*     compatible_modes
				* );
				* if (!status) {
				*     // ��������� ������
				* }
				* @endcode
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				WvkStatus requestPresentModeCompatibility(const WvkPhysicalDevicePtr wvk_physical_device_ptr, const VkPresentModeKHR& vk_present_mode, std::vector<VkPresentModeKHR>& out) const noexcept;

				

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*     ��������� ������ ����������� ������������ ����������� (surface capabilities) � �������������� ������� pNext.
				*
				* @details
				*     ����� ��������� ������������� �������� ��������� ����������� ������������ ����������� ����� �������
				*     vkGetPhysicalDeviceSurfaceCapabilities2KHR (��� ����������� ��� ����� ���������/dispatch table).
				*     ��������� ���������� ������� ������� (VkBaseInStructure) � �������� (VkBaseOutStructure) �������� ��� ��������� ����������.
				*
				*     �������� ������:
				*     - ���������, ��� ����������� Vulkan 1.1 ��� ���������� VK_KHR_surface.
				*     - ��������� ��������� VkPhysicalDeviceSurfaceInfo2KHR, �������� surface � ������� in.
				*     - ��������� ��������� VkSurfaceCapabilities2KHR, �������� ������� out.
				*     - �������� ������� ��������� surface capabilities ����� dispatch table.
				*     - ������������ ��������� ������ � ���������� ������.
				*     - ���� VK_KHR_surface �� ����������� � ���������� FEATURE_NOT_ENABLED.
				*
				* @param[in]  wvk_physical_device_ptr
				*     ��������� �� ������ ����������� ���������� Vulkan (WvkPhysicalDevicePtr).
				*
				* @param[in]  in
				*     ��������� �� ������� ������� ��������, �������������� �� VkBaseInStructure (��� nullptr, ���� �� ���������).
				*
				* @param[out] out
				*     ��������� �� ������� �������� ��������, �������������� �� VkBaseOutStructure, ���� ����� �������� ����������.
				*
				* @retval WvkStatus
				*     - VknStatusCode::SUCCESSFUL � ���� ������ �������� �������.
				*     - VknStatusCode::FAIL � ���� ��������� ������ ��� ������ Vulkan API.
				*     - VknStatusCode::FEATURE_NOT_ENABLED � ���� VK_KHR_surface �� �����������.
				*
				* @code
				* VkSurfacePresentScalingCapabilitiesEXT scaling_caps = { VK_STRUCTURE_TYPE_SURFACE_PRESENT_SCALING_CAPABILITIES_EXT };
				* WvkStatus status = surface.requestCapabilities(device, &inStruct, &scaling_caps);
				* if (!status) {
				*     std::cerr << status.getMessage() << std::endl;
				* }
				* @endcode
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				WvkStatus requestCapabilities(const WvkPhysicalDevicePtr wvk_physical_device_ptr, const VkBaseInStructure* in, VkBaseOutStructure* out) const noexcept;

			// hpp
			public:

				template<typename In, typename Out>
				inline WvkStatus requestCapabilities(const WvkPhysicalDevicePtr wvk_physical_device_ptr, const In& in, Out& out) const noexcept;

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*     ����������� ����������� ������ �������������� �������� ����������� � �������������� vkGetPhysicalDeviceSurfaceFormats2KHR.
				*
				* @details
				*     ����� ��������� �������� ����������� ���������� � �������� �����������, ��������� ������� pNext ��� �������� �������������� ��������.
				*     ������� ������������ ������ ���������� ��������� ��������, ����� ���������� ������ � ������������� ���� ������� � ��������������� ����������.
				*
				* @tparam T
				*     ��� ���������, �������������� �� VkBaseOutStructure, � ������� ����� �������� �������������� �������� ������� (��������, VkSurfaceFormatKHR + ����������).
				*
				* @param[in]  wvk_physical_device_ptr
				*     ��������� �� ������ ����������� ���������� Vulkan, ��� �������� ����������� ������.
				* @param[in]  in
				*     ��������� �� ������� ������� �������� (VkBaseInStructure), ���� nullptr, ���� �� ���������.
				* @param[in]  prop_sType
				*     �������� VkStructureType ��� ��������� T, ������� ����� �������� � pNext ������� VkSurfaceFormat2KHR.
				* @param[out] out_props
				*     ������, � ������� ����� �������� ��������� T � ����������� ����������� � ��������.
				*
				* @return
				*     ������ WvkStatus, ���������� ���������� ��������.
				*
				* @code
				* std::vector<VkDrmFormatModifierPropertiesEXT> modifiers;
				* WvkStatus status = surface->requestFormats(device, &inStruct, VK_STRUCTURE_TYPE_DRM_FORMAT_MODIFIER_PROPERTIES_EXT, modifiers);
				* if (!status) {
				*     // ��������� ������
				* }
				* @endcode
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				template<typename T>
				inline WvkStatus requestFormats(const WvkPhysicalDevicePtr wvk_physical_device_ptr, const VkBaseInStructure* in, const VkStructureType& prop_sType, std::vector<T>& out_props) const noexcept;

			// helper
			public:

				WvkStatus requestCapabilities(const WvkPhysicalDevicePtr wvk_physical_device_ptr, VkSurfaceProtectedCapabilitiesKHR& out) const noexcept;

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*     ����������� ����������� ��������������� ����������� (scaling capabilities)
				*     ��� ��������� ������ ����������� (`VkPresentModeKHR`) ����� ������� `pNext`.
				*
				* @param [in]  wvk_physical_device_ptr
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
				*     wvk_physical_device_ptr,
				*     VK_PRESENT_MODE_FIFO_KHR,
				*     scaling_caps
				* );
				* if (!status) {
				*     // ��������� ������
				* }
				* @endcode
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				WvkStatus requestScalingCompatibility(const WvkPhysicalDevicePtr wvk_physical_device_ptr, const VkPresentModeKHR& vk_present_mode, VkSurfacePresentScalingCapabilitiesEXT& out) const noexcept;		

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

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				void reset(void) noexcept;

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
