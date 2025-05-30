#ifndef CGDEV_WVK_SOURCE__WVK_QUEUE_FAMILY_H
#define CGDEV_WVK_SOURCE__WVK_QUEUE_FAMILY_H
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
		struct WvkQueueFamilyCreateInfo {
			std::optional<uint32_t> index;
			WvkInstanceDtPtr instance_dt_ptr = nullptr;
			WvkPhysicalDevicePtr wvk_physical_device_ptr = nullptr;
		}; // struct WvkQueueFamilyCreateInfo

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		/*!	\brief
		*/
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		class WvkQueueFamily : public GpuObject {

		public:

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			WvkQueueFamily(void) noexcept;

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			~WvkQueueFamily(void) noexcept;

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*     �������������� ������ WvkQueueFamily, �������� ��������� �������� � �������� ���������.
			*
			* @details
			*     ����� ��������� ���������� ��������� WvkQueueFamilyCreateInfo, ����� �������� ��������� ������� ������.
			*     ���� ��������� �� ��������, ������ ������������ � ���������� ��������� � ������������ ������ � ����� FAIL � ���������� �� ������.
			*     � ������ ������ ������ ���������� ��� �������� � ������������ ������ OK.
			*
			* @param[in] create_info
			*     ��������� � ����������� ��� �������� ��������� �������� (������ ���������, ��������� �� instance dispatch table, ��������� �� ���������� ����������).
			*
			* @return WvkStatus
			*     - VknStatusCode::SUCCESSFUL � ���� ������������� ������ �������.
			*     - VknStatusCode::FAIL � ���� ��������� ������� ������ ����������� ��������.
			*
			* @code
			* WvkQueueFamily queue_family;
			* WvkQueueFamilyCreateInfo create_info = { ... };
			* WvkStatus status = queue_family.create(create_info);
			* if (!status) {
			*     std::cerr << status.getMessage() << std::endl;
			* }
			* @endcode
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			WvkStatus create(const WvkQueueFamilyCreateInfo& create_info) noexcept;

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			void destroy(void) noexcept;					

		public:

			inline const uint32_t getIndexFamily(void) const noexcept;

		public:

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*     ��������� �������� ����������� ��������� �������� ����������� ���������� (VkQueueFamilyProperties).
			*
			* @details
			*     ����� ��������� ��� ���������������� ������� � Vulkan API:
			*     1. �������� ����� ���������� �������� �������� ��� ����������� ����������.
			*     2. ����������� �������� ���� �������� �������� � �������� ������ ��������� �� ������� � �������� ��������.
			*
			* @param[out] vk_queue_family_properties
			*     ���������, � ������� ����� �������� �������� ��������� �������� (�����, ���������� ��������, ��������� �������� � �.�.).
			*
			* @note
			*     ����� ������������, ��� ������ ��������� �������� (`m_create_info.index`) �������.
			*
			* @code
			* VkQueueFamilyProperties props;
			* queue_family.requestQueueFamilyProperties(props);
			* // props ������ ��������, ��������, flags � ���������� ��������
			* @endcode
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			inline void requestProperties(VkQueueFamilyProperties& vk_queue_family_properties) const noexcept;

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief
			*     ����������� ����������� �������� ��������� �������� ���������� � �������������� VkQueueFamilyProperties2 � ������� pNext.
			*
			* @details
			*     ����� ������������ ��� ��������� ����������� ������� ��������� �������� ����������� ���������� ����� ��������� VkQueueFamilyProperties2
			*     � �������������� ��������� (��������, VkQueueFamilyCheckpointPropertiesNV, VkQueueFamilyGlobalPriorityPropertiesEXT � ��.), ���������� ����� pNext.
			*     �������� ������ ��� ������ � ���������� Vulkan 1.1 � ���� ��� ��� ������� ���������� VK_KHR_get_physical_device_properties2.
			*
			*     �������� ������:
			*     - ��� 1. �������� ���������� �������� �������� � ����������� ����������.
			*     - ��� 2. �������� ������� ��� VkQueueFamilyProperties2 � ����������� ��������� Properties.
			*     - ��� 3. �������������� ���� sType � pNext ��� ������ ���������.
			*     - ��� 4. ����������� �������� ���� �������� �������� � ������� vkGetPhysicalDeviceQueueFamilyProperties2.
			*     - ��� 5. �������� �������� ������������� ��������� (�� �������) � �������� �������� out.
			*
			* @tparam Properties
			*     ��������� ����������� ������� ��������� �������� (��������, VkQueueFamilyCheckpointPropertiesNV).
			*
			* @param[out] out
			*     ���������, � ������� ����� �������� ����������� �������� ���������� ��������� ��������.
			*
			* @note
			*     ����� ����� ����� ������������ ������ ��� ������� ��������� Vulkan 1.1+ ��� ���������� VK_KHR_get_physical_device_properties2,
			*     � ���� ��������� Properties ������� ��� ����������.
			*
			* @code
			* VkQueueFamilyCheckpointPropertiesNV checkpointProps{};
			* queue_family.requestProperties(checkpointProps);
			* // checkpointProps ������ �������� ����������� �������� ��������� ��������
			* @endcode
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			template<typename Properties>
			inline void requestProperties(Properties& out) const noexcept;
					
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief 
			*
			* ������� �������, �� ��� �����-�� ������ ���� ��������������� gpt
			* �������� ��� ����������� ��������
			* ��������� ���������, ����� �� ��������� ;))
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			template<typename ... Chains>
			inline void requestQueueFamilyProperties(Chains &... chains) const noexcept;

		private:

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\brief ��������� ���������� ��������� WvkQueueFamilyCreateInfo, ���� �������� ������ � ����������.
			* 
			* @return [out] WvkStatus ������ ��������:
			* - VknStatusCode::SUCCESSFUL � ���� ��� ������������ ���� ������ ���������;
			* - VknStatusCode::CREATE_INFO_NO_VALID � ���� ��������� �������� ��������� ��� ������������� ������.
			* 
			* �������� ����������� ������ ��� �������� ������ � ���������� (`wvk::Build::ValidationBuildInfo::enable == true`).
			* � ��������� ������ ����� ������ ���������� `VknStatusCode::SUCCESSFUL`.
			* 
			* @code
			* WvkQueueFamily queue_family;
			* WvkStatus status = queue_family.validationCreateInfo();
			* if (status.code != VknStatusCode::SUCCESSFUL) {
			*     std::cerr << "������ ���������: " << status.message << std::endl;
			* }
			* @endcode
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			WvkStatus validationCreateInfo(void) const noexcept;

			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			/*!	\
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			void reset(void) noexcept;

		private:
				
			WvkQueueFamilyCreateInfo m_create_info = {};
		}; // class WvkQueueFamily

	} // namespace wvk

} // namespace CGDev

#include "wvk_queue_family.hpp"

#endif // CGDEV_WVK_SOURCE__WVK_QUEUE_FAMILY_H
