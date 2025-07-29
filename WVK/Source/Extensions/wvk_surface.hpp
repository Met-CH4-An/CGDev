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
#include "../wvk_instance_dispatch_table.h"
#include "../wvk_physical_device.h"
#include "../wvk_queue_family.h"

namespace CGDev {

	namespace wvk {

		namespace Extensions {

			

			

			

			

			

			

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			template<typename In, typename Out>
			inline WvkStatus WvkSurface::requestCapabilities(const WvkPhysicalDevicePtr wvk_physical_device_ptr, const In& in, Out& out) const noexcept {
				return requestCapabilities(wvk_physical_device_ptr, reinterpret_cast<const VkBaseInStructure*>(&in), reinterpret_cast<VkBaseOutStructure*>(&out));
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
				/*auto _vk_res = wvk_physical_device_ptr->invokeWithVkPhysicalDeviceMethod(
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
				}*/

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
				/*_vk_res = wvk_physical_device_ptr->invokeWithVkPhysicalDeviceMethod(
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
				}*/

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// ��� 6. ������� ��������� �������
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				return _status.setOk();
			}

		} // namespace Extensions

	} // namespace wvk

} // namespace CGDev

#endif // CGDEV_WVK_SOURCE_EXTENSIONS__WVK_SURFACE_HPP