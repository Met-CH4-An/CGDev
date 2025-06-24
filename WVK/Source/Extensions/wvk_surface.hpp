#ifndef CGDEV_WVK_SOURCE_EXTENSIONS__WVK_SURFACE_HPP
#define CGDEV_WVK_SOURCE_EXTENSIONS__WVK_SURFACE_HPP
////////////////////////////////////////////////////////////////
// ������ �������-����������
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// ������ �������������
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// ������ ������������� ������
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// ������ ��� ����������
////////////////////////////////////////////////////////////////
#include "wvk_khr_surface_dt.hpp"
#include "wvk_khr_get_surface_capabilities2_dt.hpp"
#include "../wvk_instance_dt.hpp"
#include "../wvk_physical_device.h"
#include "../wvk_queue_family.h"

namespace CGDev {

	namespace wvk {

		namespace Extensions {

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			inline WvkStatus WvkSurface::requestSupport(const WvkPhysicalDevicePtr wvk_physical_device_ptr, const WvkQueueFamilyPtr wvk_queue_family_ptr, bool& out) const noexcept {
				WvkStatus _status;

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// ��� 1. ���������, ������������ �� ���������� VK_KHR_surface
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				if constexpr (Build::WvkBuildInfo::find(WvkKhrSurfaceDT::s_getName())) {
					VkBool32 _support = VK_FALSE;
					out = false;

					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					// ��� 2. �������� vkGetPhysicalDeviceSurfaceSupportKHR ��� �������� ��������� present
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					auto _vk_res = wvk_physical_device_ptr->invokeWithVkPhysicalDeviceMethod(
						&WvkKhrSurfaceDT::wvkGetPhysicalDeviceSurfaceSupportKHR,
						m_create_info.wvk_khr_surface_dt,
						wvk_queue_family_ptr->getIndexFamily(),
						m_vk_surface,
						&_support
					);

					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					// ��� 3. ������������ ��������� ������ ������
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					if (_vk_res != VK_SUCCESS) {
						switch (_vk_res) {
						case VK_ERROR_OUT_OF_HOST_MEMORY:
							_status.append("\n\tWvkKhrSurfaceDispatchTable::wvkGetPhysicalDeviceSurfaceCapabilitiesKHR - VK_ERROR_OUT_OF_HOST_MEMORY.");
							break;
						case VK_ERROR_OUT_OF_DEVICE_MEMORY:
							_status.append("\n\tWvkKhrSurfaceDispatchTable::wvkGetPhysicalDeviceSurfaceCapabilitiesKHR - VK_ERROR_OUT_OF_DEVICE_MEMORY.");
							break;
						case VK_ERROR_SURFACE_LOST_KHR:
							_status.append("\n\tWvkKhrSurfaceDispatchTable::wvkGetPhysicalDeviceSurfaceCapabilitiesKHR - VK_ERROR_SURFACE_LOST_KHR.");
							break;
						default:
							_status.append("\n\tWvkKhrSurfaceDispatchTable::wvkGetPhysicalDeviceSurfaceCapabilitiesKHR - unknown error.");
							break;
						}
						return _status.set(VknStatusCode::FAIL);
					}

					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					// ��� 4. ���������� ��������� �������� � out
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					out = (_support == VK_TRUE);

					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					// ��� 5. ���������� �������� ������
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					return _status.setOk();
				}

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// ��� 6. ���� VK_KHR_surface �� ����������� � ���������� ��������������� ������
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				return _status.set(VknStatusCode::FEATURE_NOT_ENABLED, "\n\tWvkSurface::requestSupport - not enabled.");
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			inline WvkStatus WvkSurface::requestCapabilities(const WvkPhysicalDevicePtr wvk_physical_device_ptr, VkSurfaceCapabilitiesKHR& vk_surface_capabilities_khr) const noexcept {
				WvkStatus _status;

				if constexpr (Build::WvkBuildInfo::find(WvkKhrSurfaceDT::s_getName())) {

					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					// ��� 1. ����� ������ ��������� surface capabilities ����� invoke
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					auto _vk_res = wvk_physical_device_ptr->invokeWithVkPhysicalDeviceMethod(
						&WvkKhrSurfaceDT::wvkGetPhysicalDeviceSurfaceCapabilitiesKHR,
						m_create_info.wvk_khr_surface_dt,
						m_vk_surface,
						&vk_surface_capabilities_khr
					);

					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					// ��� 2. ��������� ������, ���� ��������� ������ �� VK_SUCCESS
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					if (_vk_res != VK_SUCCESS) {
						switch (_vk_res) {
						case VK_ERROR_OUT_OF_HOST_MEMORY:
							_status.append("\n\tWvkKhrSurfaceDispatchTable::wvkGetPhysicalDeviceSurfaceCapabilitiesKHR - VK_ERROR_OUT_OF_HOST_MEMORY.");
							break;
						case VK_ERROR_OUT_OF_DEVICE_MEMORY:
							_status.append("\n\tWvkKhrSurfaceDispatchTable::wvkGetPhysicalDeviceSurfaceCapabilitiesKHR - VK_ERROR_OUT_OF_DEVICE_MEMORY.");
							break;
						case VK_ERROR_SURFACE_LOST_KHR:
							_status.append("\n\tWvkKhrSurfaceDispatchTable::wvkGetPhysicalDeviceSurfaceCapabilitiesKHR - VK_ERROR_SURFACE_LOST_KHR.");
							break;
						default:
							_status.append("\n\tWvkKhrSurfaceDispatchTable::wvkGetPhysicalDeviceSurfaceCapabilitiesKHR - unknown error.");
							break;
						}

						// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						// ��� 3. ��������� ���� ������ � ������� �������
						// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						return _status.set(VknStatusCode::FAIL);
					}

					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					// ��� 4. ������� ��������� �������
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					return _status.setOk();
				}

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// ��� 5. ���� VK_KHR_surface �� ����������� � ���������� ��������������� ������
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				return _status.set(VknStatusCode::FEATURE_NOT_ENABLED, "\n\tWvkSurface::requestSupport - not enabled.");
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			inline WvkStatus WvkSurface::requestPresentModes(const WvkPhysicalDevicePtr wvk_physical_device_ptr, std::vector<VkPresentModeKHR>& vk_present_modes) const noexcept {
				WvkStatus _status;
				
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// ��� 1. ���������, ������������ �� ���������� VK_KHR_surface
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				if constexpr (Build::WvkBuildInfo::find(WvkKhrSurfaceDT::s_getName())) {

					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					// ��� 1. ������������� ������� �������
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					WvkStatus _status;

					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					// ��� 2. ����������� ���������� ��������� ������� �����������
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					uint32_t _count = 0;
					auto _vk_res = wvk_physical_device_ptr->invokeWithVkPhysicalDeviceMethod(
						&WvkKhrSurfaceDT::wvkGetPhysicalDeviceSurfacePresentModesKHR,
						m_create_info.wvk_khr_surface_dt,
						m_vk_surface,
						&_count,
						nullptr
					);

					if (_vk_res != VK_SUCCESS) {
						// ��������� ��������� ������
						switch (_vk_res) {
						case VK_ERROR_OUT_OF_HOST_MEMORY:
							_status.append("\n\tWvkKhrSurfaceDispatchTable::wvkGetPhysicalDeviceSurfacePresentModesKHR - VK_ERROR_OUT_OF_HOST_MEMORY.");
							break;
						case VK_ERROR_OUT_OF_DEVICE_MEMORY:
							_status.append("\n\tWvkKhrSurfaceDispatchTable::wvkGetPhysicalDeviceSurfacePresentModesKHR - VK_ERROR_OUT_OF_DEVICE_MEMORY.");
							break;
						case VK_ERROR_SURFACE_LOST_KHR:
							_status.append("\n\tWvkKhrSurfaceDispatchTable::wvkGetPhysicalDeviceSurfacePresentModesKHR - VK_ERROR_SURFACE_LOST_KHR.");
							break;
						default:
							_status.append("\n\tWvkKhrSurfaceDispatchTable::wvkGetPhysicalDeviceSurfacePresentModesKHR - ����������� ������.");
							break;
						}
						return _status.set(VknStatusCode::FAIL);
					}

					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					// ��� 3. �������� ������ ��� ������ �������
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					vk_present_modes.resize(_count);

					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					// ��� 4. ����������� ������ �������������� �������
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					_vk_res = wvk_physical_device_ptr->invokeWithVkPhysicalDeviceMethod(
						&WvkKhrSurfaceDT::wvkGetPhysicalDeviceSurfacePresentModesKHR,
						m_create_info.wvk_khr_surface_dt,
						m_vk_surface,
						&_count,
						vk_present_modes.data()
					);

					if (_vk_res != VK_SUCCESS) {
						// ��������� ��������� ������
						switch (_vk_res) {
						case VK_ERROR_OUT_OF_HOST_MEMORY:
							_status.append("\n\tWvkKhrSurfaceDispatchTable::wvkGetPhysicalDeviceSurfacePresentModesKHR - VK_ERROR_OUT_OF_HOST_MEMORY.");
							break;
						case VK_ERROR_OUT_OF_DEVICE_MEMORY:
							_status.append("\n\tWvkKhrSurfaceDispatchTable::wvkGetPhysicalDeviceSurfacePresentModesKHR - VK_ERROR_OUT_OF_DEVICE_MEMORY.");
							break;
						case VK_ERROR_SURFACE_LOST_KHR:
							_status.append("\n\tWvkKhrSurfaceDispatchTable::wvkGetPhysicalDeviceSurfacePresentModesKHR - VK_ERROR_SURFACE_LOST_KHR.");
							break;
						case VK_INCOMPLETE:
							// VK_INCOMPLETE ����� ��������, ��� ������ ��� �������� ��������
							_status.append("\n\tWvkKhrSurfaceDispatchTable::wvkGetPhysicalDeviceSurfacePresentModesKHR - VK_INCOMPLETE.");
							break;
						default:
							_status.append("\n\tWvkKhrSurfaceDispatchTable::wvkGetPhysicalDeviceSurfacePresentModesKHR - ����������� ������.");
							break;
						}
						return _status.set(VknStatusCode::FAIL);
					}

					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					// ��� 5. ���������� �������� ������
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					return _status.setOk();
				}

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// ��� 5. ���� VK_KHR_surface �� ����������� � ���������� ��������������� ������
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				return _status.set(VknStatusCode::FEATURE_NOT_ENABLED, "\n\tWvkSurface::requestSupport - not enabled.");
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			inline WvkStatus WvkSurface::requestPresentModeCompatibility(const WvkPhysicalDevicePtr wvk_physical_device_ptr, const VkPresentModeKHR& vk_present_mode, std::vector<VkPresentModeKHR>& out) const noexcept {
				WvkStatus _status;

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// ��� 1. ���������� ������� ��������� VkSurfacePresentModeEXT
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				VkSurfacePresentModeEXT _in = {};
				_in.sType = VK_STRUCTURE_TYPE_SURFACE_PRESENT_MODE_EXT;
				_in.presentMode = vk_present_mode;
				_in.pNext = nullptr;

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// ��� 2. ���������� �������� ��������� VkSurfacePresentModeCompatibilityEXT (������ ����)
				// ��� ������� ���������� ����������� �������
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				VkSurfacePresentModeCompatibilityEXT _out = {};
				_out.sType = VK_STRUCTURE_TYPE_SURFACE_PRESENT_MODE_COMPATIBILITY_EXT;
				_out.pNext = nullptr;
				_out.presentModeCount = 0;
				_out.pPresentModes = nullptr;

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// ��� 3. ������ ����� ������� � ��������� ���������� �������
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				_status = requestCapabilities(wvk_physical_device_ptr, _in, _out);
				if (!_status) {
					return _status.set(VknStatusCode::FAIL, "\n\tWvkSurface::requestCapabilities - fail.");
				}

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// ��� 4. ��������� ������ ��� ����������� ������
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				out.resize(_out.presentModeCount);
				_out.pPresentModes = out.data();

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// ��� 5. ������ ����� � ��������� ����� ����������� �������
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				_status = requestCapabilities(wvk_physical_device_ptr, _in, _out);
				if (!_status) {
					return _status.set(VknStatusCode::FAIL, "\n\tWvkSurface::requestCapabilities - fail.");
				}

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// ��� 6. ������� ��������� ����������
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				return _status.setOk();
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			inline WvkStatus WvkSurface::requestScalingCompatibility(const WvkPhysicalDevicePtr wvk_physical_device_ptr, const VkPresentModeKHR& vk_present_mode, VkSurfacePresentScalingCapabilitiesEXT& out) const noexcept {
				WvkStatus _status;
				
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// ��� 1. ���������� ��������� VkSurfacePresentModeEXT,
				// ������� ��������� ������������� present mode
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				VkSurfacePresentModeEXT _in = {};
				_in.sType = VK_STRUCTURE_TYPE_SURFACE_PRESENT_MODE_EXT;
				_in.presentMode = vk_present_mode;
				_in.pNext = nullptr;

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// ��� 2. ���������� �������� ��������� VkSurfacePresentScalingCapabilitiesEXT,
				// ������� ������� ���������
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				out.sType = VK_STRUCTURE_TYPE_SURFACE_PRESENT_SCALING_CAPABILITIES_EXT;
				out.pNext = nullptr;

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// ��� 3. ����� ����������� ������ ������� ������������ ����� ������� pNext
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				return requestCapabilities(wvk_physical_device_ptr, _in, out);
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			inline WvkStatus WvkSurface::requestCapabilities(const WvkPhysicalDevicePtr wvk_physical_device_ptr, VkSurfaceProtectedCapabilitiesKHR& out) const noexcept {
				out.sType = VK_STRUCTURE_TYPE_SURFACE_PROTECTED_CAPABILITIES_KHR;
				out.pNext = nullptr;
				return requestCapabilities(wvk_physical_device_ptr, nullptr, reinterpret_cast<VkBaseOutStructure*>(&out));
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			template<typename In, typename Out>
			inline WvkStatus WvkSurface::requestCapabilities(const WvkPhysicalDevicePtr wvk_physical_device_ptr, const In& in, Out& out) const noexcept {
				return requestCapabilities(wvk_physical_device_ptr, reinterpret_cast<const VkBaseInStructure*>(&in), reinterpret_cast<VkBaseOutStructure*>(&out));
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			inline WvkStatus WvkSurface::requestCapabilities(const WvkPhysicalDevicePtr wvk_physical_device_ptr, const VkBaseInStructure* in, VkBaseOutStructure* out) const noexcept {
				WvkStatus _status;

				if constexpr (Build::WvkBuildInfo::vulkan_api_version >= Build::VulkanVersion::VERSION_11 ||
					Build::WvkBuildInfo::find(WvkKhrGetSurfaceCapabilities2DT::s_getName())) {

					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					// ��� 1. ���������� ��������� ������� ���������� �������
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					VkPhysicalDeviceSurfaceInfo2KHR _info2 = {};
					_info2.sType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SURFACE_INFO_2_KHR;
					_info2.pNext = in;
					_info2.surface = m_vk_surface;

					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					// ��� 2. ���������� ��������� ��� ��������� �����������
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					VkSurfaceCapabilities2KHR _caps2 = {};
					_caps2.sType = VK_STRUCTURE_TYPE_SURFACE_CAPABILITIES_2_KHR;
					_caps2.pNext = out;

					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					// ��� 3. ����� Vulkan-������� ����� �����-������
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					auto _vk_res = wvk_physical_device_ptr->invokeWithVkPhysicalDeviceMethod(
						&WvkInstanceDt::wvkGetPhysicalDeviceSurfaceCapabilities2KHR,
						m_create_info.wvk_instance_dt_ptr,
						&_info2,
						&_caps2
					);

					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					// ��� 4. ��������� ��������� ������
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					if (_vk_res != VK_SUCCESS) {
						switch (_vk_res) {
						case VK_ERROR_OUT_OF_HOST_MEMORY:
							_status.append("\n\tWvkInstanceDispatchTable::wvkGetPhysicalDeviceSurfaceCapabilities2KHR - VK_ERROR_OUT_OF_HOST_MEMORY.");
							break;
						case VK_ERROR_OUT_OF_DEVICE_MEMORY:
							_status.append("\n\tWvkInstanceDispatchTable::wvkGetPhysicalDeviceSurfaceCapabilities2KHR - VK_ERROR_OUT_OF_DEVICE_MEMORY.");
							break;
						case VK_ERROR_SURFACE_LOST_KHR:
							_status.append("\n\tWvkInstanceDispatchTable::wvkGetPhysicalDeviceSurfaceCapabilities2KHR - VK_ERROR_SURFACE_LOST_KHR.");
							break;
						}
						return _status.set(VknStatusCode::FAIL);
					}

					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					// ��� 5. �������� ����������
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					return _status.setOk();
				}

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// ��� 6. ���� VK_KHR_surface �� ����������� � ���������� ��������������� ������
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				return _status.set(VknStatusCode::FEATURE_NOT_ENABLED, "\n\tWvkSurface::requestSupport - not enabled.");
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			inline WvkStatus WvkSurface::requestFormats(const WvkPhysicalDevicePtr wvk_physical_device_ptr, std::vector<VkSurfaceFormatKHR>& out) const noexcept {
				WvkStatus _status;

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// ��� 1. ���������, ������������ �� ���������� VK_KHR_surface
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				if constexpr (Build::WvkBuildInfo::find(WvkKhrSurfaceDT::s_getName())) {

					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					// ��� 2. �������� ������ � ����������� ���������� ��� �������� ��������
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					uint32_t _count = 0;
					out.clear();

					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					// ��� 3. ������ �����: �������� ���������� �������������� ��������
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					auto _vk_res = wvk_physical_device_ptr->invokeWithVkPhysicalDeviceMethod(
						&WvkKhrSurfaceDT::wvkGetPhysicalDeviceSurfaceFormatsKHR,
						m_create_info.wvk_khr_surface_dt,
						m_vk_surface,
						&_count,
						nullptr
					);

					if (_vk_res != VK_SUCCESS) {
						switch (_vk_res) {
						case VK_ERROR_OUT_OF_HOST_MEMORY:
							_status.append("\n\tWvkKhrSurfaceDispatchTable::wvkGetPhysicalDeviceSurfaceFormatsKHR - VK_ERROR_OUT_OF_HOST_MEMORY.");
							break;
						case VK_ERROR_OUT_OF_DEVICE_MEMORY:
							_status.append("\n\tWvkKhrSurfaceDispatchTable::wvkGetPhysicalDeviceSurfaceFormatsKHR - VK_ERROR_OUT_OF_DEVICE_MEMORY.");
							break;
						case VK_ERROR_SURFACE_LOST_KHR:
							_status.append("\n\tWvkKhrSurfaceDispatchTable::wvkGetPhysicalDeviceSurfaceFormatsKHR - VK_ERROR_SURFACE_LOST_KHR.");
							break;
						}
						return _status.set(VknStatusCode::FAIL);
					}

					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					// ��� 4. �������� ����� � ������� ��� ������ ���������� ���������
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					out.resize(_count);

					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					// ��� 5. ������ �����: �������� ���� �������
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					_vk_res = wvk_physical_device_ptr->invokeWithVkPhysicalDeviceMethod(
						&WvkKhrSurfaceDT::wvkGetPhysicalDeviceSurfaceFormatsKHR,
						m_create_info.wvk_khr_surface_dt,
						m_vk_surface,
						&_count,
						out.data()
					);

					if (_vk_res != VK_SUCCESS) {
						switch (_vk_res) {
						case VK_INCOMPLETE:
							_status.append("\n\tWvkKhrSurfaceDispatchTable::wvkGetPhysicalDeviceSurfaceFormatsKHR - VK_INCOMPLETE.");
							break;
						case VK_ERROR_OUT_OF_HOST_MEMORY:
							_status.append("\n\tWvkKhrSurfaceDispatchTable::wvkGetPhysicalDeviceSurfaceFormatsKHR - VK_ERROR_OUT_OF_HOST_MEMORY.");
							break;
						case VK_ERROR_OUT_OF_DEVICE_MEMORY:
							_status.append("\n\tWvkKhrSurfaceDispatchTable::wvkGetPhysicalDeviceSurfaceFormatsKHR - VK_ERROR_OUT_OF_DEVICE_MEMORY.");
							break;
						case VK_ERROR_SURFACE_LOST_KHR:
							_status.append("\n\tWvkKhrSurfaceDispatchTable::wvkGetPhysicalDeviceSurfaceFormatsKHR - VK_ERROR_SURFACE_LOST_KHR.");
							break;
						}
						return _status.set(VknStatusCode::FAIL);
					}

					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					// ��� 6. �������� ����������
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					return _status.setOk();
				}

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// ��� 7. ���� VK_KHR_surface �� ����������� � ���������� ��������������� ������
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				return _status.set(VknStatusCode::FEATURE_NOT_ENABLED, "\n\tWvkSurface::requestSupport - not enabled.");
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			template<typename T>
			inline WvkStatus WvkSurface::requestFormats(const WvkPhysicalDevicePtr wvk_physical_device_ptr, const VkBaseInStructure* in, const VkStructureType& prop_sType, std::vector<T>& out_props) const noexcept {
				WvkStatus _status;

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// ��� 1. ���������� ��������� VkPhysicalDeviceSurfaceInfo2KHR ��� �������
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				VkPhysicalDeviceSurfaceInfo2KHR _info2 = {};
				_info2.sType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SURFACE_INFO_2_KHR;
				_info2.pNext = in;
				_info2.surface = m_vk_surface;

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// ��� 2. ������ �����: �������� ���������� �������������� ��������
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				uint32_t _count = 0;
				auto _vk_res = wvk_physical_device_ptr->invokeWithVkPhysicalDeviceMethod(
					&WvkKhrGetSurfaceCapabilities2DT::wvkGetPhysicalDeviceSurfaceFormats2KHR,
					m_create_info.wvk_khr_get_surface_capabilities2_dispatch_table,
					&_info2,
					&_count,
					nullptr
				);

				if (_vk_res != VK_SUCCESS) {
					switch (_vk_res) {
					case VK_ERROR_OUT_OF_HOST_MEMORY:
						_status.append("\n\tWvkKhrGetSurfaceCapabilities2DispatchTable::wvkGetPhysicalDeviceSurfaceFormats2KHR - VK_ERROR_OUT_OF_HOST_MEMORY.");
						break;
					case VK_ERROR_OUT_OF_DEVICE_MEMORY:
						_status.append("\n\tWvkKhrGetSurfaceCapabilities2DispatchTable::wvkGetPhysicalDeviceSurfaceFormats2KHR - VK_ERROR_OUT_OF_DEVICE_MEMORY.");
						break;
					case VK_ERROR_SURFACE_LOST_KHR:
						_status.append("\n\tWvkKhrGetSurfaceCapabilities2DispatchTable::wvkGetPhysicalDeviceSurfaceFormats2KHR - VK_ERROR_SURFACE_LOST_KHR.");
						break;
					}
					return _status.set(VknStatusCode::FAIL);
				}

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// ��� 3. �������� ������ ��� ������ VkSurfaceFormat2KHR � ������ out_props
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				std::vector<VkSurfaceFormat2KHR> _formats2(_count);
				out_props.resize(_count);

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// ��� 4. ������������� �������� VkSurfaceFormat2KHR � ������� pNext
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				for (uint32_t i = 0; i < _count; ++i) {
					_formats2[i].sType = VK_STRUCTURE_TYPE_SURFACE_FORMAT_2_KHR;
					_formats2[i].pNext = &out_props[i];

					out_props[i].sType = prop_sType;
					out_props[i].pNext = nullptr;
				}

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// ��� 5. ������ �����: �������� ����������� ���������� � ��������
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				_vk_res = wvk_physical_device_ptr->invokeWithVkPhysicalDeviceMethod(
					&WvkKhrGetSurfaceCapabilities2DT::wvkGetPhysicalDeviceSurfaceFormats2KHR,
					m_create_info.wvk_khr_get_surface_capabilities2_dispatch_table,
					&_info2,
					&_count,
					_formats2.data()
				);

				if (_vk_res != VK_SUCCESS) {
					switch (_vk_res) {
					case VK_ERROR_OUT_OF_HOST_MEMORY:
						_status.append("\n\tWvkKhrGetSurfaceCapabilities2DispatchTable::wvkGetPhysicalDeviceSurfaceFormats2KHR - VK_ERROR_OUT_OF_HOST_MEMORY.");
						break;
					case VK_ERROR_OUT_OF_DEVICE_MEMORY:
						_status.append("\n\tWvkKhrGetSurfaceCapabilities2DispatchTable::wvkGetPhysicalDeviceSurfaceFormats2KHR - VK_ERROR_OUT_OF_DEVICE_MEMORY.");
						break;
					case VK_ERROR_SURFACE_LOST_KHR:
						_status.append("\n\tWvkKhrGetSurfaceCapabilities2DispatchTable::wvkGetPhysicalDeviceSurfaceFormats2KHR - VK_ERROR_SURFACE_LOST_KHR.");
						break;
					}
					return _status.set(VknStatusCode::FAIL);
				}

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// ��� 6. ������� ��������� �������
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				return _status.setOk();
			}

		} // namespace Extensions

	} // namespace wvk

} // namespace CGDev

#endif // CGDEV_WVK_SOURCE_EXTENSIONS__WVK_SURFACE_HPP