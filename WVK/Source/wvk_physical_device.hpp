#ifndef CGDEV_WVK_SOURCE__WVK_PHYSICAL_DEVICE_HPP
#define CGDEV_WVK_SOURCE__WVK_PHYSICAL_DEVICE_HPP
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
#include "wvk_instance_dispatch_table.h"

namespace CGDev {

	namespace wvk {

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		inline void WvkPhysicalDevice::requestProperties(VkPhysicalDeviceProperties& vk_physical_device_properties) const noexcept {
			m_create_info.wvk_instance_dispatch_table->wvkGetPhysicalDeviceProperties(m_vk_physical_device, &vk_physical_device_properties);
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		inline WvkStatus WvkPhysicalDevice::requestProperties(VkBaseOutStructure* out) const noexcept {
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Шаг 1. Создаём объект статуса
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			WvkStatus _status;

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Шаг 2. Проверяем поддержку Vulkan 1.1 или расширения
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			if constexpr (
				Build::WvkBuildInfo::vulkan_api_version >= Build::VulkanVersion::VERSION_11 ||
				Build::WvkBuildInfo::find("VK_KHR_get_physical_device_properties2")) {

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// Шаг 3. Подготавливаем основную структуру запроса
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				VkPhysicalDeviceProperties2 _props2 = {};
				_props2.sType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROPERTIES_2;

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// Шаг 4. Подключаем внешнюю цепочку расширенных свойств
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				_props2.pNext = out;

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// Шаг 5. Выполняем запрос через dispatch table
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				m_create_info.wvk_instance_dispatch_table->wvkGetPhysicalDeviceProperties2(m_vk_physical_device, &_props2);

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// Шаг 6. Завершаем успешно
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				return _status.setOk();
			}

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Шаг 7. Функциональность недоступна — возвращаем ошибку
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			return _status.set(
				VknStatusCode::FEATURE_NOT_ENABLED,
				"\nVulkan 1.1 or VK_KHR_get_physical_device_properties2 is not enabled."
			);
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		template<typename Out>
		inline WvkStatus WvkPhysicalDevice::requestProperties(Out& out) const noexcept {
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Шаг 1. Приводим входной объект к VkBaseOutStructure*
			// Это безопасно, так как Out должен быть производным от VkBaseOutStructure
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			return requestProperties(reinterpret_cast<VkBaseOutStructure*>(&out));
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		inline WvkStatus WvkPhysicalDevice::requestQueueFamilyProperties(VkQueueFamilyPropertiesVec1& queue_family_properties_collection) const noexcept {
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Шаг 1. Создаём объект статуса
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			WvkStatus _status;

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Шаг 2. Получаем количество семейств очередей
			// Первый вызов передаёт nullptr, чтобы узнать только количество
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			uint32_t _count = 0;
			m_create_info.wvk_instance_dispatch_table->wvkGetPhysicalDeviceQueueFamilyProperties(
				m_vk_physical_device,
				&_count,
				nullptr
			);

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Шаг 3. Выделяем место в векторе под все свойства
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			queue_family_properties_collection.resize(_count);

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Шаг 4. Повторно вызываем функцию, чтобы получить сами свойства
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			m_create_info.wvk_instance_dispatch_table->wvkGetPhysicalDeviceQueueFamilyProperties(
				m_vk_physical_device,
				&_count,
				queue_family_properties_collection.data()
			);

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Шаг 5. Возвращаем статус "успешно"
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			return _status.setOk();
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		template<typename Out>
		inline WvkStatus WvkPhysicalDevice::requestQueueFamilyProperties(std::vector<Out>& out, const VkStructureType& vk_structure_type) const noexcept {
			// Объявляем статус выполнения
			WvkStatus _status;

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Шаг 1. Проверка доступности Vulkan 1.1 или расширения VK_KHR_get_physical_device_properties2
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			if constexpr (
				Build::WvkBuildInfo::vulkan_api_version >= Build::VulkanVersion::VERSION_11 ||
				Build::WvkBuildInfo::find("VK_KHR_get_physical_device_properties2")) {

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// Шаг 2. Запрашиваем количество доступных семейств очередей
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				uint32_t _count = 0;
				m_create_info.wvk_instance_dispatch_table->wvkGetPhysicalDeviceQueueFamilyProperties2(
					m_vk_physical_device,
					&_count,
					nullptr
				);

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// Шаг 3. Выделяем временный буфер для VkQueueFamilyProperties2
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				VkQueueFamilyProperties2Vec1 _props2_collection(_count);

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// Шаг 4. Ресайзим выходной вектор и инициализируем цепочки pNext
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				out.resize(_count);
				for (uint32_t ct_0 = 0; ct_0 < _count; ++ct_0) {
					_props2_collection[ct_0] = {};
					_props2_collection[ct_0].sType = VK_STRUCTURE_TYPE_QUEUE_FAMILY_PROPERTIES_2;
					_props2_collection[ct_0].pNext = &out[ct_0]; // цепляем расширение

					out[ct_0].sType = vk_structure_type;
					out[ct_0].pNext = nullptr; // можно позже дополнять, если нужно
				}

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// Шаг 5. Получаем свойства очередей с расширением
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				m_create_info.wvk_instance_dispatch_table->wvkGetPhysicalDeviceQueueFamilyProperties2(
					m_vk_physical_device,
					&_count,
					_props2_collection.data()
				);

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// Шаг 6. Возвращаем успешный статус
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				return _status.setOk();
			}

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Шаг 7. Возвращаем ошибку, если функциональность не доступна
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			return _status.set(
				VknStatusCode::FEATURE_NOT_ENABLED,
				"\nVulkan 1.1 or VK_KHR_get_physical_device_properties2 is not enabled."
			);
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//VkResult(WvkInstanceDt:: * method_ptr)(uint32_t*, VkPhysicalDeviceGroupProperties*) const noexcept = nullptr;
				//if constexpr (Build::WvkBuildInfo::vulkan_api_version >= Build::VulkanVersion::VERSION_11) {
				//	method_ptr = &WvkInstanceDt::wvkEnumeratePhysicalDeviceGroups;
				//}
				//else if constexpr (Build::WvkBuildInfo::find("VK_KHR_device_group_creation")) {
				//	method_ptr = &WvkInstanceDt::wvkEnumeratePhysicalDeviceGroupsKHR;
				//}
				//auto* obj = m_create_info.wvk_instance_dispatch_table;
				//VkResult _vk_res = (obj->*method_ptr)(&_count, nullptr);
		inline WvkStatus WvkPhysicalDevice::checkCompatibility(const WvkPhysicalDevicePtrArr1& wvk_physical_device_collection, bool& compatibility) const noexcept {
			WvkStatus _status;

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Шаг 1. Проверка: поддерживается ли Vulkan 1.1 или расширение VK_KHR_device_group_creation
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			if constexpr (Build::WvkBuildInfo::vulkan_api_version >= Build::VulkanVersion::VERSION_11 ||
				Build::WvkBuildInfo::find("VK_KHR_device_group_creation")) {

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// Шаг 2. Получаем количество доступных физических групп
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				uint32_t _count = 0;

				VkResult _vk_res = m_create_info.wvk_instance_dispatch_table->wvkEnumeratePhysicalDeviceGroups(&_count, nullptr);

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// Шаг 3. Обработка ошибок первого вызова
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				if (_vk_res != VK_SUCCESS) {
					switch (_vk_res) {
					case VK_ERROR_OUT_OF_HOST_MEMORY:
						_status.append("\n\tWvkInstanceDispatchTable::wvkEnumeratePhysicalDeviceGroups - VK_ERROR_OUT_OF_HOST_MEMORY.");
						break;
					case VK_ERROR_OUT_OF_DEVICE_MEMORY:
						_status.append("\n\tWvkInstanceDispatchTable::wvkEnumeratePhysicalDeviceGroups - VK_ERROR_OUT_OF_DEVICE_MEMORY.");
						break;
					case VK_ERROR_INITIALIZATION_FAILED:
						_status.append("\n\tWvkInstanceDispatchTable::wvkEnumeratePhysicalDeviceGroups - VK_ERROR_INITIALIZATION_FAILED.");
						break;
					}
					return _status.set(VknStatusCode::FAIL);
				}

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// Шаг 4. Выделяем память и получаем свойства групп
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				std::vector<VkPhysicalDeviceGroupProperties> _vk_physical_device_group_properties_collection(_count);

				_vk_res = m_create_info.wvk_instance_dispatch_table->wvkEnumeratePhysicalDeviceGroups(&_count, _vk_physical_device_group_properties_collection.data());

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// Шаг 5. Обработка ошибок второго вызова
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				if (_vk_res != VK_SUCCESS) {
					switch (_vk_res) {
					case VK_ERROR_OUT_OF_HOST_MEMORY:
						_status.append("\n\tWvkInstanceDispatchTable::wvkEnumeratePhysicalDeviceGroups - VK_ERROR_OUT_OF_HOST_MEMORY.");
						break;
					case VK_ERROR_OUT_OF_DEVICE_MEMORY:
						_status.append("\n\tWvkInstanceDispatchTable::wvkEnumeratePhysicalDeviceGroups - VK_ERROR_OUT_OF_DEVICE_MEMORY.");
						break;
					case VK_ERROR_INITIALIZATION_FAILED:
						_status.append("\n\tWvkInstanceDispatchTable::wvkEnumeratePhysicalDeviceGroups - VK_ERROR_INITIALIZATION_FAILED.");
						break;
					case VK_INCOMPLETE:
						_status.append("\n\tWvkInstanceDispatchTable::wvkEnumeratePhysicalDeviceGroups - VK_INCOMPLETE.");
						break;
					}
					return _status.set(VknStatusCode::FAIL);
				}

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// Шаг 6. Вспомогательная функция: превращаем группу в множество устройств
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				auto makeDeviceSet = [](const VkPhysicalDeviceGroupProperties& group) {
					return std::unordered_set<VkPhysicalDevice>(
						group.physicalDevices,
						group.physicalDevices + group.physicalDeviceCount
					);
					};

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// Шаг 7. Создаём временный список указателей на устройства + добавляем текущее
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				std::vector<const WvkPhysicalDevice*> _wvk_physical_device_collection_temp;
				std::transform(
					wvk_physical_device_collection.begin(),
					wvk_physical_device_collection.end(),
					std::back_inserter(_wvk_physical_device_collection_temp),
					[](WvkPhysicalDevice* ptr) {
						return static_cast<const WvkPhysicalDevice*>(ptr);
					}
				);
				_wvk_physical_device_collection_temp.push_back(this);

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// Шаг 8. Проверка: находятся ли все устройства в одной группе
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				for (const auto& group : _vk_physical_device_group_properties_collection) {
					std::unordered_set<VkPhysicalDevice> device_set = makeDeviceSet(group);

					bool all_found = std::all_of(
						_wvk_physical_device_collection_temp.begin(),
						_wvk_physical_device_collection_temp.end(),
						[&](const WvkPhysicalDevice* wvk_phys_dev) {
							return device_set.contains(wvk_phys_dev->m_vk_physical_device);
						}
					);

					if (all_found) {
						compatibility = true;
						return _status.setOk();
					}
				}

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// Шаг 9. Если ни одна группа не содержит все устройства — несовместимо
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				compatibility = false;
				return _status.setOk();
			}

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Шаг 10. Поддержка устройства групп не включена
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			return _status.set(
				VknStatusCode::FEATURE_NOT_ENABLED,
				"\nVulkan 1.1 or VK_KHR_device_group_creation is not enabled."
			);
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		
		template<typename Method, typename Object, typename... Args>
		inline std::enable_if_t<
			std::is_invocable_v<Method, Object, VkPhysicalDevice, Args...>&&
			std::is_void_v<std::invoke_result_t<Method, Object, VkPhysicalDevice, Args...>>,
			void
		>
			WvkPhysicalDevice::invokeWithVkPhysicalDeviceMethod(Method&& method, Object&& object, Args&&... args) const noexcept {
			std::invoke(std::forward<Method>(method),
				std::forward<Object>(object),
				m_vk_physical_device,
				std::forward<Args>(args)...);
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		template<typename Method, typename Object, typename... Args>
		inline std::enable_if_t<
			std::is_invocable_v<Method, Object, VkPhysicalDevice, Args...> &&
			!std::is_void_v<std::invoke_result_t<Method, Object, VkPhysicalDevice, Args...>>,
			std::invoke_result_t<Method, Object, VkPhysicalDevice, Args...>
		> 
			WvkPhysicalDevice::invokeWithVkPhysicalDeviceMethod(Method&& method, Object&& object, Args&&... args) const noexcept {
			return std::invoke(std::forward<Method>(method),
				std::forward<Object>(object),
				m_vk_physical_device,
				std::forward<Args>(args)...);
		}

		template<typename Method, typename Object, typename... Args>
		inline std::enable_if_t<
			!std::is_invocable_v<Method, Object, VkPhysicalDevice, Args...>,
			void
		>
			WvkPhysicalDevice::invokeWithVkPhysicalDeviceMethod(Method&&, Object&&, Args&&...) const noexcept {
			static_assert(dependent_false <Method>::value,
				"\n[WVK Error] Метод невозможно вызвать как method(object, VkPhysicalDevice, ...)\n");
		}

		template<typename Method, typename... Args>
		inline std::enable_if_t<
			std::is_invocable_v<Method, VkPhysicalDevice, Args...>&&
			std::is_void_v<std::invoke_result_t<Method, VkPhysicalDevice, Args...>>,
			void
		>
			WvkPhysicalDevice::invokeWithVkPhysicalDeviceFunction(Method&& method, Args&&... args) const noexcept {
			std::invoke(std::forward<Method>(method),
				m_vk_physical_device,
				std::forward<Args>(args)...);
		}

		template<typename Method, typename... Args>

		inline std::enable_if_t<
			std::is_invocable_v<Method, VkPhysicalDevice, Args...> &&
			!std::is_void_v<std::invoke_result_t<Method, VkPhysicalDevice, Args...>>,
			std::invoke_result_t<Method, VkPhysicalDevice, Args...>
		> WvkPhysicalDevice::invokeWithVkPhysicalDeviceFunction(Method&& method, Args&&... args) const noexcept {
			return std::invoke(std::forward<Method>(method),
				m_vk_physical_device,
				std::forward<Args>(args)...);
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

	} // namespace wvk

} // namespace CGDev

#endif // CGDEV_WVK_SOURCE__WVK_PHYSICAL_DEVICE_HPP
