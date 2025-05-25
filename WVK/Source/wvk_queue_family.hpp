#ifndef CGDEV_SOURCE_GPU_PRIVATE_VULKAN__VKN_QUEUE_FAMILY_HPP
#define CGDEV_SOURCE_GPU_PRIVATE_VULKAN__VKN_QUEUE_FAMILY_HPP
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

namespace CGDev {

	//namespace GPU {

		//namespace Private {

			namespace wvk {

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				inline const uint32_t VknQueueFamily::getIndexFamily(void) const noexcept {
					return m_create_info.index.value();
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				 
				inline const VknQueueFamilyCreateInfo& VknQueueFamily::getCreateInfo(void) const noexcept {
					return m_create_info;
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				inline void VknQueueFamily::requestQueueFamilyProperties(VkQueueFamilyProperties& vk_queue_family_properties) const noexcept {

					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					// ��� 1. �������� ���������� ��������� �������� �������� �� ����������
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					uint32_t _count = 0;
					//m_create_info.wvk_commands->vknGetPhysicalDeviceQueueFamilyProperties(
					//	m_create_info.wvk_physical_device->getVkPhysicalDevice(),
					//	&_count,
					//	nullptr
					//);

					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					// ��� 2. �������� ��������� ������ ��� �������� ������� ���� ��������
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					VkQueueFamilyPropertiesArr1 _vk_queue_family_properties_collection(_count);

					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					// ��� 3. ����������� �������� ���� �������� � ��������� � ������
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					//m_create_info.wvk_commands->vknGetPhysicalDeviceQueueFamilyProperties(
					//	m_create_info.wvk_physical_device->getVkPhysicalDevice(),
					//	&_count,
					//	_vk_queue_family_properties_collection.data()
					//);

					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					// ��� 4. �������� �������� ������������� ��� ��������� �� �������
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					vk_queue_family_properties = _vk_queue_family_properties_collection[m_create_info.index.value()];
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				inline void VknQueueFamily::requestQueueFamilyProperties(VkQueueFamilyProperties2& vk_queue_family_properties2) const noexcept {

					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					// ��������: ���������� ������ ���� ������ ������������ Vulkan 1.1+
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					if constexpr (Build::WvkBuildInfo::vulkan_api_version >= Build::VulkanVersion::VERSION_11) {

						// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						// ��� 1. ����������� ���������� �������� ��������
						// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						uint32_t _count = 0;
						//m_create_info.wvk_commands->vknGetPhysicalDeviceQueueFamilyProperties2(
						//	m_create_info.wvk_physical_device->getVkPhysicalDevice(),
						//	&_count,
						//	nullptr
						//);

						// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						// ��� 2. �������� ������ ��� VkQueueFamilyProperties2
						// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						VkQueueFamilyProperties2Arr1 _vk_queue_family_properties2_collection(_count);

						// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						// ��� 3. �������������� ��������� � ����� sType � pNext
						// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						for (uint32_t i = 0; i < _count; ++i) {
							_vk_queue_family_properties2_collection[i] = {};
							_vk_queue_family_properties2_collection[i].sType = VK_STRUCTURE_TYPE_QUEUE_FAMILY_PROPERTIES_2;
							_vk_queue_family_properties2_collection[i].pNext = nullptr;
						}

						// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						// ��� 4. �������� �������� ���� �������� � ������
						// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						//m_create_info.wvk_commands->vknGetPhysicalDeviceQueueFamilyProperties2(
						//	m_create_info.wvk_physical_device->getVkPhysicalDevice(),
						//	&_count,
						//	_vk_queue_family_properties2_collection.data()
						//);

						// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						// ��� 5. �������� �������� ������� ��������� � �������� ��������
						// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						vk_queue_family_properties2 = _vk_queue_family_properties2_collection[m_create_info.index.value()];
					}
					else {
						// Vulkan API < 1.1 � �������� ����� VkQueueFamilyProperties2 ����������
					}
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				template<typename VkQueueFamilyXProperties>
				inline void VknQueueFamily::requestQueueFamilyProperties(VkQueueFamilyXProperties& vk_queue_family_x_properties) const noexcept {

					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					// ��� 1. ���������, �������������� �� Vulkan 1.1 ��� ����
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					if constexpr (Build::WvkBuildInfo::vulkan_api_version >= Build::VulkanVersion::VERSION_11) {

						// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						// ��� 2. ����������� ���������� �������� ��������
						// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						uint32_t _count = 0;
						//m_create_info.wvk_commands->vknGetPhysicalDeviceQueueFamilyProperties2(
						//	m_create_info.wvk_physical_device->getVkPhysicalDevice(),
						//	&_count,
						//	nullptr
						//);

						// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						// ��� 3. �������� ������� ��� �������� ��������
						// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						VkQueueFamilyProperties2Arr1 _vk_queue_family_properties2_collection(_count);                    // �������� ��������
						std::vector<VkQueueFamilyXProperties> _vk_queue_family_x_properties_collection(_count);         // ����������� ��������

						// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						// ��� 4. �������������� ��������� � ����� sType � pNext
						// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						for (uint32_t i = 0; i < _count; ++i) {
							_vk_queue_family_x_properties_collection[i] = {};
							_vk_queue_family_x_properties_collection[i].sType = vk_queue_family_x_properties.sType;
							_vk_queue_family_x_properties_collection[i].pNext = nullptr;

							_vk_queue_family_properties2_collection[i] = {};
							_vk_queue_family_properties2_collection[i].sType = VK_STRUCTURE_TYPE_QUEUE_FAMILY_PROPERTIES_2;
							_vk_queue_family_properties2_collection[i].pNext = &_vk_queue_family_x_properties_collection[i];
						}

						// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						// ��� 5. �������� �������� ���� �������� � ������
						// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						//m_create_info.wvk_commands->vknGetPhysicalDeviceQueueFamilyProperties2(
						//	m_create_info.wvk_physical_device->getVkPhysicalDevice(),
						//	&_count,
						//	_vk_queue_family_properties2_collection.data()
						//);

						// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						// ��� 6. �������� �������� ������� ��������� � �������� ��������
						// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						vk_queue_family_x_properties = _vk_queue_family_x_properties_collection[m_create_info.index.value()];
					}
					else {
						// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						// ��� 7. Vulkan < 1.1 � ������ �� ������
						// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
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

				void VknQueueFamily::requestQueueFamilyProperties(Chains &... chains) const noexcept {

					// �������� �� ������� ��������� Vulkan 1.1
					if constexpr (Build::WvkBuildInfo::vulkan_api_version >= Build::VulkanVersion::VERSION_11) {

						// �������� ���������� �������� �������� ��� ����������� ����������
						uint32_t _family_count = 0;
						//m_create_info.wvk_commands->vknGetPhysicalDeviceQueueFamilyProperties2(
						//	m_create_info.wvk_physical_device->getVkPhysicalDevice(),
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
						//	m_create_info.wvk_physical_device->getVkPhysicalDevice(),
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

		//} // namespace Private

	//} // namespace GPU

} // namespace CGDev

#endif // CGDEV_SOURCE_GPU_PRIVATE_VULKAN__VKN_QUEUE_FAMILY_HPP
