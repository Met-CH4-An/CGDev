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
#include "wvk_commands.h"
#include "wvk_physical_device.h"

namespace CGDev {

	namespace wvk {

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		/*!	\brief
		*/
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		struct WvkQueueFamilyCreateInfo {
			std::optional<uint32_t> index;
			WvkInstanceDtPtr instance_dt_ptr = nullptr;
			WvkPhysicalDevicePtr wvk_physical_device = nullptr;
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
			/*!	\brief �������������� ������ WvkQueueFamily, �������� ���������� ��������� � (�����������) �������� �� ����������.
			* 
			* @param [in] create_info ��������� � ����������� ��� �������� WvkQueueFamily (��������� �� ���������� ����������, ������ ���������, ��������� �� ������� � �.�.).
			* 
			* @return [out] WvkStatus ���������� ������ ��������:
			* - VknStatusCode::SUCCESSFUL � ���� ������������� �������;
			* - VknStatusCode::CREATE_INFO_NO_VALID � ���� �������� ��������� � ������� ������ ���������.
			* 
			* ����� ��������� ������� ��������� `create_info` � ��� �������� ��������� ��������� �������� � ������������.
			* 
			* @code
			* WvkQueueFamily queue_family;
			* WvkQueueFamilyCreateInfo create_info = { ... };
			* WvkStatus status = queue_family.create(create_info);
			* if (status.code != VknStatusCode::SUCCESSFUL) {
			*     std::cerr << "������: " << status.message << std::endl;
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

			inline const WvkQueueFamilyCreateInfo& getCreateInfo(void) const noexcept;

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
			*     �������� ������ ��� ������ � ���������� Vulkan 1.1 � ����.
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
			*     ����� ����� ����� ������������ ������ ��� ������� ��������� Vulkan 1.1+ � ���� ��������� Properties ������� ��� ����������.
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
			/*!	\brief ����������� ����������� �������� ��������� �������� ����������.
			*
			* \tparam VkQueueFamilyXProperties
			* ����������� ��� ���������, ������� ������ ���� ������ � VkQueueFamilyProperties2 ����� pNext.
			* ��������: `VkQueueFamilyCheckpointPropertiesNV`, `VkQueueFamilyGlobalPriorityPropertiesEXT` � �.�.
			*
			* \param [out] vk_queue_family_x_properties
			* ���������, � ������� ����� �������� �������� ���������� ��������� ��������.
			*
			* \details
			* ����� ���������, �������������� �� Vulkan 1.1 ��� ����, � ���� �� � ����������
			* ��������� ������� ��������:
			*
			* - ��� 1. �������� ���������� ��������� �������� �������� ����� `vknGetPhysicalDeviceQueueFamilyProperties2`.
			* - ��� 2. ������ ������ �������� `VkQueueFamilyProperties2` � ��������������� ����������� �������� `VkQueueFamilyXProperties`.
			* - ��� 3. �������������� ���� `sType` � `pNext` ��� ������ ���� ��������.
			* - ��� 4. �������� ����������� �������� ���� ��������, ������ ��� � �����������.
			* - ��� 5. �������� ����������� �������� ��������� �������� �� ������� `m_create_info.index` � �������� ��������.
			*
			* ���� �������� ������ Vulkan ���� 1.1, ����� ������ �� ������.
			*
			* \note
			* ������������� ����� ������ ����� ����� ������ ��� ������� ����������/��������� ��������� `VkQueueFamilyXProperties`,
			* � ���� ��� ������� ��� ������ Vulkan � ����������� ����������� ����������.
			*/
			//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			template<typename VkQueueFamilyXProperties>
			inline void requestQueueFamilyProperties(VkQueueFamilyXProperties& vk_queue_family_x_properties) const noexcept;

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

		private:
				
			WvkQueueFamilyCreateInfo		m_create_info = {};

		}; // class WvkQueueFamily

	} // namespace wvk

} // namespace CGDev

#include "wvk_queue_family.hpp"

#endif // CGDEV_WVK_SOURCE__WVK_QUEUE_FAMILY_H
