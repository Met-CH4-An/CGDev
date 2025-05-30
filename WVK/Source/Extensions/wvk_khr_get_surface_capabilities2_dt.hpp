#ifndef CGDEV_WVK_SOURCE_EXTENSIONS__WVK_KHR_GET_SURFACE_CAPABILITIES2_DT_HPP
#define CGDEV_WVK_SOURCE_EXTENSIONS__WVK_KHR_GET_SURFACE_CAPABILITIES2_DT_HPP
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
			struct WvkKhrGetSurfaceCapabilities2DTCreateInfo {
				WvkLoaderPtr wvk_loader = nullptr;
				WvkInstancePtr wvk_instance = nullptr;
			}; // WvkKhrGetSurfaceCapabilities2DTCreateInfo

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			class WvkKhrGetSurfaceCapabilities2DT : public GpuObject {

			public:

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				inline static constexpr std::string_view s_getName(void) noexcept {
					return s_name;
				}

			private:

				inline static constexpr std::string_view s_name = "VK_KHR_get_surface_capabilities2";

			public:

				inline VkResult wvkGetPhysicalDeviceSurfaceCapabilities2KHR(VkPhysicalDevice physicalDevice, const VkPhysicalDeviceSurfaceInfo2KHR* pSurfaceInfo, VkSurfaceCapabilities2KHR* pSurfaceCapabilities) const noexcept {
					return m_vkGetPhysicalDeviceSurfaceCapabilities2KHR(physicalDevice, pSurfaceInfo, pSurfaceCapabilities); }
				inline VkResult wvkGetPhysicalDeviceSurfaceFormats2KHR(VkPhysicalDevice physicalDevice, const VkPhysicalDeviceSurfaceInfo2KHR* pSurfaceInfo, uint32_t* pSurfaceFormatCount, VkSurfaceFormat2KHR* pSurfaceFormats) const noexcept {
					return m_vkGetPhysicalDeviceSurfaceFormats2KHR(physicalDevice, pSurfaceInfo, pSurfaceFormatCount, pSurfaceFormats); }

			private:

				PFN_vkGetPhysicalDeviceSurfaceCapabilities2KHR m_vkGetPhysicalDeviceSurfaceCapabilities2KHR = VK_NULL_HANDLE;
				PFN_vkGetPhysicalDeviceSurfaceFormats2KHR m_vkGetPhysicalDeviceSurfaceFormats2KHR = VK_NULL_HANDLE;

			public:

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				inline WvkKhrGetSurfaceCapabilities2DT(void) noexcept {}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				inline ~WvkKhrGetSurfaceCapabilities2DT(void) noexcept {}

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
				inline WvkStatus create(const WvkKhrGetSurfaceCapabilities2DTCreateInfo& create_info) noexcept {
					WvkStatus _status;

					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					// ��� 1. ��������� ���������� ��������� ��������.
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					m_create_info = create_info;

					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					// ��� 2. ��������� �������� ������.
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~					
					_status = validationCreateInfo();

					if (!_status) {
						reset();
						return _status.set(VknStatusCode::FAIL, "\n\tWvkKhrGetSurfaceCapabilities2DT::validationCreateInfo() - fail.");
					}					

					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					// ��� 3. ��������� ��������� �� Vulkan-�������, ��������� � Surface.
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					_status = loadVulkanCommand();

					if (!_status) {
						reset();
						return _status.set(VknStatusCode::FAIL, "\n\tWvkKhrGetSurfaceCapabilities2DT::loadVulkanCommand() - fail.");
					}

					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					// ��� 4. �������� ���������� ��������.
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					m_valid = true;
					return _status.setOk();
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				inline void destroy(void) noexcept {
					reset();
				}

			public:

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
				inline WvkStatus validationCreateInfo(void) const noexcept {
					WvkStatus _status;

					if (m_create_info.wvk_loader == nullptr) {
						return _status.set(VknStatusCode::FAIL, "\n\tWvkKhrGetSurfaceCapabilities2DispatchTableCreateInfo::wvk_loader - nullptr.");
					}
					else if (m_create_info.wvk_instance == nullptr) {
						return _status.set(VknStatusCode::FAIL, "\n\tWvkKhrGetSurfaceCapabilities2DispatchTableCreateInfo::wvk_instance - nullptr.");
					}

					return _status.setOk();
				}

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
				inline WvkStatus loadVulkanCommand(void) noexcept {
					WvkStatus _status;

					std::vector<WvkVulkanProcedureInfo> _procedures = {
						{ "vkGetPhysicalDeviceSurfaceCapabilities2KHR", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceSurfaceCapabilities2KHR) },
						{ "vkGetPhysicalDeviceSurfaceFormats2KHR", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceSurfaceFormats2KHR) }
					};

					_status = m_create_info.wvk_instance->invokeWithVkInstanceMethod(
						&WvkLoader::loadProcedure,
						m_create_info.wvk_loader,
						_procedures
					);

					if (!_status) {
						return _status.set(VknStatusCode::FAIL,
							"\n\tWvkInstance::invokeWithVkInstanceMethod - fail.");
					}

					return _status.setOk();
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				inline void reset(void) noexcept {
					m_vkGetPhysicalDeviceSurfaceCapabilities2KHR = VK_NULL_HANDLE;
					m_vkGetPhysicalDeviceSurfaceFormats2KHR = VK_NULL_HANDLE;

					m_create_info = {};

					m_valid = false;
				}

			private:

				WvkKhrGetSurfaceCapabilities2DTCreateInfo m_create_info = {};
			}; // class WvkKhrGetSurfaceCapabilities2DT

		} // namespace Extensions

	} // namespace wvk

} // namespace CGDev

#endif // CGDEV_WVK_SOURCE_EXTENSIONS__WVK_KHR_GET_SURFACE_CAPABILITIES2_DT_HPP
