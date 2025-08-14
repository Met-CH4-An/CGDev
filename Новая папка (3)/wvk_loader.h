#ifndef CGDEV_WVK_SOURCE__WVK_LOADER_H
#define CGDEV_WVK_SOURCE__WVK_LOADER_H
////////////////////////////////////////////////////////////////
// ������ �������-����������
////////////////////////////////////////////////////////////////
#include "_wvk.h"
////////////////////////////////////////////////////////////////
// ������ �������������
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// ������ ������������� ������
////////////////////////////////////////////////////////////////
#include "wvk_base_object.h"
////////////////////////////////////////////////////////////////
// ������ ��� ����������
////////////////////////////////////////////////////////////////

namespace CGDev {

	namespace wvk {

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		/*!	\brief
		*/
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		struct WvkVulkanProcedureInfo {
			const char* name;       ///< �������� ������� Vulkan, �������� "vkDestroySurfaceKHR"
			void** targetPtr;       ///< ��������� �� ����������, ���� ����� ������� ����������� �����
			WvkVulkanProcedureInfo(const char* n, void** ptr)
				: name(n), targetPtr(ptr) {
			}
		}; // struct WvkVulkanProcedureInfo

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		/*!	\brief
		*/
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		struct WvkLoaderCreateInfo {
		}; // WvkLoaderCreateInfo

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		/*!	\brief
		*/
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		class WvkLoader : public GpuObject {

		public:

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			WvkLoader(void) noexcept;

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			~WvkLoader(void) noexcept;

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			* * �������������� ��������� Vulkan-������� � �������������� ���������� ����������.
			*
			* @details
			* ����� ��������� ������������� �� ������ ��������� `WvkLoaderCreateInfo`.
			* ��������� �������� ������� ������.
			* ����� �������� ������������� �������� ���������� ���������� (`WvkLoaderImpl`)
			* � ������������� ������� `vkGetInstanceProcAddr`. ���� ����� ��� ����������� �������� �
			* ���������� `reset()` � ������������ ������ ������.
			*
			* @param[in] create_info
			*	��������� � ����������� ������������� ����������.
			*
			* @return
			*	������ `WvkStatus`, ����������� ��������� ���������� ��������.
			*
			* @retval VknStatusCode::OK
			*	������������� ��������� �������.
			* @retval VknStatusCode::ALREADY_INITIALIZED
			*	����� ������ ��������, ��������� ��� ���������������.
			* @retval VknStatusCode::FAIL
			*	��������� ������ ��� ����� �� ������ �������������.
			*
			* @code
			* WvkLoaderCreateInfo create_info = {};
			* create_info.log = &logger;
			* create_info.vkn_commands = &vkn_commands;
			* 
			* WvkLoader loader;
			* WvkStatus status = loader.create(create_info);
			* if (!status) {
			*     std::cerr << status.message() << std::endl;
			* }
			* @endcode
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			WvkStatus create(const WvkLoaderCreateInfo& create_info) noexcept;

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			void destroy(void) noexcept;

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief �������� �������� Vulkan ������ ���������� (loader-level) � �������������� vkGetInstanceProcAddr.
			*
			* @param[in,out] wvk_vulkan_procedure_collection1
			* * ��������� � ���������� �������� Vulkan (��� + ��������� �� ����� �������� ������).
			*
			* @return
			* * ������ WvkStatus, ���������� ��� � ��������� �� ������, ���� �������� �� �������.
			*
			* @code
			* std::vector<WvkVulkanProcedureInfo> procedures = {
			*     { "vkCreateInstance", reinterpret_cast<void**>(&m_vkCreateInstance) },
			*     { "vkEnumerateInstanceVersion", reinterpret_cast<void**>(&m_vkEnumerateInstanceVersion) }
			* };
			* _status = wvk_loader->loadProcedure(procedures);
			* if (!_status) {
			*     // ��������� ������
			* }
			* @endcode
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			WvkStatus loadProcedure(std::vector<WvkVulkanProcedureInfo>& wvk_vulkan_procedure_collection1) const noexcept;

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			* * ��������� ��������� �� Vulkan-������� ��� ��������� ���������� Vulkan.
			*
			* @details
			* * ����� �������� ������ ������� Vulkan � ������� vkGetInstanceProcAddr � ����������
			* * �� � ���������, �������� � ��������� `wvk_vulkan_procedure_collection1`.
			* * � ������ ������� �� ����� �� �������� ���������� ������ � ����� FAIL � �������
			* * �� ��������� �������.
			*
			* @param[in] vk_instance
			* * ����� Vulkan-��������, ��� �������� ����� ��������� �������.
			*
			* @param[in] wvk_vulkan_procedure_collection1
			* * ��������� ���������� ��������, ���������� ����� ������� � ��������� ��� ������.
			*
			* @return
			* * ������ WvkStatus, ����������� ��������� ���������� ��������.
			*
			* @retval VknStatusCode::OK
			* * ��� ������� ������� ���������.
			* @retval VknStatusCode::FAIL
			* * ���� ��� ��������� ������� �� ���� �������, ����������� � � ���������.
			*
			* @code
			* std::vector<WvkVulkanProcedureInfo> procs = {
			*     { "vkDestroyInstance", reinterpret_cast<void**>(&vkDestroyInstance) },
			*     { "vkEnumeratePhysicalDevices", reinterpret_cast<void**>(&vkEnumeratePhysicalDevices) }
			* };
			* 
			* WvkStatus status = loader.loadProcedure(instance, procs);
			* if (!status) {
			*     std::cerr << status.message() << std::endl;
			* }
			* @endcode
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			WvkStatus loadProcedure(VkInstance vk_instance, std::vector<WvkVulkanProcedureInfo>& wvk_vulkan_procedure_collection1) const noexcept;

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief �������� �������� Vulkan ������ ���������� (loader-level) � �������������� vkGetInstanceProcAddr.
			*
			* @param[in,out] wvk_vulkan_procedure_collection1
			* * ��������� � ���������� �������� Vulkan (��� + ��������� �� ����� �������� ������).
			*
			* @return
			* * ������ WvkStatus, ���������� ��� � ��������� �� ������, ���� �������� �� �������.
			*
			* @code
			* std::vector<WvkVulkanProcedureInfo> procedures = {
			*     { "vkCreateInstance", reinterpret_cast<void**>(&m_vkCreateInstance) },
			*     { "vkEnumerateInstanceVersion", reinterpret_cast<void**>(&m_vkEnumerateInstanceVersion) }
			* };
			* _status = wvk_loader->loadProcedure(procedures);
			* if (!_status) {
			*     // ��������� ������
			* }
			* @endcode
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			WvkStatus loadProcedure(VkInstance vk_instance, VkDevice vk_device, std::vector<WvkVulkanProcedureInfo>& wvk_vulkan_procedure_collection1) const noexcept;
		
		// hpp
		public:

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			inline const WvkLoaderCreateInfo& getCreateInfo(void) const noexcept;

		// hpp
		private:

		// cpp
		private:

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			WvkStatus validationCreateInfo(void) const noexcept;

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			* * ���������� ���������� ��������� ���������� `WvkLoader` � ��������� ���������.
			*
			* @details
			* * ����� ����������� �������, ������� ��������� `create_info`, ���������� ���������
			*   �� `vkGetInstanceProcAddr`, ������� ���������� ���������� � �������� ������ ��� ����������.
			*
			* * ���������� ��� ������� � `create`, ���� ��� ����������������� ����������.
			*
			* @code
			* WvkLoader loader;
			* // ... ����� ������������� ...
			* loader.reset(); // ������� � ������������ ���� ��������
			* @endcode
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			void reset(void) noexcept;

		private:

			WvkLoaderCreateInfo m_create_info = {};
			PFN_vkGetInstanceProcAddr m_vkGetInstanceProcAddr = VK_NULL_HANDLE;
			struct WvkLoaderImpl;
			std::unique_ptr<WvkLoaderImpl> m_loader_impl = nullptr;
		}; // class WvkLoader

	} // namespace wvk

} // namespace CGDev

#include "wvk_loader.hpp"

#endif // CGDEV_WVK_SOURCE__WVK_LOADER_H
