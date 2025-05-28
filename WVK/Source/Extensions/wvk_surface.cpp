////////////////////////////////////////////////////////////////
// секция форвард-декларации
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// секция заголовочного файла
////////////////////////////////////////////////////////////////
#include "wvk_surface.h"
////////////////////////////////////////////////////////////////
// секция имплементации
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// секция для остального
////////////////////////////////////////////////////////////////
#include "MSWindows/wvk_surface_mswindows.h"
using WvkSurfacePlatform = CGDev::wvk::Extensions::mswindows::WvkSurfaceMSWindows;

#include "../wvk_instance_dt.hpp"
#include "../wvk_physical_device.h"
#include "wvk_khr_surface_dispatch_table.h"

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

				if constexpr (wvk::Build::ValidationBuildInfo::enable == true) {
							
					_status = validationCreateInfo();

							
					if (_status.m_code != VknStatusCode::SUCCESSFUL) {
						_status.m_code = VknStatusCode::FAIL;
						_status.append("\n\tVknExtDebugUtils::validationCreateInfo() - fail");
						return _status;
					}
				}

				m_surface_impl = std::make_unique<WvkSurfaceImpl>();

				_status = createVkSurface();

				if (!_status) {
					_status.set(VknStatusCode::FAIL, "\n\tWvkSurface::createVkSurface - fail.");
					return _status;
				}
												
				return _status.setOk();
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			void WvkSurface::destroy(void) noexcept {

				//m_create_info.wvk_khr_surface_commands->vkDestroyDebugUtilsMessengerEXT();

				// ~~~~~~~~~~~~~~~~
				// очистка данных
				// ~~~~~~~~~~~~~~~~

				m_create_info = {};
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			WvkStatus WvkSurface::validationCreateInfo(void) const noexcept {
				WvkStatus _status;

				if (m_create_info.wvk_instance_dispatch_table == nullptr) {
					return _status.set(VknStatusCode::FAIL, "\n\tWvkSurfaceCreateInfo::wvk_instance_dispatch_table - nullptr.");
				}
				else if (m_create_info.wvk_khr_surface_dispatch_table == nullptr) {
					return _status.set(VknStatusCode::FAIL, "\n\tWvkSurfaceCreateInfo::wvk_khr_surface_dispatch_table - nullptr.");
				}
				else if (m_create_info.wvk_khr_get_surface_capabilities2_dispatch_table == nullptr) {
					return _status.set(VknStatusCode::FAIL, "\n\tWvkSurfaceCreateInfo::wvk_khr_get_surface_capabilities2_dispatch_table - nullptr.");
				}
				else if (m_create_info.wvk_surface_platform_create_info == nullptr) {
					return _status.set(VknStatusCode::FAIL, "\n\tWvkSurfaceCreateInfo::wvk_surface_platform_create_info - nullptr.");
				}				
			
				return _status.setOk();
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					
			WvkStatus WvkSurface::createVkSurface() noexcept {
				WvkStatus _status;
						
				m_surface_impl->create(*static_cast<mswindows::WvkSurfaceMSWindowsCreateInfo*>(m_create_info.wvk_surface_platform_create_info));
						
				m_surface_impl->requestVkSurface(m_vk_surface);

				return _status.setOk();
			}

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		} // namespace Extensions

	} // namespace wvk

} // namespace CGDev