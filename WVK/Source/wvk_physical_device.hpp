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
			// ��� 1. ������ ������ �������
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			WvkStatus _status;

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// ��� 2. ��������� ��������� Vulkan 1.1 ��� ����������
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			if constexpr (
				Build::WvkBuildInfo::vulkan_api_version >= Build::VulkanVersion::VERSION_11 ||
				Build::WvkBuildInfo::find("VK_KHR_get_physical_device_properties2")) {

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// ��� 3. �������������� �������� ��������� �������
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				VkPhysicalDeviceProperties2 _props2 = {};
				_props2.sType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROPERTIES_2;

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// ��� 4. ���������� ������� ������� ����������� �������
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				_props2.pNext = out;

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// ��� 5. ��������� ������ ����� dispatch table
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				m_create_info.wvk_instance_dispatch_table->wvkGetPhysicalDeviceProperties2(m_vk_physical_device, &_props2);

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// ��� 6. ��������� �������
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				return _status.setOk();
			}

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// ��� 7. ���������������� ���������� � ���������� ������
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
			// ��� 1. �������� ������� ������ � VkBaseOutStructure*
			// ��� ���������, ��� ��� Out ������ ���� ����������� �� VkBaseOutStructure
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			return requestProperties(reinterpret_cast<VkBaseOutStructure*>(&out));
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		inline WvkStatus WvkPhysicalDevice::requestQueueFamilyProperties(VkQueueFamilyPropertiesVec1& queue_family_properties_collection) const noexcept {
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// ��� 1. ������ ������ �������
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			WvkStatus _status;

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// ��� 2. �������� ���������� �������� ��������
			// ������ ����� ������� nullptr, ����� ������ ������ ����������
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			uint32_t _count = 0;
			m_create_info.wvk_instance_dispatch_table->wvkGetPhysicalDeviceQueueFamilyProperties(
				m_vk_physical_device,
				&_count,
				nullptr
			);

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// ��� 3. �������� ����� � ������� ��� ��� ��������
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			queue_family_properties_collection.resize(_count);

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// ��� 4. �������� �������� �������, ����� �������� ���� ��������
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			m_create_info.wvk_instance_dispatch_table->wvkGetPhysicalDeviceQueueFamilyProperties(
				m_vk_physical_device,
				&_count,
				queue_family_properties_collection.data()
			);

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// ��� 5. ���������� ������ "�������"
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			return _status.setOk();
		}

		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

		template<typename Out>
		inline WvkStatus WvkPhysicalDevice::requestQueueFamilyProperties(std::vector<Out>& out, const VkStructureType& vk_structure_type) const noexcept {
			// ��������� ������ ����������
			WvkStatus _status;

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// ��� 1. �������� ����������� Vulkan 1.1 ��� ���������� VK_KHR_get_physical_device_properties2
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			if constexpr (
				Build::WvkBuildInfo::vulkan_api_version >= Build::VulkanVersion::VERSION_11 ||
				Build::WvkBuildInfo::find("VK_KHR_get_physical_device_properties2")) {

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// ��� 2. ����������� ���������� ��������� �������� ��������
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				uint32_t _count = 0;
				m_create_info.wvk_instance_dispatch_table->wvkGetPhysicalDeviceQueueFamilyProperties2(
					m_vk_physical_device,
					&_count,
					nullptr
				);

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// ��� 3. �������� ��������� ����� ��� VkQueueFamilyProperties2
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				VkQueueFamilyProperties2Vec1 _props2_collection(_count);

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// ��� 4. �������� �������� ������ � �������������� ������� pNext
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				out.resize(_count);
				for (uint32_t ct_0 = 0; ct_0 < _count; ++ct_0) {
					_props2_collection[ct_0] = {};
					_props2_collection[ct_0].sType = VK_STRUCTURE_TYPE_QUEUE_FAMILY_PROPERTIES_2;
					_props2_collection[ct_0].pNext = &out[ct_0]; // ������� ����������

					out[ct_0].sType = vk_structure_type;
					out[ct_0].pNext = nullptr; // ����� ����� ���������, ���� �����
				}

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// ��� 5. �������� �������� �������� � �����������
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				m_create_info.wvk_instance_dispatch_table->wvkGetPhysicalDeviceQueueFamilyProperties2(
					m_vk_physical_device,
					&_count,
					_props2_collection.data()
				);

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// ��� 6. ���������� �������� ������
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				return _status.setOk();
			}

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// ��� 7. ���������� ������, ���� ���������������� �� ��������
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
			// ��� 1. ��������: �������������� �� Vulkan 1.1 ��� ���������� VK_KHR_device_group_creation
			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			if constexpr (Build::WvkBuildInfo::vulkan_api_version >= Build::VulkanVersion::VERSION_11 ||
				Build::WvkBuildInfo::find("VK_KHR_device_group_creation")) {

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// ��� 2. �������� ���������� ��������� ���������� �����
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				uint32_t _count = 0;

				VkResult _vk_res = m_create_info.wvk_instance_dispatch_table->wvkEnumeratePhysicalDeviceGroups(&_count, nullptr);

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// ��� 3. ��������� ������ ������� ������
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
				// ��� 4. �������� ������ � �������� �������� �����
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				std::vector<VkPhysicalDeviceGroupProperties> _vk_physical_device_group_properties_collection(_count);

				_vk_res = m_create_info.wvk_instance_dispatch_table->wvkEnumeratePhysicalDeviceGroups(&_count, _vk_physical_device_group_properties_collection.data());

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// ��� 5. ��������� ������ ������� ������
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
				// ��� 6. ��������������� �������: ���������� ������ � ��������� ���������
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				auto makeDeviceSet = [](const VkPhysicalDeviceGroupProperties& group) {
					return std::unordered_set<VkPhysicalDevice>(
						group.physicalDevices,
						group.physicalDevices + group.physicalDeviceCount
					);
					};

				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				// ��� 7. ������ ��������� ������ ���������� �� ���������� + ��������� �������
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
				// ��� 8. ��������: ��������� �� ��� ���������� � ����� ������
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
				// ��� 9. ���� �� ���� ������ �� �������� ��� ���������� � ������������
				// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
				compatibility = false;
				return _status.setOk();
			}

			// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
			// ��� 10. ��������� ���������� ����� �� ��������
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
