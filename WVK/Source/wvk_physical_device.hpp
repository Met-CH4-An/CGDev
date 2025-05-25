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

namespace CGDev {

	namespace wvk {

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		template<typename VkPhysicalDeviceXProperties>
		inline void WvkPhysicalDevice::requestPhysicalDeviceProperties(VkPhysicalDeviceXProperties& vk_physical_device_x_properties) const noexcept {
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// Шаг 1. Проверка версии Vulkan API на минимальное требование 1.1
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			if constexpr (Build::WvkBuildInfo::vulkan_api_version >= Build::VulkanVersion::VERSION_11) {

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// Шаг 2. Создаём базовую структуру VkPhysicalDeviceProperties2
				// и инициализируем поле sType
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				VkPhysicalDeviceProperties2 _vk_physical_device_properties2;
				_vk_physical_device_properties2.sType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROPERTIES_2;

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// Шаг 3. Устанавливаем pNext на переданную расширенную структуру,
				// чтобы драйвер Vulkan заполнил её во время вызова
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				_vk_physical_device_properties2.pNext = &vk_physical_device_x_properties;

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// Шаг 4. Запрашиваем свойства физического устройства через базовый метод
				// с заполненной цепочкой pNext
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				requestPhysicalDeviceProperties(_vk_physical_device_properties2);
			}
			else {
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// Vulkan < 1.1: метод ничего не делает, расширенные свойства
				// недоступны
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			}
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
		> WvkPhysicalDevice::invokeWithVkPhysicalDeviceMethod(Method&& method, Args&&... args) const noexcept {
			return std::invoke(std::forward<Method>(method),
				m_vk_physical_device,
				std::forward<Args>(args)...);
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

	} // namespace wvk

} // namespace CGDev

#endif // CGDEV_WVK_SOURCE__WVK_PHYSICAL_DEVICE_HPP
