// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025 Metelev Met-CH4-An Andrey
// Project Lead: Metelev Andrey
// Main Technical Assistant: StarDev aka ChatGPT
#ifndef CGDEV_WVK_SOURCE__WVK_INSTANCE_HPP
#define CGDEV_WVK_SOURCE__WVK_INSTANCE_HPP
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

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		inline const WvkInstanceCreateInfo& WvkInstance::getCreateInfo(void) const noexcept {
			return m_create_info;
		}

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		inline const WvkPhysicalDeviceUptrVec2& WvkInstance::getWvkPhysicalDevices(void) const noexcept {
			return m_wvk_physical_devices;
		}

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		//template<typename Method, typename Object, typename... Args>
		//inline std::enable_if_t<
		//	std::is_invocable_v<Method, Object, VkInstance, Args...>&&
		//	std::is_void_v<std::invoke_result_t<Method, Object, VkInstance, Args...>>,
		//	void
		//>
		//	WvkInstance::invokeWithVkInstanceMethod(Method&& method, Object&& object, Args&&... args) {
		//	std::invoke(std::forward<Method>(method),
		//		std::forward<Object>(object),
		//		m_vk_instance,
		//		std::forward<Args>(args)...);
		//}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		//template<typename Method, typename Object, typename... Args>
		//inline std::enable_if_t<
		//	std::is_invocable_v<Method, Object, VkInstance, Args...> &&
		//	!std::is_void_v<std::invoke_result_t<Method, Object, VkInstance, Args...>>,
		//	std::invoke_result_t<Method, Object, VkInstance, Args...>
		//>
		//	WvkInstance::invokeWithVkInstanceMethod(Method&& method, Object&& object, Args&&... args) {
		//	return std::invoke(std::forward<Method>(method),
		//		std::forward<Object>(object),
		//		m_vk_instance,
		//		std::forward<Args>(args)...);
		//}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		//template<typename Method, typename Object, typename... Args>
		//inline std::enable_if_t<
		//	!std::is_invocable_v<Method, Object, VkInstance, Args...>,
		//	void
		//>
		//	WvkInstance::invokeWithVkInstanceMethod(Method&&, Object&&, Args&&...) {
		//	static_assert(dependent_false <Method>::value,
		//		"\n[WVK Error] Метод невозможно вызвать как method(object, VkInstance, ...)\n");
		//}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		//template<typename Method, typename... Args>
		//inline std::enable_if_t<
		//	std::is_invocable_v<Method, VkInstance, Args...>&&
		//	std::is_void_v<std::invoke_result_t<Method, VkInstance, Args...>>,
		//	void
		//>
		//	WvkInstance::invokeWithVkInstanceFunction(Method&& method, Args&&... args) {
		//	std::invoke(std::forward<Method>(method),
		//		m_vk_instance,
		//		std::forward<Args>(args)...);
			//(*method)(m_vk_instance, std::forward<Args>(args)...);
		//}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		
		//template<typename Method, typename... Args>
		//inline std::enable_if_t<
		//	std::is_invocable_v<Method, VkInstance, Args...> &&
		//	!std::is_void_v<std::invoke_result_t<Method, VkInstance, Args...>>,
		//	std::invoke_result_t<Method, VkInstance, Args...>
		//>
		//	WvkInstance::invokeWithVkInstanceFunction(Method&& method, Args&&... args) {
		//	return std::invoke(std::forward<Method>(method),
		//		m_vk_instance,
		//		std::forward<Args>(args)...);
		//}

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		inline const WvkDispatchTableUptr& WvkInstance::getWvkDispatchTable(void) const noexcept {
			return m_wvk_instance_dispatch_table_ptr;
		}

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		inline const VkInstance& WvkInstance::getVkInstance(void) const noexcept {
			return m_vk_instance;
		}

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

	} // namespace wvk

} // namespace CGDev

#endif // CGDEV_WVK_SOURCE__WVK_INSTANCE_HPP
