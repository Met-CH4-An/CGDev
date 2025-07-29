#ifndef CGDEV_WVK_SOURCE_EXTENSIONS__WVK_KHR_SURFACE_DT_HPP
#define CGDEV_WVK_SOURCE_EXTENSIONS__WVK_KHR_SURFACE_DT_HPP
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
#include "../wvk_loader.h"
#include "../wvk_instance.h"

namespace CGDev {

	namespace wvk {

		namespace Extensions {

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			struct WvkKhrSurfaceDTCreateInfo {
				WvkLoaderPtr wvk_loader = nullptr;
				WvkInstancePtr wvk_instance = nullptr;
			}; // WvkKhrSurfaceDTCreateInfo

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			class WvkKhrSurfaceDT : public GpuObject {

			public:

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				inline static constexpr std::string_view s_getName(void) noexcept {
					return s_name;
				}

			private:

				inline static constexpr std::string_view s_name = "VK_KHR_surface";

			public:
						
				inline void wvkDestroySurfaceKHR(VkSurfaceKHR surface, const VkAllocationCallbacks* pAllocator) const noexcept {
					m_create_info.wvk_instance->invokeWithVkInstanceFunction(m_vkDestroySurfaceKHR, surface, pAllocator); }
				inline VkResult wvkGetPhysicalDeviceSurfaceSupportKHR(VkPhysicalDevice physicalDevice, uint32_t queueFamilyIndex, VkSurfaceKHR surface, VkBool32* pSupported) const noexcept {
					return m_vkGetPhysicalDeviceSurfaceSupportKHR(physicalDevice, queueFamilyIndex, surface, pSupported); }
				inline VkResult wvkGetPhysicalDeviceSurfaceCapabilitiesKHR(VkPhysicalDevice physicalDevice, VkSurfaceKHR surface, VkSurfaceCapabilitiesKHR* pSurfaceCapabilities) const noexcept {
					return m_vkGetPhysicalDeviceSurfaceCapabilitiesKHR(physicalDevice, surface, pSurfaceCapabilities); }
				inline VkResult wvkGetPhysicalDeviceSurfaceFormatsKHR(VkPhysicalDevice physicalDevice, VkSurfaceKHR surface, uint32_t* pSurfaceFormatCount, VkSurfaceFormatKHR* pSurfaceFormats) const noexcept {
					return m_vkGetPhysicalDeviceSurfaceFormatsKHR(physicalDevice, surface, pSurfaceFormatCount, pSurfaceFormats); }
				inline VkResult wvkGetPhysicalDeviceSurfacePresentModesKHR(VkPhysicalDevice physicalDevice, VkSurfaceKHR surface, uint32_t* pPresentModeCount, VkPresentModeKHR* pPresentModes) const noexcept {
					return m_vkGetPhysicalDeviceSurfacePresentModesKHR(physicalDevice, surface, pPresentModeCount, pPresentModes); }
						
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
				inline WvkKhrSurfaceDT(void) noexcept {}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				inline ~WvkKhrSurfaceDT(void) noexcept {}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				* �������������� ������ WvkKhrSurfaceDT, �������� ��������� � �������� Vulkan-������.
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
				* WvkKhrSurfaceDT surface;
				* WvkKhrSurfaceDTCreateInfo info = { ... };
				* WvkStatus status = surface.create(info);
				* if (!status) {
				*     // ��������� ������
				* }
				* @endcode
				*
				* @see WvkKhrSurfaceDT::validationCreateInfo
				* @see WvkKhrSurfaceDT::loadVulkanCommand
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				inline WvkStatus create(const WvkKhrSurfaceDTCreateInfo& create_info) noexcept {
					WvkStatus _status;

					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					// ��� 1. �������� �� ��������� �������������
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					if (m_valid) {
						return _status.set(VknStatusCode::ALREADY_INITIALIZED);
					}

					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					// ��� 2. ��������� ���������� ��������� ��������.
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					m_create_info = create_info;

					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					// ��� 3. ��������� �������� ������.
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~				
					_status = validationCreateInfo();

					if (_status.m_code != VknStatusCode::SUCCESSFUL) {
						reset();
						return _status.set(VknStatusCode::FAIL, "\n\tWvkKhrSurfaceDispatchTable::validationCreateInfo() - fail.");
					}
					
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					// ��� 4. ��������� ��������� �� Vulkan-�������, ��������� � Surface.
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					_status = loadVulkanCommand();

					if (_status.m_code != VknStatusCode::SUCCESSFUL) {
						reset();
						return _status.set(VknStatusCode::FAIL, "\n\tWvkKhrSurfaceDispatchTable::loadVulkanCommand() - fail.");
					}

					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					// ��� 5. �������� ���������� ��������.
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
						
			private:

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				* ��������� ������������ ��������� WvkKhrSurfaceCreateInfo.
				*
				* @details
				* ����� ��������� ������� ��������� ������� ������, ����������� ��� ����������
				* ������ WvkKhrSurfaceDT. �� ������� ����� ����������� ������ ������� ���������
				* �� WvkCommands. ���� �� ����� nullptr, ������������ ������.
				*
				* @retval VknStatusCode::SUCCESSFUL
				* ��� ������������ ������ ������������.
				*
				* @retval VknStatusCode::FAIL
				* ��������� �� WvkCommands ����� nullptr.
				*
				* @code
				* WvkKhrSurfaceDT surface;
				* WvkStatus status = surface.validationCreateInfo();
				* if (!status) {
				*     // ��������� ������
				* }
				* @endcode
				*
				* @see WvkKhrSurfaceDT::loadVulkanCommand
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				inline WvkStatus validationCreateInfo(void) const noexcept {
					WvkStatus _status;

					if (m_create_info.wvk_loader == nullptr) {
						return _status.set(VknStatusCode::FAIL, "\n\tWvkKhrSurfaceDispatchTableCreateInfo::wvk_loader - nullptr.");
					}
					else if (m_create_info.wvk_instance == nullptr) {
						return _status.set(VknStatusCode::FAIL, "\n\tWvkKhrSurfaceDispatchTableCreateInfo::wvk_instance - nullptr.");
					}

					return _status.setOk();
				}

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
				* WvkKhrSurfaceDT table;
				* WvkStatus status = table.loadVulkanCommand();
				* if (!status) {
				*     std::cerr << "�� ������� ��������� ������� VK_KHR_surface: " << status.message() << std::endl;
				* }
				* @endcode
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				inline WvkStatus loadVulkanCommand(void) noexcept {
					WvkStatus _status;

					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					// ��� 1. ���������� ������ ��������, ������� ���������� ���������
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					std::vector<WvkVulkanProcedureInfo> _procedures = {
						{ "vkDestroySurfaceKHR", reinterpret_cast<void**>(&m_vkDestroySurfaceKHR) },
						{ "vkGetPhysicalDeviceSurfaceSupportKHR", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceSurfaceSupportKHR) },
						{ "vkGetPhysicalDeviceSurfaceCapabilitiesKHR", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceSurfaceCapabilitiesKHR) },
						{ "vkGetPhysicalDeviceSurfaceFormatsKHR", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceSurfaceFormatsKHR) },
						{ "vkGetPhysicalDeviceSurfacePresentModesKHR", reinterpret_cast<void**>(&m_vkGetPhysicalDeviceSurfacePresentModesKHR) }
					};

					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					// ��� 2. ����� ������ �������� ����� ������ � VkInstance
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					//_status = m_create_info.wvk_instance->invokeWithVkInstanceMethod(
					//	&WvkLoader::loadProcedure,
					//	m_create_info.wvk_loader,
					//	_procedures
					//);

					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					// ��� 3. �������� ���������� ��������
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					if (!_status) {
						return _status.set(VknStatusCode::FAIL,
							"\n\tWvkInstance::invokeWithVkInstanceMethod - fail.");
					}

					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					// ��� 4. ������� ��������� �������
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					return _status.setOk();
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				inline void reset(void) noexcept {
					m_vkDestroySurfaceKHR = VK_NULL_HANDLE;
					m_vkGetPhysicalDeviceSurfaceSupportKHR = VK_NULL_HANDLE;
					m_vkGetPhysicalDeviceSurfaceCapabilitiesKHR = VK_NULL_HANDLE;
					m_vkGetPhysicalDeviceSurfaceFormatsKHR = VK_NULL_HANDLE;
					m_vkGetPhysicalDeviceSurfacePresentModesKHR = VK_NULL_HANDLE;

					m_create_info = {};

					m_valid = false;
				}

			private:

				WvkKhrSurfaceDTCreateInfo m_create_info = {};
			}; // class WvkKhrSurfaceDT

		} // namespace Extensions

	} // namespace wvk

} // namespace CGDev

#endif // CGDEV_WVK_SOURCE_EXTENSIONS__WVK_KHR_SURFACE_DT_HPP
