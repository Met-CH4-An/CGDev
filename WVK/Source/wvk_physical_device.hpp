#ifndef CGDEV_WVK_SOURCE__WVK_PHYSICAL_DEVICE_HPP
#define CGDEV_WVK_SOURCE__WVK_PHYSICAL_DEVICE_HPP
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

	namespace wvk {

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		template<typename VkPhysicalDeviceXProperties>
		inline void WvkPhysicalDevice::requestPhysicalDeviceProperties(VkPhysicalDeviceXProperties& vk_physical_device_x_properties) const noexcept {
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// ��� 1. �������� ������ Vulkan API �� ����������� ���������� 1.1
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			if constexpr (Build::WvkBuildInfo::vulkan_api_version >= Build::VulkanVersion::VERSION_11) {

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// ��� 2. ������ ������� ��������� VkPhysicalDeviceProperties2
				// � �������������� ���� sType
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				VkPhysicalDeviceProperties2 _vk_physical_device_properties2;
				_vk_physical_device_properties2.sType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROPERTIES_2;

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// ��� 3. ������������� pNext �� ���������� ����������� ���������,
				// ����� ������� Vulkan �������� � �� ����� ������
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				_vk_physical_device_properties2.pNext = &vk_physical_device_x_properties;

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// ��� 4. ����������� �������� ����������� ���������� ����� ������� �����
				// � ����������� �������� pNext
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				requestPhysicalDeviceProperties(_vk_physical_device_properties2);
			}
			else {
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// Vulkan < 1.1: ����� ������ �� ������, ����������� ��������
				// ����������
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
				"\n[WVK Error] ����� ���������� ������� ��� method(object, VkPhysicalDevice, ...)\n");
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
