#ifndef CGDEV_WVK_SOURCE__WVK_LOADER_DISPATCH_TABLE_H
#define CGDEV_WVK_SOURCE__WVK_LOADER_DISPATCH_TABLE_H
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
		struct WvkLoaderDispatchTableCreateInfo {
			WvkLoaderPtr wvk_loader = nullptr;
		}; // struct WvkLoaderDispatchTableCreateInfo

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		/*!	\brief
		*/
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		class WvkLoaderDispatchTable : public GpuObject {

		public:

			// ~~~~~~~~~~~~~~~~
			// Vulkan 1.0
			// ~~~~~~~~~~~~~~~~
			inline VkResult wvkCreateInstance(const VkInstanceCreateInfo* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkInstance* pInstance) const noexcept;
			inline VkResult wvkEnumerateInstanceExtensionProperties(const char* pLayerName, uint32_t* pPropertyCount, VkExtensionProperties* pProperties) const noexcept;
			inline VkResult wvkEnumerateInstanceLayerProperties(uint32_t* pPropertyCount, VkLayerProperties* pProperties) const noexcept;

			// ~~~~~~~~~~~~~~~~
			// Vulkan 1.1
			// ~~~~~~~~~~~~~~~~
			inline VkResult wvkEnumerateInstanceVersion(uint32_t* pApiVersion) const noexcept;

		private:

			// ~~~~~~~~~~~~~~~~
			// Vulkan 1.0
			// ~~~~~~~~~~~~~~~~
			PFN_vkCreateInstance                           m_vkCreateInstance = VK_NULL_HANDLE;
			PFN_vkEnumerateInstanceExtensionProperties     m_vkEnumerateInstanceExtensionProperties = VK_NULL_HANDLE;
			PFN_vkEnumerateInstanceLayerProperties         m_vkEnumerateInstanceLayerProperties = VK_NULL_HANDLE;

			// ~~~~~~~~~~~~~~~~
			// Vulkan 1.1
			// ~~~~~~~~~~~~~~~~
			PFN_vkEnumerateInstanceVersion                 m_vkEnumerateInstanceVersion = VK_NULL_HANDLE;

		public:

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			WvkLoaderDispatchTable(void) noexcept;

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			~WvkLoaderDispatchTable(void) noexcept;

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			* ������ ������� ��������������� ������� Vulkan �� ������ ���������� ����������.
			*
			* @details
			* ����� ��������� �������� �� ��������� �������������, ��������� ���������,
			* ��������� ��������� ������� ������,
			* �������������� ������� ������� ����� ���������� ��������� ��������.
			*
			* @param[in] create_info
			* ��������� � ����������� �������� ������� �������� �������.
			*
			* @return
			* ���������� ������ ��������, ���������� ����� ��� ������� �������.
			*
			* @retval VknStatusCode::OK
			* ������� ������� �������.
			* @retval VknStatusCode::ALREADY_INITIALIZED
			* ������� ��� ���� ������� �����.
			* @retval VknStatusCode::FAIL
			* ������ ��������� ��� �������� ��������.
			*
			* @code
			* WvkLoaderDispatchTable table;
			* WvkLoaderDispatchTableCreateInfo info = { /* ��������� * / };
			* WvkStatus status = table.create(info);
			* if (!status) {
			*     std::cerr << "������ �������� ������� ���������������: " << status.message() << std::endl;
			* }
			* @endcode
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			WvkStatus create(const WvkLoaderDispatchTableCreateInfo& create_info) noexcept;

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			void destroy(void) noexcept;

		private:

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			* * ��������� �������� ���������� ��������� `WvkLoaderDispatchTableCreateInfo`.
			*
			* @details
			* * ����� ���������, ��� � ��������� `m_create_info` ��������� `wvk_loader` �� ����� `nullptr`.
			* * ���� ��������� �������, ������������ �������� ������. ����� � ������ � ��������������� ����������.
			*
			* @return
			* * ������ `WvkStatus`, ���������� ��������� ��������.
			*
			* @retval VknStatusCode::OK
			* * ��� ������������ ���� ������ ���������.
			* @retval VknStatusCode::FAIL
			* * ���� �� ������������ ����� �� ������ (� ���������, `wvk_loader == nullptr`).
			*
			* @code
			* WvkLoaderDispatchTable table;
			* // ... ���������� table.m_create_info.wvk_loader ...
			* WvkStatus status = table.validationCreateInfo();
			* if (!status) {
			*     std::cerr << status.message() << std::endl;
			* }
			* @endcode
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			WvkStatus validationCreateInfo(void) const noexcept;

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*  ��������� ��������� �� ���������� Vulkan-������� ����� `WvkLoader`.
			*
			* @details
			*  ����� �������� `WvkLoader::loadProcedure`, ��������� ������ ��������
			*  ��������� �������� Vulkan API (���������� ������� �� �������� ����������).
			*  ������ ������� �������������� ���������, � ������� ����� ������� ���������.
			*
			* @return
			*  ������ ����������. ���� ���� �� ���� ��������� �� ��������� � ������������ FAIL.
			*
			* @code
			*  WvkLoaderDispatchTable dispatch_table;
			*  WvkStatus status = dispatch_table.loadProcedure();
			*  if (!status) {
			*      // ��������� ������ �������� ��������
			*  }
			* @endcode
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			WvkStatus loadProcedure(void) noexcept;

			void reset(void) noexcept;

		private:

			WvkLoaderDispatchTableCreateInfo m_create_info;
		}; // class WvkLoaderDispatchTable

	} // namespace wvk

} // namespace CGDev

#include "wvk_loader_dispatch_table.hpp"

#endif // CGDEV_WVK_SOURCE__WVK_LOADER_DISPATCH_TABLE_H
