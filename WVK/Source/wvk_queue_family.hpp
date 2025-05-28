#ifndef CGDEV_WVK_SOURCE__WVK_QUEUE_FAMILY_HPP
#define CGDEV_WVK_SOURCE__WVK_QUEUE_FAMILY_HPP
////////////////////////////////////////////////////////////////
// секция форвард-декларации
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// секция имплементации
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// секция родительского класса
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
// секция для остального
////////////////////////////////////////////////////////////////
#include "wvk_instance_dt.hpp"
#include "Extensions/wvk_khr_get_physical_device_properties2_dt.hpp"

namespace CGDev {

	//namespace GPU {

		//namespace Private {

			namespace wvk {

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				inline const uint32_t WvkQueueFamily::getIndexFamily(void) const noexcept {
					return m_create_info.index.value();
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				 
				inline const WvkQueueFamilyCreateInfo& WvkQueueFamily::getCreateInfo(void) const noexcept {
					return m_create_info;
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				inline void WvkQueueFamily::requestProperties(VkQueueFamilyProperties& vk_queue_family_properties) const noexcept {
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					// Шаг 1. Запрашиваем количество семейств очередей у физического устройства
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					uint32_t _count = 0;
					m_create_info.wvk_physical_device->invokeWithVkPhysicalDeviceMethod(
						&WvkInstanceDt::wvkGetPhysicalDeviceQueueFamilyProperties,
						m_create_info.instance_dt_ptr,
						&_count,
						nullptr
					);

					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					// Шаг 2. Выделяем вектор для хранения свойств всех семейств очередей
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					std::vector<VkQueueFamilyProperties> _vk_queue_family_properties_collection(_count);

					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					// Шаг 3. Запрашиваем свойства всех семейств очередей
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					m_create_info.wvk_physical_device->invokeWithVkPhysicalDeviceMethod(
						&WvkInstanceDt::wvkGetPhysicalDeviceQueueFamilyProperties,
						m_create_info.instance_dt_ptr,
						&_count,
						_vk_queue_family_properties_collection.data()
					);

					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					// Шаг 4. Копируем свойства интересующего семейства по индексу в выходной параметр
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					vk_queue_family_properties = _vk_queue_family_properties_collection[m_create_info.index.value()];
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				template<typename Properties>
				inline void WvkQueueFamily::requestProperties(Properties& out) const noexcept {
					if constexpr (Build::WvkBuildInfo::vulkan_api_version >= Build::VulkanVersion::VERSION_11) {

						// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						// Шаг 1. Запрашиваем количество семейств очередей у физического устройства
						// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						uint32_t _count = 0;
						m_create_info.wvk_physical_device->invokeWithVkPhysicalDeviceMethod(
							&WvkInstanceDt::wvkGetPhysicalDeviceQueueFamilyProperties2,
							m_create_info.instance_dt_ptr,
							&_count,
							nullptr
						);

						// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						// Шаг 2. Выделяем массивы для VkQueueFamilyProperties2 и расширенной структуры Properties
						// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						std::vector<VkQueueFamilyProperties2> _props2(_count);
						std::vector<Properties> _base_out(_count);
						for (uint32_t ct_0 = 0; ct_0 < _count; ++ct_0) {
							// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
							// Шаг 3. Инициализация структур: sType и pNext
							// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
							_base_out[ct_0] = {};
							_base_out[ct_0].sType = out.sType;
							_base_out[ct_0].pNext = nullptr;

							_props2[ct_0] = {};
							_props2[ct_0].sType = VK_STRUCTURE_TYPE_QUEUE_FAMILY_PROPERTIES_2;
							_props2[ct_0].pNext = &_base_out[ct_0];
						}

						// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						// Шаг 4. Запрашиваем свойства всех семейств очередей
						// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						m_create_info.wvk_physical_device->invokeWithVkPhysicalDeviceMethod(
							&WvkInstanceDt::wvkGetPhysicalDeviceQueueFamilyProperties2,
							m_create_info.instance_dt_ptr,
							&_count,
							_props2.data()
						);

						// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						// Шаг 5. Копируем свойства интересующего семейства по индексу
						// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						out = _base_out[m_create_info.index.value()];
					}
					else if constexpr (Build::WvkBuildInfo::find(Extensions::WvkKhrGetPhysicalDeviceProperties2DT::s_getName())) {
						//m_create_info.wvk_physical_device->invokeWithVkPhysicalDeviceMethod(
						//	&Extensions::WvkKhrGetPhysicalDeviceProperties2DT::wvkGetPhysicalDeviceQueueFamilyProperties2KHR,
						//	m_create_info.instance_dt_ptr,
						//	&_count,
						//	_props2.data()
						//);
						
						
					}
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				template<typename VkQueueFamilyXProperties>
				inline void WvkQueueFamily::requestQueueFamilyProperties(VkQueueFamilyXProperties& vk_queue_family_x_properties) const noexcept {

					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					// Шаг 1. Проверяем, поддерживается ли Vulkan 1.1 или выше
					// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					if constexpr (Build::WvkBuildInfo::vulkan_api_version >= Build::VulkanVersion::VERSION_11) {

						// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						// Шаг 2. Запрашиваем количество семейств очередей
						// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						uint32_t _count = 0;
						//m_create_info.wvk_commands->vknGetPhysicalDeviceQueueFamilyProperties2(
						//	m_create_info.wvk_physical_device->getVkPhysicalDevice(),
						//	&_count,
						//	nullptr
						//);

						// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						// Шаг 3. Выделяем массивы под свойства семейств
						// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						VkQueueFamilyProperties2Arr1 _vk_queue_family_properties2_collection(_count);                    // Основные свойства
						std::vector<VkQueueFamilyXProperties> _vk_queue_family_x_properties_collection(_count);         // Расширенные свойства

						// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						// Шаг 4. Инициализируем структуры — задаём sType и pNext
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
						// Шаг 5. Получаем свойства всех семейств в массив
						// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						//m_create_info.wvk_commands->vknGetPhysicalDeviceQueueFamilyProperties2(
						//	m_create_info.wvk_physical_device->getVkPhysicalDevice(),
						//	&_count,
						//	_vk_queue_family_properties2_collection.data()
						//);

						// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						// Шаг 6. Копируем свойства нужного семейства в выходной параметр
						// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						vk_queue_family_x_properties = _vk_queue_family_x_properties_collection[m_create_info.index.value()];
					}
					else {
						// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
						// Шаг 7. Vulkan < 1.1 — ничего не делаем
						// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
					}
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				
				// Реализация с index_sequence
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

					// Проверка на наличие поддержки Vulkan 1.1
					if constexpr (Build::WvkBuildInfo::vulkan_api_version >= Build::VulkanVersion::VERSION_11) {

						// Получаем количество семейств очередей для физического устройства
						uint32_t _family_count = 0;
						//m_create_info.wvk_commands->vknGetPhysicalDeviceQueueFamilyProperties2(
						//	m_create_info.wvk_physical_device->getVkPhysicalDevice(),
						//	&_family_count,
						//	nullptr
						//);

						constexpr size_t _chains_сount = sizeof...(Chains);

						// Тип для каждой "цепочки" структур
						using ChainArray = std::tuple<std::vector<Chains>...>;

						// Выделяем память под 6 копий каждой структуры
						ChainArray chains_array{
							std::vector<Chains>(_family_count)... // Каждая структура дублируется _family_count раз
						};

						// Основной массив VkQueueFamilyProperties2
						std::vector<VkQueueFamilyProperties2> properties2_list(_family_count);

						for (uint32_t i = 0; i < _family_count; ++i) {
							// Инициализируем VkQueueFamilyProperties2
							properties2_list[i] = {};
							properties2_list[i].sType = VK_STRUCTURE_TYPE_QUEUE_FAMILY_PROPERTIES_2;
							properties2_list[i].pNext = nullptr;

							void** pNextPtr = reinterpret_cast<void**>(&properties2_list[i].pNext);

							// Для каждого типа структур в tuple цепляем их к текущему VkQueueFamilyProperties2
							std::apply([&](auto&... vectors) {
								// Распаковываем вектора
								((vectors[i].sType = chains.sType, // копируем sType
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


						auto& vec = std::get<0>(chains_array);     // второй std::vector<...>
						auto& elem = vec[1];                       // второй элемент вектора

						// Вспомогательная лямбда для копирования из tuple в параметры
						auto copy_to_chains = [&](uint32_t id, auto&... chains_refs) {
							copy_to_chains_impl(id, std::make_index_sequence<sizeof...(Chains)>(), chains_array, chains_refs...);
							};

						copy_to_chains(m_create_info.index.value(), chains...);

					}

					else {
						// В версиях Vulkan < 1.1 метод ничего не делает
					}
				}

				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

			} // namespace wvk

		//} // namespace Private

	//} // namespace GPU

} // namespace CGDev

#endif // CGDEV_WVK_SOURCE__WVK_QUEUE_FAMILY_HPP
