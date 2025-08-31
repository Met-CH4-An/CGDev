// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025 Metelev Met-CH4-An Andrey
// Project Lead: Metelev Andrey
// Main Technical Assistant: StarDev aka ChatGPT
////////////////////////////////////////////////////////////////
// ������ �������-����������
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// ������ ������������� �����
////////////////////////////////////////////////////////////////
#include "wvk_surface.h"
////////////////////////////////////////////////////////////////
// ������ �������������
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// ������ ��� ����������
////////////////////////////////////////////////////////////////
#include "MSWindows/wvk_surface_mswindows.h"
using WvkSurfacePlatform = CGDev::wvk::Extensions::mswindows::WvkSurfaceMSWindows;

#include "../wvk_physical_device.h"
#include "../wvk_instance.h" 

namespace CGDev {

	namespace wvk {

		namespace Extensions {
			
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			struct WvkSurface::WvkSurfaceImpl {
			public:
				WvkSurfaceImpl() = default;
				~WvkSurfaceImpl() = default;

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				WvkStatus create(mswindows::WvkSurfaceMSWindowsCreateInfo& create_info) {
					if (Build::SurfaceBuildInfo::platform_type == Build::PlatformType::MSWindows) {
						WvkStatus _status;
															
						m_surface_platform.create(create_info);

						return _status.setOk();
					}
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				void requestVkSurface(VkSurfaceKHR& vk_surface_khr) {
					m_surface_platform.requestVkSurface(vk_surface_khr);
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				void destroy() {

				}

			private:
				WvkSurfacePlatform m_surface_platform;
			};

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			WvkSurface::WvkSurface(void) noexcept {
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			WvkSurface::~WvkSurface(void) noexcept {
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			WvkStatus WvkSurface::create(const WvkSurfaceCreateInfo& create_info) noexcept {
				WvkStatus _status;

				m_create_info = create_info;
							
				_status = validationCreateInfo();
						
				if (!_status) {
					reset();
					return _status.set(VknStatusCode::FAIL, "\n\tVknExtDebugUtils::validationCreateInfo() - fail.");
				}

				m_surface_impl = std::make_unique<WvkSurfaceImpl>();

				_status = createVkSurface();

				if (!_status) {
					reset();
					return _status.set(VknStatusCode::FAIL, "\n\tWvkSurface::createVkSurface - fail.");
				}
							
				m_valid = true;
				return _status.setOk();
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			void WvkSurface::destroy(void) noexcept {

				//m_create_info.wvk_khr_surface_commands->vkDestroyDebugUtilsMessengerEXT();

				// ~~~~~~~~~~~~~~~~
				// ������� ������
				// ~~~~~~~~~~~~~~~~

				m_create_info = {};
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			WvkStatus WvkSurface::requestFormats(const WvkPhysicalDevicePtr wvk_physical_device_ptr, std::vector<VkSurfaceFormatKHR>& out) const noexcept {
				WvkStatus _status;

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// ��� 1. ���������, ������������ �� ���������� VK_KHR_surface
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*if constexpr (Build::find(WvkKhrSurfaceDT::s_getName())) {

					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					// ��� 2. �������� ������ � ����������� ���������� ��� �������� ��������
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					uint32_t _count = 0;
					out.clear();

					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					// ��� 3. ������ �����: �������� ���������� �������������� ��������
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					//m_create_info.wvk_instance_dt_ptr->
					//auto _vk_res = wvk_physical_device_ptr->invokeWithVkPhysicalDeviceMethod(
					//	&WvkKhrSurfaceDT::wvkGetPhysicalDeviceSurfaceFormatsKHR,
					//	m_create_info.wvk_khr_surface_dt,
					//	m_vk_surface,
					//	&_count,
					//	nullptr
					//);

					/*if (_vk_res != VK_SUCCESS) {
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
					}*/

					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					// ��� 4. �������� ����� � ������� ��� ������ ���������� ���������
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					//out.resize(_count);

					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					// ��� 5. ������ �����: �������� ���� �������
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					//_vk_res = wvk_physical_device_ptr->invokeWithVkPhysicalDeviceMethod(
					//	&WvkKhrSurfaceDT::wvkGetPhysicalDeviceSurfaceFormatsKHR,
					//	m_create_info.wvk_khr_surface_dt,
					//	m_vk_surface,
					//	&_count,
					//	out.data()
					//);

					/*if (_vk_res != VK_SUCCESS) {
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
					}*/

					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					// ��� 6. �������� ����������
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					return _status.setOk();
				//}*/

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// ��� 7. ���� VK_KHR_surface �� ����������� � ���������� ��������������� ������
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				return _status.set(VknStatusCode::FEATURE_NOT_ENABLED, "\n\tVK_KHR_surface - not enabled.");
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			WvkStatus WvkSurface::requestSupport(const WvkPhysicalDevicePtr wvk_physical_device_ptr, const WvkQueueFamilyPtr wvk_queue_family_ptr, bool& out) const noexcept {
				WvkStatus _status;

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// ��� 1. ���������, ������������ �� ���������� VK_KHR_surface
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*if constexpr (Build::find(WvkKhrSurfaceDT::s_getName())) {
					/*VkBool32 _support = VK_FALSE;
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
					return _status.setOk();*/
				//}*/

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// ��� 6. ���� VK_KHR_surface �� ����������� � ���������� ��������������� ������
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				return _status.set(VknStatusCode::FEATURE_NOT_ENABLED, "\n\tWvkSurface::requestSupport - not enabled.");
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			WvkStatus WvkSurface::requestPresentModes(const WvkPhysicalDevicePtr wvk_physical_device_ptr, std::vector<VkPresentModeKHR>& vk_present_modes) const noexcept {
				WvkStatus _status;

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// ��� 1. ���������, ������������ �� ���������� VK_KHR_surface
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//if constexpr (Build::find(WvkKhrSurfaceDT::s_getName())) {

					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					// ��� 1. ������������� ������� �������
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*WvkStatus _status;

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
					return _status.setOk();*/
				//}

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// ��� 5. ���� VK_KHR_surface �� ����������� � ���������� ��������������� ������
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				return _status.set(VknStatusCode::FEATURE_NOT_ENABLED, "\n\tWvkSurface::requestSupport - not enabled.");
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			WvkStatus WvkSurface::requestCapabilities(const WvkPhysicalDevicePtr wvk_physical_device_ptr, VkSurfaceCapabilitiesKHR& vk_surface_capabilities_khr) const noexcept {
				WvkStatus _status;

				//if constexpr (Build::find(WvkKhrSurfaceDT::s_getName())) {

					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					// ��� 1. ����� ������ ��������� surface capabilities ����� invoke
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*auto _vk_res = wvk_physical_device_ptr->invokeWithVkPhysicalDeviceMethod(
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
					return _status.setOk();*/
				//}

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// ��� 5. ���� VK_KHR_surface �� ����������� � ���������� ��������������� ������
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				return _status.set(VknStatusCode::FEATURE_NOT_ENABLED, "\n\tWvkSurface::requestSupport - not enabled.");
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			WvkStatus WvkSurface::requestCapabilities(const WvkPhysicalDevicePtr wvk_physical_device_ptr, const VkBaseInStructure* in, VkBaseOutStructure* out) const noexcept {
				WvkStatus _status;

				//if constexpr (Build::vulkan_api_version >= Build::VulkanVersion::VERSION_11 ||
					/*Build::find(WvkKhrGetSurfaceCapabilities2DT::s_getName())) {

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
						&WvkInstanceDispatchTable::wvkGetPhysicalDeviceSurfaceCapabilities2KHR,
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
					}*/

					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					// ��� 5. �������� ����������
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					//return _status.setOk();
				//}

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// ��� 6. ���� VK_KHR_surface �� ����������� � ���������� ��������������� ������
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				return _status.set(VknStatusCode::FEATURE_NOT_ENABLED, "\n\tWvkSurface::requestSupport - not enabled.");
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			WvkStatus WvkSurface::requestCapabilities(const WvkPhysicalDevicePtr wvk_physical_device_ptr, VkSurfaceProtectedCapabilitiesKHR& out) const noexcept {
				out.sType = VK_STRUCTURE_TYPE_SURFACE_PROTECTED_CAPABILITIES_KHR;
				out.pNext = nullptr;
				return requestCapabilities(wvk_physical_device_ptr, nullptr, reinterpret_cast<VkBaseOutStructure*>(&out));
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			WvkStatus WvkSurface::requestPresentModeCompatibility(const WvkPhysicalDevicePtr wvk_physical_device_ptr, const VkPresentModeKHR& vk_present_mode, std::vector<VkPresentModeKHR>& out) const noexcept {
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

			WvkStatus WvkSurface::requestScalingCompatibility(const WvkPhysicalDevicePtr wvk_physical_device_ptr, const VkPresentModeKHR& vk_present_mode, VkSurfacePresentScalingCapabilitiesEXT& out) const noexcept {
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

			WvkStatus WvkSurface::validationCreateInfo(void) const noexcept {
				WvkStatus _status;

				//if (m_create_info.wvk_instance_dt_ptr == nullptr) {
				//	return _status.set(VknStatusCode::FAIL, "\n\tWvkSurfaceCreateInfo::wvk_instance_dt_ptr - nullptr.");
				//}
				//else if (m_create_info.wvk_khr_surface_dt == nullptr) {
				//	return _status.set(VknStatusCode::FAIL, "\n\tWvkSurfaceCreateInfo::wvk_khr_surface_dt - nullptr.");
				//}
				//else if (m_create_info.wvk_khr_get_surface_capabilities2_dispatch_table == nullptr) {
				//	return _status.set(VknStatusCode::FAIL, "\n\tWvkSurfaceCreateInfo::wvk_khr_get_surface_capabilities2_dispatch_table - nullptr.");
				//}
				//else if (m_create_info.wvk_surface_platform_create_info == nullptr) {
				//	return _status.set(VknStatusCode::FAIL, "\n\tWvkSurfaceCreateInfo::wvk_surface_platform_create_info - nullptr.");
				//}				
			
				return _status.setOk();
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					
			WvkStatus WvkSurface::createVkSurface() noexcept {
				WvkStatus _status;

				VkWin32SurfaceCreateInfoKHR _create_info = {
					.sType = VK_STRUCTURE_TYPE_WIN32_SURFACE_CREATE_INFO_KHR,
					.pNext = nullptr,
					.flags = 0,
					.hinstance = static_cast<HINSTANCE>(m_create_info.win32.hinstance),
					.hwnd = static_cast<HWND>(m_create_info.win32.hwnd),
				};
				auto _vk_result = m_create_info.wvk_instance_ptr->getWvkDispatchTable()->wvkCreateWin32SurfaceKHR(&_create_info, nullptr, &m_vk_surface);
				//m_surface_impl->create(*static_cast<mswindows::WvkSurfaceMSWindowsCreateInfo*>(m_create_info.wvk_surface_platform_create_info));
						
				//m_surface_impl->requestVkSurface(m_vk_surface);

				return _status.setOk();
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			void WvkSurface::reset(void) noexcept {
				m_create_info = {};
				m_vk_surface = VK_NULL_HANDLE;

				m_valid = false;
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		} // namespace Extensions

	} // namespace wvk

} // namespace CGDev