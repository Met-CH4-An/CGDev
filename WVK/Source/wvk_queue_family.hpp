#ifndef CGDEV_WVK_SOURCE__WVK_QUEUE_FAMILY_HPP
#define CGDEV_WVK_SOURCE__WVK_QUEUE_FAMILY_HPP
////////////////////////////////////////////////////////////////
// ������ �������-����������
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// ������ �������������
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// ������ ������������� ������
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// ������ ��� ����������
////////////////////////////////////////////////////////////////
#include "wvk_instance_dispatch_table.h"
#include "wvk_physical_device.h"
#include "Extensions/wvk_khr_get_physical_device_properties2_dispatch_table.hpp"

namespace CGDev {

	namespace wvk {

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		inline const uint32_t WvkQueueFamily::getIndexFamily(void) const noexcept {
			return m_create_info.index.value();
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		inline void WvkQueueFamily::requestProperties(VkQueueFamilyProperties& vk_queue_family_properties) const noexcept {
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// ��� 1. ����������� ���������� �������� �������� � ����������� ����������
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			uint32_t _count = 0;
			m_create_info.wvk_physical_device_ptr->invokeWithVkPhysicalDeviceMethod(
				&WvkInstanceDt::wvkGetPhysicalDeviceQueueFamilyProperties,
				m_create_info.instance_dt_ptr,
				&_count,
				nullptr
			);

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// ��� 2. �������� ������ ��� �������� ������� ���� �������� ��������
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			std::vector<VkQueueFamilyProperties> _vk_queue_family_properties_collection(_count);

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// ��� 3. ����������� �������� ���� �������� ��������
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			m_create_info.wvk_physical_device_ptr->invokeWithVkPhysicalDeviceMethod(
				&WvkInstanceDt::wvkGetPhysicalDeviceQueueFamilyProperties,
				m_create_info.instance_dt_ptr,
				&_count,
				_vk_queue_family_properties_collection.data()
			);

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// ��� 4. �������� �������� ������������� ��������� �� ������� � �������� ��������
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			vk_queue_family_properties = _vk_queue_family_properties_collection[m_create_info.index.value()];
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		template<typename Properties>
		inline void WvkQueueFamily::requestProperties(Properties& out) const noexcept {
			if constexpr (Build::WvkBuildInfo::vulkan_api_version >= Build::VulkanVersion::VERSION_11 ||
				Build::WvkBuildInfo::find(Extensions::WvkKhrGetPhysicalDeviceProperties2DispatchTable::s_getName())) {

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// ��� 1. ����������� ���������� �������� �������� � ����������� ����������
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				uint32_t _count = 0;
				m_create_info.wvk_physical_device_ptr->invokeWithVkPhysicalDeviceMethod(
					&WvkInstanceDt::wvkGetPhysicalDeviceQueueFamilyProperties2,
					m_create_info.instance_dt_ptr,
					&_count,
					nullptr
				);

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// ��� 2. �������� ������� ��� VkQueueFamilyProperties2 � ����������� ��������� Properties
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				std::vector<VkQueueFamilyProperties2> _props2(_count);
				std::vector<Properties> _base_out(_count);
				for (uint32_t ct_0 = 0; ct_0 < _count; ++ct_0) {
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					// ��� 3. ������������� ��������: sType � pNext
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					_base_out[ct_0] = {};
					_base_out[ct_0].sType = out.sType;
					_base_out[ct_0].pNext = nullptr;

					_props2[ct_0] = {};
					_props2[ct_0].sType = VK_STRUCTURE_TYPE_QUEUE_FAMILY_PROPERTIES_2;
					_props2[ct_0].pNext = &_base_out[ct_0];
				}

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// ��� 4. ����������� �������� ���� �������� ��������
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				m_create_info.wvk_physical_device_ptr->invokeWithVkPhysicalDeviceMethod(
					&WvkInstanceDt::wvkGetPhysicalDeviceQueueFamilyProperties2,
					m_create_info.instance_dt_ptr,
					&_count,
					_props2.data()
				);

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// ��� 5. �������� �������� ������������� ��������� �� �������
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				out = _base_out[m_create_info.index.value()];
			}
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				
		// ���������� � index_sequence
		template <typename... Chains, size_t... Is>
		static void copy_to_chains_impl(size_t id,
			std::index_sequence<Is...>,
			const std::tuple<std::vector<Chains>...>& chains_array,
			Chains&... chains_refs) {
				//constexpr size_t _family_index = 1;
				((chains_refs = std::get<Is>(chains_array)[id]), ...);
		}
		template <typename ... Chains>

		void WvkQueueFamily::requestQueueFamilyProperties(Chains &... chains) const noexcept {

			// �������� �� ������� ��������� Vulkan 1.1
			if constexpr (Build::WvkBuildInfo::vulkan_api_version >= Build::VulkanVersion::VERSION_11) {

				// �������� ���������� �������� �������� ��� ����������� ����������
				uint32_t _family_count = 0;
				//m_create_info.wvk_commands->vknGetPhysicalDeviceQueueFamilyProperties2(
				//	m_create_info.wvk_physical_device_ptr->getVkPhysicalDevice(),
				//	&_family_count,
				//	nullptr
				//);

				constexpr size_t _chains_�ount = sizeof...(Chains);

				// ��� ��� ������ "�������" ��������
				using ChainArray = std::tuple<std::vector<Chains>...>;

				// �������� ������ ��� 6 ����� ������ ���������
				ChainArray chains_array{
					std::vector<Chains>(_family_count)... // ������ ��������� ����������� _family_count ���
				};

				// �������� ������ VkQueueFamilyProperties2
				std::vector<VkQueueFamilyProperties2> properties2_list(_family_count);

				for (uint32_t i = 0; i < _family_count; ++i) {
					// �������������� VkQueueFamilyProperties2
					properties2_list[i] = {};
					properties2_list[i].sType = VK_STRUCTURE_TYPE_QUEUE_FAMILY_PROPERTIES_2;
					properties2_list[i].pNext = nullptr;

					void** pNextPtr = reinterpret_cast<void**>(&properties2_list[i].pNext);

					// ��� ������� ���� �������� � tuple ������� �� � �������� VkQueueFamilyProperties2
					std::apply([&](auto&... vectors) {
						// ������������� �������
						((vectors[i].sType = chains.sType, // �������� sType
						vectors[i].pNext = nullptr,
						*pNextPtr = &vectors[i],
						pNextPtr = reinterpret_cast<void**>(&vectors[i].pNext)), ...);
						}, chains_array);
				}

				//m_create_info.wvk_commands->vknGetPhysicalDeviceQueueFamilyProperties2(
				//	m_create_info.wvk_physical_device_ptr->getVkPhysicalDevice(),
				//	&_family_count,
				//	properties2_list.data()
				//);


				auto& vec = std::get<0>(chains_array);     // ������ std::vector<...>
				auto& elem = vec[1];                       // ������ ������� �������

				// ��������������� ������ ��� ����������� �� tuple � ���������
				auto copy_to_chains = [&](uint32_t id, auto&... chains_refs) {
					copy_to_chains_impl(id, std::make_index_sequence<sizeof...(Chains)>(), chains_array, chains_refs...);
					};

				copy_to_chains(m_create_info.index.value(), chains...);

			}

			else {
				// � ������� Vulkan < 1.1 ����� ������ �� ������
			}
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

	} // namespace wvk

} // namespace CGDev

#endif // CGDEV_WVK_SOURCE__WVK_QUEUE_FAMILY_HPP
