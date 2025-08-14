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
#include "wvk_dispatch_table.h"
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
		class WvkLoaderDispatchTable : public WvkDispatchTable {

		public:

			// =======================================
			// [Category]: Global
			// =======================================

			// ~~~~~~~~~~~~~~~~
			// [Version] 1.0
			// ~~~~~~~~~~~~~~~~
			inline VkResult wvkCreateInstance(const VkInstanceCreateInfo* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkInstance* pInstance) const noexcept;
			inline VkResult wvkEnumerateInstanceExtensionProperties(const char* pLayerName, uint32_t* pPropertyCount, VkExtensionProperties* pProperties) const noexcept;
			inline VkResult wvkEnumerateInstanceLayerProperties(uint32_t* pPropertyCount, VkLayerProperties* pProperties) const noexcept;

			// ~~~~~~~~~~~~~~~~
			// Vulkan 1.1
			// ~~~~~~~~~~~~~~~~
			inline VkResult wvkEnumerateInstanceVersion(uint32_t* pApiVersion) const noexcept;

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

		// hpp
		public:

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			inline const WvkLoaderDispatchTableCreateInfo& getCreateInfo(void) const noexcept;

		// cpp
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
			/*!	\brief ��������� ��������� �� ���������� ������� ���������� Vulkan.
			*
			* @details
			* ����� ��������� ������ ���������� �������� Vulkan, ��������� �� �������� ��������,
			* � �������� ����� �������� �� `wvk_loader`, ������� �������� ������ ���� �������.
			* � ������ ������ ������������� ������ FAIL � ���������� ���.
			*
			* @return
			* WvkStatus � ��������� ���������� �������� ��������.
			*
			* @code
			* WvkLoaderDispatchTable loader_dispatch_table;
			* WvkStatus status = loader_dispatch_table.loadProcedure();
			* if (!status) {
			*     // ��������� ������ �������� ���������� ������� Vulkan
			* }
			* @endcode
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			WvkStatus loadProcedure(void) noexcept;

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			void reset(void) noexcept;

		private:

			WvkLoaderDispatchTableCreateInfo m_create_info;
		}; // class WvkLoaderDispatchTable

	} // namespace wvk

} // namespace CGDev

#include "wvk_loader_dispatch_table.hpp"

#endif // CGDEV_WVK_SOURCE__WVK_LOADER_DISPATCH_TABLE_H
