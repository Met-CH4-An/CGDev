#ifndef CGDEV_SOURCE_GPU_PRIVATE_VULKAN__VKN_QUEUE_FAMILY_H
#define CGDEV_SOURCE_GPU_PRIVATE_VULKAN__VKN_QUEUE_FAMILY_H
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

	//namespace GPU {

		//namespace Private {

			namespace wvk {

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				struct VknQueueFamilyCreateInfo {

					std::optional<uint32_t>			index;
					WvkCommandsPtr					wvk_commands = nullptr;
					WvkPhysicalDevicePtr			wvk_physical_device = nullptr;

				}; // struct VknQueueFamilyCreateInfo





				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				/*!	\brief
				*/
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				class VknQueueFamily : public GpuObject {

				public:

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					VknQueueFamily(void) noexcept;

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					~VknQueueFamily(void) noexcept;

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief �������������� ������ VknQueueFamily, �������� ���������� ��������� � (�����������) �������� �� ����������.
					* 
					* @param [in] create_info ��������� � ����������� ��� �������� VknQueueFamily (��������� �� ���������� ����������, ������ ���������, ��������� �� ������� � �.�.).
					* 
					* @return [out] WvkStatus ���������� ������ ��������:
					* - VknStatusCode::SUCCESSFUL � ���� ������������� �������;
					* - VknStatusCode::CREATE_INFO_NO_VALID � ���� �������� ��������� � ������� ������ ���������.
					* 
					* ����� ��������� ������� ��������� `create_info` � ��� �������� ��������� ��������� �������� � ������������.
					* 
					* @code
					* VknQueueFamily queue_family;
					* VknQueueFamilyCreateInfo create_info = { ... };
					* WvkStatus status = queue_family.create(create_info);
					* if (status.code != VknStatusCode::SUCCESSFUL) {
					*     std::cerr << "������: " << status.message << std::endl;
					* }
					* @endcode
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					WvkStatus create(const VknQueueFamilyCreateInfo& create_info) noexcept;

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					void destroy(void) noexcept;					

				public:

					inline const VknQueueFamilyCreateInfo& getCreateInfo(void) const noexcept;

					inline const uint32_t getIndexFamily(void) const noexcept;

				public:

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief ��������� �������� ����������� ��������� �������� ����������� ����������.
					* 
					* ������������ ��� ��������� ������������� ��������� ��������, ����� ��� ��������� �������, compute � transfer ��������.
					* ����� ������� �������� ����� ���������� ��������, ����� ��������� ���������� ��� ����, � � ����� ���������� ������ �� ��������� �������.
					* 
					* @param[out] vk_queue_family_properties ���������, � ������� ����� �������� �������� ��������� ��������.
					* 
					* @note ����� ������������, ��� `m_create_info.index` ��� �������.
					*
					* @code
					* VkQueueFamilyProperties props;
					* queue_family.requestQueueFamilyProperties(props);
					* // props ������ ��������, ��������, flags � ���������� ��������
					* @endcode
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					inline void requestQueueFamilyProperties(VkQueueFamilyProperties& vk_queue_family_properties) const noexcept;

					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					/*!	\brief ����������� �������� ���������� ��������� �������� � �������������� ��������� VkQueueFamilyProperties2.
					*
					* ������������ � ������, ���� ���������� ������� � ���������� Vulkan 1.1 ��� ����. ����� ���������
					* �������� ����������� �������� ��������� ��������, ������������ ������� ������� ���������� ����� `pNext`.
					* �� ������ ������ ������� �� ������������ (pNext = nullptr).
					*
					* @param[out] vk_queue_family_properties2 ���������, � ������� ����� �������� �������� ������������� ���������.
					*
					* @note ����� �� ������ ������, ���� ������ ���������� Vulkan ���� ������ 1.1.
					*
					* @code
					* VkQueueFamilyProperties2 props2{};
					* queue_family.requestQueueFamilyProperties(props2);
					* // props2.queueFamilyProperties �������� �����, queueCount � ������
					* @endcode
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					inline void requestQueueFamilyProperties(VkQueueFamilyProperties2& vk_queue_family_properties2) const noexcept;
					
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
					/*!	\brief ��������� ���������� ��������� VknQueueFamilyCreateInfo, ���� �������� ������ � ����������.
					* 
					* @return [out] WvkStatus ������ ��������:
					* - VknStatusCode::SUCCESSFUL � ���� ��� ������������ ���� ������ ���������;
					* - VknStatusCode::CREATE_INFO_NO_VALID � ���� ��������� �������� ��������� ��� ������������� ������.
					* 
					* �������� ����������� ������ ��� �������� ������ � ���������� (`wvk::Build::ValidationBuildInfo::enable == true`).
					* � ��������� ������ ����� ������ ���������� `VknStatusCode::SUCCESSFUL`.
					* 
					* @code
					* VknQueueFamily queue_family;
					* WvkStatus status = queue_family.validationCreateInfo();
					* if (status.code != VknStatusCode::SUCCESSFUL) {
					*     std::cerr << "������ ���������: " << status.message << std::endl;
					* }
					* @endcode
					*/
					//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					WvkStatus validationCreateInfo(void) const noexcept;

				private:
				
					VknQueueFamilyCreateInfo		m_create_info = {};

				}; // class VknQueueFamily

			} // namespace wvk

		//} // namespace Private

	//} // namespace GPU

} // namespace CGDev

#include "wvk_queue_family.hpp"

#endif // CGDEV_SOURCE_GPU_PRIVATE_VULKAN__VKN_QUEUE_FAMILY_H
